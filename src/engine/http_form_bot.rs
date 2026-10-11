use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use tokio::time::{timeout, Duration};

pub struct HttpFormBot;

impl HttpFormBot {
    /// Inyecta el mensaje de prospección y diagnóstico de riesgo directamente en el formulario web
    pub async fn submit_form_direct(client: &Client, page_url: &str, empresa_nombre: &str, multa_estimada: f64) -> String {
        let mensaje_prospeccion = format!(
            "AVISO DE AUDITORÍA B2B: Estimado equipo de {}. Nuestro sistema detectó riesgos normativos evaluados en ${:.2} MXN (STPS/INAI/SAT/IMSS). Evite sanciones y descargue su kit de inmunidad y blindaje legal verificado por Firma Pericial Mallen ($5,940.00 MXN) en: https://mallen.lat/checkout",
            empresa_nombre, multa_estimada
        );

        let task = timeout(Duration::from_millis(3500), async {
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
                let input_selector = Selector::parse("input, textarea, select").unwrap();

                let mut action = page_url.to_string();
                let mut params = HashMap::new();
                let mut found = false;

                for form in document.select(&form_selector) {
                    found = true;
                    if let Some(act) = form.value().attr("action") {
                        if let Ok(abs_url) = target_url.join(act) {
                            action = abs_url.to_string();
                        }
                    }

                    for input in form.select(&input_selector) {
                        let name = input.value().attr("name").unwrap_or("");
                        if name.is_empty() {
                            continue;
                        }

                        let input_type = input.value().attr("type").unwrap_or("text").to_lowercase();
                        let placeholder = input.value().attr("placeholder").unwrap_or("").to_lowercase();
                        let name_lower = name.to_lowercase();

                        if name_lower.contains("message") || name_lower.contains("mensaje") ||
                           name_lower.contains("comment") || placeholder.contains("mensaje") ||
                           input_type == "textarea" {
                            params.insert(name.to_string(), mensaje_prospeccion.clone());
                        } else if input_type == "email" || name_lower.contains("email") || name_lower.contains("correo") {
                            params.insert(name.to_string(), "auditoria@mallen.lat".to_string());
                        } else if name_lower.contains("name") || name_lower.contains("nombre") {
                            params.insert(name.to_string(), "Firma Pericial Mallen — Auditoría B2B".to_string());
                        } else if name_lower.contains("phone") || name_lower.contains("telefono") || name_lower.contains("celular") {
                            params.insert(name.to_string(), "5500000000".to_string());
                        } else if let Some(val) = input.value().attr("value") {
                            params.insert(name.to_string(), val.to_string());
                        }
                    }
                    break;
                }

                (found, action, params)
            };

            if !form_found || form_params.is_empty() {
                return "SIN_FORMULARIO_VALIDO".to_string();
            }

            match client.post(&form_action).form(&form_params).send().await {
                Ok(res) if res.status().is_success() => "PROSPECCION_ENVIADA_EXITO".to_string(),
                Ok(res) => format!("HTTP_POST_ERR_{}", res.status().as_u16()),
                Err(_) => "HTTP_POST_TIMEOUT".to_string(),
            }
        }).await;

        match task {
            Ok(res) => res,
            Err(_) => "TIMEOUT_GLOBAL".to_string(),
        }
    }
}
