use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

pub struct HttpFormBot;

impl HttpFormBot {
    /// Convierte texto con tildes/símbolos a ASCII puro seguro para servidores legacy (ISO-8859-1)
    /// o formularios antiguos que corrompen caracteres multibyte.
    pub fn to_ascii_safe(input: &str) -> String {
        input
            .replace('Á', "A").replace('á', "a")
            .replace('É', "E").replace('é', "e")
            .replace('Í', "I").replace('í', "i")
            .replace('Ó', "O").replace('ó', "o")
            .replace('Ú', "U").replace('ú', "u")
            .replace('Ü', "U").replace('ü', "u")
            .replace('Ñ', "N").replace('ñ', "n")
            .replace('®', "")
            .replace('°', "")
            .replace('—', "-")
            .replace('–', "-")
    }

    /// Codifica de forma segura un identificador para querystring (?ref=...) sin espacios crudos
    pub fn encode_url_ref(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        for b in raw.trim().bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => out.push(b as char),
                b' ' => out.push_str("%20"),
                _ => out.push_str(&format!("%{:02X}", b)),
            }
        }
        out
    }

    /// Detecta si el documento HTML o un formulario específico tiene una pasarela Anti-Bot activa
    /// (Cloudflare Turnstile, Challenge JS, Google reCAPTCHA v2/v3, hCaptcha).
    pub fn detect_antibot_protection(html_lower: &str, form_html_lower: &str) -> Option<&'static str> {
        // 1. Bloqueo de página completa por Cloudflare Challenge / Interstitial
        if (html_lower.contains("just a moment...") || html_lower.contains("attention required! | cloudflare"))
            && (html_lower.contains("__cf_chl_") || html_lower.contains("_cf_chl_opt") || html_lower.contains("cf-browser-verification"))
        {
            return Some("BLOQUEO_WAF_CLOUDFLARE_CHALLENGE");
        }

        // 2. Captchas incrustados dentro del formulario de contacto
        if form_html_lower.contains("cf-turnstile") || form_html_lower.contains("challenges.cloudflare.com/turnstile") {
            return Some("PROTECCION_ANTIBOT_CLOUDFLARE_TURNSTILE");
        }
        if form_html_lower.contains("h-captcha") || form_html_lower.contains("hcaptcha.com") {
            return Some("PROTECCION_ANTIBOT_HCAPTCHA");
        }
        if form_html_lower.contains("g-recaptcha") || form_html_lower.contains("recaptcha/api.js") || form_html_lower.contains("grecaptcha.execute") {
            return Some("PROTECCION_ANTIBOT_GOOGLE_RECAPTCHA");
        }
        if form_html_lower.contains("friendly-captcha") || form_html_lower.contains("frc-captcha") {
            return Some("PROTECCION_ANTIBOT_FRIENDLY_CAPTCHA");
        }

        None
    }

    /// Detecta si el servidor o formulario declara una codificación antigua (no UTF-8)
    pub fn requires_ascii_fallback(html_lower: &str, form_accept_charset: &str) -> bool {
        let ac = form_accept_charset.to_lowercase();
        if !ac.is_empty() && !ac.contains("utf-8") && !ac.contains("utf8") {
            return true;
        }
        if html_lower.contains("charset=iso-8859-1")
            || html_lower.contains("charset=windows-1252")
            || html_lower.contains("charset=\"iso-8859-1\"")
        {
            return true;
        }
        false
    }

    /// Inyecta el aviso preventivo de vulnerabilidad pública en el formulario de contacto real
    pub async fn submit_form_direct(
        client: &Client,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
    ) -> String {
        let encoded_ref = Self::encode_url_ref(empresa_nombre);

        // Evitamos el signo '$' pegado a dígitos para prevenir falsos positivos de filtros WAF o corrupción en PHP legacy
        let mensaje_utf8 = format!(
            "AVISO PREVENTIVO DE CUMPLIMIENTO: Estimado equipo directivo de {}. Durante un diagnostico automatizado de fuentes publicas (DENUE / SABG-LFPDPPP / PROFECO / STPS), nuestro motor detecto brechas normativas expuestas con una contingencia estimada en {:0.2} MXN. Consulte su pre-diagnostico tecnico sin costo y su expediente de blindaje en: https://mallen.lat/checkout?ref={}",
            empresa_nombre.trim(),
            multa_estimada,
            encoded_ref
        );

        let task = timeout(Duration::from_millis(4500), async {
            let resp = match client
                .get(page_url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                .header("Accept-Language", "es-MX,es;q=0.9,en;q=0.8")
                .send()
                .await
            {
                Ok(r) => {
                    if r.status().as_u16() == 403 || r.status().as_u16() == 503 {
                        let srv = r.headers().get("server").and_then(|v| v.to_str().ok()).unwrap_or("").to_lowercase();
                        if srv.contains("cloudflare") || r.headers().contains_key("cf-mitigated") {
                            return "BLOQUEO_WAF_CLOUDFLARE_CHALLENGE".to_string();
                        }
                    }
                    if !r.status().is_success() {
                        return "ERR_HTTP_GET".to_string();
                    }
                    r
                }
                Err(_) => return "ERR_HTTP_GET".to_string(),
            };

            let target_url = resp.url().clone();
            let html_text = match resp.text().await {
                Ok(t) => t,
                Err(_) => return "ERR_READ_BODY".to_string(),
            };

            let html_lower = html_text.to_lowercase();
            if let Some(waf_code) = Self::detect_antibot_protection(&html_lower, "") {
                return waf_code.to_string();
            }

            let (form_found, antibot_detected, form_action, form_method_is_get, form_params) = {
                let document = Html::parse_document(&html_text);
                let form_selector = Selector::parse("form").unwrap();
                let field_selector = Selector::parse("input, textarea, select").unwrap();
                let option_selector = Selector::parse("option").unwrap();

                let mut chosen_action = target_url.to_string();
                let mut chosen_params = HashMap::new();
                let mut found_valid_contact_form = false;
                let mut detected_captcha_reason: Option<&'static str> = None;
                let mut method_is_get = false;

                for form in document.select(&form_selector) {
                    let form_role = form.value().attr("role").unwrap_or("").to_lowercase();
                    let form_id = form.value().attr("id").unwrap_or("").to_lowercase();
                    let form_class = form.value().attr("class").unwrap_or("").to_lowercase();
                    let form_action_raw = form.value().attr("action").unwrap_or("").trim();
                    let form_method = form.value().attr("method").unwrap_or("post").to_lowercase();
                    let form_accept_charset = form.value().attr("accept-charset").unwrap_or("");

                    let act_lower = form_action_raw.to_lowercase();
                    if form_role == "search"
                        || act_lower.contains("search")
                        || act_lower.contains("login")
                        || act_lower.contains("cart")
                        || act_lower.contains("checkout")
                        || form_id.contains("search")
                        || form_id.contains("login")
                        || form_class.contains("search")
                    {
                        continue;
                    }

                    let form_inner_lower = form.html().to_lowercase();

                    let use_ascii_msg = Self::requires_ascii_fallback(&html_lower, form_accept_charset);
                    let final_message = if use_ascii_msg {
                        Self::to_ascii_safe(&mensaje_utf8)
                    } else {
                        mensaje_utf8.clone()
                    };
                    let sender_name = if use_ascii_msg {
                        "FIRMA W16 MALLEN - Compliance".to_string()
                    } else {
                        "FIRMA W16 MALLEN® — Compliance".to_string()
                    };

                    let mut params = HashMap::new();
                    let mut has_message_field = false;

                    for field in form.select(&field_selector) {
                        let tag_name = field.value().name();
                        let name = field.value().attr("name").unwrap_or("").trim();
                        if name.is_empty() {
                            continue;
                        }

                        let input_type = field.value().attr("type").unwrap_or("text").to_lowercase();
                        let style = field.value().attr("style").unwrap_or("").to_lowercase().replace(' ', "");
                        let class = field.value().attr("class").unwrap_or("").to_lowercase();
                        let id_attr = field.value().attr("id").unwrap_or("").to_lowercase();
                        let name_lower = name.to_lowercase();
                        let placeholder = field.value().attr("placeholder").unwrap_or("").to_lowercase();
                        let aria_hidden = field.value().attr("aria-hidden").unwrap_or("").to_lowercase();

                        // 1. Evasión avanzada de Honeypots (CSS oculto, off-screen, aria-hidden, nombres trampa)
                        if style.contains("display:none")
                            || style.contains("visibility:hidden")
                            || style.contains("opacity:0")
                            || style.contains("left:-999")
                            || style.contains("position:absolute;left:-")
                            || class.contains("honeypot")
                            || class.contains("hp-field")
                            || class.contains("wpcf7-quiz")
                            || id_attr.contains("honeypot")
                            || name_lower.contains("honeypot")
                            || name_lower.contains("_hp")
                            || name_lower == "url"
                            || name_lower == "website"
                            || name_lower == "website_url"
                            || name_lower == "fax"
                            || aria_hidden == "true"
                            || field.value().attr("tabindex") == Some("-1")
                            || field.value().attr("autocomplete") == Some("off") && name_lower.contains("trap")
                        {
                            continue;
                        }

                        // Ignorar botones de reset o button vacíos
                        if input_type == "reset" || input_type == "button" || input_type == "file" || input_type == "image" {
                            continue;
                        }

                        // 2. Preservar tokens CSRF ocultos legítimos (ej. _token, _wpnonce, _wpcf7, authenticity_token)
                        if input_type == "hidden" {
                            if let Some(val) = field.value().attr("value") {
                                params.insert(name.to_string(), val.to_string());
                            }
                            continue;
                        }

                        // 3. Manejo de <select> (escoger primera opción con value no vacío)
                        if tag_name == "select" {
                            for opt in field.select(&option_selector) {
                                let val = opt.value().attr("value").unwrap_or("").trim();
                                if !val.is_empty() {
                                    params.insert(name.to_string(), val.to_string());
                                    break;
                                }
                            }
                            continue;
                        }

                        // 4. Manejo de checkboxes de aceptación de Aviso de Privacidad / Términos
                        if input_type == "checkbox" {
                            if name_lower.contains("priv")
                                || name_lower.contains("term")
                                || name_lower.contains("accept")
                                || name_lower.contains("acepto")
                                || name_lower.contains("gdpr")
                                || name_lower.contains("consent")
                                || field.value().attr("required").is_some()
                            {
                                let cb_val = field.value().attr("value").unwrap_or("1");
                                params.insert(name.to_string(), cb_val.to_string());
                            }
                            continue;
                        }

                        // 5. Mapeo inteligente de campos de contacto
                        if tag_name == "textarea"
                            || name_lower.contains("message")
                            || name_lower.contains("mensaje")
                            || name_lower.contains("comment")
                            || name_lower.contains("coment")
                            || name_lower.contains("consulta")
                            || name_lower.contains("duda")
                            || placeholder.contains("mensaje")
                            || placeholder.contains("message")
                            || placeholder.contains("escribe")
                        {
                            params.insert(name.to_string(), final_message.clone());
                            has_message_field = true;
                        } else if input_type == "email"
                            || name_lower.contains("email")
                            || name_lower.contains("correo")
                            || name_lower.contains("mail")
                        {
                            params.insert(name.to_string(), "dictamenes@mallen.lat".to_string());
                        } else if name_lower.contains("subject") || name_lower.contains("asunto") {
                            let subj = if use_ascii_msg {
                                format!("Aviso Preventivo de Cumplimiento - {}", Self::to_ascii_safe(empresa_nombre))
                            } else {
                                format!("Aviso Preventivo de Cumplimiento — {}", empresa_nombre)
                            };
                            params.insert(name.to_string(), subj);
                        } else if name_lower.contains("name")
                            || name_lower.contains("nombre")
                            || placeholder.contains("nombre")
                        {
                            params.insert(name.to_string(), sender_name.clone());
                        } else if input_type == "tel"
                            || name_lower.contains("phone")
                            || name_lower.contains("telefono")
                            || name_lower.contains("celular")
                            || name_lower.contains("tel")
                            || name_lower.contains("whatsapp")
                        {
                            params.insert(name.to_string(), "5550000000".to_string());
                        } else if name_lower.contains("company")
                            || name_lower.contains("empresa")
                            || name_lower.contains("organiza")
                        {
                            params.insert(name.to_string(), "FIRMA W16 MALLEN".to_string());
                        } else if field.value().attr("required").is_some() && input_type == "text" {
                            params.insert(name.to_string(), "Cumplimiento Normativo".to_string());
                        }
                    }

                    if has_message_field && !params.is_empty() {
                        // Verificar si este formulario de contacto tiene Captcha / Turnstile activo
                        if let Some(captcha_tag) = Self::detect_antibot_protection(&html_lower, &form_inner_lower) {
                            detected_captcha_reason = Some(captcha_tag);
                            continue;
                        }

                        if !form_action_raw.is_empty() && !form_action_raw.starts_with('#') && !form_action_raw.starts_with("javascript:") {
                            if let Ok(abs_url) = target_url.join(form_action_raw) {
                                chosen_action = abs_url.to_string();
                            }
                        }
                        method_is_get = form_method == "get";
                        chosen_params = params;
                        found_valid_contact_form = true;
                        detected_captcha_reason = None;
                        break;
                    }
                }

                (found_valid_contact_form, detected_captcha_reason, chosen_action, method_is_get, chosen_params)
            };

            if !form_found {
                if let Some(captcha_code) = antibot_detected {
                    return captcha_code.to_string();
                }
                return "SIN_FORMULARIO_CONTACTO".to_string();
            }

            let origin_str = format!(
                "{}://{}",
                target_url.scheme(),
                target_url.host_str().unwrap_or("localhost")
            );

            let req_builder = if form_method_is_get {
                client.get(&form_action).query(&form_params)
            } else {
                client
                    .post(&form_action)
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                    .form(&form_params)
            };

            match req_builder
                .header("Referer", target_url.as_str())
                .header("Origin", origin_str)
                .header("Accept", "text/html,application/xhtml+xml,application/json;q=0.9,*/*;q=0.8")
                .send()
                .await
            {
                Ok(res) if res.status().is_success() || res.status().is_redirection() => {
                    let body_resp = res.text().await.unwrap_or_default().to_lowercase();
                    if body_resp.contains("captcha") && (body_resp.contains("invalid") || body_resp.contains("error") || body_resp.contains("obligatorio")) {
                        "PROTECCION_ANTIBOT_CAPTCHA_RECHAZADO".to_string()
                    } else {
                        "PROSPECCION_ENVIADA_EXITO".to_string()
                    }
                }
                Ok(res) => format!("HTTP_POST_ERR_{}", res.status().as_u16()),
                Err(_) => "HTTP_POST_TIMEOUT".to_string(),
            }
        })
        .await;

        match task {
            Ok(res) => res,
            Err(_) => "TIMEOUT_GLOBAL".to_string(),
        }
    }
}
