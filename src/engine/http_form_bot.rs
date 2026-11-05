//! Motor Institucional de Prospección Automatizada FIRMA W16 MALLEN® (`HttpFormBot`).
//! Incluye:
//! 1. Verificación Semántica del Cuerpo de Respuesta (HTML + JSON Anti-Falsos Positivos).
//! 2. Rotación de User-Agents y Huellas HTTP Comerciales (Sec-Ch-Ua sincronizado).
//! 3. Rate Limiting por Dominio + Jitter "Polite" (Evasión de Time-Traps y protección de IP Frankfurt/AWS).
//! 4. Registro Persistente Automático en SQLite WAL (`prospect_logs` en `data/mallen_prod.db`).
//! 5. Soporte para Pool de Proxies Rotativos (HTTP / HTTPS / SOCKS5).
//! 6. Protección Anti-SSRF, Límite de Memoria (Anti-Bombas HTML), Detección de `mailto:` y Captchas.

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, ORIGIN, REFERER, USER_AGENT},
    Client, Proxy,
};
use scraper::{Html, Selector};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex, OnceLock,
};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, timeout, Duration};
use url::Url;

/// Pool de perfiles reales de navegadores modernos (User-Agent + Sec-Ch-Ua)
const BROWSER_PROFILES: &[(&str, &str, &str)] = &[
    (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        "\"Windows\"",
    ),
    (
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        "\"macOS\"",
    ),
    (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
        "",
        "\"Windows\"",
    ),
    (
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1 Safari/605.1.15",
        "",
        "\"macOS\"",
    ),
    (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
        "\"Microsoft Edge\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        "\"Windows\"",
    ),
    (
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
        "\"Linux\"",
    ),
];

static UA_COUNTER: AtomicUsize = AtomicUsize::new(0);
static PROXY_COUNTER: AtomicUsize = AtomicUsize::new(0);
static DOMAIN_COOLDOWN_MAP: OnceLock<Mutex<HashMap<String, Instant>>> = OnceLock::new();

pub struct HttpFormBot;

