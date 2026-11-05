use axum::{
    extract::Form,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use reqwest::Client;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use worker_inegi_engine::engine::http_form_bot::HttpFormBot;

#[derive(Default)]
struct CapturedSubmissions {
    wp_payload: Mutex<Option<HashMap<String, String>>>,
    legacy_payload: Mutex<Option<HashMap<String, String>>>,
    select_payload: Mutex<Option<HashMap<String, String>>>,
}

#[tokio::test]
async fn test_http_form_bot_all_8_real_world_scenarios() {
    let state = Arc::new(CapturedSubmissions::default());

    let state_wp = state.clone();
    let state_legacy = state.clone();
    let state_select = state.clone();

    let app = Router::new()
        // ESCENARIO 1: WordPress Contact Form 7 con CSRF + 3 tipos de Honeypot
        .route(
            "/wp-contact",
            get(|| async {
                Html(
                    r#"<!DOCTYPE html>
                    <html><head><meta charset="UTF-8"></head><body>
                        <form role="search" action="/search"><input name="q"></form>
                        <form action="/wp-contact/submit" method="POST">
                            <input type="hidden" name="_wpcf7" value="99812">
                            <input type="hidden" name="_wpnonce" value="sec_token_abc123">
                            <input type="text" name="website_url" style="display:none" value="">
                            <input type="text" name="trap_field" tabindex="-1" value="">
                            <input type="text" name="your-name" required>
                            <input type="email" name="your-email" required>
                            <input type="tel" name="your-tel">
                            <textarea name="your-message"></textarea>
                            <input type="submit" value="Enviar">
                        </form>
                    </body></html>"#,
                )
            }),
        )
        .route(
            "/wp-contact/submit",
            post(move |Form(params): Form<HashMap<String, String>>| {
                let st = state_wp.clone();
                async move {
                    // Si el bot cayó en el honeypot, el servidor devuelve 400
                    if params.contains_key("website_url") || params.contains_key("trap_field") {
                        return (StatusCode::BAD_REQUEST, "Honeypot triggered").into_response();
                    }
                    // Si el bot perdió el token CSRF oculto, devuelve 403
                    if params.get("_wpnonce").map(|s| s.as_str()) != Some("sec_token_abc123") {
                        return (StatusCode::FORBIDDEN, "Missing CSRF").into_response();
                    }
                    *st.wp_payload.lock().unwrap() = Some(params);
                    (StatusCode::OK, "Gracias por su mensaje").into_response()
                }
            }),
        )
        // ESCENARIO 2: Servidor Legacy ISO-8859-1 (Prueba de codificación sin corromper caracteres)
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
        // ESCENARIO 3: Formulario Corporativo con <select> y Checkbox de Privacidad obligatorio
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
        // ESCENARIO 4: Cloudflare Turnstile activo en el formulario
        .route(
            "/protected-turnstile",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <input name="email" type="email">
                            <textarea name="mensaje"></textarea>
                            <div class="cf-turnstile" data-sitekey="0x4AAAAAA"></div>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        // ESCENARIO 5: Google reCAPTCHA activo en el formulario
        .route(
            "/protected-recaptcha",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <input name="email" type="email">
                            <textarea name="message"></textarea>
                            <div class="g-recaptcha" data-sitekey="6Lc_test"></div>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        // ESCENARIO 6: hCaptcha activo en el formulario
        .route(
            "/protected-hcaptcha",
            get(|| async {
                Html(
                    r#"<html><body>
                        <form action="/send" method="POST">
                            <input name="email" type="email">
                            <textarea name="message"></textarea>
                            <div class="h-captcha" data-sitekey="10000000-ffff-ffff-ffff-000000000001"></div>
                        </form>
                    </body></html>"#,
                )
            }),
        )
        // ESCENARIO 7: Bloqueo de página completa por Cloudflare WAF (403 cf-mitigated)
        .route(
            "/cloudflare-waf-block",
            get(|| async {
                let mut headers = HeaderMap::new();
                headers.insert("server", "cloudflare".parse().unwrap());
                headers.insert("cf-mitigated", "challenge".parse().unwrap());
                (
                    StatusCode::FORBIDDEN,
                    headers,
                    Html("<html><head><title>Just a moment...</title></head><body><script>window._cf_chl_opt={};</script></body></html>"),
                )
            }),
        )
        // ESCENARIO 8: Sitio sin formulario de contacto (solo login y búsqueda)
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

    let client = Client::builder().build().unwrap();

    // Prueba 1: WordPress con CSRF y Honeypots
    let r1 = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-contact", base), "DISTRIBUIDORA ALSEA", 1245958.00).await;
    assert_eq!(r1, "PROSPECCION_ENVIADA_EXITO");
    let wp_data = state.wp_payload.lock().unwrap().clone().unwrap();
    assert!(!wp_data.contains_key("website_url"), "No debe rellenar honeypot display:none");
    assert!(!wp_data.contains_key("trap_field"), "No debe rellenar honeypot tabindex=-1");
    assert_eq!(wp_data.get("_wpnonce").unwrap(), "sec_token_abc123", "Debe preservar CSRF token");
    assert!(wp_data.get("your-message").unwrap().contains("https://mallen.lat/checkout?ref=DISTRIBUIDORA%20ALSEA"), "URL ref debe estar codificada sin espacios crudos");

    // Prueba 2: Servidor Legacy ISO-8859-1 (ASCII Puro sin caracteres corruptos)
    let r2 = HttpFormBot::submit_form_direct(&client, &format!("{}/legacy-iso", base), "CAMIONES Y GRÚAS DEL PACÍFICO", 850000.50).await;
    assert_eq!(r2, "PROSPECCION_ENVIADA_EXITO");
    let leg_data = state.legacy_payload.lock().unwrap().clone().unwrap();
    let leg_msg = leg_data.get("mensaje").unwrap();
    assert!(leg_msg.is_ascii(), "En servidores ISO-8859-1 el mensaje debe ser 100% ASCII limpio para evitar corrupción de tildes/®");
    assert!(leg_msg.contains("CAMIONES Y GRUAS DEL PACIFICO"));

    // Prueba 3: Formulario con <select> y Checkbox de Privacidad
    let r3 = HttpFormBot::submit_form_direct(&client, &format!("{}/corp-select", base), "GRUPO INDUSTRIAL BIMBO", 940000.0).await;
    assert_eq!(r3, "PROSPECCION_ENVIADA_EXITO");

    // Prueba 4: Detección honesta de Cloudflare Turnstile
    let r4 = HttpFormBot::submit_form_direct(&client, &format!("{}/protected-turnstile", base), "EMPRESA TURNSTILE", 500000.0).await;
    assert_eq!(r4, "PROTECCION_ANTIBOT_CLOUDFLARE_TURNSTILE");

    // Prueba 5: Detección honesta de Google reCAPTCHA
    let r5 = HttpFormBot::submit_form_direct(&client, &format!("{}/protected-recaptcha", base), "EMPRESA RECAPTCHA", 500000.0).await;
    assert_eq!(r5, "PROTECCION_ANTIBOT_GOOGLE_RECAPTCHA");

    // Prueba 6: Detección honesta de hCaptcha
    let r6 = HttpFormBot::submit_form_direct(&client, &format!("{}/protected-hcaptcha", base), "EMPRESA HCAPTCHA", 500000.0).await;
    assert_eq!(r6, "PROTECCION_ANTIBOT_HCAPTCHA");

    // Prueba 7: Detección de bloqueo Cloudflare WAF Challenge (403)
    let r7 = HttpFormBot::submit_form_direct(&client, &format!("{}/cloudflare-waf-block", base), "EMPRESA WAF", 500000.0).await;
    assert_eq!(r7, "BLOQUEO_WAF_CLOUDFLARE_CHALLENGE");

    // Prueba 8: Sitio sin formulario de contacto (solo buscador y login)
    let r8 = HttpFormBot::submit_form_direct(&client, &format!("{}/only-login-search", base), "EMPRESA SIN FORM", 500000.0).await;
    assert_eq!(r8, "SIN_FORMULARIO_CONTACTO");

    println!("✓ LOS 8 ESCENARIOS DEL FORM BOT SUPERADOS AL 100%");
}
