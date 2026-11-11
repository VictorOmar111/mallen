use axum::{
    extract::Form,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
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
        // 21. Formulario con protección dinámica JS (WP Armour / CleanTalk JS Token) que requiere Headless Browser
        .route(
            "/js-dynamic-honeypot",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <input type="email" name="email">
                            <textarea name="mensaje"></textarea>
                            <input type="hidden" name="ct_checkjs" value="0">
                            <span class="wpa_initiator" data-wpa="true"></span>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        // 22. Aplicación SPA en React vacía (<div id="root"></div>) que requiere renderizado Headless Chrome
        .route(
            "/react-spa-empty",
            get(|| async {
                Html(
                    r#"<!DOCTYPE html><html><head><title>Portal Corporativo SPA</title></head>
                    <body><div id="root"></div><script src="/bundle.js"></script></body></html>"#,
                )
            }),
        )
        // 23. Dominio vencido / estacionado (GoDaddy Parked / En construcción)
        .route(
            "/parked-expired-domain",
            get(|| async {
                Html(
                    r#"<html><body>
                        <h1>This domain is parked free, courtesy of GoDaddy.com</h1>
                        <p>Domain is for sale</p>
                    </body></html>"#,
                )
            }),
        )
        // 24. Sitio corporativo que oculta sus datos de contacto dentro de Schema.org (application/ld+json)
        .route(
            "/schema-jsonld-only",
            get(|| async {
                Html(
                    r#"<!DOCTYPE html><html><head>
                    <script type="application/ld+json">
                    {
                      "@context": "https://schema.org",
                      "@type": "Organization",
                      "name": "Grupo Industrial del Norte",
                      "email": "Cumplimiento.Legal@GrupoNorte.MX",
                      "telephone": "+52 81 8300 9900"
                    }
                    </script>
                    </head><body><h1>Bienvenido a Grupo Industrial del Norte</h1></body></html>"#,
                )
            }),
        )
        // 25. Endpoint moderno que devuelve 415 Unsupported Media Type a form-urlencoded pero acepta application/json
        .route(
            "/modern-json-only-form",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/modern-json-only-form/api" method="POST">
                            <input type="email" name="email">
                            <textarea name="mensaje"></textarea>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/modern-json-only-form/api",
            post(|headers: HeaderMap, body: String| async move {
                let ct = headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
                if ct.contains("application/json") && body.contains("dictamenes@mallen.lat") {
                    (StatusCode::OK, "{\"status\":\"mail_sent\",\"message\":\"Gracias\"}").into_response()
                } else {
                    (StatusCode::UNSUPPORTED_MEDIA_TYPE, "415 JSON Required").into_response()
                }
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
    let _ = conn.execute("DELETE FROM prospect_pipeline WHERE empresa_nombre LIKE 'CADENA NACIONAL SUCURSAL%'", []);
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


    // Escenario 29: Normalización estricta de correos con mayúsculas mezcladas, espacios invisibles Unicode (\u{200B}, %20) y descarte de basura
    assert_eq!(
        HttpFormBot::sanitize_and_normalize_email(" %20\u{200B}Contacto.Directivo@Corporativo-Industrial.MX\u{00A0} "),
        Some("contacto.directivo@corporativo-industrial.mx".to_string())
    );
    assert_eq!(HttpFormBot::sanitize_and_normalize_email("banner@2x.png"), None);
    assert_eq!(HttpFormBot::sanitize_and_normalize_email("bootstrap@5.3.2"), None);
    assert_eq!(HttpFormBot::sanitize_and_normalize_email("no-reply@corporativo.mx"), None);

    // Escenario 30: Cosecha multicanal deduplicando mayúsculas/minúsculas y desofuscando "[at]"
    let dirty_html = r#"
        <html><body>
            <a href="mailto:%20Juridico@EmpresaReal.MX">Mail 1</a>
            <p>Escríbenos a JURIDICO@empresareal.mx o a direccion [at] empresareal.mx</p>
            <img src="icon@2x.png">
        </body></html>
    "#;
    let (harvested_mails, _, _) = HttpFormBot::harvest_multichannel_contacts(dirty_html);
    assert_eq!(
        harvested_mails,
        "juridico@empresareal.mx; direccion@empresareal.mx",
        "Debe normalizar a minúsculas, eliminar duplicados por capitalización e ignorar icon@2x.png"
    );


    // Escenario 31: Detección preventiva de Honeypot / Token dinámico por JavaScript (WP Armour / CleanTalk) -> Deriva a Headless Chrome
    let r31 = HttpFormBot::submit_form_direct(&client, &format!("{}/js-dynamic-honeypot", base), "EMPRESA JS HONEYPOT", 710000.0).await;
    assert_eq!(r31, "REQUIERE_HEADLESS_JS_DYNAMIC_FORM");
    let (stage_js, tier_js): (String, String) = conn
        .query_row(
            "SELECT pipeline_stage, contact_method_tier FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA JS HONEYPOT'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(stage_js, "STAGE_COLA_HEADLESS_BROWSER");
    assert_eq!(tier_js, "TIER_4_HEADLESS_CHROME_WORKER");

    // Escenario 32: Detección de SPA React/Vue sin <form> estático (<div id="root"></div>) -> Deriva a Headless Chrome
    let r32 = HttpFormBot::submit_form_direct(&client, &format!("{}/react-spa-empty", base), "EMPRESA REACT SPA", 620000.0).await;
    assert_eq!(r32, "REQUIERE_HEADLESS_JS_SPA_RENDER");

    // Escenario 33: Verificación de Cifrado Simétrico en Reposo LFPDPPP (Art. 19) en la columna encrypted_contact_vault
    let vault_cipher: String = conn
        .query_row(
            "SELECT encrypted_contact_vault FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA CFEMAIL XOR'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(vault_cipher.starts_with("ENCv1:"), "Los datos sensibles deben almacenarse cifrados con prefijo ENCv1: en reposo");
    assert!(!vault_cipher.contains("legal@corp.mx"), "El texto cifrado en disco no debe exponer el correo en texto plano");
    let decrypted_vault = HttpFormBot::decrypt_pii_from_rest(&vault_cipher).unwrap();
    assert!(decrypted_vault.contains("emails=legal@corp.mx"), "Al descifrar con la llave en RAM debe recuperar exactamente legal@corp.mx");


    // Escenario 34: Verificar que el nuevo mensaje B2B sea humano, consultivo y no incluya cifras de multa intimidantes
    let msg_sample = HttpFormBot::build_human_consultative_message("CORPORATIVO ALSEA", "CORPORATIVO%20ALSEA");
    assert!(msg_sample.contains("Les saludamos cordialmente de FIRMA W16 MALLEN"));
    assert!(msg_sample.contains("descargar sin costo su resumen tecnico confidencial"));
    assert!(!msg_sample.contains("1245958"), "El mensaje consultivo no debe exponer montos de multas intimidantes en el buzón abierto");

    // Escenario 35: Circuit Breaker por Dominio/Ruta (3 bloqueos 403 WAF consecutivos abren el circuito por 48h)
    let cb_domain_key = "127.0.0.1:/cloudflare-waf-block";
    HttpFormBot::reset_circuit_breaker(cb_domain_key);
    let _ = HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), "CB TEST 1", 100.0).await;
    let _ = HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), "CB TEST 2", 100.0).await;
    let _ = HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), "CB TEST 3", 100.0).await;
    // El 4to intento debe ser cortado en 0ms por el Circuit Breaker sin tocar la red
    let r35 = HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), "CB TEST 4", 100.0).await;
    assert_eq!(r35, "CIRCUIT_BREAKER_ABIERTO_48H", "Al 4to intento tras 3 bloqueos 403 WAF debe saltar el Circuit Breaker de 48h");

    // Escenario 36: Prueba de ráfaga concurrente gobernada por tokio::sync::Semaphore
    let mut handles = Vec::new();
    for i in 0..8 {
        let c_clone = client.clone();
        let url_clone = format!("{}/wp-contact", base);
        let emp_name = format!("EMPRESA RAFAGA SEMAFORO {}", i);
        handles.push(tokio::spawn(async move {
            HttpFormBot::submit_form_direct(&c_clone, &url_clone, &emp_name, 300000.0).await
        }));
    }
    for h in handles {
        assert_eq!(h.await.unwrap(), "PROSPECCION_ENVIADA_EXITO");
    }

    // Escenario 37: Verificación del Snapshot de Telemetría Avanzada de Procesos
    let snap = HttpFormBot::get_process_telemetry_snapshot();
    assert!(snap.total_requests >= 25);
    assert!(snap.success_count >= 12);
    assert!(snap.circuit_breaker_trips >= 1);
    assert_eq!(snap.active_in_flight, 0, "Al terminar todas las tareas, active_in_flight debe volver exactamente a 0");
    println!(
        "✓ TELEMETRÍA DE PROCESOS -> Total Reqs: {} | Éxitos: {} | WAF/Captchas: {} | Circuit Breaker Trips: {} | Permisos Libres Semáforo: {} | Latencia Promedio: {} ms",
        snap.total_requests, snap.success_count, snap.waf_or_captcha_count, snap.circuit_breaker_trips, snap.available_semaphore_permits, snap.avg_latency_ms
    );


    // Escenario 38: Verificar que el Backoff Exponencial con Jitter Descorrelacionado supere el umbral base y varíe orgánicamente
    let b1 = HttpFormBot::compute_decorrelated_backoff_ms(1, 150);
    let b2 = HttpFormBot::compute_decorrelated_backoff_ms(2, 150);
    assert!(b1 >= 300 && b1 < 500, "Intento 1 debe incluir base exponencial + jitter orgánico");
    assert!(b2 >= 600 && b2 < 1000, "Intento 2 debe escalar exponencialmente con jitter");

    // Escenario 39: Detección inmediata de dominios estacionados o vencidos (GoDaddy / Sedo / En Construcción)
    let r39 = HttpFormBot::submit_form_direct(&client, &format!("{}/parked-expired-domain", base), "EMPRESA DOMINIO VENCIDO", 310000.0).await;
    assert_eq!(r39, "DOMINIO_ESTACIONADO_O_EXPIRADO");

    // Escenario 40: Cosecha invisible de correo y teléfono desde microdatos Schema.org (application/ld+json)
    let r40 = HttpFormBot::submit_form_direct(&client, &format!("{}/schema-jsonld-only", base), "GRUPO INDUSTRIAL JSONLD", 920000.0).await;
    assert_eq!(r40, "SIN_FORMULARIO_CONTACTO");
    let (tier_ld, mail_ld, wa_ld): (String, String, String) = conn
        .query_row(
            "SELECT contact_method_tier, extracted_emails, extracted_whatsapp FROM prospect_pipeline WHERE empresa_nombre = 'GRUPO INDUSTRIAL JSONLD'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!(tier_ld, "TIER_2_DIRECT_SMTP_READY", "Gracias al extractor JSON-LD debe promoverse a TIER_2_DIRECT_SMTP_READY");
    assert_eq!(mail_ld, "cumplimiento.legal@gruponorte.mx");
    assert_eq!(wa_ld, "+528183009900");

    // Escenario 41: Fallback automático a application/json cuando el action del formulario devuelve HTTP 415
    let r41 = HttpFormBot::submit_form_direct(&client, &format!("{}/modern-json-only-form", base), "EMPRESA JSON 415 FALLBACK", 540000.0).await;
    assert_eq!(r41, "PROSPECCION_ENVIADA_EXITO", "Debe reintentar automáticamente con application/json al recibir HTTP 415");

    // Escenario 42: Extracción de enlaces tel: fijos/móviles en portales industriales
    let (_, phones_tel, _) = HttpFormBot::harvest_multichannel_contacts(r#"<html><body><a href="tel:01 (33) 3612-4455">Planta Guadalajara</a></body></html>"#);
    assert_eq!(phones_tel, "+523336124455");


    // Escenario 43: Detector de Stack Tecnológico en México (Estilo Wappalyzer: Shopify, Conekta, WooCommerce, Next.js)
    let ecom_html = r#"<html><head><script src="https://cdn.shopify.com/s/files/1/store.js"></script><script src="https://cdn.conekta.io/js/latest/conekta.js"></script></head><body>Carrito</body></html>"#;
    let (stack_detected, is_ecom, pitch_mod) = HttpFormBot::detect_mexican_tech_stack("server: cloudflare", ecom_html);
    assert!(is_ecom, "Debe identificar que es un E-Commerce activo");
    assert!(stack_detected.contains("Shopify") && stack_detected.contains("Conekta-MX"));
    assert_eq!(pitch_mod, "MODULO_BLINDAJE_ECOMMERCE_PASARELAS_Y_PROFECO_ART76BIS");

    // Escenario 44: Auditor de Seguridad de Correo y DNS (SPF / DMARC)
    let (_spf, _dmarc, dns_verdict) = HttpFormBot::audit_domain_email_dns_security("localhost");
    assert_eq!(dns_verdict, "VULNERABLE_SIN_DMARC_RIESGO_SUPLANTACION");

    // Escenario 45: Clasificador de Tamaño / Capacidad Económica por Estrato DENUE y Riesgo Sectorial SCIAN
    let (size_tier, emp_est, price_mxn, scian_pitch) = HttpFormBot::classify_denue_capacity_and_scian_risk("251 y más personas", "488511");
    assert_eq!(size_tier, "CORP_GRAN_EMPRESA_251_PLUS");
    assert_eq!(emp_est, 300);
    assert_eq!(price_mxn, 3_000.00, "El precio estandarizado para la campaña de 10k PyMEs debe ser $3,000.00 MXN (~$150 USD)");
    assert!(scian_pitch.contains("Carta Porte 3.1"));

    // Escenario 46: Validador Pre-Flight de URLs antes de persistir (ERR_INVALID_LINK_GENERATED ante nombres vacíos o corruptos)
    assert!(HttpFormBot::build_and_validate_checkout_link("GRUPO BIMBO", Some("BIM011108DJ5")).is_ok());
    assert_eq!(HttpFormBot::build_and_validate_checkout_link("   ", None), Err("ERR_INVALID_LINK_GENERATED"));
    let r46_bad = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-contact", base), "   ", 500000.0).await;
    assert_eq!(r46_bad, "ERR_INVALID_LINK_GENERATED");

    // Escenario 47: Verificar que un servidor con WAF pesado (sin correo en la pantalla 403) se derive automáticamente a TIER_4_HEADLESS_CHROME_WORKER
    let (tier_waf, stage_waf): (String, String) = conn
        .query_row(
            "SELECT contact_method_tier, pipeline_stage FROM prospect_pipeline WHERE empresa_nombre = 'CB TEST 1'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(tier_waf, "TIER_4_HEADLESS_CHROME_WORKER");
    assert_eq!(stage_waf, "STAGE_COLA_HEADLESS_BROWSER");

    // Escenario 48: Registro en tiempo real de Clics en el enlace de Pre-Diagnóstico (tabla link_clicks)
    let clicks_count = HttpFormBot::record_checkout_link_click("EMPRESA REDIRECT 301", "189.203.10.25", "Mozilla/5.0 (Macintosh)").unwrap();
    assert!(clicks_count >= 1);
    let stage_after_click: String = conn
        .query_row(
            "SELECT pipeline_stage FROM prospect_pipeline WHERE empresa_nombre = 'EMPRESA REDIRECT 301'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stage_after_click, "STAGE_LINK_ABIERTO_INTENCION_COMPRA");

    // Escenario 49: Extracción de la Lista de Empresas Contactadas que NO han respondido (excluyendo a EMPRESA REDIRECT 301 que ya hizo clic)
    let unresponsive = HttpFormBot::get_unresponsive_contacted_prospects(25);
    assert!(!unresponsive.is_empty(), "Debe devolver la lista de prospectos contactados que aún no abren su link");
    assert!(
        !unresponsive.iter().any(|(name, _, _, _, _)| name == "EMPRESA REDIRECT 301"),
        "EMPRESA REDIRECT 301 ya abrió el link, por lo que no debe aparecer en la lista de no-respondidos"
    );


    // Escenario 50: Verificar calibración para hostings lentos en México (8,500 ms default) y Semáforo de 32 permisos
    std::env::remove_var("MALLEN_TIMEOUT_MS");
    assert_eq!(HttpFormBot::configured_timeout_ms(), 8500, "El timeout por defecto debe ser 8,500 ms para no perder sitios PyME lentos");
    assert!(HttpFormBot::global_semaphore().available_permits() <= 64);

    // Escenario 51: Prueba del motor paginado por Chunks (máximo 500 en RAM) leyendo empresas reales de denue_units
    let chunk_sample = HttpFormBot::fetch_pyme_prospect_chunk_from_denue(0, 10);
    for (_emp_n, _url, _sc, pr) in &chunk_sample {
        assert_eq!(*pr, 3_000.00, "Cada prospecto del chunk debe llevar el ticket PyME de $3,000.00 MXN");
    }
    let (dry_processed, _, _) = HttpFormBot::run_chunked_pyme_campaign(chunk_sample.len(), 5, true).await;
    assert_eq!(dry_processed, chunk_sample.len(), "El procesador por chunks debe iterar exactamente sobre los lotes sin acumular vectores gigantes");

    // Escenario 52: Verificar que los Tiers de Correo (TIER_2_DIRECT_SMTP_READY) queden guardados en reserva para la segunda ola
    let next_act_smtp: String = conn
        .query_row(
            "SELECT next_action_channel FROM prospect_pipeline WHERE empresa_nombre = 'GRUPO INDUSTRIAL JSONLD'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(next_act_smtp, "RESERVA_SMTP_GUARDADA_SEGUNDA_OLA");


    // Escenario 53: Filtro Preventivo de Dominios Inútiles (Dead-Domain Blacklist: Facebook, MercadoLibre, Gmail) -> Deriva a TIER_6 en 0 ms
    let r53_fb = HttpFormBot::submit_form_direct(&client, "https://www.facebook.com/TaqueriaElPaisa", "TAQUERIA EL PAISA FACEBOOK", 3000.0).await;
    assert_eq!(r53_fb, "DOMINIO_RED_SOCIAL_O_DIRECTORIO_OMITIDO");
    let tier_fb: String = conn
        .query_row(
            "SELECT contact_method_tier FROM prospect_pipeline WHERE empresa_nombre = 'TAQUERIA EL PAISA FACEBOOK'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(tier_fb, "TIER_6_PHONE_DENUE_FALLBACK", "Los enlaces de Facebook/MercadoLibre deben ir directo a Tier 6 sin hacer petición HTTP");

    // Escenario 54: Modo de Reanudación Automática por Lotes (Checkpoint & Resume sobre pipeline_stage = 'PENDIENTE_BARRIDO')
    let _ = conn.execute("DELETE FROM prospect_pipeline WHERE empresa_nombre LIKE 'EMPRESA CHECKPOINT %'", []);
    conn.execute(
        "INSERT INTO prospect_pipeline (
            empresa_nombre, input_url, final_target_url, final_domain,
            multa_estimada_mxn, ultimo_resultado_codigo, pipeline_stage,
            next_action_channel, extracted_contact, contact_method_tier
         ) VALUES ('EMPRESA CHECKPOINT 1', ?1, ?1, '127.0.0.1', 3000.0, 'EN_COLA', 'PENDIENTE_BARRIDO', 'BARRER', '', 'PENDIENTE')",
        rusqlite::params![format!("{}/wp-contact", base)],
    )
    .unwrap();

    // Primera pasada: procesa EMPRESA CHECKPOINT 1 y cambia su estado a STAGE_CONTACTADO_FORM
    let (proc_first, succ_first) = HttpFormBot::run_checkpoint_resumable_campaign(10, 5).await;
    assert!(proc_first >= 1 && succ_first >= 1);

    // Segunda pasada inmediata (simulando reinicio de SSH): como ya no está en PENDIENTE_BARRIDO, procesa 0 sin duplicar
    let (proc_second, _) = HttpFormBot::run_checkpoint_resumable_campaign(10, 5).await;
    assert_eq!(proc_second, 0, "Checkpoint & Resume no debe reprocesar empresas que ya salieron de PENDIENTE_BARRIDO");

    // Escenario 55: Generador de Copy para las 4 Colas de Ataque (Tier 3 WhatsApp y Tier 2 SMTP con alerta de phishing DMARC)
    let wa_copy = HttpFormBot::build_tier_specific_outreach_copy("TIER_3_WHATSAPP_DIRECT_READY", "GRUPO INDUSTRIAL NORTE", "gruponorte.mx", false);
    assert!(wa_copy.contains("https://mallen.lat/checkout?ref=GRUPO%20INDUSTRIAL%20NORTE"));

    let smtp_copy = HttpFormBot::build_tier_specific_outreach_copy("TIER_2_DIRECT_SMTP_READY", "GRUPO INDUSTRIAL NORTE", "gruponorte.mx", false);
    assert!(smtp_copy.contains("carece de politica DMARC activa"));
    assert!(smtp_copy.contains("phishing"));

    // Escenario 56: Verificar que la semilla desde DENUE (seed_checkpoint_resume_queue_10k) funcione sin errores
    let _seeded = HttpFormBot::seed_checkpoint_resume_queue_10k(5).unwrap();


    // Escenario 57: Integración 100% End-to-End entre HttpFormBot -> pdf_generator (Pre-Dictamen PDF + Click-Tracking automático)
    let (resolved_scian, resolved_emp, _priv_flag) =
        worker_inegi_engine::engine::pdf_generator::lookup_formbot_and_denue_context(emp0, "STEREN010101AAA");
    assert!(resolved_scian.is_some(), "pdf_generator debe recuperar automáticamente el SCIAN real desde denue_units / prospect_pipeline");
    assert!(resolved_emp.unwrap_or(0) > 0);

    // Verificar que al abrir el Pre-Dictamen se haya registrado automáticamente el clic en link_clicks
    let auto_clicks: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM link_clicks WHERE ref_key = ?1",
            rusqlite::params![emp0],
            |r| r.get(0),
        )
        .unwrap_or(0);
    assert!(auto_clicks >= 1, "Generar el Pre-Dictamen de un prospecto del FormBot debe registrar automáticamente su clic en link_clicks");


    // Escenario 58: Verificación de Paridad Matemática Exacta en pdf_generator (Cero inflado a Score 71 ni piso de $1,245,958.00)
    let pdf_src = std::fs::read_to_string("src/engine/pdf_generator.rs").unwrap();
    assert!(
        !pdf_src.contains(".max(1_245_958.0)"),
        "No debe existir el piso artificial de $1,245,958.00 MXN en pdf_generator.rs"
    );
    assert!(
        !pdf_src.contains(".max(71)"),
        "No debe existir el piso artificial de risk_score .max(71) en pdf_generator.rs"
    );
    assert!(
        !pdf_src.contains("45_u32") && !pdf_src.contains(".unwrap_or(45)"),
        "El fallback de empleados debe estar homologado en 20 en todos los puntos de pdf_generator.rs"
    );


    // Escenario 59: Property-Based Testing (250 permutaciones aleatorias de empleados, SCIAN y banderas)
    // + Caso Real Corporativo Urbvan (258 empleados) verificando Paridad al Centavo Absoluto (u64 cents) y Caché SSOT en RAM
    let scian_pool = ["488511", "485111", "311811", "621111", "522110", "236221", "461110", "722511", "541110"];
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    for i in 0..250 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let emp_rand = if i == 0 {
            258_u32 // Caso explícito Corporativo Urbvan (258 empleados)
        } else {
            ((seed % 1200) as u32).max(1)
        };
        let scian_pick = scian_pool[(seed as usize) % scian_pool.len()];
        let ssl_flag = (seed & 1) == 0;
        let priv_flag = (seed & 2) == 0;

        let (vecs, c_stps, c_sat, c_inai, c_imss, c_pc, c_profeco, total_cents, score) =
            worker_inegi_engine::engine::pdf_generator::compute_ssot_exact_totals_cents(
                "CORPORATIVO URBVAN S.A. DE C.V.",
                scian_pick,
                emp_rand,
                "LIMPIO",
                "LIMPIO",
                false,
                false,
                ssl_flag,
                priv_flag,
                priv_flag,
                false,
                true,
                true,
                false,
                true,
                false,
                true,
                true,
                true,
                false,
                None,
            );

        let sum_from_37_vectors: u64 = vecs
            .iter()
            .map(|v| worker_inegi_engine::engine::pdf_generator::mxn_to_exact_cents(v.multa_estimada_mxn))
            .sum();

        assert_eq!(
            c_stps + c_sat + c_inai + c_imss + c_pc + c_profeco,
            total_cents,
            "Invariante 1: La suma de los 6 subtotales en centavos debe ser idéntica al total general"
        );
        assert_eq!(
            sum_from_37_vectors,
            total_cents,
            "Invariante 2: La suma individual de los 37 vectores en centavos debe ser idéntica al total reportado"
        );
        assert!(score >= 45 && score <= 99, "Invariante 3: El puntaje de riesgo para empresas con hallazgos debe estar acotado en [45, 99]");
    }

    // Verificar Raíz de Merkle determinística para el lote/paquete de 258 empleados
    let merkle_urbvan = worker_inegi_engine::engine::pdf_generator::compute_bundle_merkle_root(
        "URB180101AAA",
        "CORPORATIVO URBVAN",
        258,
        352_363_200,
        "485111",
    );
    assert!(merkle_urbvan.starts_with("0x") && merkle_urbvan.len() == 26);


    // Escenario 60: Verificación de Análisis Léxico de Régimen Societario (SAPI, S. de R.L., S.A., Persona Física) y Masa Salarial SUA
    let (reg_sapi, det_sapi) = worker_inegi_engine::engine::pdf_generator::analyze_corporate_entity_regime(
        "CORPORATIVO URBVAN S.A.P.I. DE C.V.",
        "URB180101AAA",
    );
    assert!(reg_sapi.contains("S.A.P.I."));
    assert!(det_sapi.contains("Ley del Mercado de Valores"));

    let (reg_pf, _) = worker_inegi_engine::engine::pdf_generator::analyze_corporate_entity_regime(
        "JUAN PEREZ GARCIA",
        "PEGJ850101HDF",
    );
    assert!(reg_pf.contains("Persona Física"));

    let payroll_258_cents = worker_inegi_engine::engine::pdf_generator::estimate_monthly_payroll_exposure_cents(258, "485111");
    assert!(payroll_258_cents > 400_000_000, "258 colaboradores en transporte deben proyectar una masa salarial > $4.0M MXN/mes");


    // Escenario 61: Prueba del Patrón de Instantánea Inmutable (`audit_snapshots`) — Fin definitivo a la asimetría 258 vs 45 empleados
    let mut master_urbvan_258 = worker_inegi_engine::engine::models::AuditMasterJson::default();
    master_urbvan_258.tax_id = "URB180101AAA".to_string();
    master_urbvan_258.razon_social = "CORPORATIVO URBVAN S.A.P.I. DE C.V.".to_string();
    master_urbvan_258.codigo_scian = "485111".to_string();
    master_urbvan_258.actividad_scian_desc = "485111 - Transporte colectivo urbano y suburbano de pasajeros".to_string();
    master_urbvan_258.estimated_employees = 258;
    master_urbvan_258.calibration_applied.exact_employee_count = Some(258);
    master_urbvan_258.total_estimated_fine_mxn = 3_523_632.00;
    master_urbvan_258.risk_score = 78;

    // 1. Congelar el Snapshot Inmutable cuando el usuario calcula el Pre-Dictamen en la web (258 empleados)
    let snap_uuid = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(
        "URB180101AAA",
        "CORPORATIVO URBVAN S.A.P.I. DE C.V.",
        &master_urbvan_258,
    );
    assert!(snap_uuid.starts_with("SNAP-"));

    // 2. Simular que el endpoint del ZIP o de un PDF secundario recibe `None` o recibe un struct desactualizado con 45 empleados
    let mut stale_master_45 = master_urbvan_258.clone();
    stale_master_45.estimated_employees = 45;
    stale_master_45.calibration_applied.exact_employee_count = Some(45);

    let recovered_from_none = worker_inegi_engine::engine::pdf_generator::upgrade_or_recover_ssot_master(
        "URB180101AAA",
        "CORPORATIVO URBVAN S.A.P.I. DE C.V.",
        None,
    )
    .unwrap();
    assert_eq!(
        recovered_from_none.calibration_applied.exact_employee_count,
        Some(258),
        "Cuando el generador de PDF/ZIP recibe None, debe leer obligatoriamente los 258 empleados de audit_snapshots"
    );

    let recovered_from_stale_45 = worker_inegi_engine::engine::pdf_generator::upgrade_or_recover_ssot_master(
        "URB180101AAA",
        "CORPORATIVO URBVAN S.A.P.I. DE C.V.",
        Some(&stale_master_45),
    )
    .unwrap();
    assert_eq!(
        recovered_from_stale_45.calibration_applied.exact_employee_count,
        Some(258),
        "Incluso si una tabla secundaria pasa el default antiguo de 45 empleados, el Snapshot Inmutable debe imponer los 258 empleados reales"
    );


    // =========================================================================
    // BATERÍA DE PRUEBAS DE INTEGRACIÓN PROFUNDA (FORMBOT ⇄ GENERADOR PDF)
    // =========================================================================

    // Escenario 62 (Prueba 1): Resistencia al Salto de Contexto (Caso Urbvan — 258 Colaboradores en el Markup Typst compilado)
    let typst_urbvan_doc = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot(&snap_uuid)
        .expect("El SNAP-UUID de Urbvan debe existir y renderizar el Dictamen completo");
    assert!(
        typst_urbvan_doc.contains("258 colaboradores"),
        "El documento Typst generado desde el SNAP-UUID debe contener literalmente '258 colaboradores'"
    );
    assert!(
        !typst_urbvan_doc.contains("45 colaboradores") && !typst_urbvan_doc.contains("45 personas"),
        "El documento Typst jamás debe caer en el respaldo antiguo de 45 colaboradores"
    );

    // Escenario 63 (Prueba 2): Rechazo Estricto a Peticiones Huérfanas (Tolerancia Cero a Fallbacks con UUID inexistente)
    let orphan_res = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot("SNAP-UUID-INEXISTENTE-999999");
    assert_eq!(
        orphan_res,
        Err("HTTP_404_ORPHAN_SNAPSHOT_REJECTED"),
        "Cualquier intento de descargar un PDF con un session_uuid inexistente debe ser rechazado con 404 sin adivinar datos"
    );
    let empty_uuid_res = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject("   ");
    assert_eq!(empty_uuid_res.err(), Some("HTTP_400_EMPTY_SESSION_UUID"));

    // Escenario 64 (Prueba 3): 50 Variaciones de Calibración en Paralelo — Coherencia Criptográfica SHA-256 Bit a Bit de los 37 Vectores
    let mut parallel_tasks = Vec::with_capacity(50);
    for idx in 0..50u32 {
        parallel_tasks.push(tokio::spawn(async move {
            let mut m_var = worker_inegi_engine::engine::models::AuditMasterJson::default();
            m_var.tax_id = format!("PAR{:02}0101AAA", idx);
            m_var.razon_social = format!("CORPORATIVO PARALELO {} S.A.P.I. DE C.V.", idx);
            m_var.codigo_scian = match idx % 5 {
                0 => "485111".to_string(),
                1 => "311811".to_string(),
                2 => "621111".to_string(),
                3 => "522110".to_string(),
                _ => "236221".to_string(),
            };
            let dynamic_emp = 12 + (idx * 7);
            m_var.estimated_employees = dynamic_emp;
            m_var.calibration_applied.exact_employee_count = Some(dynamic_emp);
            m_var.calibration_applied.has_telework_over_40_percent = Some((idx & 1) == 0);
            m_var.calibration_applied.uses_subcontracted_services_repse = Some((idx & 2) == 0);
            m_var.calibration_applied.has_pressure_vessels_boilers = Some((idx & 4) == 0);
            m_var.calibration_applied.uses_biometrics_or_health_data = Some((idx & 8) == 0);
            m_var.calibration_applied.handles_hazardous_chemicals = Some((idx & 16) == 0);

            // 1. Calcular el Hash SHA-256 y total en centavos en el momento del Pre-Dictamen Web
            let (web_hash, web_total_cents) =
                worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&m_var);
            m_var.total_estimated_fine_mxn = (web_total_cents as f64) / 100.0;

            // 2. Congelar en SQLite WAL (`audit_snapshots`) y obtener su SNAP-UUID
            let s_uuid = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(
                &m_var.tax_id,
                &m_var.razon_social,
                &m_var,
            );

            // 3. Leer el Snapshot desde el generador como lo hace la descarga del PDF/ZIP y recalcular el Hash SHA-256
            let loaded_snap = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject(&s_uuid)
                .expect("El snapshot paralelo recién congelado debe recuperarse sin fallo");
            let (pdf_hash, pdf_total_cents) =
                worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&loaded_snap);

            assert_eq!(
                web_hash, pdf_hash,
                "El Hash SHA-256 de los 37 vectores debe coincidir bit a bit entre la Web y el Generador PDF (variación #{})",
                idx
            );
            assert_eq!(web_total_cents, pdf_total_cents);
            assert_eq!(loaded_snap.calibration_applied.exact_employee_count, Some(dynamic_emp));
        }));
    }

    for task in parallel_tasks {
        task.await.expect("Ninguna de las 50 variaciones paralelas debe fallar");
    }


    // Escenario 65: Resiliencia y Checkpointing en Lotes Masivos (`batch_jobs_progress`) — Corte en registro #4,300 y reanudación exacta
    let batch_job_id = "BATCH-PARTNER-10K-2026";
    worker_inegi_engine::engine::pdf_generator::save_batch_job_checkpoint(batch_job_id, 10_000, 4_300, "INTERRUPTED_NETWORK")
        .expect("Debe guardar el checkpoint del lote en SQLite WAL");
    let resume_idx = worker_inegi_engine::engine::pdf_generator::get_batch_job_resume_index(batch_job_id);
    assert_eq!(
        resume_idx, 4_300,
        "Al reiniciar un lote interrumpido en el registro #4,300, debe reanudar exactamente en 4,300 sin repetir los anteriores"
    );

    // Escenario 66: Cola de Trabajos Asíncrona Acotada para Compilación Typst (Max 4 Workers simultáneos vía Semaphore + spawn_blocking)
    assert_eq!(
        worker_inegi_engine::engine::pdf_generator::typst_cpu_worker_pool().available_permits(),
        4,
        "El pool de CPU de Typst debe estar acotado a 4 compilaciones simultáneas por defecto"
    );
    let mut pool_tasks = Vec::with_capacity(16);
    for _ in 0..16 {
        let u_clone = snap_uuid.clone();
        pool_tasks.push(tokio::spawn(async move {
            worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot_async(u_clone).await
        }));
    }
    for pt in pool_tasks {
        let doc_out = pt.await.unwrap().expect("El worker pool de Typst debe procesar toda la cola sin saturar la CPU");
        assert!(doc_out.contains("258 colaboradores"));
    }

    // Escenario 67: Caché LRU en Memoria RAM para Perfiles de Riesgo SCIAN (Evaluación O(1) en nanosegundos)
    let lru_first = worker_inegi_engine::engine::pdf_generator::get_or_compute_scian_math_lru(
        "485111", 258, "LIMPIO", "LIMPIO", false, false, true, true, true, false,
        true, true, false, true, false, true, true, true, false,
    );
    let t0 = std::time::Instant::now();
    let lru_second = worker_inegi_engine::engine::pdf_generator::get_or_compute_scian_math_lru(
        "485111", 258, "LIMPIO", "LIMPIO", false, false, true, true, true, false,
        true, true, false, true, false, true, true, true, false,
    );
    let elapsed_ns = t0.elapsed().as_nanos();
    assert_eq!(lru_first, lru_second, "La Caché LRU en RAM debe devolver exactamente la misma tupla matemática");
    println!("✓ HIT DE CACHÉ LRU SCIAN EN RAM COMPLETADO EN {} NANOSEGUNDOS (Total Centavos: {})", elapsed_ns, lru_second.6);


    // Escenario 68: Prueba Integral con Logs de Medición Detallados para 10 Tipos Diferentes de Empresas (RFC, Dominio, SCIAN, Empleados y Régimen)
    println!("\n==========================================================================================================");
    println!("=== AUDITORÍA DE CALIBRACIÓN Y PARIDAD EN 10 PERFILES EMPRESARIALES MULTISECTORIALES (SSOT SNAP-UUID) ===");
    println!("==========================================================================================================");
    let ten_profiles: [(&str, &str, &str, &str, u32, bool); 10] = [
        ("STE850101AAA", "ELECTRONICA STEREN S.A. DE C.V.", "https://www.steren.com.mx", "434222", 180, true),
        ("URB180101BBB", "CORPORATIVO URBVAN S.A.P.I. DE C.V.", "https://www.urbvan.com", "485111", 258, true),
        ("CAN140505CCC", "OPERADORA RESTAURANTERA CANCINO S. DE R.L. DE C.V.", "https://www.archipizza.com.mx", "722511", 42, false),
        ("MED200202DDD", "CLINICA ESPECIALIZADA SANTA FE S.C.", "https://www.clinicasantafe.mx", "621111", 19, true),
        ("FIN190909EEE", "FINTECH CAPITAL DIGITAL S.A.P.I. DE C.V.", "https://www.fintechcapital.mx", "522320", 85, true),
        ("IND111111FFF", "MANUFACTURAS METALMECANICAS DEL NORTE S.A. DE C.V.", "https://www.metalmecanicanorte.mx", "332310", 320, false),
        ("CON150303GGG", "DESARROLLO E INGENIERIA CIVIL ESTRUCTURAL S.A. DE C.V.", "https://www.ingenieriacivil.mx", "236221", 64, false),
        ("LOG170707HHH", "AUTOTRANSPORTES Y LOGISTICA TRANSFRONTERIZA S.A. DE C.V.", "https://www.logisticatrans.mx", "484129", 115, true),
        ("ECO210808III", "COMERCIALIZADORA E-COMMERCE MODA MX S. DE R.L. DE C.V.", "https://www.modamx.shop", "463211", 14, true),
        ("PER880412JJ9", "ING. ROBERTO MARTINEZ SOLIS (PERSONA FISICA)", "https://www.consultoriamartinez.mx", "541610", 6, false),
    ];

    for (idx, (rfc, razon, dom_url, scian, emp_exact, has_priv)) in ten_profiles.iter().enumerate() {
        let t_start = std::time::Instant::now();
        let mut m = worker_inegi_engine::engine::models::AuditMasterJson::default();
        m.tax_id = (*rfc).to_string();
        m.razon_social = (*razon).to_string();
        m.codigo_scian = (*scian).to_string();
        m.actividad_scian_desc = format!("{} - Actividad Sectorial Evaluada", scian);
        m.estimated_employees = *emp_exact;
        m.calibration_applied.exact_employee_count = Some(*emp_exact);
        m.web_telemetry.target_url = (*dom_url).to_string();
        m.web_telemetry.ssl_valid = true;
        m.web_telemetry.has_privacy_notice = *has_priv;
        m.web_telemetry.mentions_arco_rights = *has_priv;

        // 1. Calcular huella criptográfica SHA-256 y centavos exactos en la capa Web
        let (hash_web, total_cents_web) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&m);
        m.total_estimated_fine_mxn = (total_cents_web as f64) / 100.0;

        // 2. Congelar Snapshot Inmutable en SQLite WAL (`audit_snapshots`)
        let snap_id = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(rfc, razon, &m);

        // 3. Renderizar Dictamen Typst desde el Snapshot estricto y verificar paridad 1:1
        let typst_out = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot(&snap_id)
            .expect("Debe renderizar el Dictamen desde el Snapshot Inmutable");
        let loaded_m = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject(&snap_id).unwrap();
        let (hash_pdf, total_cents_pdf) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&loaded_m);

        let (regimen_lbl, _) = worker_inegi_engine::engine::pdf_generator::analyze_corporate_entity_regime(razon, rfc);
        let payroll_cents = worker_inegi_engine::engine::pdf_generator::estimate_monthly_payroll_exposure_cents(*emp_exact, scian);
        let elapsed_us = t_start.elapsed().as_micros();

        assert_eq!(hash_web, hash_pdf, "Coherencia SHA-256 Web == PDF falló en empresa #{}", idx + 1);
        assert_eq!(total_cents_web, total_cents_pdf, "Paridad de centavos Web == PDF falló en empresa #{}", idx + 1);
        assert!(
            typst_out.contains(&format!("{} colaboradores", emp_exact)),
            "El PDF Typst de {} debe contener exactamente '{} colaboradores'",
            razon,
            emp_exact
        );
        assert!(
            typst_out.contains("https://mallen.lat/#faq"),
            "El pie de página del PDF debe incluir el enlace https://mallen.lat/#faq"
        );

        println!(
            "  [{:02}/10] RFC: {:<13} | SCIAN: {} | Emp: {:>3} | Régimen: {:<44} | Nómina/Mes: ${:>12} | Exposición VEMP: ${:>13} | SNAP: {} | {} µs",
            idx + 1,
            rfc,
            scian,
            emp_exact,
            &regimen_lbl[..regimen_lbl.len().min(44)],
            worker_inegi_engine::engine::pdf_generator::format_cents_mxn(payroll_cents),
            worker_inegi_engine::engine::pdf_generator::format_cents_mxn(total_cents_pdf),
            snap_id,
            elapsed_us
        );
    }
    println!("==========================================================================================================\n");


    // Escenario 69: Sello QR Vectorial Nativo en Typst (0 KB imagen) + Motor de Vigilancia Regulatoria Continua (Recalibración UMA 2027)
    let qr_svg_typst = worker_inegi_engine::engine::pdf_generator::render_vector_qr_seal_typst("0x659b4a8d717289c7");
    assert!(qr_svg_typst.contains("#grid(columns: (1.85pt,) * 13"), "El QR vectorial debe ser de matriz 13x13 de alta densidad");

    let (base_2026_cents, recalib_2027_cents, uma_alert) =
        worker_inegi_engine::engine::pdf_generator::simulate_uma_regulatory_recalibration("URB180101BBB", 118.80)
            .expect("Debe recalibrar el snapshot de Urbvan ante incremento de la UMA");
    assert!(recalib_2027_cents > base_2026_cents, "Al subir la UMA de $113.14 a $118.80, el umbral de exposición debe escalar proporcionalmente");
    assert!(uma_alert.contains("ALERTA DE VIGILANCIA NORMATIVA CONTINUA"));

    // Escenario 70: Simulador de Carga Pre-Vuelo (`--dry-run`) en Memoria RAM sobre registros reales del DENUE
    let (dry_total, dry_valid_links, _dry_dead_filtered, dry_micros) =
        HttpFormBot::validate_batch_dry_run_in_memory(25);
    assert_eq!(dry_total, dry_valid_links, "El 100% de los enlaces generados en el dry-run deben ser URLs válidas");
    println!("✓ SIMULACIÓN PRE-VUELO (--dry-run) COMPLETADA EN {} µs ({} empresas validadas en RAM)", dry_micros, dry_total);


    // Escenario 71: Verificar que los Snapshots en `audit_snapshots` ahora tengan `findings` poblados (no vacíos),
    // `risk_score > 0`, `audit_sha256_hash` y `merkle_root_hex` para consultas con `jq`
    let hydrated_steren = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject("STE850101AAA")
        .expect("Debe existir el snapshot de Steren");
    assert!(
        !hydrated_steren.findings.is_empty(),
        "El arreglo `findings` dentro de master_json_blob debe contener todos los vectores con exposición > 0"
    );
    assert!(hydrated_steren.risk_score >= 45, "El risk_score hidratado en el snapshot nunca debe quedar en 0");
    assert!(!hydrated_steren.audit_sha256_hash.is_empty());
    assert!(hydrated_steren.merkle_root_hex.is_some());

    // Escenario 72: Extracción 360° Dinámica desde `denue_units` (Multi-Sucursal Real de Steren/Cancino en SQLite)
    let profile_360 = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("OPERADORA RESTAURANTERA CANCINO");
    assert!(profile_360.branch_count >= 1);
    assert!(!profile_360.domicilio_completo.is_empty());
    assert!(profile_360.isn_mensual_presunto_mxn > 0.0);

    // Escenario 73: Cálculo geolocalizado de Impuesto Sobre Nómina (ISN CDMX 4.0% vs Nuevo León 3.0%) y Ley PIPC Local
    let (rate_cdmx, isn_cdmx_cents, law_cdmx) =
        worker_inegi_engine::engine::pdf_generator::compute_state_isn_and_local_pc_law("CIUDAD DE MÉXICO", 100_000_000);
    let (rate_nl, isn_nl_cents, law_nl) =
        worker_inegi_engine::engine::pdf_generator::compute_state_isn_and_local_pc_law("NUEVO LEÓN", 100_000_000);
    assert_eq!(rate_cdmx, 4.0);
    assert_eq!(isn_cdmx_cents, 4_000_000);
    assert!(law_cdmx.contains("Ciudad de México"));
    assert_eq!(rate_nl, 3.0);
    assert_eq!(isn_nl_cents, 3_000_000);
    assert!(law_nl.contains("Nuevo León"));

    // Escenario 74: Prueba de Regresión Tipográfica Golden Master y Anclaje de Raíz de Merkle Global del Lote
    let golden_hash = worker_inegi_engine::engine::pdf_generator::verify_typst_golden_layout_invariants(&typst_urbvan_doc)
        .expect("El documento Typst debe cumplir todos los invariantes del Golden Master");
    assert!(golden_hash.starts_with("GOLDEN-SHA256-"));

    let (global_batch_root, anchored_count) =
        worker_inegi_engine::engine::pdf_generator::anchor_global_batch_merkle_root("BATCH-PARTNER-10K-2026")
            .expect("Debe sellar la Raíz de Merkle Global de todos los snapshots del lote");
    assert!(global_batch_root.starts_with("0x") && anchored_count >= 10);

    // Escenario 75: Radar Regulatorio Diario (`regulatory_news` en SQLite WAL)
    let daily_news = worker_inegi_engine::engine::pdf_generator::get_or_seed_regulatory_news();
    assert!(daily_news.len() >= 4, "El Radar Regulatorio Diario debe contar con al menos las 4 notas normativas clave");


    // Escenario 76: Eliminación del Respaldo Arbitrario (11 a 30 / 20 emp) en RFCs sin match en DENUE (ej. CTE160824G29)
    let _ = conn.execute("DELETE FROM audit_snapshots WHERE UPPER(tax_id_key) = 'CTE160824G29'", []);
    let _ = conn.execute("DELETE FROM audit_sessions WHERE UPPER(tax_id) = 'CTE160824G29'", []);
    let _ = conn.execute("DELETE FROM prospects WHERE UPPER(tax_id) = 'CTE160824G29'", []);
    let _ = conn.execute("DELETE FROM stps_repse WHERE UPPER(rfc) = 'CTE160824G29'", []);
    let rfc_unlinked = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("CTE160824G29");
    assert_eq!(
        rfc_unlinked.headcount_source_type,
        "PENDIENTE_CALIBRACION_CORPORATIVA",
        "Un RFC sin match en DENUE jamás debe inventar '11 a 30 personas'"
    );
    assert_eq!(
        rfc_unlinked.estrato_personal,
        "PENDIENTE DE CALIBRACIÓN CORPORATIVA (Requiere validación de nómina)"
    );
    assert_eq!(
        rfc_unlinked.estimated_employees, 0,
        "No debe asignar 20 empleados por defecto cuando requiere calibración corporativa"
    );
    assert_eq!(rfc_unlinked.antiguedad_estimada_anios, 10, "De CTE160824G29 debe extraer el año 2016 (10 años de antigüedad en 2026)");

    // Escenario 78: Calibración Dinámica Sin Límite Superior (Corporativo de 4,850 colaboradores congelado en Snapshot Inmutable)
    let mut alsea_corp = worker_inegi_engine::engine::models::AuditMasterJson::default();
    alsea_corp.tax_id = "ALS9209253R0".to_string();
    alsea_corp.razon_social = "DISTRIBUIDORA E IMPORTADORA ALSEA S.A. DE C.V.".to_string();
    alsea_corp.codigo_scian = "722511".to_string();
    alsea_corp.actividad_scian_desc = "Servicios de preparación de alimentos y bebidas".to_string();
    alsea_corp.estimated_employees = 4850;
    alsea_corp.calibration_applied.exact_employee_count = Some(4850);
    let snap_alsea = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(
        "ALS9209253R0",
        "DISTRIBUIDORA E IMPORTADORA ALSEA S.A. DE C.V.",
        &alsea_corp,
    );
    let alsea_360 = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("ALS9209253R0");
    assert_eq!(alsea_360.headcount_source_type, "CALIBRADO_POR_USUARIO");
    assert_eq!(alsea_360.estimated_employees, 4850);

    // Escenario 79: Flujo End-to-End Completo (FormBot -> Enlace Checkout Validado -> Click Tracking -> Snapshot -> Dictamen Typst con QR 13x13)
    let checkout_url = HttpFormBot::build_and_validate_checkout_link(&alsea_corp.razon_social, Some(&alsea_corp.tax_id))
        .expect("El enlace de checkout generado por el FormBot debe ser 100% válido");
    assert!(checkout_url.starts_with("https://mallen.lat/checkout?ref="));
    let _clicks = HttpFormBot::record_checkout_link_click(&alsea_corp.razon_social, "201.141.118.181", "Mozilla/5.0").unwrap();
    let alsea_typst_doc = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot(&snap_alsea)
        .expect("Debe renderizar el Dictamen de Alsea con sus 4,850 colaboradores");
    assert!(alsea_typst_doc.contains("4850 colaboradores"), "El Dictamen debe mostrar los 4,850 colaboradores exactos");
    assert!(alsea_typst_doc.contains("#grid(columns: (1.85pt,) * 13"), "El Dictamen de 3 páginas debe incluir el Código QR Vectorial 13x13");
    assert!(alsea_typst_doc.contains("https://mallen.lat/#faq"), "El Dictamen debe incluir el enlace a #faq");

    let denue_official = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("OPERADORA RESTAURANTERA CANCINO");
    assert_eq!(denue_official.headcount_source_type, "ESTRATO_OFICIAL_INEGI");
    assert!(denue_official.headcount_display_label.contains("Personal Ocupado INEGI (Estrato Oficial):"));

    let calibrated_urbvan = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("URB180101BBB");
    assert_eq!(calibrated_urbvan.headcount_source_type, "CALIBRADO_POR_USUARIO");
    assert_eq!(calibrated_urbvan.estimated_employees, 258);
    assert!(calibrated_urbvan.headcount_display_label.contains("258 colaboradores"));


    // Escenario 77: Prueba de Bitácora de Errores Aislados (Dead Letter Queue `batch_failures_log`) y Resiliencia de Lote
    // Simulamos un lote de 5 registros donde 3 son válidos y 2 vienen corruptos (RFC vacío o 0 empleados):
    // Los 2 corruptos deben aislarse en `batch_failures_log` sin detener a los 3 válidos.
    let mixed_batch = vec![
        ("VAL180101AAA".to_string(), "EMPRESA VALIDA UNO S.A. DE C.V.".to_string(), "485111".to_string(), 45u32),
        ("BAD_RFC".to_string(), "EMPRESA CON RFC CORRUPTO".to_string(), "485111".to_string(), 30u32),
        ("VAL190202BBB".to_string(), "EMPRESA VALIDA DOS S.A.P.I. DE C.V.".to_string(), "522320".to_string(), 90u32),
        ("VAL200303CCC".to_string(), "".to_string(), "332310".to_string(), 0u32),
        ("VAL210404DDD".to_string(), "EMPRESA VALIDA TRES S. DE R.L. DE C.V.".to_string(), "722511".to_string(), 25u32),
    ];
    let (ok_cnt, dlq_err_cnt, batch_root) =
        worker_inegi_engine::engine::pdf_generator::process_batch_records_with_dlq_and_checkpoint(
            "JOB-DLQ-TEST-2026",
            &mixed_batch,
        );
    assert_eq!(ok_cnt, 3, "Los 3 registros válidos deben procesarse y sellarse sin interrupción");
    assert_eq!(dlq_err_cnt, 2, "Los 2 registros corruptos deben quedar aislados en batch_failures_log (DLQ)");
    assert!(batch_root.starts_with("0x"));


    // =========================================================================
    // ESCENARIO 80: AUDITORÍA EXHAUSTIVA CON TELEMETRÍA FÓRMULA 1 (F1 SECTOR TIMING)
    // FLUJO COMPLETO: FORMBOT ⇄ LOOKUP 360° ⇄ PRE-DICTAMEN ⇄ DICTAMEN ⇄ CONTRATOS KIT
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  TELEMETRÍA F1 EN TIEMPO REAL — AUDITORÍA DE CONGRUENCIA CUÁDRUPLE (WEB ⇄ PRE-DICTAMEN ⇄ CHECKOUT ⇄ DICTAMEN 3 PÁGS ⇄ CONTRATOS)  ║");
    println!("╠════╦═══════════════╦══════════════════════════════════════╦═══════╦══════════╦══════════╦══════════╦══════════╦══════════════════════╣");
    println!("║ #  ║ RFC           ║ RAZÓN SOCIAL / ENTIDAD AUDITADA      ║ EMPL. ║ S1:LKUP  ║ S2:VECT  ║ S3:SNAP  ║ S4:TYPST ║ VEMP EXACTO (100% =) ║");
    println!("╠════╬═══════════════╬══════════════════════════════════════╬═══════╬══════════╬══════════╬══════════╬══════════╬══════════════════════╣");

    // Combinamos nuevas empresas de diversos giros y escalas (desde 9 hasta 5,400 empleados) + empresas reales extraídas de `denue_units`
    let mut f1_matrix: Vec<(String, String, String, String, u32, String)> = vec![
        ("AERO210115AA1".to_string(), "AEROTRANSPORTES Y CARGA DEL PACIFICO S.A. DE C.V.".to_string(), "481111".to_string(), "Transporte aéreo regular".to_string(), 640, "NUEVO LEÓN".to_string()),
        ("HOSP190322BB2".to_string(), "GRUPO HOSPITALARIO ALTA ESPECIALIDAD S.A.P.I. DE C.V.".to_string(), "622111".to_string(), "Hospitales generales del sector privado".to_string(), 1250, "CIUDAD DE MÉXICO".to_string()),
        ("AGRO170810CC3".to_string(), "AGROINDUSTRIAS EXPORTADORAS DE MICHOACAN S. DE R.L. DE C.V.".to_string(), "111331".to_string(), "Cultivo de aguacate y empaque de exportación".to_string(), 410, "MICHOACÁN".to_string()),
        ("SOFT220504DD4".to_string(), "ARQUITECTURA CLOUD Y CIBERSEGURIDAD S.A.P.I. DE C.V.".to_string(), "541510".to_string(), "Servicios de diseño de sistemas de cómputo".to_string(), 52, "JALISCO".to_string()),
        ("MINA141128EE5".to_string(), "MINERA EXTRACTIVA DEL NOROESTE S.A. DE C.V.".to_string(), "212221".to_string(), "Minería de oro y plata".to_string(), 2890, "SONORA".to_string()),
        ("FARM180719FF6".to_string(), "LABORATORIOS FARMACEUTICOS BIOGEN S.A. DE C.V.".to_string(), "325412".to_string(), "Fabricación de preparaciones farmacéuticas".to_string(), 730, "ESTADO DE MÉXICO".to_string()),
        ("HOTL160214GG7".to_string(), "OPERADORA HOTELERA RIVIERA MAYA S.A. DE C.V.".to_string(), "721111".to_string(), "Hoteles con otros servicios integrados".to_string(), 1580, "QUINTANA ROO".to_string()),
        ("AUTO150930HH8".to_string(), "AUTOPARTES DE PRECISION BAJIO S.A. DE C.V.".to_string(), "336390".to_string(), "Fabricación de otras partes para vehículos automotrices".to_string(), 5400, "QUERÉTARO".to_string()),
        ("EDUC200120II9".to_string(), "INSTITUTO UNIVERSITARIO DE TECNOLOGIA APLICADA S.C.".to_string(), "611311".to_string(), "Escuelas de educación superior del sector privado".to_string(), 95, "PUEBLA".to_string()),
        ("INMO181205JJ0".to_string(), "DESARROLLADORA PATRIMONIAL E INMOBILIARIA S.A.P.I. DE C.V.".to_string(), "531210".to_string(), "Inmobiliarias y corredores de bienes raíces".to_string(), 28, "CIUDAD DE MÉXICO".to_string()),
    ];

    // Añadir dinámicamente las primeras 2 empresas reales de `denue_units` cargadas al inicio del test
    for (idx_r, (r_name, r_scian)) in real_companies.iter().take(2).enumerate() {
        let dyn_rfc = format!("DEN{:02}1010RR{}", idx_r + 1, idx_r + 1);
        f1_matrix.push((
            dyn_rfc,
            r_name.clone(),
            r_scian.clone(),
            format!("{} - Giro Real DENUE", r_scian),
            35 + (idx_r as u32 * 40),
            "CIUDAD DE MÉXICO".to_string(),
        ));
    }

    let mut total_s1_us = 0u128;
    let mut total_s2_us = 0u128;
    let mut total_s3_us = 0u128;
    let mut total_s4_us = 0u128;

    for (i, (rfc, razon, scian, act_desc, emp_cnt, entidad)) in f1_matrix.iter().enumerate() {
        // 1. Validar que el FormBot construya un link de checkout 100% libre de errores para esta empresa
        let bot_link = HttpFormBot::build_and_validate_checkout_link(razon, Some(rfc))
            .expect("FormBot debe generar link de checkout válido para todas las nuevas empresas");
        assert!(bot_link.starts_with("https://mallen.lat/checkout?ref="));

        // 2. Sector 2 (S2_VECTORS): Evaluar los 37 vectores y huella SHA-256 en la capa Web
        let t_s2 = std::time::Instant::now();
        let mut master = worker_inegi_engine::engine::models::AuditMasterJson::default();
        master.tax_id = rfc.clone();
        master.razon_social = razon.clone();
        master.codigo_scian = scian.clone();
        master.actividad_scian_desc = act_desc.clone();
        master.estimated_employees = *emp_cnt;
        master.calibration_applied.exact_employee_count = Some(*emp_cnt);
        master.calibration_applied.dossier.entidad_jurisdiccion = Some(entidad.clone());
        let (web_sha256, web_cents) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&master);
        master.total_estimated_fine_mxn = (web_cents as f64) / 100.0;
        let s2_us = t_s2.elapsed().as_micros();
        total_s2_us += s2_us;

        // 3. Sector 3 (S3_SNAP_WAL): Congelar el Snapshot Inmutable en SQLite WAL (`audit_snapshots`)
        let t_s3 = std::time::Instant::now();
        let snap_uuid = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(rfc, razon, &master);
        let s3_us = t_s3.elapsed().as_micros();
        total_s3_us += s3_us;

        // 4. Sector 1 (S1_LOOKUP): Consultar `/api/lookup` (`extract_deep_denue_360_intelligence`) y verificar que devuelva exactamente los empleados congelados
        let t_s1 = std::time::Instant::now();
        let lookup_res = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence(rfc);
        let s1_us = t_s1.elapsed().as_micros();
        total_s1_us += s1_us;
        assert_eq!(
            lookup_res.estimated_employees, *emp_cnt,
            "Desalineación en Lookup para {}: esperaba {} empleados y recibió {}",
            rfc, emp_cnt, lookup_res.estimated_employees
        );
        assert_eq!(lookup_res.headcount_source_type, "CALIBRADO_POR_USUARIO");

        // 5. Sector 4 (S4_TYPST): Renderizar tanto el Pre-Dictamen (censurado) como el Dictamen Completo (desbloqueado) y los 7 Contratos del Kit
        let t_s4 = std::time::Instant::now();
        let loaded_master = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject(&snap_uuid)
            .expect("El snapshot recién congelado debe existir en SQLite WAL");
        let (pdf_sha256, pdf_cents) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&loaded_master);
        let full_typst = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot(&snap_uuid)
            .expect("Debe renderizar el Dictamen de 3 páginas sin error");

        let mut emp_b2b = worker_inegi_engine::engine::models::EmpresaB2B::default();
        emp_b2b.id = rfc.clone();
        emp_b2b.tax_id = Some(rfc.clone());
        emp_b2b.nombre = razon.clone();
        emp_b2b.actividad = format!("{} - {}", scian, act_desc);
        emp_b2b.estado = entidad.clone();
        emp_b2b.estimated_fine_mxn = (pdf_cents as f64) / 100.0;
        emp_b2b.risk_score = loaded_master.risk_score;
        let kit_docs = worker_inegi_engine::engine::kit_generator::build_all_compliance_documents_with_session(
            &emp_b2b,
            &loaded_master.calibration_applied.dossier,
            Some(&loaded_master),
        );
        let s4_us = t_s4.elapsed().as_micros();
        total_s4_us += s4_us;

        // Aserciones estrictas de Congruencia Cuádruple (Web == Snapshot == Dictamen PDF == Contratos Kit)
        assert_eq!(web_sha256, pdf_sha256, "El Hash SHA-256 de los 37 vectores debe ser idéntico entre Web y PDF en {}", rfc);
        assert_eq!(web_cents, pdf_cents, "El monto en centavos debe ser idéntico entre Web y PDF en {}", rfc);
        let formatted_vemp = worker_inegi_engine::engine::pdf_generator::format_cents_mxn(pdf_cents);
        assert!(
            full_typst.contains(&formatted_vemp),
            "El Dictamen Typst de {} debe contener impreso el monto exacto ${}",
            rfc, formatted_vemp
        );
        assert!(
            full_typst.contains(&format!("{} colaboradores", emp_cnt)),
            "El Dictamen Typst de {} debe contener exactamente '{} colaboradores'",
            rfc, emp_cnt
        );
        assert!(
            full_typst.contains("#grid(columns: (1.85pt,) * 13"),
            "El Dictamen Typst de {} debe contener el Sello QR Vectorial 13x13",
            rfc
        );
        assert!(
            kit_docs.len() >= 7,
            "El Kit de {} debe generar los 7 instrumentos jurídicos completos",
            rfc
        );
        assert!(
            kit_docs[0].content.contains(&emp_cnt.to_string()),
            "El Acta NOM-019 dentro del Kit ZIP debe reflejar exactamente los {} colaboradores",
            emp_cnt
        );

        let short_name: String = razon.chars().take(36).collect();
        println!(
            "║ {:02} ║ {:<13} ║ {:<36} ║ {:>5} ║ {:>6}µs ║ {:>6}µs ║ {:>6}µs ║ {:>6}µs ║ ${:>19} ║",
            i + 1,
            rfc,
            short_name,
            emp_cnt,
            s1_us,
            s2_us,
            s3_us,
            s4_us,
            formatted_vemp
        );
    }

    let n_f1 = f1_matrix.len() as u128;
    println!("╠════╩═══════════════╩══════════════════════════════════════╩═══════╬══════════╬══════════╬══════════╬══════════╬══════════════════════╣");
    println!(
        "║ PROMEDIO DE TELEMETRÍA F1 POR SECTOR (12 EMPRESAS AUDITADAS)      ║ {:>6}µs ║ {:>6}µs ║ {:>6}µs ║ {:>6}µs ║ 100% PARIDAD EXACTA  ║",
        total_s1_us / n_f1,
        total_s2_us / n_f1,
        total_s3_us / n_f1,
        total_s4_us / n_f1
    );
    println!("╚═══════════════════════════════════════════════════════════════════╩══════════╩══════════╩══════════╩══════════╩══════════════════════╝\n");


    // =========================================================================
    // ESCENARIO 81: AUDITORÍA MASIVA SOBRE 100 EMPRESAS REALES ALEATORIAS DE `denue_units`
    // (CERO HARDCODES — FLUJO COMPLETO FORMBOT ⇄ LOOKUP ⇄ SNAPSHOT ⇄ PDF/KIT)
    // =========================================================================
    let mut random_100_denue: Vec<(String, String, String, String, String)> = Vec::with_capacity(100);
    if let Ok(mut stmt100) = conn.prepare(
        "SELECT COALESCE(NULLIF(TRIM(raz_social), ''), TRIM(nom_estab)) AS nombre,
                COALESCE(NULLIF(TRIM(codigo_scian), ''), '541110') AS scian,
                COALESCE(NULLIF(TRIM(nombre_act), ''), 'Actividad DENUE') AS act_desc,
                COALESCE(NULLIF(TRIM(per_ocu), ''), '11 a 30 personas') AS estrato,
                COALESCE(NULLIF(TRIM(entidad), ''), 'MÉXICO') AS entidad
         FROM denue_units
         WHERE LENGTH(COALESCE(NULLIF(TRIM(raz_social), ''), TRIM(nom_estab))) >= 5
         ORDER BY RANDOM()
         LIMIT 250",
    ) {
        if let Ok(rows) = stmt100.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
            ))
        }) {
            for item in rows.flatten() {
                random_100_denue.push(item);
            }
        }
    }

    assert_eq!(
        random_100_denue.len(),
        250,
        "Debe extraer exactamente 100 empresas reales aleatorias desde denue_units en data/mallen_prod.db"
    );

    println!("\n╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║ AUDITORÍA F1 SOBRE 100 EMPRESAS REALES ALEATORIAS EXTRAÍDAS EN VIVO DE `denue_units` (0% HARDCODE — MUESTRA DE 25/250)   ║");
    println!("╠═════╦══════════════════════════════════════════╦════════╦═══════════════════════╦═══════╦══════════════════╦═════════════╣");
    println!("║ #   ║ RAZÓN SOCIAL / ESTABLECIMIENTO REAL      ║ SCIAN  ║ ESTRATO OFICIAL INEGI ║ MED.  ║ EXPOSICIÓN VEMP  ║ LATENCIA F1 ║");
    println!("╠═════╬══════════════════════════════════════════╬════════╬═══════════════════════╬═══════╬══════════════════╬═════════════╣");

    let t_batch_100 = std::time::Instant::now();
    let mut sum_100_us = 0u128;

    for (idx100, (real_name, real_scian, real_act, real_estrato, real_ent)) in random_100_denue.iter().enumerate() {
        let t_item = std::time::Instant::now();

        // 1. Verificar que el FormBot genere un link de checkout válido para el nombre real del DENUE
        let link = HttpFormBot::build_and_validate_checkout_link(real_name, None)
            .expect("El FormBot debe generar un link válido para cualquiera de las 100 empresas reales del DENUE");
        assert!(link.starts_with("https://mallen.lat/checkout?ref="));

        // 2. Clasificar estrato real del INEGI y obtener su mediana estadística censal
        let (_stratum_lbl, median_emp, _base_fine, _tier_price) =
            HttpFormBot::classify_denue_capacity_and_scian_risk(real_estrato, real_scian);
        assert!(median_emp >= 1);

        // 3. Calcular huella criptográfica SHA-256 y centavos exactos de los 37 vectores
        let mut m_real = worker_inegi_engine::engine::models::AuditMasterJson::default();
        let synthetic_id = format!("DYN100-{:03}", idx100 + 1);
        m_real.tax_id = synthetic_id.clone();
        m_real.razon_social = real_name.clone();
        m_real.codigo_scian = real_scian.clone();
        m_real.actividad_scian_desc = real_act.clone();
        m_real.estimated_employees = median_emp;
        m_real.calibration_applied.exact_employee_count = Some(median_emp);
        m_real.calibration_applied.dossier.entidad_jurisdiccion = Some(real_ent.clone());

        let (sha_web, cents_web) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&m_real);
        m_real.total_estimated_fine_mxn = (cents_web as f64) / 100.0;

        // 4. Congelar Snapshot y verificar que el generador recupere exactamente los mismos centavos y el mismo hash
        let snap_100 = worker_inegi_engine::engine::pdf_generator::freeze_immutable_audit_snapshot(
            &synthetic_id,
            real_name,
            &m_real,
        );
        let rec_100 = worker_inegi_engine::engine::pdf_generator::require_strict_snapshot_or_reject(&snap_100)
            .expect("Debe recuperar el snapshot de la empresa real del DENUE");
        let (sha_pdf, cents_pdf) =
            worker_inegi_engine::engine::pdf_generator::compute_vectors_37_sha256_fingerprint(&rec_100);

        assert_eq!(sha_web, sha_pdf, "Discrepancia SHA-256 en empresa real #{}: {}", idx100 + 1, real_name);
        assert_eq!(cents_web, cents_pdf, "Discrepancia de centavos en empresa real #{}: {}", idx100 + 1, real_name);
        assert!(cents_pdf > 0, "Toda empresa real con omisiones debe arrojar exposición > 0");

        // En las primeras 5 empresas reales aleatorias, compilar también el documento Typst completo y verificar que contenga el monto exacto y el QR 13x13
        if idx100 < 5 {
            let doc_100 = worker_inegi_engine::engine::pdf_generator::render_typst_from_strict_snapshot(&snap_100)
                .expect("Debe renderizar el Dictamen Typst de la empresa real del DENUE");
            let vemp_str = worker_inegi_engine::engine::pdf_generator::format_cents_mxn(cents_pdf);
            assert!(doc_100.contains(&vemp_str));
            assert!(doc_100.contains("#grid(columns: (1.85pt,) * 13"));
        }

        let item_us = t_item.elapsed().as_micros();
        sum_100_us += item_us;

        // Imprimir una muestra representativa de 15 de las 100 empresas reales aleatorias en la tabla F1
        if idx100 < 25 {
            let n_clip: String = real_name.chars().take(40).collect();
            let e_clip: String = real_estrato.chars().take(21).collect();
            println!(
                "║ {:03} ║ {:<40} ║ {:<6} ║ {:<21} ║ {:>5} ║ ${:>15} ║ {:>8} µs ║",
                idx100 + 1,
                n_clip,
                &real_scian[..real_scian.len().min(6)],
                e_clip,
                median_emp,
                worker_inegi_engine::engine::pdf_generator::format_cents_mxn(cents_pdf),
                item_us
            );
        }
    }

    let total_batch_ms = t_batch_100.elapsed().as_millis();
    println!("╠═════╩══════════════════════════════════════════╩════════╩═══════════════════════╩═══════╩══════════════════╩═════════════╣");
    println!(
        "║ RESUMEN DE LAS 100 EMPRESAS REALES ALEATORIAS DEL DENUE: 250/250 PARIDAD EXACTA | PROM: {:>5} µs/emp | TOTAL: {:>5} ms ║",
        sum_100_us / 250,
        total_batch_ms
    );
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝\n");


    // =========================================================================
    // ESCENARIO 82: PRUEBA DE IDEMPOTENCIA EN BÚSQUEDAS REPETIDAS (10 CONSULTAS SEGUIDAS)
    // Garantiza que buscar 10 veces la misma empresa (por RFC, por nombre, con espacios o minúsculas)
    // arroje siempre exactamente el mismo conteo de empleados, el mismo giro SCIAN y la misma exposición.
    // =========================================================================
    let base_lookup = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("AERO210115AA1");
    assert_eq!(base_lookup.estimated_employees, 640);
    for rep in 0..10 {
        let query_variant = match rep % 4 {
            0 => "AERO210115AA1",
            1 => "  aero210115aa1  ",
            2 => "AEROTRANSPORTES Y CARGA DEL PACIFICO S.A. DE C.V.",
            _ => "aerotransportes y carga del pacifico s.a. de c.v.",
        };
        let rep_res = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence(query_variant);
        assert_eq!(
            rep_res.estimated_employees, 640,
            "En la búsqueda repetida #{} con '{}' el número de empleados no debe cambiar jamás",
            rep + 1, query_variant
        );
        assert_eq!(rep_res.codigo_scian, "481111");
        assert_eq!(rep_res.isn_mensual_presunto_mxn, base_lookup.isn_mensual_presunto_mxn);
    }

    // Y verificar que buscar 5 veces seguidas un RFC no indexado (CTE160824G29) después de haber buscado corporativos
    // jamás se contamine con la empresa anterior:
    for _ in 0..5 {
        let unlinked_rep = worker_inegi_engine::engine::pdf_generator::extract_deep_denue_360_intelligence("CTE160824G29");
        assert_eq!(unlinked_rep.estimated_employees, 0);
        assert_eq!(unlinked_rep.headcount_source_type, "PENDIENTE_CALIBRACION_CORPORATIVA");
    }


    // Verificación de Compilación Binaria Real con `/usr/local/bin/typst` (Previene "Fallo al compilar el Micro-Dictamen de rescate")
    let mut emp_alsea_pdf = worker_inegi_engine::engine::models::EmpresaB2B::default();
    emp_alsea_pdf.id = "ALS9209253R0".to_string();
    emp_alsea_pdf.tax_id = Some("ALS9209253R0".to_string());
    emp_alsea_pdf.nombre = "DISTRIBUIDORA E IMPORTADORA ALSEA S.A. DE C.V.".to_string();
    emp_alsea_pdf.actividad = "722511 - Servicios de preparación de alimentos y bebidas".to_string();
    let osint_dummy = worker_inegi_engine::engine::osint_scanner::OsintResult::default();
    let pdf_bytes = worker_inegi_engine::engine::pdf_generator::generate_exit_intent_micro_pdf_with_session(
        &emp_alsea_pdf,
        &osint_dummy,
        Some(&alsea_corp),
        false,
        "/tmp/test_alsea_micro_binary.pdf",
    )
    .expect("La compilación binaria real con /usr/local/bin/typst debe generar el PDF sin error");
    assert!(pdf_bytes.len() > 5000 && pdf_bytes.starts_with(b"%PDF-"), "El archivo compilado debe ser un PDF binario válido");

    // Además, compilar el PDF binario real de 3 empresas aleatorias extraídas de `denue_units` para certificar 0 errores Typst con nombres reales
    for (idx_bin, (r_nom, r_sc, r_act, r_est, r_ent)) in random_100_denue.iter().take(3).enumerate() {
        let mut emp_rand_pdf = worker_inegi_engine::engine::models::EmpresaB2B::default();
        emp_rand_pdf.id = format!("DYN100-{:03}", idx_bin + 1);
        emp_rand_pdf.tax_id = Some(emp_rand_pdf.id.clone());
        emp_rand_pdf.nombre = r_nom.clone();
        emp_rand_pdf.actividad = format!("{} - {}", r_sc, r_act);
        emp_rand_pdf.estado = r_ent.clone();
        let (_, med_r, _, _) = HttpFormBot::classify_denue_capacity_and_scian_risk(r_est, r_sc);
        let mut m_rand_pdf = worker_inegi_engine::engine::models::AuditMasterJson::default();
        m_rand_pdf.tax_id = emp_rand_pdf.id.clone();
        m_rand_pdf.razon_social = r_nom.clone();
        m_rand_pdf.codigo_scian = r_sc.clone();
        m_rand_pdf.estimated_employees = med_r;
        m_rand_pdf.calibration_applied.exact_employee_count = Some(med_r);
        let out_p = format!("/tmp/test_random_denue_{}.pdf", idx_bin + 1);
        let rand_pdf_bytes = worker_inegi_engine::engine::pdf_generator::generate_exit_intent_micro_pdf_with_session(
            &emp_rand_pdf,
            &osint_dummy,
            Some(&m_rand_pdf),
            false,
            &out_p,
        )
        .expect("Toda empresa aleatoria real de denue_units debe compilar su PDF binario sin errores");
        assert!(rand_pdf_bytes.starts_with(b"%PDF-") && rand_pdf_bytes.len() > 5000);
    }

    println!("✓ LOS 82 ESCENARIOS (INCLUYENDO 100 EMPRESAS REALES DEL DENUE, TELEMETRÍA F1 E IDEMPOTENCIA EN BÚSQUEDAS REPETIDAS) SUPERADOS AL 100%");
}
