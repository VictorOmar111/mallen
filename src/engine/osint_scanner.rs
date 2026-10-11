use reqwest::Client;
use serde::{Serialize, Deserialize};
use regex::Regex;
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsintResult {
    pub domain_found: Option<String>,
    pub has_ssl: bool,
    pub has_privacy_policy: bool,
    pub privacy_policy_url: Option<String>,
    pub contact_form_url: Option<String>,
    pub has_form_inputs: bool,
    pub direct_email: Option<String>,
    pub direct_phone: Option<String>,
    pub tech_stack: Option<String>,
    pub footprint_score: u32,
    pub http_status: u16,
}

pub async fn run_osint_scan(client: &Client, site_url: Option<&str>, _nombre_empresa: &str) -> OsintResult {
    let mut result = OsintResult {
        domain_found: None,
        has_ssl: false,
        has_privacy_policy: false,
        privacy_policy_url: None,
        contact_form_url: None,
        has_form_inputs: false,
        direct_email: None,
        direct_phone: None,
        tech_stack: None,
        footprint_score: 0,
        http_status: 0,
    };

    let target_url = match site_url {
        Some(url) if !url.is_empty() => {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                format!("https://{}", url)
            } else {
                url.to_string()
            }
        }
        _ => return result,
    };

    result.domain_found = Some(target_url.clone());
    if target_url.starts_with("https://") {
        result.has_ssl = true;
    }

    // Política de Reintentos Exponenciales con tipo u64 explícito
    let mut response_opt = None;
    let mut attempts: u32 = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        match client.get(&target_url).send().await {
            Ok(resp) => {
                response_opt = Some(resp);
                break;
            }
            Err(_) => {
                attempts += 1;
                let backoff_ms = (500 * attempts.pow(2)) as u64;
                sleep(Duration::from_millis(backoff_ms)).await;
            }
        }
    }

    if let Some(response) = response_opt {
        result.http_status = response.status().as_u16();
        if response.status().is_success() {
            result.footprint_score += 30;
            if let Ok(body) = response.text().await {
                let body_lower = body.to_lowercase();

                if body_lower.contains("wp-content") {
                    result.tech_stack = Some("WordPress".to_string());
                } else if body_lower.contains("shopify") {
                    result.tech_stack = Some("Shopify".to_string());
                } else if body_lower.contains("woocommerce") {
                    result.tech_stack = Some("WooCommerce".to_string());
                }

                if body_lower.contains("aviso de privacidad") || body_lower.contains("politica de privacidad") || body_lower.contains("privacy") {
                    result.has_privacy_policy = true;
                    result.footprint_score += 35;
                }

                if body_lower.contains("<form") && (body_lower.contains("textarea") || body_lower.contains("type=\"email\"") || body_lower.contains("type='email'")) {
                    result.has_form_inputs = true;
                }

                if let Ok(re_email) = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}") {
                    if let Some(mat) = re_email.find(&body) {
                        result.direct_email = Some(mat.as_str().to_string());
                    }
                }

                if let Ok(re_phone) = Regex::new(r"(\+?52)?\s?\(?\d{2,3}\)?[\s.-]?\d{3,4}[\s.-]?\d{4}") {
                    if let Some(mat) = re_phone.find(&body) {
                        result.direct_phone = Some(mat.as_str().to_string());
                    }
                }
            }
        }
    }

    let base_url = target_url.trim_end_matches('/');
    let contact_test = format!("{}/contacto", base_url);
    
    if let Ok(resp) = client.get(&contact_test).send().await {
        if resp.status().is_success() {
            result.contact_form_url = Some(contact_test);
            result.has_form_inputs = true;
        }
    } else {
        result.contact_form_url = Some(format!("{}/", base_url));
    }

    result
}
