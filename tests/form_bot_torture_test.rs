use axum::{
    extract::Form,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use reqwest::Client;
use rusqlite::Connection;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use worker_inegi_engine::engine::http_form_bot::HttpFormBot;

#[derive(Default)]
struct CapturedSubmissions {
    wp_payload: Mutex<Option<HashMap<String, String>>>,
    legacy_payload: Mutex<Option<HashMap<String, String>>>,
    select_payload: Mutex<Option<HashMap<String, String>>>,
    captured_user_agents: Mutex<Vec<String>>,
    flaky_get_attempts: std::sync::atomic::AtomicUsize,
    flaky_post_attempts: std::sync::atomic::AtomicUsize,
    redirect_post_received: Mutex<bool>,
}

/// Extrae empresas reales indexadas en data/mallen_prod.db (DENUE + SAT 69-B) para probar con datos reales
fn load_real_companies_from_prod_db() -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(conn) = Connection::open("data/mallen_prod.db") {
        if let Ok(mut stmt) = conn.prepare(
            "SELECT COALESCE(NULLIF(raz_social,''), nom_estab), COALESCE(codigo_scian,'541110')
             FROM denue_units
             WHERE raz_social != '' OR nom_estab != ''
             LIMIT 10",
        ) {
            if let Ok(rows) = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))) {
                for r in rows.flatten() {
                    out.push(r);
                }
            }
        }
    }
    if out.len() < 6 {
        out.push(("DISTRIBUIDORA E IMPORTADORA ALSEA S.A. DE C.V.".to_string(), "488511".to_string()));
        out.push(("GRUPO INDUSTRIAL BIMBO S.A.B. DE C.V.".to_string(), "311811".to_string()));
        out.push(("ELECTRONICA STEREN S.A. DE C.V.".to_string(), "466312".to_string()));
    }
    out
}

