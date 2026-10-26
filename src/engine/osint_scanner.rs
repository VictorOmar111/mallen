//! Escáner OSINT de Doble Capa:
//! - Capa 1 (Fast Path): Dual TLS (`strict` vs `insecure`) para capturar certificados SSL vencidos
//!   (Art. 19 LFPDPPP y Art. 128 BIS LFPC) sin abortar la lectura del DOM.
//! - Capa 2 (SPA Fallback): Renderizado Headless Chrome controlado para sitios en React/Next.js/Vue.
//! - Vector Subdominios Huérfanos: Consulta no intrusiva a Certificate Transparency (`crt.sh`).

use crate::engine::cleaner::{clean_url, sanitize_pii_text};
use crate::engine::models::WebOsintTelemetry;
use crate::engine::network::NetworkClientManager;
use futures::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::time::{sleep, timeout, Duration};
use url::Url;

static RE_EMAIL_OSINT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").expect("Regex email inválida")
});

static RE_PHONE_OSINT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\+?52)?\s?\(?\d{2,3}\)?[\s.-]?\d{3,4}[\s.-]?\d{4}").expect("Regex phone inválida")
});

/// Regex para detectar números de registro de Contratos de Adhesión ante PROFECO (RRPAC)
static RE_PROFECO_RRPAC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(registro\s+(p[uú]blico\s+de\s+)?contratos?\s+de\s+adhesi[oó]n|rrpac|profeco\s*[:#-]?\s*\d{2,6}[-/]\d{4})").expect("Regex RRPAC inválida")
});

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