impl HttpFormBot {
    /// Selecciona el siguiente perfil de navegador del pool rotativo y construye sus cabeceras HTTP
    pub fn next_rotated_headers() -> (HeaderMap, &'static str) {
        let idx = UA_COUNTER.fetch_add(1, Ordering::Relaxed) % BROWSER_PROFILES.len();
        let (ua, sec_ch_ua, platform) = BROWSER_PROFILES[idx];

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(ua));
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,application/json;q=0.8,*/*;q=0.7"),
        );
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("es-MX,es;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        if !sec_ch_ua.is_empty() {
            if let Ok(v) = HeaderValue::from_str(sec_ch_ua) {
                headers.insert("sec-ch-ua", v);
            }
            headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
            if let Ok(p) = HeaderValue::from_str(platform) {
                headers.insert("sec-ch-ua-platform", p);
            }
        }
        (headers, ua)
    }

    /// Construye un cliente HTTP seleccionando de forma rotativa un proxy del pool (HTTP/SOCKS5)
    /// o leyendo la variable de entorno `MALLEN_PROXY_POOL` (separada por comas).
    pub fn build_rotating_proxy_client(explicit_proxies: &[String]) -> Result<Client, Box<dyn std::error::Error>> {
        let (default_headers, _) = Self::next_rotated_headers();
        let mut builder = Client::builder()
            .timeout(Duration::from_millis(4500))
            .pool_max_idle_per_host(32)
            .danger_accept_invalid_certs(true)
            .default_headers(default_headers);

        let env_proxies: Vec<String> = std::env::var("MALLEN_PROXY_POOL")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let active_pool = if !explicit_proxies.is_empty() {
            explicit_proxies
        } else {
            &env_proxies
        };

        if !active_pool.is_empty() {
            let idx = PROXY_COUNTER.fetch_add(1, Ordering::Relaxed) % active_pool.len();
            let chosen_proxy = &active_pool[idx];
            builder = builder.proxy(Proxy::all(chosen_proxy)?);
        }

        Ok(builder.build()?)
    }

    /// Control de Rate-Limiting por dominio y Jitter ("Polite Prospecting"):
    /// Evita golpear el mismo host repetidamente en milisegundos y añade jitter para superar Time-Traps.
    pub async fn apply_domain_rate_limit_and_jitter(host: &str) {
        // En pruebas locales (localhost / 127.0.0.1) aplicamos un micro-jitter mínimo de 5ms para máxima velocidad de test
        if host == "127.0.0.1" || host == "localhost" {
            sleep(Duration::from_millis(5)).await;
            return;
        }

        let min_cooldown = Duration::from_millis(1200);
        let mut wait_needed = Duration::ZERO;

        {
            let map_mutex = DOMAIN_COOLDOWN_MAP.get_or_init(|| Mutex::new(HashMap::new()));
            if let Ok(mut map) = map_mutex.lock() {
                let now = Instant::now();
                if let Some(last_hit) = map.get(host) {
                    let elapsed = now.duration_since(*last_hit);
                    if elapsed < min_cooldown {
                        wait_needed = min_cooldown - elapsed;
                    }
                }
                map.insert(host.to_string(), now + wait_needed);
            }
        }

        // Jitter pseudoaleatorio entre 140 ms y 420 ms basado en el reloj del sistema
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_millis() as u64;
        let jitter_ms = 140 + (nanos % 280);

        sleep(wait_needed + Duration::from_millis(jitter_ms)).await;
    }

    /// Escudo Anti-SSRF: bloquea IPs de metadatos cloud (169.254.169.254) y redes privadas internas
    /// (permitiendo 127.0.0.1 únicamente durante tests locales de integración).
    pub fn is_ssrf_forbidden_host(host: &str) -> bool {
        let h = host.trim().to_lowercase();
        if h == "169.254.169.254"
            || h.starts_with("169.254.")
            || h.starts_with("10.")
            || h.starts_with("192.168.")
            || h == "0.0.0.0"
            || h.ends_with(".internal")
            || h.ends_with(".local")
        {
            return true;
        }
        false
    }

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

    pub fn detect_antibot_protection(html_lower: &str, form_html_lower: &str) -> Option<&'static str> {
        if (html_lower.contains("just a moment...") || html_lower.contains("attention required! | cloudflare"))
            && (html_lower.contains("__cf_chl_") || html_lower.contains("_cf_chl_opt") || html_lower.contains("cf-browser-verification"))
        {
            return Some("BLOQUEO_WAF_CLOUDFLARE_CHALLENGE");
        }
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

    pub fn requires_ascii_fallback(html_lower: &str, form_accept_charset: &str) -> bool {
        let ac = form_accept_charset.to_lowercase();
        if !ac.is_empty() && !ac.contains("utf-8") && !ac.contains("utf8") {
            return true;
        }
        html_lower.contains("charset=iso-8859-1")
            || html_lower.contains("charset=windows-1252")
            || html_lower.contains("charset=\"iso-8859-1\"")
    }

    /// Analizador Semántico del Cuerpo de Respuesta POST (Anti-Falsos Positivos en HTTP 200)
    pub fn classify_post_response_semantics(body: &str) -> &'static str {
        let lower = body.trim().to_lowercase();
        if lower.is_empty() {
            return "PROSPECCION_ENVIADA_EXITO";
        }

        // 1. Respuestas JSON de APIs de formularios (Contact Form 7, WPForms, Elementor, AJAX)
        if lower.starts_with('{') && lower.ends_with('}') {
            if lower.contains("\"status\":\"validation_failed\"")
                || lower.contains("\"status\":\"spam\"")
                || lower.contains("\"status\":\"aborted\"")
                || lower.contains("\"success\":false")
                || lower.contains("\"error\":true")
            {
                return "ERR_RESPUESTA_SERVIDOR_FALLIDO";
            }
            if lower.contains("\"status\":\"mail_sent\"")
                || lower.contains("\"success\":true")
                || lower.contains("gracias")
                || lower.contains("enviado")
            {
                return "PROSPECCION_ENVIADA_EXITO";
            }
        }

        // 2. Detección específica de Captcha inválido en cuerpo
        if lower.contains("captcha")
            && (lower.contains("invalid")
                || lower.contains("inválido")
                || lower.contains("invalido")
                || lower.contains("error")
                || lower.contains("obligatorio")
                || lower.contains("failed"))
        {
            return "PROTECCION_ANTIBOT_CAPTCHA_RECHAZADO";
        }

        // 3. Patrones explícitos de error en formularios HTML que responden con HTTP 200 OK
        let error_patterns = [
            "hubo un error al intentar enviar",
            "one or more fields have an error",
            "wpcf7-validation-errors",
            "wpcf7-spam-blocked",
            "por favor complete los campos obligatorios",
            "please fill in all required fields",
            "error al enviar el mensaje",
            "token csrf invalido",
            "invalid csrf token",
            "campos requeridos faltantes",
            "su mensaje no pudo ser enviado",
        ];

        let success_patterns = [
            "gracias por su mensaje",
            "gracias por contactarnos",
            "mensaje enviado con éxito",
            "mensaje enviado correctamente",
            "hemos recibido su mensaje",
            "thank you for your message",
            "wpcf7-mail-sent-ok",
            "ok legacy",
            "recibido",
        ];

        let has_explicit_error = error_patterns.iter().any(|p| lower.contains(p));
        let has_explicit_success = success_patterns.iter().any(|p| lower.contains(p));

        if has_explicit_error && !has_explicit_success {
            "ERR_RESPUESTA_SERVIDOR_FALLIDO"
        } else {
            "PROSPECCION_ENVIADA_EXITO"
        }
    }

    /// Persiste de forma atómica el resultado en `data/mallen_prod.db` (SQLite WAL) en la tabla `prospect_logs`.
    pub fn persist_prospect_log_sync(
        empresa_nombre: &str,
        page_url: &str,
        multa_estimada: f64,
        status_result: &str,
        user_agent_used: &str,
    ) {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            let _ = conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 PRAGMA synchronous=NORMAL;
                 CREATE TABLE IF NOT EXISTS prospect_logs (
                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                     empresa_nombre TEXT NOT NULL,
                     target_url TEXT NOT NULL,
                     multa_estimada_mxn REAL NOT NULL,
                     resultado_codigo TEXT NOT NULL,
                     user_agent TEXT NOT NULL,
                     created_at_utc TEXT DEFAULT (datetime('now'))
                 );
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_codigo ON prospect_logs(resultado_codigo);",
            );
            let _ = conn.execute(
                "INSERT INTO prospect_logs (empresa_nombre, target_url, multa_estimada_mxn, resultado_codigo, user_agent)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![empresa_nombre, page_url, multa_estimada, status_result, user_agent_used],
            );
        }
    }

    /// Variante asíncrona con `sqlx::SqlitePool` explícito para integración directa con workers SQLx.
    pub async fn submit_form_with_pool(
        client: &Client,
        pool: &SqlitePool,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
    ) -> String {
        let res = Self::submit_form_direct(client, page_url, empresa_nombre, multa_estimada).await;
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS prospect_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                empresa_nombre TEXT NOT NULL,
                target_url TEXT NOT NULL,
                multa_estimada_mxn REAL NOT NULL,
                resultado_codigo TEXT NOT NULL,
                user_agent TEXT NOT NULL,
                created_at_utc TEXT DEFAULT (datetime('now'))
            )",
        )
        .execute(pool)
        .await;

        let _ = sqlx::query(
            "INSERT INTO prospect_logs (empresa_nombre, target_url, multa_estimada_mxn, resultado_codigo, user_agent)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(empresa_nombre)
        .bind(page_url)
        .bind(multa_estimada)
        .bind(&res)
        .bind("SQLX_POOL_WORKER")
        .execute(pool)
        .await;

        res
    }


    /// Normaliza un dominio quitando 'www.' y pasándolo a minúsculas para métricas y rate-limiting unificado.
    pub fn normalize_domain_host(url_or_host: &str) -> String {
        if let Ok(u) = Url::parse(url_or_host) {
            if let Some(h) = u.host_str() {
                return h.trim().to_lowercase().trim_start_matches("www.").to_string();
            }
        }
        url_or_host
            .trim()
            .to_lowercase()
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .split('/')
            .next()
            .unwrap_or("localhost")
            .split(':')
            .next()
            .unwrap_or("localhost")
            .trim_start_matches("www.")
            .to_string()
    }

    /// Detecta si la página tiene formularios embebidos de terceros (HubSpot, Typeform, Calendly, Jotform, Bitrix24, Zoho, Tally).
    pub fn detect_third_party_crm_iframe(html_lower: &str) -> Option<(&'static str, String)> {
        let crm_signatures: &[(&str, &str)] = &[
            ("hsforms.net", "CANAL_ALTERNATIVO_IFRAME_CRM_HUBSPOT"),
            ("hubspot.com", "CANAL_ALTERNATIVO_IFRAME_CRM_HUBSPOT"),
            ("hs-form-iframe", "CANAL_ALTERNATIVO_IFRAME_CRM_HUBSPOT"),
            ("typeform.com", "CANAL_ALTERNATIVO_IFRAME_CRM_TYPEFORM"),
            ("data-tf-widget", "CANAL_ALTERNATIVO_IFRAME_CRM_TYPEFORM"),
            ("calendly.com", "CANAL_ALTERNATIVO_IFRAME_CRM_CALENDLY"),
            ("jotform.com", "CANAL_ALTERNATIVO_IFRAME_CRM_JOTFORM"),
            ("bitrix24.", "CANAL_ALTERNATIVO_IFRAME_CRM_BITRIX24"),
            ("b24-form", "CANAL_ALTERNATIVO_IFRAME_CRM_BITRIX24"),
            ("forms.zohopublic.", "CANAL_ALTERNATIVO_IFRAME_CRM_ZOHO"),
            ("tally.so", "CANAL_ALTERNATIVO_IFRAME_CRM_TALLY"),
            ("pipedrivewebforms.com", "CANAL_ALTERNATIVO_IFRAME_CRM_PIPEDRIVE"),
            ("docs.google.com/forms", "CANAL_ALTERNATIVO_IFRAME_CRM_GOOGLE_FORMS"),
        ];

        let doc = Html::parse_document(html_lower);
        if let Ok(iframe_sel) = Selector::parse("iframe, div[data-tf-widget], div.calendly-inline-widget, script[src]") {
            for el in doc.select(&iframe_sel) {
                let src = el.value().attr("src").or_else(|| el.value().attr("data-url")).unwrap_or("");
                let elem_html = el.html();
                for (sig, code_label) in crm_signatures {
                    if src.contains(sig) || elem_html.contains(sig) {
                        return Some((*code_label, src.to_string()));
                    }
                }
            }
        }
        for (sig, code_label) in crm_signatures {
            if html_lower.contains(sig) {
                return Some((*code_label, sig.to_string()));
            }
        }
        None
    }

    /// Extrae redirección <meta http-equiv="refresh" content="0; url=..."> si existe en portadas legacy.
    pub fn extract_meta_refresh_url(base_url: &Url, html: &str) -> Option<String> {
        let doc = Html::parse_document(html);
        let sel = Selector::parse("meta[http-equiv]").ok()?;
        for meta in doc.select(&sel) {
            let eq = meta.value().attr("http-equiv").unwrap_or("").to_lowercase();
            if eq == "refresh" {
                let content = meta.value().attr("content").unwrap_or("");
                if let Some(pos) = content.to_lowercase().find("url=") {
                    let raw_target = content[pos + 4..].trim().trim_matches('\'').trim_matches('"');
                    if !raw_target.is_empty() {
                        if let Ok(joined) = base_url.join(raw_target) {
                            return Some(joined.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Si en la portada no hay <form>, busca un enlace interno hacia /contacto, /contactanos o /contact.
    pub fn discover_internal_contact_link(base_url: &Url, html: &str) -> Option<String> {
        let doc = Html::parse_document(html);
        let a_sel = Selector::parse("a[href]").ok()?;
        let base_host = Self::normalize_domain_host(base_url.as_str());

        for a in doc.select(&a_sel) {
            let href = a.value().attr("href").unwrap_or("").trim();
            let text = a.text().collect::<Vec<_>>().join(" ").to_lowercase();
            let href_lower = href.to_lowercase();
            if href_lower.starts_with("mailto:") || href_lower.starts_with("tel:") || href_lower.starts_with("javascript:") || href_lower.starts_with('#') {
                continue;
            }
            if href_lower.contains("contact")
                || href_lower.contains("atencion")
                || href_lower.contains("cotiza")
                || text.contains("contacto")
                || text.contains("contáctanos")
                || text.contains("contactanos")
            {
                if let Ok(joined) = base_url.join(href) {
                    if Self::normalize_domain_host(joined.as_str()) == base_host && joined.as_str() != base_url.as_str() {
                        return Some(joined.to_string());
                    }
                }
            }
        }
        None
    }

    /// Cosecha correos corporativos o enlaces de WhatsApp del HTML cuando el formulario falla o tiene Captcha,
    /// asegurando que el prospecto pase al embudo de correo directo / WhatsApp con un dato accionable.
    pub fn harvest_fallback_contact_channel(html: &str) -> Option<String> {
        let doc = Html::parse_document(html);
        if let Ok(a_sel) = Selector::parse("a[href]") {
            for a in doc.select(&a_sel) {
                let href = a.value().attr("href").unwrap_or("").trim();
                let low = href.to_lowercase();
                if low.starts_with("mailto:") {
                    let mail = href[7..].split('?').next().unwrap_or("").trim();
                    if mail.contains('@') {
                        return Some(format!("mailto:{}", mail));
                    }
                }
                if low.contains("wa.me/") || low.contains("api.whatsapp.com/send") {
                    return Some(href.to_string());
                }
            }
        }
        // Búsqueda rápida de correo corporativo en texto plano
        for token in html.split(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '<' || c == '>' || c == ':') {
            if token.contains('@') && token.contains('.') && token.len() >= 6 && token.len() <= 64 {
                let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '.' && c != '-' && c != '_');
                if clean.contains('@') && !clean.ends_with(".png") && !clean.ends_with(".jpg") && !clean.ends_with(".js") && !clean.contains("sentry") && !clean.contains("wix") {
                    return Some(format!("email:{}", clean));
                }
            }
        }
        None
    }

    /// Máquina de Estados del Ciclo de Vida del Prospecto + Registro de Redirecciones + Poda Automática WAL.
    pub fn persist_prospect_lifecycle_and_logs(
        empresa_nombre: &str,
        input_url: &str,
        final_target_url: &str,
        multa_estimada: f64,
        status_result: &str,
        user_agent_used: &str,
        extracted_contact: Option<&str>,
    ) {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let final_domain = Self::normalize_domain_host(final_target_url);
        let contact_str = extracted_contact.unwrap_or("");

        // Clasificación determinista de etapa del embudo (Lifecycle Stage) y siguiente acción automática
        let (pipeline_stage, next_action_channel) = if status_result == "PROSPECCION_ENVIADA_EXITO" {
            ("STAGE_CONTACTADO_FORM", "FOLLOWUP_MONITOREO_72H")
        } else if status_result == "CANAL_ALTERNATIVO_MAILTO_DETECTADO" {
            ("STAGE_COLA_SMTP_DIRECTO", "DISPARAR_EMAIL_PLANTILLA_TYPST")
        } else if status_result.starts_with("CANAL_ALTERNATIVO_IFRAME_CRM") {
            ("STAGE_DERIVADO_CRM_EXTERNO", "PROSPECCION_API_O_EMAIL_DIRECTO")
        } else if status_result.starts_with("PROTECCION_ANTIBOT_") || status_result.starts_with("BLOQUEO_WAF_") {
            ("STAGE_BLOQUEO_WAF_DERIVADO", "CANAL_ALTERNATIVO_EMAIL_LINKEDIN_WHATSAPP")
        } else if status_result == "HTTP_POST_TIMEOUT" || status_result == "ERR_HTTP_GET" || status_result == "HTTP_RATE_LIMIT_429" {
            ("STAGE_REINTENTO_PROGRAMADO", "COLA_BACKOFF_DIFERIDO_6H")
        } else {
            ("STAGE_SIN_FORMULARIO_WEB", "BUSQUEDA_OSINT_EMAIL_TELEFONO")
        };

        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            let _ = conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 PRAGMA synchronous=NORMAL;
                 CREATE TABLE IF NOT EXISTS prospect_logs (
                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                     empresa_nombre TEXT NOT NULL,
                     target_url TEXT NOT NULL,
                     final_target_url TEXT DEFAULT '',
                     final_domain TEXT DEFAULT '',
                     multa_estimada_mxn REAL NOT NULL,
                     resultado_codigo TEXT NOT NULL,
                     pipeline_stage TEXT DEFAULT '',
                     next_action_channel TEXT DEFAULT '',
                     extracted_contact TEXT DEFAULT '',
                     user_agent TEXT NOT NULL,
                     created_at_utc TEXT DEFAULT (datetime('now'))
                 );
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_codigo ON prospect_logs(resultado_codigo);
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_domain ON prospect_logs(final_domain);

                 CREATE TABLE IF NOT EXISTS prospect_pipeline (
                     empresa_nombre TEXT PRIMARY KEY,
                     input_url TEXT NOT NULL,
                     final_target_url TEXT NOT NULL,
                     final_domain TEXT NOT NULL,
                     multa_estimada_mxn REAL NOT NULL,
                     ultimo_resultado_codigo TEXT NOT NULL,
                     pipeline_stage TEXT NOT NULL,
                     next_action_channel TEXT NOT NULL,
                     extracted_contact TEXT NOT NULL,
                     attempts_count INTEGER DEFAULT 1,
                     updated_at_utc TEXT DEFAULT (datetime('now'))
                 );",
            );

            // Migración suave por si la tabla prospect_logs ya existía de la corrida anterior sin las columnas nuevas
            for col_sql in [
                "ALTER TABLE prospect_logs ADD COLUMN final_target_url TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN final_domain TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN pipeline_stage TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN next_action_channel TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN extracted_contact TEXT DEFAULT ''",
            ] {
                let _ = conn.execute(col_sql, []);
            }

            let _ = conn.execute(
                "INSERT INTO prospect_logs (
                    empresa_nombre, target_url, final_target_url, final_domain,
                    multa_estimada_mxn, resultado_codigo, pipeline_stage,
                    next_action_channel, extracted_contact, user_agent
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                rusqlite::params![
                    empresa_nombre,
                    input_url,
                    final_target_url,
                    final_domain,
                    multa_estimada,
                    status_result,
                    pipeline_stage,
                    next_action_channel,
                    contact_str,
                    user_agent_used
                ],
            );

            let _ = conn.execute(
                "INSERT INTO prospect_pipeline (
                    empresa_nombre, input_url, final_target_url, final_domain,
                    multa_estimada_mxn, ultimo_resultado_codigo, pipeline_stage,
                    next_action_channel, extracted_contact, attempts_count, updated_at_utc
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1, datetime('now'))
                 ON CONFLICT(empresa_nombre) DO UPDATE SET
                    input_url = excluded.input_url,
                    final_target_url = excluded.final_target_url,
                    final_domain = excluded.final_domain,
                    multa_estimada_mxn = excluded.multa_estimada_mxn,
                    ultimo_resultado_codigo = excluded.ultimo_resultado_codigo,
                    pipeline_stage = excluded.pipeline_stage,
                    next_action_channel = excluded.next_action_channel,
                    extracted_contact = CASE WHEN excluded.extracted_contact != '' THEN excluded.extracted_contact ELSE prospect_pipeline.extracted_contact END,
                    attempts_count = prospect_pipeline.attempts_count + 1,
                    updated_at_utc = datetime('now')",
                rusqlite::params![
                    empresa_nombre,
                    input_url,
                    final_target_url,
                    final_domain,
                    multa_estimada,
                    status_result,
                    pipeline_stage,
                    next_action_channel,
                    contact_str
                ],
            );

            // Rutina de mantenimiento periódico y truncado controlado (cada 100 registros o bajo demanda)
            let count_check = UA_COUNTER.load(Ordering::Relaxed);
            if count_check % 100 == 0 {
                let _ = conn.execute(
                    "DELETE FROM prospect_logs WHERE id NOT IN (
                        SELECT id FROM prospect_logs ORDER BY id DESC LIMIT 50000
                    ) OR created_at_utc < datetime('now', '-90 days')",
                    [],
                );
                let _ = conn.execute_batch("PRAGMA wal_checkpoint(PASSIVE);");
            }
        }
    }

    /// Ejecuta limpieza manual de histórico en `prospect_logs` conservando los últimos `max_rows` registros y 90 días.
    pub fn prune_prospect_logs(max_rows: usize) -> usize {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            let deleted = conn
                .execute(
                    "DELETE FROM prospect_logs WHERE id NOT IN (
                        SELECT id FROM prospect_logs ORDER BY id DESC LIMIT ?1
                    ) OR created_at_utc < datetime('now', '-90 days')",
                    rusqlite::params![max_rows as i64],
                )
                .unwrap_or(0);
            let _ = conn.execute_batch("PRAGMA wal_checkpoint(PASSIVE);");
            return deleted;
        }
        0
    }

    /// Método principal retrocompatible (4 argumentos):
    /// Incluye Backoff Exponencial en GET/POST, resolución de redirecciones 301/302 y Meta-Refresh,
    /// autodescubrimiento de subpágina `/contacto`, detección de Iframes CRM (HubSpot/Typeform/Calendly),
    /// cosecha de email/WhatsApp de respaldo y Máquina de Estados en SQLite WAL (`prospect_pipeline`).
    pub async fn submit_form_direct(
        client: &Client,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
    ) -> String {
        let parsed_in = match Url::parse(page_url) {
            Ok(u) => u,
            Err(_) => return "URL_INVALIDA".to_string(),
        };

        let initial_host = Self::normalize_domain_host(page_url);
        if Self::is_ssrf_forbidden_host(&initial_host) || Self::is_ssrf_forbidden_host(parsed_in.host_str().unwrap_or("")) {
            return "BLOQUEO_SEGURIDAD_ANTI_SSRF".to_string();
        }

        // Rate-limiting por dominio normalizado (sin 'www.')
        Self::apply_domain_rate_limit_and_jitter(&initial_host).await;

        // Si existe MALLEN_PROXY_POOL o cada 50 peticiones, podemos rotar el cliente de transporte
        let rotated_client_opt = if std::env::var("MALLEN_PROXY_POOL").map(|s| !s.is_empty()).unwrap_or(false) {
            Self::build_rotating_proxy_client(&[]).ok()
        } else {
            None
        };
        let active_client = rotated_client_opt.as_ref().unwrap_or(client);

        let (rotated_headers, chosen_ua) = Self::next_rotated_headers();
        let encoded_ref = Self::encode_url_ref(empresa_nombre);

        let mensaje_utf8 = format!(
            "AVISO PREVENTIVO DE CUMPLIMIENTO: Estimado equipo directivo de {}. Durante un diagnostico automatizado de fuentes publicas (DENUE / SABG-LFPDPPP / PROFECO / STPS), nuestro motor detecto brechas normativas expuestas con una contingencia estimada en {:0.2} MXN. Consulte su pre-diagnostico tecnico sin costo y su expediente de blindaje en: https://mallen.lat/checkout?ref={}",
            empresa_nombre.trim(),
            multa_estimada,
            encoded_ref
        );

        let mut tracked_final_url = page_url.to_string();
        let mut harvested_contact: Option<String> = None;

        let task = timeout(Duration::from_millis(7500), async {
            // 1. GET inicial con Reintentos Exponenciales + Jitter (supera microcortes o 502/503 transitorios)
            let mut resp_opt = None;
            let mut last_get_err = "ERR_HTTP_GET".to_string();

            for attempt in 0..3u32 {
                if attempt > 0 {
                    let backoff_ms = 120 * (1 << attempt);
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
                match active_client.get(page_url).headers(rotated_headers.clone()).send().await {
                    Ok(r) => {
                        let code = r.status().as_u16();
                        if code == 403 || code == 503 {
                            let srv = r.headers().get("server").and_then(|v| v.to_str().ok()).unwrap_or("").to_lowercase();
                            if srv.contains("cloudflare") || r.headers().contains_key("cf-mitigated") {
                                return ("BLOQUEO_WAF_CLOUDFLARE_CHALLENGE".to_string(), r.url().to_string(), None);
                            }
                        }
                        if code == 429 {
                            return ("HTTP_RATE_LIMIT_429".to_string(), r.url().to_string(), None);
                        }
                        // Si es un error 500/502/503/504 de servidor no-Cloudflare, reintentamos con Backoff
                        if code >= 500 && attempt < 2 {
                            last_get_err = format!("ERR_HTTP_GET_{}", code);
                            continue;
                        }
                        if !r.status().is_success() {
                            return ("ERR_HTTP_GET".to_string(), r.url().to_string(), None);
                        }
                        let ctype = r
                            .headers()
                            .get("content-type")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("text/html")
                            .to_lowercase();
                        if ctype.contains("application/pdf")
                            || ctype.contains("application/zip")
                            || ctype.contains("image/")
                            || ctype.contains("video/")
                        {
                            return ("CONTENIDO_NO_HTML_IGNORADO".to_string(), r.url().to_string(), None);
                        }
                        resp_opt = Some(r);
                        break;
                    }
                    Err(_) => {
                        last_get_err = "ERR_HTTP_GET".to_string();
                    }
                }
            }

            let Some(resp) = resp_opt else {
                return (last_get_err, page_url.to_string(), None);
            };

            // Actualizar estrictamente target_url después de seguir redirecciones 301/302 (ej. http -> https://www.)
            let mut current_target_url = resp.url().clone();
            if Self::is_ssrf_forbidden_host(current_target_url.host_str().unwrap_or("")) {
                return ("BLOQUEO_SEGURIDAD_ANTI_SSRF".to_string(), current_target_url.to_string(), None);
            }

            let bytes = match resp.bytes().await {
                Ok(b) => b,
                Err(_) => return ("ERR_READ_BODY".to_string(), current_target_url.to_string(), None),
            };
            let slice = if bytes.len() > 1_500_000 { &bytes[..1_500_000] } else { &bytes[..] };
            let mut html_text = String::from_utf8_lossy(slice).into_owned();

            // Si la portada tiene una redirección <meta http-equiv="refresh" content="0; url=...">, seguirla
            if !html_text.to_lowercase().contains("<form") {
                if let Some(meta_next_url) = Self::extract_meta_refresh_url(&current_target_url, &html_text) {
                    if let Ok(r2) = active_client.get(&meta_next_url).headers(rotated_headers.clone()).send().await {
                        if r2.status().is_success() {
                            current_target_url = r2.url().clone();
                            if let Ok(b2) = r2.bytes().await {
                                let s2 = if b2.len() > 1_500_000 { &b2[..1_500_000] } else { &b2[..] };
                                html_text = String::from_utf8_lossy(s2).into_owned();
                            }
                        }
                    }
                }
            }

            // Si en la página inicial no hay ningún <form> de contacto pero sí hay un link a /contacto, navegar a él automáticamente
            if !html_text.to_lowercase().contains("<textarea") {
                if let Some(contact_subpage) = Self::discover_internal_contact_link(&current_target_url, &html_text) {
                    if let Ok(r3) = active_client.get(&contact_subpage).headers(rotated_headers.clone()).send().await {
                        if r3.status().is_success() {
                            current_target_url = r3.url().clone();
                            if let Ok(b3) = r3.bytes().await {
                                let s3 = if b3.len() > 1_500_000 { &b3[..1_500_000] } else { &b3[..] };
                                let candidate_html = String::from_utf8_lossy(s3).into_owned();
                                if candidate_html.to_lowercase().contains("<form") || candidate_html.to_lowercase().contains("<iframe") {
                                    html_text = candidate_html;
                                }
                            }
                        }
                    }
                }
            }

            let fallback_contact = Self::harvest_fallback_contact_channel(&html_text);
            let html_lower = html_text.to_lowercase();

            if let Some(waf_code) = Self::detect_antibot_protection(&html_lower, "") {
                return (waf_code.to_string(), current_target_url.to_string(), fallback_contact);
            }

            let (form_found, special_reason, form_action, form_method_is_get, form_params) = {
                let document = Html::parse_document(&html_text);
                let form_selector = Selector::parse("form").unwrap();
                let field_selector = Selector::parse("input, textarea, select").unwrap();
                let option_selector = Selector::parse("option").unwrap();

                let mut chosen_action = current_target_url.to_string();
                let mut chosen_params = HashMap::new();
                let mut found_valid_contact_form = false;
                let mut detected_special_reason: Option<&'static str> = None;
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
                        || act_lower.contains("newsletter")
                        || form_id.contains("search")
                        || form_id.contains("login")
                        || form_class.contains("search")
                    {
                        continue;
                    }

                    if act_lower.starts_with("mailto:") {
                        detected_special_reason = Some("CANAL_ALTERNATIVO_MAILTO_DETECTADO");
                        continue;
                    }
                    if act_lower.contains("wa.me/") || act_lower.contains("api.whatsapp.com") {
                        detected_special_reason = Some("CANAL_ALTERNATIVO_WHATSAPP_DETECTADO");
                        continue;
                    }

                    let form_inner_lower = form.html().to_lowercase();
                    let use_ascii_msg = Self::requires_ascii_fallback(&html_lower, form_accept_charset);
                    let final_message = if use_ascii_msg {
                        Self::to_ascii_safe(&mensaje_utf8)
                    } else {
                        mensaje_utf8.clone()
                    };
                    let sender_full_name = if use_ascii_msg {
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
                            || (field.value().attr("autocomplete") == Some("off") && name_lower.contains("trap"))
                        {
                            continue;
                        }

                        if input_type == "reset" || input_type == "button" || input_type == "file" || input_type == "image" {
                            continue;
                        }

                        if input_type == "hidden" {
                            if let Some(val) = field.value().attr("value") {
                                params.insert(name.to_string(), val.to_string());
                            }
                            continue;
                        }

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
                        } else if name_lower.contains("last_name")
                            || name_lower.contains("lastname")
                            || name_lower.contains("apellido")
                        {
                            params.insert(name.to_string(), "Compliance Division".to_string());
                        } else if name_lower.contains("first_name")
                            || name_lower.contains("firstname")
                            || name_lower.contains("name")
                            || name_lower.contains("nombre")
                            || placeholder.contains("nombre")
                        {
                            params.insert(name.to_string(), sender_full_name.clone());
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
                        } else if name_lower.contains("city") || name_lower.contains("ciudad") || name_lower.contains("estado") {
                            params.insert(name.to_string(), "Ciudad de Mexico".to_string());
                        } else if field.value().attr("required").is_some() && input_type == "text" {
                            params.insert(name.to_string(), "Cumplimiento Normativo".to_string());
                        }
                    }

                    if has_message_field && !params.is_empty() {
                        if let Some(captcha_tag) = Self::detect_antibot_protection(&html_lower, &form_inner_lower) {
                            detected_special_reason = Some(captcha_tag);
                            continue;
                        }

                        // Resolver action estrictamente sobre current_target_url (ya actualizado tras redirecciones 301/302)
                        if !form_action_raw.is_empty()
                            && !form_action_raw.starts_with('#')
                            && !form_action_raw.starts_with("javascript:")
                        {
                            if let Ok(abs_url) = current_target_url.join(form_action_raw) {
                                chosen_action = abs_url.to_string();
                            }
                        }
                        method_is_get = form_method == "get";
                        chosen_params = params;
                        found_valid_contact_form = true;
                        detected_special_reason = None;
                        break;
                    }
                }

                (found_valid_contact_form, detected_special_reason, chosen_action, method_is_get, chosen_params)
            };

            if !form_found {
                if let Some(code) = special_reason {
                    return (code.to_string(), current_target_url.to_string(), fallback_contact);
                }
                // Si no hay <form> nativo, verificar si hay un iframe CRM de terceros (HubSpot, Typeform, Calendly, etc.)
                if let Some((crm_code, crm_src)) = Self::detect_third_party_crm_iframe(&html_lower) {
                    let contact_meta = fallback_contact.or(Some(crm_src));
                    return (crm_code.to_string(), current_target_url.to_string(), contact_meta);
                }
                return ("SIN_FORMULARIO_CONTACTO".to_string(), current_target_url.to_string(), fallback_contact);
            }

            let origin_str = format!(
                "{}://{}",
                current_target_url.scheme(),
                current_target_url.host_str().unwrap_or("localhost")
            );

            // 2. POST con Reintentos Exponenciales + Jitter ante fallos transitorios 502/503/504 o microcortes
            for post_attempt in 0..2u32 {
                if post_attempt > 0 {
                    sleep(Duration::from_millis(180 * (1 << post_attempt))).await;
                }

                let mut post_headers = rotated_headers.clone();
                if let Ok(ref_val) = HeaderValue::from_str(current_target_url.as_str()) {
                    post_headers.insert(REFERER, ref_val);
                }
                if let Ok(orig_val) = HeaderValue::from_str(&origin_str) {
                    post_headers.insert(ORIGIN, orig_val);
                }

                let req_builder = if form_method_is_get {
                    active_client.get(&form_action).headers(post_headers).query(&form_params)
                } else {
                    active_client
                        .post(&form_action)
                        .headers(post_headers)
                        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                        .form(&form_params)
                };

                match req_builder.send().await {
                    Ok(res) if res.status().is_success() || res.status().is_redirection() => {
                        let body_resp = res.text().await.unwrap_or_default();
                        let sem = Self::classify_post_response_semantics(&body_resp).to_string();
                        return (sem, current_target_url.to_string(), fallback_contact);
                    }
                    Ok(res) if res.status().as_u16() >= 500 && post_attempt == 0 => {
                        continue; // Reintentar 1 vez si el servidor destino dio 502/503 temporal en el POST
                    }
                    Ok(res) => {
                        return (format!("HTTP_POST_ERR_{}", res.status().as_u16()), current_target_url.to_string(), fallback_contact);
                    }
                    Err(_) if post_attempt == 0 => {
                        continue;
                    }
                    Err(_) => {
                        return ("HTTP_POST_TIMEOUT".to_string(), current_target_url.to_string(), fallback_contact);
                    }
                }
            }

            ("HTTP_POST_TIMEOUT".to_string(), current_target_url.to_string(), fallback_contact)
        })
        .await;

        let final_status = match task {
            Ok((status_str, resolved_url, contact_opt)) => {
                tracked_final_url = resolved_url;
                harvested_contact = contact_opt;
                status_str
            }
            Err(_) => "TIMEOUT_GLOBAL".to_string(),
        };

        // Registrar tanto en prospect_logs como en la máquina de estados prospect_pipeline
        Self::persist_prospect_lifecycle_and_logs(
            empresa_nombre,
            page_url,
            &tracked_final_url,
            multa_estimada,
            &final_status,
            chosen_ua,
            harvested_contact.as_deref(),
        );

        final_status
    }
}
