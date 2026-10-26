use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

pub struct HttpFormBot;

impl HttpFormBot {
    /// Inyecta el aviso preventivo de vulnerabilidad pública en el formulario de contacto real
    pub async fn submit_form_direct(
        client: &Client,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
    ) -> String {
        let rfc_o_hash = empresa_nombre;
        let mensaje_prospeccion = format!(
            "AVISO PREVENTIVO DE CUMPLIMIENTO: Estimado equipo directivo de {}. Durante un diagnóstico automatizado de fuentes públicas (DENUE/INAI/PROFECO/STPS), nuestro motor detectó brechas normativas expuestas con una contingencia estimada en ${:.2} MXN. Consulte su pre-diagnóstico técnico sin costo y descargue su Kit de Blindaje Pericial en: https://mallen.lat/checkout?ref={}",
            empresa_nombre, multa_estimada, rfc_o_hash
        );

        let task = timeout(Duration::from_millis(4500), async {
            let resp = match client.get(page_url).send().await {
                Ok(r) if r.status().is_success() => r,
                _ => return "ERR_HTTP_GET".to_string(),
            };

            let target_url = resp.url().clone();
            let html_text = match resp.text().await {
                Ok(t) => t,
                _ => return "ERR_READ_BODY".to_string(),
            };

            let (form_found, form_action, form_params) = {
                let document = Html::parse_document(&html_text);
                let form_selector = Selector::parse("form").unwrap();
                let field_selector = Selector::parse("input, textarea").unwrap();

                let mut chosen_action = page_url.to_string();
                let mut chosen_params = HashMap::new();
                let mut found_valid_contact_form = false;

                for form in document.select(&form_selector) {
                    // Ignorar formularios de búsqueda o login
                    let form_role = form.value().attr("role").unwrap_or("").to_lowercase();
                    let form_action_raw = form.value().attr("action").unwrap_or("");
                    if form_role == "search" || form_action_raw.to_lowercase().contains("search") || form_action_raw.to_lowercase().contains("login") {
                        continue;
                    }

                    let mut params = HashMap::new();
                    let mut has_message_field = false;

                    for field in form.select(&field_selector) {
                        let tag_name = field.value().name(); // "input" o "textarea"
                        let name = field.value().attr("name").unwrap_or("");
                        if name.is_empty() {
                            continue;
                        }

                        let input_type = field.value().attr("type").unwrap_or("text").to_lowercase();
                        let style = field.value().attr("style").unwrap_or("").to_lowercase();
                        let class = field.value().attr("class").unwrap_or("").to_lowercase();
                        let name_lower = name.to_lowercase();
                        let placeholder = field.value().attr("placeholder").unwrap_or("").to_lowercase();

                        // 1. Evasión de Honeypots (no rellenar trampas anti-bot invisibles)
                        if style.contains("display:none")
                            || style.contains("visibility:hidden")
                            || class.contains("honeypot")
                            || name_lower.contains("honeypot")
                            || name_lower.contains("website_url")
                            || field.value().attr("tabindex") == Some("-1")
                        {
                            continue;
                        }

                        // 2. Preservar tokens CSRF ocultos legítimos (ej. _token, _wpnonce, _wpcf7)
                        if input_type == "hidden" {
                            if let Some(val) = field.value().attr("value") {
                                params.insert(name.to_string(), val.to_string());
                            }
                            continue;
                        }

                        // 3. Mapeo inteligente de campos
                        if tag_name == "textarea"
                            || name_lower.contains("message")
                            || name_lower.contains("mensaje")
                            || name_lower.contains("comment")
                            || name_lower.contains("asunto")
                            || placeholder.contains("mensaje")
                        {
                            params.insert(name.to_string(), mensaje_prospeccion.clone());
                            has_message_field = true;
                        } else if input_type == "email" || name_lower.contains("email") || name_lower.contains("correo") {
                            params.insert(name.to_string(), "dictamenes@mallen.lat".to_string());
                        } else if name_lower.contains("name") || name_lower.contains("nombre") {
                            params.insert(name.to_string(), "Firma Pericial Mallen — Compliance B2B".to_string());
                        } else if input_type == "tel" || name_lower.contains("phone") || name_lower.contains("telefono") || name_lower.contains("celular") {
                            params.insert(name.to_string(), "5500000000".to_string());
                        } else if name_lower.contains("company") || name_lower.contains("empresa") {
                            params.insert(name.to_string(), "Mallen GRC & Auditoría".to_string());
                        }
                    }

                    // Solo seleccionamos el formulario si realmente tiene un campo de mensaje/contacto
                    if has_message_field && !params.is_empty() {
                        if !form_action_raw.is_empty() {
                            if let Ok(abs_url) = target_url.join(form_action_raw) {
                                chosen_action = abs_url.to_string();
                            }
                        }
                        chosen_params = params;
                        found_valid_contact_form = true;
                        break;
                    }
                }

                (found_valid_contact_form, chosen_action, chosen_params)
            };

            if !form_found || form_params.is_empty() {
                return "SIN_FORMULARIO_CONTACTO".to_string();
            }

            match client
                .post(&form_action)
                .header("Referer", page_url)
                .form(&form_params)
                .send()
                .await
            {
                Ok(res) if res.status().is_success() || res.status().is_redirection() => {
                    "PROSPECCION_ENVIADA_EXITO".to_string()
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
