#[allow(dead_code)]
use crate::engine::models::EmpresaB2B;
use reqwest::Client;
use std::time::Duration;

pub async fn deep_scan_website(empresa: &mut EmpresaB2B, client: &Client) {
    let url_str = match &empresa.sitio_web {
        Some(url) if !url.trim().is_empty() => url.clone(),
        _ => return,
    };

    let formatted_url = if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
        format!("http://{}", url_str)
    } else {
        url_str
    };

    if formatted_url.starts_with("https://") {
        empresa.audit_flags.push("SSL_ACTIVO".to_string());
    } else {
        empresa.audit_flags.push("SIN_SSL_ENCRIPTACION".to_string());
    }

    match client.get(&formatted_url).timeout(Duration::from_secs(3)).send().await {
        Ok(response) => {
            empresa.status_web = Some(response.status().as_u16());
            if let Ok(html) = response.text().await {
                let html_lower = html.to_lowercase();

                if html_lower.contains("aviso de privacidad") || html_lower.contains("derechos arco") || html_lower.contains("lfpdppp") {
                    empresa.audit_flags.push("LFPDPPP_AVISO_PRIVACIDAD_DETECTADO".to_string());
                } else {
                    empresa.audit_flags.push("INAI_SIN_AVISO_PRIVACIDAD".to_string());
                }

                if html_lower.contains("carrito") || html_lower.contains("checkout") || html_lower.contains("stripe") || html_lower.contains("shopify") {
                    empresa.audit_flags.push("PROFECO_ECOMMERCE_DETECTADO".to_string());
                }
            }
        }
        Err(_) => {
            empresa.audit_flags.push("SITIO_WEB_INACCESIBLE".to_string());
        }
    }
}