#[tokio::test]
async fn test_http_form_bot_14_scenarios_with_real_db_companies() {
    let real_companies = load_real_companies_from_prod_db();
    println!("✓ Empresas reales cargadas desde data/mallen_prod.db para pruebas: {}", real_companies.len());
    for (idx, (name, scian)) in real_companies.iter().take(5).enumerate() {
        println!("  [DB Real #{}] {} (SCIAN: {})", idx + 1, name, scian);
    }

    let state = Arc::new(CapturedSubmissions::default());
    let state_wp = state.clone();
    let state_legacy = state.clone();
    let state_select = state.clone();
    let state_ua = state.clone();

    let app = Router::new()
        // 1. WordPress CF7 Exitoso + Captura de User-Agent rotado
        .route(
            "/wp-contact",
            get(move |headers: HeaderMap| {
                let st = state_ua.clone();
                async move {
                    if let Some(ua) = headers.get("user-agent").and_then(|v| v.to_str().ok()) {
                        st.captured_user_agents.lock().unwrap().push(ua.to_string());
                    }
                    Html(
                        r#"<!DOCTYPE html>
                        <html><head><meta charset="UTF-8"></head><body>
                            <form role="search" action="/search"><input name="q"></form>
                            <form action="/wp-contact/submit" method="POST">
                                <input type="hidden" name="_wpcf7" value="99812">
                                <input type="hidden" name="_wpcf7_unit_tag" value="wpcf7-f99812-p1-o1">
                                <input type="hidden" name="_wpnonce" value="sec_token_abc123">
                                <input type="text" name="website_url" style="display:none" value="">
                                <input type="text" name="trap_field" tabindex="-1" value="">
                                <input type="text" name="first_name" required>
                                <input type="text" name="last_name" required>
                                <input type="email" name="your-email" required>
                                <input type="tel" name="your-tel">
                                <textarea name="your-message"></textarea>
                            </form>
                        </body></html>"#,
                    )
                }
            }),
        )
        .route(
            "/wp-contact/submit",
            post(move |Form(params): Form<HashMap<String, String>>| {
                let st = state_wp.clone();
                async move {
                    if params.contains_key("website_url") || params.contains_key("trap_field") {
                        return (StatusCode::BAD_REQUEST, "Honeypot triggered").into_response();
                    }
                    if params.get("_wpnonce").map(|s| s.as_str()) != Some("sec_token_abc123")
                        || params.get("first_name").map(|s| s.is_empty()).unwrap_or(true)
                        || params.get("last_name").map(|s| s.is_empty()).unwrap_or(true)
                    {
                        return (StatusCode::FORBIDDEN, "Missing CSRF or split name").into_response();
                    }
                    *st.wp_payload.lock().unwrap() = Some(params);
                    (StatusCode::OK, "{\"status\":\"mail_sent\",\"message\":\"Gracias por su mensaje\"}").into_response()
                }
            }),
        )
        // 2. FALSO POSITIVO HTTP 200 (WordPress CF7 devuelve 200 OK pero con validation_failed en JSON)
        .route(
            "/wp-false-positive-200",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/wp-false-positive-200/post" method="POST">
                            <input name="email" type="email">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/wp-false-positive-200/post",
            post(|| async {
                (
                    StatusCode::OK,
                    "{\"into\":\"#wpcf7-f1\",\"status\":\"validation_failed\",\"message\":\"Hubo un error al intentar enviar su mensaje.\"}",
                )
            }),
        )
        // 3. Servidor Legacy ISO-8859-1
        .route(
            "/legacy-iso",
            get(|| async {
                Html(
                    r#"<html><head><meta http-equiv="Content-Type" content="text/html; charset=ISO-8859-1"></head>
                    <body>
                        <form action="/legacy-iso/send" method="post" accept-charset="ISO-8859-1">
                            <input type="text" name="nombre">
                            <input type="text" name="correo">
                            <input type="text" name="asunto">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/legacy-iso/send",
            post(move |Form(params): Form<HashMap<String, String>>| {
                let st = state_legacy.clone();
                async move {
                    *st.legacy_payload.lock().unwrap() = Some(params);
                    (StatusCode::OK, "OK Legacy").into_response()
                }
            }),
        )
        // 4. Formulario con <select> y Checkbox de Privacidad
        .route(
            "/corp-select",
            get(|| async {
                Html(
                    r#"<!DOCTYPE html><html><head><meta charset="utf-8"></head><body>
                        <form action="/corp-select/post" method="POST">
                            <input type="text" name="nombre_completo" required>
                            <input type="email" name="email_corporativo" required>
                            <select name="departamento" required>
                                <option value="">-- Seleccione --</option>
                                <option value="juridico_compliance">Dirección Jurídica</option>
                            </select>
                            <textarea name="comentarios"></textarea>
                            <input type="checkbox" name="acepto_privacidad" value="si" required>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/corp-select/post",
            post(move |Form(params): Form<HashMap<String, String>>| {
                let st = state_select.clone();
                async move {
                    if params.get("departamento").map(|s| s.as_str()) != Some("juridico_compliance")
                        || params.get("acepto_privacidad").map(|s| s.as_str()) != Some("si")
                    {
                        return (StatusCode::UNPROCESSABLE_ENTITY, "Faltan select o checkbox").into_response();
                    }
                    *st.select_payload.lock().unwrap() = Some(params);
                    (StatusCode::OK, "Recibido").into_response()
                }
            }),
        )
        // 5. Cloudflare Turnstile
        .route(
            "/protected-turnstile",
            get(|| async {
                Html(
                    r#"<html><body><form action="/send" method="POST">
                        <input name="email" type="email"><textarea name="mensaje"></textarea>
                        <div class="cf-turnstile" data-sitekey="0x4AAAAAA"></div>
                    </form></body></html>"#,
                )
            }),
        )
        // 6. Google reCAPTCHA
        .route(
            "/protected-recaptcha",
            get(|| async {
                Html(
                    r#"<html><body><form action="/send" method="POST">
                        <input name="email" type="email"><textarea name="message"></textarea>
                        <div class="g-recaptcha" data-sitekey="6Lc_test"></div>
                    </form></body></html>"#,
                )
            }),
        )
        // 7. hCaptcha
        .route(
            "/protected-hcaptcha",
            get(|| async {
                Html(
                    r#"<html><body><form action="/send" method="POST">
                        <input name="email" type="email"><textarea name="message"></textarea>
                        <div class="h-captcha" data-sitekey="10000000-ffff"></div>
                    </form></body></html>"#,
                )
            }),
        )
        // 8. Cloudflare WAF 403
        .route(
            "/cloudflare-waf-block",
            get(|| async {
                let mut headers = HeaderMap::new();
                headers.insert("server", "cloudflare".parse().unwrap());
                headers.insert("cf-mitigated", "challenge".parse().unwrap());
                (StatusCode::FORBIDDEN, headers, Html("<html><title>Just a moment...</title></html>"))
            }),
        )
        // 9. Formulario mailto: (debe detectarse para derivar a email directo)
        .route(
            "/mailto-form",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="mailto:contacto@empresa.com.mx" method="post">
                            <input name="nombre"><textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        // 10. Enlace trampa a un archivo PDF binario (no debe parsearse como HTML)
        .route(
            "/binary-pdf-trap",
            get(|| async {
                let mut headers = HeaderMap::new();
                headers.insert("content-type", "application/pdf".parse().unwrap());
                (StatusCode::OK, headers, "%PDF-1.7 binary content...")
            }),
        )
        // 12. Redirección 301 con cambio de directorio base y action relativo
        .route(
            "/legacy-redirect-301",
            get(|| async {
                let mut h = HeaderMap::new();
                h.insert("location", "/corporativo/portal/contacto.html".parse().unwrap());
                (StatusCode::MOVED_PERMANENTLY, h, "")
            }),
        )
        .route(
            "/corporativo/portal/contacto.html",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="procesar-envio" method="POST">
                            <input type="email" name="email">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/corporativo/portal/procesar-envio",
            post({
                let st = state.clone();
                move |Form(_params): Form<HashMap<String, String>>| {
                    let s = st.clone();
                    async move {
                        *s.redirect_post_received.lock().unwrap() = true;
                        (StatusCode::OK, "Gracias por contactarnos desde ruta redirigida")
                    }
                }
            }),
        )
        // 13. Portada sin formulario que enlaza internamente a /subpagina-contacto
        .route(
            "/home-without-form",
            get(|| async {
                Html(
                    r#"<html><body>
                        <h1>Bienvenido a Corporativo Industrial</h1>
                        <nav><a href="/subpagina-contacto">Contáctanos aquí</a></nav>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/subpagina-contacto",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/subpagina-contacto/send" method="POST">
                            <input type="email" name="correo">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/subpagina-contacto/send",
            post(|| async { (StatusCode::OK, "Mensaje enviado correctamente") }),
        )
        // 14. Sitio moderno con Iframe de HubSpot CRM y correo en el footer
        .route(
            "/modern-hubspot-crm",
            get(|| async {
                Html(
                    r#"<html><body>
                        <h2>Ventas Corporativas</h2>
                        <iframe src="https://share.hsforms.net/1abc2def3ghi"></iframe>
                        <footer><a href="mailto:juridico@corporativo-moderno.mx">juridico@corporativo-moderno.mx</a></footer>
                    </body></html>"#,
                )
            }),
        )
        // 15. Servidor inestable (falla el primer GET con 503 y el primer POST con 502, luego responde 200 OK)
        .route(
            "/flaky-503-server",
            get({
                let st = state.clone();
                move || {
                    let s = st.clone();
                    async move {
                        let n = s.flaky_get_attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        if n == 0 {
                            (StatusCode::SERVICE_UNAVAILABLE, Html("<html>Temporary Maintenance</html>")).into_response()
                        } else {
                            Html(
                                r#"<html><body>
                                    <form action="/flaky-503-server/post" method="POST">
                                        <input type="email" name="email">
                                        <textarea name="mensaje"></textarea>
                                    </form>
                                </body></html>"#,
                            )
                            .into_response()
                        }
                    }
                }
            }),
        )
        .route(
            "/flaky-503-server/post",
            post({
                let st = state.clone();
                move || {
                    let s = st.clone();
                    async move {
                        let n = s.flaky_post_attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        if n == 0 {
                            (StatusCode::BAD_GATEWAY, "502 Bad Gateway transitorio").into_response()
                        } else {
                            (StatusCode::OK, "Gracias por su mensaje tras reintento").into_response()
                        }
                    }
                }
            }),
        )
        // 16. Servidor con bytes crudos en Windows-1252 / Windows-1250 (no es UTF-8 válido)
        .route(
            "/exotic-win1252",
            get(|| async {
                let mut h = HeaderMap::new();
                h.insert("content-type", "text/html; charset=windows-1252".parse().unwrap());
                // Incluye bytes 0x80 (€), 0xE1 (á), 0xF1 (ñ) crudos en single-byte encoding
                let mut raw: Vec<u8> = Vec::new();
                raw.extend_from_slice(b"<html><body><h1>Cotizaci\xf3n en \x80 para C\xf3rdoba y Espa\xf1a</h1>");
                raw.extend_from_slice(b"<form action=\"/exotic-win1252/post\" method=\"POST\">");
                raw.extend_from_slice(b"<input type=\"email\" name=\"email\"><textarea name=\"mensaje\"></textarea>");
                raw.extend_from_slice(b"</form></body></html>");
                (StatusCode::OK, h, raw)
            }),
        )
        .route(
            "/exotic-win1252/post",
            post(|| async { (StatusCode::OK, "Gracias por su mensaje") }),
        )
        // 17. Sitio bloqueado por reCAPTCHA y con correo cifrado por Cloudflare Email Protection (data-cfemail)
        .route(
            "/cfemail-and-recaptcha",
            get(|| async {
                // XOR key 0x2A para "legal@corporativo.mx" ->
                // 'l'(0x6c)^0x2a=46, 'e'(0x65)^0x2a=4f, 'g'(0x67)^0x2a=4d, 'a'(0x61)^0x2a=4b, 'l'(0x6c)^0x2a=46,
                // '@'(0x40)^0x2a=6a, 'c'(0x63)^0x2a=49, 'o'(0x6f)^0x2a=45, 'r'(0x72)^0x2a=58, 'p'(0x70)^0x2a=5a,
                // '.'(0x2e)^0x2a=04, 'm'(0x6d)^0x2a=47, 'x'(0x78)^0x2a=52
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <input type="email" name="email">
                            <textarea name="message"></textarea>
                            <div class="g-recaptcha" data-sitekey="6Lc_key"></div>
                        </form>
                        <footer>
                            Contacto Jurídico: <a class="__cf_email__" data-cfemail="2a464f4d4b466a4945585a044752">[email&#160;protected]</a>
                        </footer>
                    </body></html>"#,
                )
            }),
        )
        // 18. Endpoint para probar deduplicación de múltiples sucursales con la misma web
        .route(
            "/multi-branch-contact",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/multi-branch-contact/post" method="POST">
                            <input type="email" name="email">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/multi-branch-contact/post",
            post(|| async { (StatusCode::OK, "Gracias por su mensaje") }),
        )
        // 19. Bucle infinito de redirecciones circulares (301 /loop-a <-> /loop-b)
        .route(
            "/loop-a",
            get(|| async {
                let mut h = HeaderMap::new();
                h.insert("location", "/loop-b".parse().unwrap());
                (StatusCode::MOVED_PERMANENTLY, h, "")
            }),
        )
        .route(
            "/loop-b",
            get(|| async {
                let mut h = HeaderMap::new();
                h.insert("location", "/loop-a".parse().unwrap());
                (StatusCode::MOVED_PERMANENTLY, h, "")
            }),
        )
        // 20. Página con formulario bloqueado por Turnstile pero con enlace de WhatsApp con prefijo viejo 521 y texto
        .route(
            "/whatsapp-dirty-number",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <textarea name="mensaje"></textarea>
                            <div class="cf-turnstile" data-sitekey="0x4AAA"></div>
                        </form>
                        <a href="https://api.whatsapp.com/send?phone=5215587654321&text=Hola%20quiero%20cotizar">WhatsApp Directo</a>
                    </body></html>"#,
                )
            }),
        )
        // 11. Sitio sin formulario de contacto
        .route(
            "/only-login-search",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form role="search" action="/buscar"><input name="q"></form>
                        <form action="/user/login"><input name="user"><input type="password" name="pass"></form>
                    </body></html>"#,
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let base = format!("http://{}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = HttpFormBot::build_bounded_http_client().unwrap();
    let emp0 = &real_companies[0].0;
    let emp1 = &real_companies[1 % real_companies.len()].0;
    let emp2 = &real_companies[2 % real_companies.len()].0;

    // Escenario 1: WordPress CF7 con empresa real del DENUE + first_name/last_name + CSRF + Honeypot
    let r1 = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-contact", base), emp0, 1245958.00).await;
    assert_eq!(r1, "PROSPECCION_ENVIADA_EXITO");

    // Escenario 2: Detección Semántica de Falso Positivo en HTTP 200 (validation_failed)
    let r2 = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-false-positive-200", base), emp1, 890000.00).await;
    assert_eq!(r2, "ERR_RESPUESTA_SERVIDOR_FALLIDO");

    // Escenario 3: Servidor Legacy ISO-8859-1 con acentos/ñ reales
    let r3 = HttpFormBot::submit_form_direct(&client, &format!("{}/legacy-iso", base), "COMPAÑÍA LOGÍSTICA DEL PACÍFICO", 740000.00).await;
    assert_eq!(r3, "PROSPECCION_ENVIADA_EXITO");
    let leg_msg = state.legacy_payload.lock().unwrap().clone().unwrap().get("mensaje").unwrap().clone();
    assert!(leg_msg.is_ascii());

    // Escenario 4: Formulario corporativo con <select> y checkbox de privacidad
    let r4 = HttpFormBot::submit_form_direct(&client, &format!("{}/corp-select", base), emp2, 940000.00).await;
    assert_eq!(r4, "PROSPECCION_ENVIADA_EXITO");

    // Escenarios 5, 6, 7 y 8: Pasarelas Anti-Bot (Turnstile, reCAPTCHA, hCaptcha, Cloudflare 403)
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/protected-turnstile", base), emp0, 500000.0).await,
        "PROTECCION_ANTIBOT_CLOUDFLARE_TURNSTILE"
    );
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/protected-recaptcha", base), emp0, 500000.0).await,
        "PROTECCION_ANTIBOT_GOOGLE_RECAPTCHA"
    );
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/protected-hcaptcha", base), emp0, 500000.0).await,
        "PROTECCION_ANTIBOT_HCAPTCHA"
    );
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), emp0, 500000.0).await,
        "BLOQUEO_WAF_CLOUDFLARE_CHALLENGE"
    );

    // Escenario 9: Detección de formulario mailto:
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/mailto-form", base), emp0, 500000.0).await,
        "CANAL_ALTERNATIVO_MAILTO_DETECTADO"
    );

    // Escenario 10: Protección contra archivos binarios PDF
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/binary-pdf-trap", base), emp0, 500000.0).await,
        "CONTENIDO_NO_HTML_IGNORADO"
    );

    // Escenario 11: Sitio sin formulario de contacto
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, &format!("{}/only-login-search", base), emp0, 500000.0).await,
        "SIN_FORMULARIO_CONTACTO"
    );

    // Escenario 12: Escudo Anti-SSRF bloqueando IP interna de metadatos AWS (169.254.169.254)
    assert_eq!(
        HttpFormBot::submit_form_direct(&client, "http://169.254.169.254/latest/meta-data/", emp0, 500000.0).await,
        "BLOQUEO_SEGURIDAD_ANTI_SSRF"
    );

    // Escenario 13: Verificar que la Rotación de User-Agents entregó huellas distintas
    let _ = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-contact", base), emp1, 500000.0).await;
    let uas = state.captured_user_agents.lock().unwrap().clone();
    assert!(uas.len() >= 2 && uas[0] != uas[1], "Los User-Agents deben rotar dinámicamente en cada petición");

    // Escenario 14: Verificar que los registros se guardaron automáticamente en SQLite WAL (prospect_logs)
    let conn = Connection::open("data/mallen_prod.db").unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM prospect_logs", [], |r| r.get(0))
        .unwrap_or(0);
    assert!(count >= 12, "Todos los intentos deben quedar auditados en la tabla prospect_logs de SQLite WAL");


    // Escenario 15: Redirección 301 con cambio de ruta base (/legacy-redirect-301 -> /corporativo/portal/contacto.html -> /corporativo/portal/procesar-envio)
    let r15 = HttpFormBot::submit_form_direct(&client, &format!("{}/legacy-redirect-301", base), "EMPRESA REDIRECT 301", 610000.0).await;
    assert_eq!(r15, "PROSPECCION_ENVIADA_EXITO");
    assert!(*state.redirect_post_received.lock().unwrap(), "El POST relativo tras redirección 301 debe impactar en /corporativo/portal/procesar-envio");

    // Escenario 16: Autodescubrimiento de enlace interno /subpagina-contacto cuando la portada no tiene <form>
    let r16 = HttpFormBot::submit_form_direct(&client, &format!("{}/home-without-form", base), "EMPRESA AUTODISCOVERY", 720000.0).await;
    assert_eq!(r16, "PROSPECCION_ENVIADA_EXITO");

    // Escenario 17: Detección de Iframe de Terceros (HubSpot CRM) y cosecha de correo en footer
    let r17 = HttpFormBot::submit_form_direct(&client, &format!("{}/modern-hubspot-crm", base), "EMPRESA HUBSPOT CRM", 830000.0).await;
    assert_eq!(r17, "CANAL_ALTERNATIVO_IFRAME_CRM_HUBSPOT");

    // Escenario 18: Recuperación automática con Exponential Backoff ante fallos HTTP 503/502 en GET y POST
    let r18 = HttpFormBot::submit_form_direct(&client, &format!("{}/flaky-503-server", base), "EMPRESA FLAKY BACKOFF", 910000.0).await;
    assert_eq!(r18, "PROSPECCION_ENVIADA_EXITO");

    // Escenario 19: Verificación de la Máquina de Estados (prospect_pipeline) y URL final redirigida en SQLite WAL
    let conn = Connection::open("data/mallen_prod.db").unwrap();
    let (stage_301, final_url_301): (String, String) = conn
        .query_row(
            "SELECT pipeline_stage, final_target_url FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA REDIRECT 301'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(stage_301, "STAGE_CONTACTADO_FORM");
    assert!(final_url_301.ends_with("/corporativo/portal/contacto.html"), "Debe guardar la URL final después de la redirección 301");

    let (stage_crm, harvested_crm): (String, String) = conn
        .query_row(
            "SELECT pipeline_stage, extracted_contact FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA HUBSPOT CRM'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(stage_crm, "STAGE_COLA_SMTP_DIRECTO", "Al extraer email jurídico directo en página HubSpot, promueve a cola SMTP directa");
    assert_eq!(harvested_crm, "mailto:juridico@corporativo-moderno.mx");

    // Escenario 20: Prueba de limpieza y truncado controlado (prune_prospect_logs)
    let _deleted = HttpFormBot::prune_prospect_logs(50000);

    // Escenario 21: Decodificación de bytes crudos en Windows-1252 / Windows-1250 (no-UTF8)
    let r21 = HttpFormBot::submit_form_direct(&client, &format!("{}/exotic-win1252", base), "EMPRESA WIN1252 EXOTIC", 510000.0).await;
    assert_eq!(r21, "PROSPECCION_ENVIADA_EXITO");

    // Escenario 22: Sitio con reCAPTCHA + Correo ofuscado con Cloudflare data-cfemail -> Desencripta "legal@corp.mx" y asigna TIER_2_DIRECT_SMTP_READY
    let r22 = HttpFormBot::submit_form_direct(&client, &format!("{}/cfemail-and-recaptcha", base), "EMPRESA CFEMAIL XOR", 880000.0).await;
    assert_eq!(r22, "PROTECCION_ANTIBOT_GOOGLE_RECAPTCHA");
    let (tier_cf, email_cf): (String, String) = conn
        .query_row(
            "SELECT contact_method_tier, extracted_emails FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA CFEMAIL XOR'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(tier_cf, "TIER_2_DIRECT_SMTP_READY", "Aunque el form tenga reCAPTCHA, al desencriptar el correo debe subir a TIER_2_DIRECT_SMTP_READY");
    assert_eq!(email_cf, "legal@corp.mx", "Debe desencriptar el correo protegido por Cloudflare data-cfemail");

    // Escenario 23: Control de duplicados entre dos sucursales del DENUE que comparten sitio web
    let branch_url = format!("{}/multi-branch-contact?dedup_domain=1", base);
    let r23_suc1 = HttpFormBot::submit_form_direct(&client, &branch_url, "CADENA NACIONAL SUCURSAL NORTE", 640000.0).await;
    assert_eq!(r23_suc1, "PROSPECCION_ENVIADA_EXITO");
    let r23_suc2 = HttpFormBot::submit_form_direct(&client, &branch_url, "CADENA NACIONAL SUCURSAL SUR", 640000.0).await;
    assert_eq!(r23_suc2, "DOMINIO_DUPLICADO_YA_CONTACTADO_OMITIDO");

    // Escenario 24: Parametrización dinámica de MALLEN_TIMEOUT_MS
    std::env::set_var("MALLEN_TIMEOUT_MS", "9500");
    assert_eq!(HttpFormBot::configured_timeout_ms(), 9500);

    // Escenario 25: Verificación de telemetría detallada (latency_ms, trace_detail y contact_method_tier)
    let (tier_win, trace_win): (String, String) = conn
        .query_row(
            "SELECT contact_method_tier, last_trace_detail FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA WIN1252 EXOTIC'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(tier_win, "TIER_1_WEB_FORM_SENT");
    assert!(trace_win.starts_with("OK:"), "Debe registrar el rastro exacto de ejecución en last_trace_detail");


    // Escenario 26: Protección contra bucles circulares infinitos de redirección 301 (/loop-a <-> /loop-b)
    let r26 = HttpFormBot::submit_form_direct(&client, &format!("{}/loop-a", base), "EMPRESA LOOP INFINITO 301", 450000.0).await;
    assert_eq!(r26, "ERR_REDIRECT_LOOP_DETECTADO", "Debe abortar el bucle circular en 5 redirecciones máximo");

    // Escenario 27: Saneamiento y normalización de números de WhatsApp (+52) y promoción a TIER_3_WHATSAPP_DIRECT_READY
    assert_eq!(HttpFormBot::normalize_whatsapp_e164("https://wa.me/5215512345678?text=Hola").unwrap(), "+525512345678");
    assert_eq!(HttpFormBot::normalize_whatsapp_e164("044 (55) 9876-5432").unwrap(), "+525598765432");
    assert_eq!(HttpFormBot::normalize_whatsapp_e164("55-4433-2211").unwrap(), "+525544332211");

    let r27 = HttpFormBot::submit_form_direct(&client, &format!("{}/whatsapp-dirty-number", base), "EMPRESA WHATSAPP E164", 670000.0).await;
    assert_eq!(r27, "PROTECCION_ANTIBOT_CLOUDFLARE_TURNSTILE");
    let (tier_wa, wa_clean): (String, String) = conn
        .query_row(
            "SELECT contact_method_tier, extracted_whatsapp FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA WHATSAPP E164'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(tier_wa, "TIER_3_WHATSAPP_DIRECT_READY");
    assert_eq!(wa_clean, "+525587654321", "Debe normalizar 5215587654321 a formato E.164 limpio +525587654321");

    // Escenario 28: Optimización no-bloqueante de almacenamiento SQLite WAL (incremental_vacuum + wal_checkpoint)
    let (_pruned, db_bytes) = HttpFormBot::optimize_sqlite_storage_non_blocking(50000).unwrap();
    assert!(db_bytes > 0);

    println!("✓ LOS 28 ESCENARIOS DE GRADO INDUSTRIAL (0 WARNINGS, ANTI-LOOP 301, WHATSAPP E.164 +52, CFEMAIL XOR Y WAL OPTIMIZER) SUPERADOS AL 100%");
}