/// Función original 100% retrocompatible con todos tus módulos existentes.
pub async fn run_osint_scan(
    client: &Client,
    site_url: Option<&str>,
    _nombre_empresa: &str,
) -> OsintResult {
    let mut result = OsintResult::default();

    // Aplicar escudo Anti-SSRF y normalización
    let Some(target_url) = site_url.and_then(clean_url) else {
        return result;
    };

    result.domain_found = Some(target_url.clone());

    // Ejecutamos también la telemetría profunda Dual-TLS para saber si el certificado es realmente válido
    let telemetry = run_deep_web_telemetry(&target_url, None).await;
    result.has_ssl = telemetry.ssl_valid;
    result.http_status = telemetry.http_status;
    result.has_privacy_policy = telemetry.has_privacy_notice;

    let mut response_opt = None;
    let mut attempts: u32 = 0;

    while attempts < 2 {
        match client.get(&target_url).send().await {
            Ok(resp) => {
                response_opt = Some(resp);
                break;
            }
            Err(_) => {
                attempts += 1;
                let backoff_ms = (400 * attempts.pow(2)) as u64;
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
                } else if body_lower.contains("__next") || body_lower.contains("_next/static") {
                    result.tech_stack = Some("Next.js (SPA)".to_string());
                }

                if result.has_privacy_policy
                    || body_lower.contains("aviso de privacidad")
                    || body_lower.contains("politica de privacidad")
                    || body_lower.contains("privacy")
                {
                    result.has_privacy_policy = true;
                    result.footprint_score += 35;
                }

                if body_lower.contains("<form")
                    && (body_lower.contains("textarea")
                        || body_lower.contains("type=\"email\"")
                        || body_lower.contains("type='email'"))
                {
                    result.has_form_inputs = true;
                }

                if let Some(mat) = RE_EMAIL_OSINT.find(&body) {
                    result.direct_email = Some(mat.as_str().to_string());
                }

                if let Some(mat) = RE_PHONE_OSINT.find(&body) {
                    result.direct_phone = Some(mat.as_str().to_string());
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

/// Motor OSINT de Doble Capa para construir `WebOsintTelemetry`:
/// 1. Verifica validez real del certificado TLS (`strict` vs `insecure`).
/// 2. Si detecta estructura SPA vacía (`<div id="root"></div>`), activa Headless Chrome con cierre limpio.
/// 3. Audita Aviso de Privacidad, Derechos ARCO, E-commerce, RRPAC PROFECO y Subdominios en `crt.sh`.
pub async fn run_deep_web_telemetry(
    raw_domain_or_url: &str,
    proxy_url: Option<&str>,
) -> WebOsintTelemetry {
    let mut telemetry = WebOsintTelemetry::default();

    let Some(safe_url) = clean_url(raw_domain_or_url) else {
        telemetry.ssl_valid = false;
        telemetry.ssl_error_reason = Some("Dominio inválido o bloqueado por política Anti-SSRF".to_string());
        return telemetry;
    };

    telemetry.target_url = safe_url.clone();

    let Ok(dual) = NetworkClientManager::build_dual_tls_pair(proxy_url) else {
        return telemetry;
    };

    // CAPA 1: Fast Path con verificación Dual TLS
    let mut html_body = String::new();
    match dual.strict.get(&safe_url).send().await {
        Ok(resp) => {
            telemetry.ssl_valid = safe_url.starts_with("https://");
            telemetry.http_status = resp.status().as_u16();
            html_body = resp.text().await.unwrap_or_default();
        }
        Err(strict_err) => {
            // El certificado falló o el servidor rechazó TLS estricto: probamos con el cliente tolerante
            telemetry.ssl_valid = false;
            telemetry.ssl_error_reason = Some(format!(
                "Certificado SSL/TLS inválido, expirado o autofirmado ({})",
                strict_err
            ));

            if let Ok(fallback_resp) = dual.insecure.get(&safe_url).send().await {
                telemetry.http_status = fallback_resp.status().as_u16();
                html_body = fallback_resp.text().await.unwrap_or_default();
            }
        }
    }

    // CAPA 2: Detección de "Efecto Espejo" en aplicaciones SPA (React, Vue, Next.js, Angular)
    if needs_headless_spa_fallback(&html_body) {
        if let Some(rendered_dom) = render_spa_with_headless_chrome(&safe_url).await {
            html_body = rendered_dom;
            telemetry.is_spa_rendered = true;
        }
    }

    // Sanitizamos cualquier PII personal antes de analizar o registrar fragmentos
    let clean_html = sanitize_pii_text(&html_body);
    let lower = clean_html.to_lowercase();

    // Vector LFPDPPP (SABG / ex-INAI): Aviso de Privacidad y Derechos ARCO
    telemetry.has_privacy_notice = lower.contains("aviso de privacidad")
        || lower.contains("política de privacidad")
        || lower.contains("politica de privacidad")
        || lower.contains("tratamiento de datos personales");

    telemetry.mentions_arco_rights = lower.contains("derechos arco")
        || (lower.contains("acceso") && lower.contains("rectificación") && lower.contains("cancelación"))
        || lower.contains("oposición al tratamiento");

    // Vector PROFECO Art. 76 BIS / SAT Art. 113-A (Prioridad de Hallazgo Digital)
    telemetry.has_ecommerce_cart = lower.contains("agregar al carrito")
        || lower.contains("añadir al carrito")
        || lower.contains("checkout")
        || lower.contains("woocommerce")
        || lower.contains("shopify")
        || lower.contains("mercadopago")
        || lower.contains("stripe.com");

    // Vector PROFECO Arts. 85-87: Contrato de Adhesión registrado en RRPAC
    telemetry.has_profeco_rrpac_number = RE_PROFECO_RRPAC.is_match(&clean_html);

    // Vector Subdominios Huérfanos (Certificate Transparency crt.sh)
    if let Ok(parsed) = Url::parse(&safe_url) {
        if let Some(host) = parsed.host_str() {
            let root_domain = host.trim_start_matches("www.");
            telemetry.orphan_subdomains =
                discover_orphan_subdomains_crtsh(&dual.strict, root_domain).await;
        }
    }

    telemetry
}

/// Determina si el HTML estático es un cascarón SPA vacío que requiere renderizado JS en Headless Chrome.
fn needs_headless_spa_fallback(html: &str) -> bool {
    if html.is_empty() {
        return false;
    }
    let lower = html.to_lowercase();
    let has_privacy_already = lower.contains("aviso de privacidad");
    if has_privacy_already {
        return false; // Si el SSR ya entregó el Aviso de Privacidad, ahorramos RAM y no abrimos Chrome
    }

    let is_spa_shell = lower.contains("<div id=\"root\"></div>")
        || lower.contains("<div id=\"app\"></div>")
        || lower.contains("<div id=\"__next\">")
        || lower.contains("window.__next_f");

    is_spa_shell
}

/// Renderiza una página SPA utilizando `chromiumoxide` con bandera de bajo consumo de RAM
/// y cierre explícito del proceso para evitar procesos Chromium zombies en Linux.
async fn render_spa_with_headless_chrome(target_url: &str) -> Option<String> {
    use chromiumoxide::browser::{Browser, BrowserConfig};

    let config = BrowserConfig::builder()
        .no_sandbox()
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--disable-dev-shm-usage")
        .arg("--js-flags=--max-old-space-size=128")
        .build()
        .ok()?;

    let render_result = timeout(Duration::from_secs(7), async {
        let (mut browser, mut handler) = Browser::launch(config).await.ok()?;
        let handle_task = tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        let page_html = match browser.new_page(target_url).await {
            Ok(page) => {
                let _ = page.wait_for_navigation().await;
                let content = page.content().await.ok();
                let _ = page.close().await;
                content
            }
            Err(_) => None,
        };

        let _ = browser.close().await;
        handle_task.abort();
        page_html
    })
    .await;

    render_result.ok().flatten()
}

#[derive(Deserialize)]
struct CrtShEntry {
    name_value: String,
}

/// Consulta los logs públicos de Certificate Transparency (`crt.sh`) para detectar subdominios
/// administrativos o de desarrollo olvidados (`dev.`, `test.`, `facturacion.`, `rh.`).
async fn discover_orphan_subdomains_crtsh(client: &Client, domain: &str) -> Vec<String> {
    let url = format!("https://crt.sh/?q=%.{}&output=json", domain);
    let Ok(Ok(resp)) = timeout(Duration::from_millis(2200), client.get(&url).send()).await else {
        return Vec::new();
    };

    let Ok(entries) = resp.json::<Vec<CrtShEntry>>().await else {
        return Vec::new();
    };

    let mut risky_subdomains = HashSet::new();
    let sensitive_prefixes = [
        "dev.",
        "staging.",
        "test.",
        "admin.",
        "rh.",
        "nomina.",
        "facturacion.",
        "api.",
        "portal.",
        "intranet.",
    ];

    for entry in entries.into_iter().take(60) {
        for sub in entry.name_value.lines() {
            let clean = sub.trim().to_lowercase().replace("*.", "");
            if sensitive_prefixes.iter().any(|p| clean.starts_with(p)) {
                risky_subdomains.insert(clean);
            }
        }
    }

    risky_subdomains.into_iter().take(5).collect()
}
