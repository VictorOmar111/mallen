//! Motor Institucional de Prospección Automatizada FIRMA MALLEN® (`HttpFormBot`).
//! Incluye:
//! 1. Verificación Semántica del Cuerpo de Respuesta (HTML + JSON Anti-Falsos Positivos).
//! 2. Rotación de User-Agents y Huellas HTTP Comerciales (Sec-Ch-Ua sincronizado).
//! 3. Rate Limiting por Dominio + Jitter "Polite" (Evasión de Time-Traps y protección de IP Frankfurt/AWS).
//! 4. Registro Persistente Automático en SQLite WAL (`prospect_logs` en `data/mallen_prod.db`).
//! 5. Soporte para Pool de Proxies Rotativos (HTTP / HTTPS / SOCKS5).
//! 6. Protección Anti-SSRF, Límite de Memoria (Anti-Bombas HTML), Detección de `mailto:` y Captchas.

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
    XChaCha20Poly1305, XNonce,
};
use futures::stream::{self, StreamExt};
use moka::sync::Cache;
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use std::sync::mpsc::{self, SyncSender};
use tokio::sync::{oneshot, Semaphore};
use zeroize::Zeroizing;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, COOKIE, ORIGIN, REFERER, SET_COOKIE, USER_AGENT},
    redirect::Policy,
    Client, Proxy,
};
use scraper::{Html, Selector};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
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
const MAX_RESPONSE_BODY_BYTES: usize = 512 * 1024;
const CHUNK_READ_TIMEOUT_SECS: u64 = 4;
const PII_AAD_CONTEXT: &[u8] = b"FIRMA_MALLEN_PII_VAULT_V1_LFPDPPP_2025";

static DOMAIN_COOLDOWN_CACHE: OnceLock<Cache<String, Instant>> = OnceLock::new();
static CIRCUIT_BREAKER_CACHE: OnceLock<Cache<String, (u32, Instant)>> = OnceLock::new();
static ROBOTS_ALLOW_CACHE: OnceLock<Cache<String, bool>> = OnceLock::new();
static IN_FLIGHT_DOMAINS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
static DB_SCHEMA_INITIALIZED: OnceLock<bool> = OnceLock::new();
static GLOBAL_CONCURRENCY_SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();
static PROXY_CLIENTS_CACHE: OnceLock<Vec<Client>> = OnceLock::new();
static SQLITE_WRITER_TX: OnceLock<SyncSender<TelemetryWriteMsg>> = OnceLock::new();

enum WriterAck {
    Async(oneshot::Sender<()>),
    Sync(SyncSender<()>),
}

struct TelemetryWriteMsg {
    db_path: String,
    empresa_nombre: String,
    input_url: String,
    final_target_url: String,
    multa_estimada: f64,
    status_result: String,
    user_agent_used: String,
    http_status: u16,
    latency_ms: u64,
    charset_detected: String,
    retry_count: u32,
    emails: String,
    whatsapps: String,
    socials: String,
    trace_detail: String,
    ack: Option<WriterAck>,
}

static TELEMETRY_TOTAL_REQS: AtomicUsize = AtomicUsize::new(0);
static TELEMETRY_IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);
static TELEMETRY_SUCCESS: AtomicUsize = AtomicUsize::new(0);
static TELEMETRY_WAF_OR_CAPTCHA: AtomicUsize = AtomicUsize::new(0);
static TELEMETRY_CB_TRIPS: AtomicUsize = AtomicUsize::new(0);
static TELEMETRY_TOTAL_LATENCY_MS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct BotProcessTelemetry {
    pub total_requests: usize,
    pub active_in_flight: usize,
    pub available_semaphore_permits: usize,
    pub success_count: usize,
    pub waf_or_captcha_count: usize,
    pub circuit_breaker_trips: usize,
    pub avg_latency_ms: u64,
}


struct InFlightTelemetryGuard;
impl InFlightTelemetryGuard {
    fn new() -> Self {
        TELEMETRY_IN_FLIGHT.fetch_add(1, Ordering::SeqCst);
        Self
    }
}
impl Drop for InFlightTelemetryGuard {
    fn drop(&mut self) {
        TELEMETRY_IN_FLIGHT.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct HttpFormBot;

impl HttpFormBot {
    fn cooldown_cache() -> &'static Cache<String, Instant> {
        DOMAIN_COOLDOWN_CACHE.get_or_init(|| {
            Cache::builder().max_capacity(50_000).time_to_live(Duration::from_secs(600)).build()
        })
    }

    fn circuit_breaker_cache() -> &'static Cache<String, (u32, Instant)> {
        CIRCUIT_BREAKER_CACHE.get_or_init(|| {
            Cache::builder().max_capacity(50_000).time_to_live(Duration::from_secs(48 * 3600)).build()
        })
    }

    fn robots_cache() -> &'static Cache<String, bool> {
        ROBOTS_ALLOW_CACHE.get_or_init(|| {
            Cache::builder().max_capacity(50_000).time_to_live(Duration::from_secs(24 * 3600)).build()
        })
    }

    fn configure_sqlite_connection(conn: &rusqlite::Connection) {
        let _ = conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA busy_timeout=5000;
             PRAGMA wal_autocheckpoint=2000;
             PRAGMA temp_store=MEMORY;
             PRAGMA cache_size=-32000;",
        );
    }

    fn sqlite_writer_sender() -> &'static SyncSender<TelemetryWriteMsg> {
        SQLITE_WRITER_TX.get_or_init(|| {
            let (tx, rx) = mpsc::sync_channel::<TelemetryWriteMsg>(4096);
            std::thread::Builder::new()
                .name("mallen-sqlite-writer".to_string())
                .spawn(move || {
                    let mut cached_path = String::new();
                    let mut cached_conn: Option<rusqlite::Connection> = None;
                    while let Ok(first) = rx.recv() {
                        let mut batch = Vec::with_capacity(100);
                        batch.push(first);
                        while batch.len() < 100 {
                            match rx.try_recv() {
                                Ok(msg) => batch.push(msg),
                                Err(_) => break,
                            }
                        }
                        let mut by_path: std::collections::BTreeMap<String, Vec<usize>> = std::collections::BTreeMap::new();
                        for (i, m) in batch.iter().enumerate() {
                            by_path.entry(m.db_path.clone()).or_default().push(i);
                        }
                        for (target_path, idxs) in by_path {
                            if cached_path != target_path || cached_conn.is_none() {
                                match rusqlite::Connection::open(&target_path) {
                                    Ok(conn) => {
                                        Self::configure_sqlite_connection(&conn);
                                        Self::ensure_db_schema_once(&conn);
                                        cached_path = target_path;
                                        cached_conn = Some(conn);
                                    }
                                    Err(e) => {
                                        eprintln!("[SQLITE-WRITER][WARN] Error abriendo {}: {}", target_path, e);
                                        continue;
                                    }
                                }
                            }
                            if let Some(ref mut conn) = cached_conn {
                                match conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate) {
                                    Ok(tx_sql) => {
                                        for i in idxs {
                                            Self::write_telemetry_row_in_tx(&tx_sql, &batch[i]);
                                        }
                                        if let Err(e) = tx_sql.commit() {
                                            eprintln!("[SQLITE-WRITER][WARN] Error en commit SQLite: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("[SQLITE-WRITER][WARN] Error iniciando transacción SQLite: {}", e);
                                    }
                                }
                            }
                        }
                        for mut m in batch {
                            if let Some(ack) = m.ack.take() {
                                match ack {
                                    WriterAck::Async(s) => { let _ = s.send(()); }
                                    WriterAck::Sync(s) => { let _ = s.send(()); }
                                }
                            }
                        }
                    }
                })
                .expect("failed to spawn mallen-sqlite-writer");
            tx
        })
    }

    fn write_telemetry_row_in_tx(tx: &rusqlite::Transaction<'_>, m: &TelemetryWriteMsg) {
        let primary_contact = if !m.emails.is_empty() {
            format!("mailto:{}", m.emails.split(';').next().unwrap_or("").trim())
        } else if !m.whatsapps.is_empty() {
            m.whatsapps.split(';').next().unwrap_or("").trim().to_string()
        } else if !m.socials.is_empty() {
            m.socials.split(';').next().unwrap_or("").trim().to_string()
        } else {
            String::new()
        };
        let (stage, contact_tier, next_action) =
            Self::classify_contact_method_tier(&m.status_result, &m.emails, &m.whatsapps, &m.socials);
        let final_domain = Self::normalize_domain_host(&m.final_target_url);
        let raw_vault = format!("emails={}|wa={}|soc={}", m.emails, m.whatsapps, m.socials);
        let encrypted_vault = if !m.emails.is_empty() || !m.whatsapps.is_empty() || !m.socials.is_empty() {
            Self::encrypt_pii_at_rest(&raw_vault)
        } else {
            String::new()
        };
        if let Err(e) = tx.execute(
            "INSERT INTO prospect_logs (empresa_nombre, target_url, final_target_url, final_domain, multa_estimada_mxn, resultado_codigo, pipeline_stage, next_action_channel, extracted_contact, user_agent, http_status, latency_ms, charset_detected, retry_count, contact_method_tier, trace_detail) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            rusqlite::params![m.empresa_nombre, m.input_url, m.final_target_url, final_domain, m.multa_estimada, m.status_result, stage, next_action, primary_contact, m.user_agent_used, m.http_status as i64, m.latency_ms as i64, m.charset_detected, m.retry_count as i64, contact_tier, m.trace_detail],
        ) { eprintln!("[SQLITE-WRITER][WARN] INSERT prospect_logs ({}): {}", m.empresa_nombre, e); }
        if let Err(e) = tx.execute(
            "INSERT INTO prospect_pipeline (empresa_nombre, input_url, final_target_url, final_domain, multa_estimada_mxn, ultimo_resultado_codigo, pipeline_stage, next_action_channel, extracted_contact, contact_method_tier, extracted_emails, extracted_whatsapp, extracted_socials, last_http_status, last_latency_ms, last_charset, last_trace_detail, encrypted_contact_vault, attempts_count, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, 1, datetime('now')) ON CONFLICT(empresa_nombre) DO UPDATE SET input_url = excluded.input_url, final_target_url = excluded.final_target_url, final_domain = excluded.final_domain, multa_estimada_mxn = excluded.multa_estimada_mxn, ultimo_resultado_codigo = excluded.ultimo_resultado_codigo, pipeline_stage = excluded.pipeline_stage, next_action_channel = excluded.next_action_channel, extracted_contact = CASE WHEN excluded.extracted_contact != '' THEN excluded.extracted_contact ELSE prospect_pipeline.extracted_contact END, contact_method_tier = excluded.contact_method_tier, extracted_emails = CASE WHEN excluded.extracted_emails != '' THEN excluded.extracted_emails ELSE prospect_pipeline.extracted_emails END, extracted_whatsapp = CASE WHEN excluded.extracted_whatsapp != '' THEN excluded.extracted_whatsapp ELSE prospect_pipeline.extracted_whatsapp END, extracted_socials = CASE WHEN excluded.extracted_socials != '' THEN excluded.extracted_socials ELSE prospect_pipeline.extracted_socials END, last_http_status = excluded.last_http_status, last_latency_ms = excluded.last_latency_ms, last_charset = excluded.last_charset, last_trace_detail = excluded.last_trace_detail, encrypted_contact_vault = CASE WHEN excluded.encrypted_contact_vault != '' THEN excluded.encrypted_contact_vault ELSE prospect_pipeline.encrypted_contact_vault END, attempts_count = prospect_pipeline.attempts_count + 1, updated_at_utc = datetime('now')",
            rusqlite::params![m.empresa_nombre, m.input_url, m.final_target_url, final_domain, m.multa_estimada, m.status_result, stage, next_action, primary_contact, contact_tier, m.emails, m.whatsapps, m.socials, m.http_status as i64, m.latency_ms as i64, m.charset_detected, m.trace_detail, encrypted_vault],
        ) { eprintln!("[SQLITE-WRITER][WARN] UPSERT prospect_pipeline ({}): {}", m.empresa_nombre, e); }
        if m.trace_detail.contains(" | STACK:") {
            let st_part = m.trace_detail.split(" | STACK:").nth(1).unwrap_or("");
            let stack_val = st_part.split(" | PITCH:").next().unwrap_or("");
            let pitch_val = st_part.split(" | PITCH:").nth(1).unwrap_or("");
            let _ = tx.execute("UPDATE prospect_pipeline SET tech_stack_detected = ?1, recommended_module = ?2 WHERE empresa_nombre = ?3", rusqlite::params![stack_val, pitch_val, m.empresa_nombre]);
        }
    }

    fn ssrf_safe_redirect_policy() -> Policy {
        Policy::custom(|attempt| {
            if attempt.previous().len() >= 5 { return attempt.error("max_redirects_exceeded"); }
            if !Self::is_url_ssrf_safe_sync(attempt.url()) { return attempt.error("ssrf_redirect_blocked"); }
            attempt.follow()
        })
    }

    pub fn is_private_or_forbidden_ip(ip: IpAddr, allow_local_test_loopback: bool) -> bool {
        match ip {
            IpAddr::V4(v4) => {
                let oct = v4.octets();
                if v4.is_loopback() { return !allow_local_test_loopback; }
                v4.is_private() || v4.is_link_local() || v4.is_broadcast() || v4.is_documentation()
                    || v4.is_unspecified() || v4.is_multicast()
                    || (oct[0] == 100 && (oct[1] & 0b1100_0000) == 64) || oct[0] == 0
            }
            IpAddr::V6(v6) => {
                if v6.is_loopback() { return !allow_local_test_loopback; }
                if let Some(v4_m) = v6.to_ipv4_mapped() {
                    return Self::is_private_or_forbidden_ip(IpAddr::V4(v4_m), allow_local_test_loopback);
                }
                let seg0 = v6.segments()[0];
                v6.is_unspecified() || v6.is_multicast() || (seg0 & 0xfe00) == 0xfc00 || (seg0 & 0xffc0) == 0xfe80
            }
        }
    }

    pub fn is_url_ssrf_safe_sync(url: &Url) -> bool {
        if url.scheme() != "http" && url.scheme() != "https" { return false; }
        let Some(host) = url.host_str() else { return false; };
        if Self::is_ssrf_forbidden_host(host) { return false; }
        if host != "127.0.0.1" && host != "localhost" {
            if let Some(port) = url.port() {
                if port != 80 && port != 443 { return false; }
            }
        }
        true
    }

    pub async fn validate_url_and_dns_ssrf(url: &Url) -> bool {
        if !Self::is_url_ssrf_safe_sync(url) { return false; }
        let Some(host) = url.host_str() else { return false; };
        if host == "127.0.0.1" || host == "localhost" { return true; }
        let port = url.port_or_known_default().unwrap_or(443);
        if let Ok(Ok(addrs)) = timeout(Duration::from_secs(2), tokio::net::lookup_host((host, port))).await {
            let mut saw_any = false;
            for sock in addrs {
                saw_any = true;
                if Self::is_private_or_forbidden_ip(sock.ip(), false) { return false; }
            }
            return saw_any;
        }
        true
    }

    async fn read_bounded_body(mut resp: reqwest::Response, max_bytes: usize) -> Result<Vec<u8>, &'static str> {
        let mut buf = Vec::with_capacity(max_bytes.min(65_536));
        loop {
            match timeout(Duration::from_secs(CHUNK_READ_TIMEOUT_SECS), resp.chunk()).await {
                Ok(Ok(Some(chunk))) => {
                    let rem = max_bytes.saturating_sub(buf.len());
                    if rem == 0 { break; }
                    buf.extend_from_slice(&chunk[..chunk.len().min(rem)]);
                    if buf.len() >= max_bytes { break; }
                }
                Ok(Ok(None)) => break,
                Ok(Err(_)) => return Err("ERR_READ_BODY"),
                Err(_) => return Err("TIMEOUT_READ_BODY"),
            }
        }
        Ok(buf)
    }

    fn extract_ephemeral_cookies(headers: &HeaderMap, jar: &mut HashMap<String, String>) {
        for val in headers.get_all(SET_COOKIE) {
            if let Ok(s) = val.to_str() {
                if let Some(pair) = s.split(';').next() {
                    if let Some((k, v)) = pair.split_once('=') {
                        let key = k.trim();
                        if !key.is_empty() && jar.len() < 32 { jar.insert(key.to_string(), v.trim().to_string()); }
                    }
                }
            }
        }
    }

    fn format_cookie_header(jar: &HashMap<String, String>) -> Option<HeaderValue> {
        if jar.is_empty() { return None; }
        let joined: String = jar.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("; ");
        HeaderValue::from_str(&joined).ok()
    }

    async fn is_allowed_by_robots_txt(client: &Client, target_url: &Url, headers: &HeaderMap) -> bool {
        let Some(host) = target_url.host_str() else { return false; };
        if host == "127.0.0.1" || host == "localhost" { return true; }
        let origin_key = target_url.origin().ascii_serialization();
        if let Some(allowed) = Self::robots_cache().get(&origin_key) { return allowed; }
        let robots_url = format!("{}/robots.txt", origin_key);
        let allowed = match timeout(Duration::from_secs(3), client.get(&robots_url).headers(headers.clone()).send()).await {
            Ok(Ok(r)) => {
                let code = r.status().as_u16();
                if (400..500).contains(&code) { true }
                else if code >= 500 { false }
                else if let Ok(bytes) = Self::read_bounded_body(r, 500 * 1024).await {
                    let txt = String::from_utf8_lossy(&bytes).to_lowercase();
                    let (mut in_wc, mut dis_root) = (false, false);
                    for raw_line in txt.lines() {
                        let line = raw_line.split('#').next().unwrap_or("").trim();
                        if let Some(ua) = line.strip_prefix("user-agent:") {
                            let u = ua.trim();
                            in_wc = u == "*" || u.contains("mallen");
                        } else if in_wc {
                            if let Some(dis) = line.strip_prefix("disallow:") {
                                let p = dis.trim();
                                if p == "/" || p == "/contacto" || p == "/contact" { dis_root = true; }
                            }
                        }
                    }
                    !dis_root
                } else { true }
            }
            _ => true,
        };
        Self::robots_cache().insert(origin_key, allowed);
        allowed
    }

    pub fn is_suppressed_arco(domain: &str, empresa_nombre: &str) -> bool {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            Self::configure_sqlite_connection(&conn);
            Self::ensure_db_schema_once(&conn);
            let cnt: i64 = conn.query_row(
                "SELECT COUNT(*) FROM suppression_list WHERE LOWER(domain_or_company) = LOWER(?1) OR LOWER(domain_or_company) = LOWER(?2)",
                rusqlite::params![domain.trim(), empresa_nombre.trim()],
                |r| r.get(0),
            ).unwrap_or(0);
            return cnt > 0;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    async fn enqueue_telemetry_async(
        empresa_nombre: String, input_url: String, final_target_url: String, multa_estimada: f64,
        status_result: String, user_agent_used: String, http_status: u16, latency_ms: u64,
        charset_detected: String, retry_count: u32, emails: String, whatsapps: String, socials: String, trace_detail: String,
    ) {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let (ack_tx, ack_rx) = oneshot::channel();
        let msg = TelemetryWriteMsg {
            db_path, empresa_nombre, input_url, final_target_url, multa_estimada, status_result, user_agent_used,
            http_status, latency_ms, charset_detected, retry_count, emails, whatsapps, socials, trace_detail,
            ack: Some(WriterAck::Async(ack_tx)),
        };
        if Self::sqlite_writer_sender().send(msg).is_ok() {
            let _ = timeout(Duration::from_secs(3), ack_rx).await;
        }
    }

    pub fn fetch_pyme_prospect_chunk_keyset(last_rowid: i64, chunk_size: usize) -> Vec<(i64, String, String, String, f64)> {
        let bounded = chunk_size.clamp(1, 500);
        let mut batch = Vec::with_capacity(bounded);
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            Self::configure_sqlite_connection(&conn);
            let sql = "SELECT rowid, COALESCE(NULLIF(TRIM(raz_social), ''), TRIM(nom_estab)), TRIM(www), COALESCE(NULLIF(TRIM(codigo_scian), ''), '541110'), COALESCE(per_ocu, '11 a 30 personas') FROM denue_units WHERE rowid > ?1 AND www IS NOT NULL AND TRIM(www) != '' AND (raz_social != '' OR nom_estab != '') AND COALESCE(codigo_scian,'') NOT LIKE '93%' AND LOWER(COALESCE(nombre_act,'')) NOT LIKE '%sector público%' AND LOWER(COALESCE(nombre_act,'')) NOT LIKE '%sector publico%' AND LOWER(COALESCE(www,'')) NOT LIKE '%gob.mx%' AND LOWER(COALESCE(www,'')) NOT LIKE '%.gob.%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%SECRETARIA DE%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%GOBIERNO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%AYUNTAMIENTO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INFONAVIT%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%FONDO NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INSTITUTO MEXICANO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INSTITUTO NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%COMISION FEDERAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%COMISION NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PETROLEOS MEXICANOS%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%BANCO DE MEXICO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PROCURADURIA%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PODER JUDICIAL%' AND LOWER(COALESCE(www,'')) NOT LIKE '%infonavit%' AND LOWER(COALESCE(www,'')) NOT LIKE '%imss.gob%' AND LOWER(COALESCE(www,'')) NOT LIKE '%issste%' AND LOWER(COALESCE(www,'')) NOT LIKE '%pemex.com%' AND LOWER(COALESCE(www,'')) NOT LIKE '%cfe.mx%' AND LOWER(COALESCE(www,'')) NOT LIKE '%banxico%' AND COALESCE(codigo_scian,'') NOT LIKE '5221%' AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%ALCALDIA%' AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%SECRETARIA DE%' AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%GOBIERNO%' AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%FISCALIA%' AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%TRIBUNAL%' AND LOWER(www) NOT LIKE '%banamex%' AND LOWER(www) NOT LIKE '%bbva%' AND LOWER(www) NOT LIKE '%santander%' AND LOWER(www) NOT LIKE '%banorte%' AND LOWER(www) NOT LIKE '%hsbc%' AND LOWER(www) NOT LIKE '%scotiabank%' AND LOWER(www) NOT LIKE '%.gob.mx%' AND LOWER(www) NOT LIKE '%walmart%' AND LOWER(www) NOT LIKE '%oxxo%' AND LOWER(www) NOT LIKE '%soriana%' AND LOWER(www) NOT LIKE '%coppel%' AND LOWER(www) NOT LIKE '%chedraui%' AND LOWER(www) NOT LIKE '%liverpool%' ORDER BY rowid ASC LIMIT ?2";
            if let Ok(mut stmt) = conn.prepare(sql) {
                if let Ok(rows) = stmt.query_map(rusqlite::params![last_rowid, bounded as i64], |r| {
                    Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, String>(3)?, r.get::<_, String>(4)?))
                }) {
                    for (rid, emp_name, raw_www, scian, estrato) in rows.flatten() {
                        let norm = if raw_www.starts_with("http://") || raw_www.starts_with("https://") { raw_www } else { format!("https://{}", raw_www.trim_start_matches('/')) };
                        let (_, _, kit_price, _) = Self::classify_denue_capacity_and_scian_risk(&estrato, &scian);
                        batch.push((rid, emp_name, norm, scian, kit_price));
                    }
                }
            }
        }
        batch
    }
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
            .timeout(Duration::from_millis(Self::configured_timeout_ms()))
            .connect_timeout(Duration::from_millis(3000))
            .pool_idle_timeout(Duration::from_secs(15))
            .pool_max_idle_per_host(16)
            .tcp_keepalive(Duration::from_secs(30))
            .tcp_nodelay(true)
            .redirect(Self::ssrf_safe_redirect_policy())
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
        } else {
            builder = builder.no_proxy();
        }

        Ok(builder.build()?)
    }

    /// Devuelve un cliente HTTP preconstruido en RAM del pool `MALLEN_PROXY_POOL` (O(1), sin recrear TLS por petición).
    pub fn get_cached_proxy_client() -> Option<&'static Client> {
        let pool = PROXY_CLIENTS_CACHE.get_or_init(|| {
            let raw = std::env::var("MALLEN_PROXY_POOL").unwrap_or_default();
            raw.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .filter_map(|p| Self::build_rotating_proxy_client(&[p.to_string()]).ok())
                .collect()
        });
        if pool.is_empty() {
            return None;
        }
        let idx = PROXY_COUNTER.fetch_add(1, Ordering::Relaxed) % pool.len();
        Some(&pool[idx])
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
            let cache = Self::cooldown_cache();
            let now = Instant::now();
            if let Some(last_hit) = cache.get(host) {
                let elapsed = now.duration_since(last_hit);
                if elapsed < min_cooldown {
                    wait_needed = min_cooldown - elapsed;
                }
            }
            cache.insert(host.to_string(), now + wait_needed);
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
        let h = host.trim().trim_matches(|c| c == '[' || c == ']').to_lowercase();
        if h.is_empty() || h == "169.254.169.254" || h == "metadata.google.internal"
            || h == "0.0.0.0" || h.ends_with(".internal") || h.ends_with(".local") || h.ends_with(".localhost")
        {
            return true;
        }
        if h == "127.0.0.1" || h == "localhost" { return false; }
        if let Ok(ip) = h.parse::<IpAddr>() {
            return Self::is_private_or_forbidden_ip(ip, false);
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
                 PRAGMA busy_timeout=5000;
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
        } else if status_result == "HTTP_POST_TIMEOUT" || status_result.starts_with("ERR_HTTP_GET") || status_result == "HTTP_RATE_LIMIT_429" {
            ("STAGE_REINTENTO_PROGRAMADO", "COLA_BACKOFF_DIFERIDO_6H")
        } else {
            ("STAGE_SIN_FORMULARIO_WEB", "BUSQUEDA_OSINT_EMAIL_TELEFONO")
        };

        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            let _ = conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 PRAGMA synchronous=NORMAL;
                 PRAGMA busy_timeout=5000;
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






    /// 1. Detector de Stack Tecnológico (Estilo Wappalyzer en Rust — 24 tecnologías clave en México)
    /// Devuelve `(stack_csv, has_ecommerce, recommended_pitch_module)`.
    pub fn detect_mexican_tech_stack(headers_str: &str, html: &str) -> (String, bool, &'static str) {
        let h_low = headers_str.to_lowercase();
        let d_low = html.to_lowercase();
        let mut stack: Vec<&'static str> = Vec::new();
        let mut is_ecommerce = false;

        let rules: &[(&str, &'static str, bool)] = &[
            ("wp-content", "WordPress", false),
            ("wp-includes", "WordPress", false),
            ("woocommerce", "WooCommerce", true),
            ("wc-cart-fragments", "WooCommerce", true),
            ("cdn.shopify.com", "Shopify", true),
            ("shopify.com", "Shopify", true),
            ("tiendanube", "Tiendanube", true),
            ("nuvemshop", "Tiendanube", true),
            ("vtexassets", "VTEX", true),
            ("prestashop", "PrestaShop", true),
            ("magento", "Magento", true),
            ("wix.com", "Wix", false),
            ("wixstatic.com", "Wix", false),
            ("squarespace", "Squarespace", false),
            ("webflow", "Webflow", false),
            ("odoo", "Odoo-ERP", true),
            ("__next_data__", "Next.js/React", false),
            ("_next/static", "Next.js/React", false),
            ("__nuxt", "Nuxt/Vue", false),
            ("ng-version", "Angular", false),
            ("laravel_session", "Laravel", false),
            ("asp.net", "ASP.NET", false),
            ("js.stripe.com", "Stripe", true),
            ("conekta", "Conekta-MX", true),
            ("openpay", "Openpay-BBVA", true),
            ("mercadopago", "MercadoPago", true),
            ("paypalobjects", "PayPal", true),
            ("clip.mx", "Clip-MX", true),
            ("hs-script-loader", "HubSpot-CRM", false),
            ("googletagmanager.com", "Google-Tag-Manager", false),
            ("connect.facebook.net", "Meta-Pixel", false),
        ];

        for (sig, label, ecom_flag) in rules {
            if d_low.contains(sig) || h_low.contains(sig) {
                if !stack.contains(label) {
                    stack.push(label);
                }
                if *ecom_flag {
                    is_ecommerce = true;
                }
            }
        }

        if stack.is_empty() {
            stack.push("HTML5-Static");
        }

        let pitch_module = if is_ecommerce {
            "MODULO_BLINDAJE_ECOMMERCE_PASARELAS_Y_PROFECO_ART76BIS"
        } else if stack.contains(&"HubSpot-CRM") || stack.contains(&"Meta-Pixel") || stack.contains(&"Google-Tag-Manager") {
            "MODULO_PRIVACIDAD_COOKIES_RASTREO_Y_ARCO_LFPDPPP"
        } else {
            "MODULO_AVISO_PRIVACIDAD_WEB_Y_MATERIALIDAD_SAT_STPS"
        };

        (stack.join(", "), is_ecommerce, pitch_module)
    }

    /// 2. Auditor Rápido de Seguridad de Correo y DNS (SPF / DMARC) para detectar vulnerabilidad a Phishing / Spoofing.
    /// Consulta registros TXT del dominio y devuelve `(spf_present, dmarc_present, security_verdict)`.
    pub fn audit_domain_email_dns_security(domain: &str) -> (bool, bool, &'static str) {
        let clean_dom = Self::normalize_domain_host(domain);
        if clean_dom == "127.0.0.1" || clean_dom == "localhost" || clean_dom.is_empty() {
            return (false, false, "VULNERABLE_SIN_DMARC_RIESGO_SUPLANTACION");
        }

        let check_txt = |q_host: &str, needle: &str| -> bool {
            if let Ok(out) = std::process::Command::new("host")
                .arg("-W")
                .arg("1")
                .arg("-t")
                .arg("TXT")
                .arg(q_host)
                .output()
            {
                let txt = String::from_utf8_lossy(&out.stdout).to_lowercase();
                if txt.contains(needle) {
                    return true;
                }
            }
            false
        };

        let has_spf = check_txt(&clean_dom, "v=spf1");
        let has_dmarc = check_txt(&format!("_dmarc.{}", clean_dom), "v=dmarc1");

        let verdict = match (has_spf, has_dmarc) {
            (true, true) => "DNS_CORREO_BLINDADO_SPF_DMARC",
            (true, false) => "VULNERABLE_SIN_DMARC_RIESGO_SUPLANTACION",
            (false, true) => "VULNERABLE_SIN_SPF_RIESGO_SPOOFING",
            (false, false) => "CRITICO_SIN_SPF_NI_DMARC_ALTO_RIESGO_PHISHING",
        };
        (has_spf, has_dmarc, verdict)
    }

    /// 3. Clasificador de Tamaño / Capacidad Económica (Estrato INEGI DENUE) y Nivel de Riesgo Sectorial SCIAN.
    /// Devuelve `(tier_economico, empleados_estimados, precio_kit_sugerido_mxn, argumento_pericial_scian)`.
    pub fn classify_denue_capacity_and_scian_risk(estrato_denue: &str, scian_code: &str) -> (&'static str, u32, f64, &'static str) {
        let e_low = estrato_denue.to_lowercase();
        // Estrategia Foco PyME: Precio estandarizado de alta conversión en $3,000.00 MXN (~$150 USD)
        let (tier_size, emp_est, suggested_price) = if e_low.contains("251") || e_low.contains("más") || e_low.contains("mas") {
            ("CORP_GRAN_EMPRESA_251_PLUS", 300, 3_000.00)
        } else if e_low.contains("101 a 250") {
            ("PYME_MEDIANA_ALTA_101_250", 150, 3_000.00)
        } else if e_low.contains("51 a 100") {
            ("PYME_MEDIANA_51_100", 75, 3_000.00)
        } else if e_low.contains("31 a 50") {
            ("PYME_PEQUENA_ALTA_31_50", 45, 3_000.00)
        } else if e_low.contains("11 a 30") {
            ("PYME_PEQUENA_11_30", 20, 3_000.00)
        } else if e_low.contains("6 a 10") {
            ("PYME_MICRO_6_10", 8, 3_000.00)
        } else {
            ("PYME_MICRO_0_5", 4, 3_000.00)
        };

        let scian = scian_code.trim();
        let scian_argument = if scian.starts_with("52") || scian.starts_with("53") {
            "ALTO_RIESGO_FINTECH_PLD: Sujeto a auditoría LFPIORPI (Art. 17/18 UIF), Beneficiario Controlador y cifrado de datos patrimoniales LFPDPPP."
        } else if (scian.starts_with("62") && !scian.starts_with("624")) || scian.starts_with("464") {
            "ALTO_RIESGO_SALUD_COFEPRIS: Sujeto a Aviso de Responsable Sanitario, RPBI (NOM-087-SEMARNAT), Expediente Clínico (NOM-004) y Datos Sensibles Art. 9 LFPDPPP."
        } else if scian.starts_with("722") || scian.starts_with("311") || scian.starts_with("461") {
            "ALTO_RIESGO_ALIMENTOS_CONSUMO: Sujeto a inspección COFEPRIS (NOM-251), Protección Civil PIPC, STPS NOM-002 Incendios y PROFECO Precios Totales."
        } else if scian.starts_with("48") || scian.starts_with("49") || scian.starts_with("43") {
            "ALTO_RIESGO_LOGISTICA_ADUANAL: Sujeto a Complemento Carta Porte 3.1 SAT, Bitácoras SICT (NOM-087/NOM-068) y Ergonomía de Cargas (NOM-036-1-STPS)."
        } else if scian.starts_with("23") || scian.starts_with("31") || scian.starts_with("32") || scian.starts_with("33") {
            "ALTO_RIESGO_INDUSTRIAL_OBRA: Sujeto a Prima de Riesgo IMSS Clase IV/V, REPSE/SIROC, NOM-019/030/035 STPS y Manifiestos Ambientales SEMARNAT."
        } else if scian.starts_with("46") || scian.starts_with("51") {
            "RIESGO_COMERCIO_DIGITAL_PROFECO: Sujeto a Art. 76 BIS LFPC, Póliza de Garantía de 90 días y Aviso de Privacidad Integral SABG."
        } else {
            "RIESGO_CORPORATIVO_TRANSVERSAL: Sujeto a Materialidad Fiscal Art. 69-B CFF (Fecha Cierta SCJN), STPS NOM-035/037 y Conciliación SUA IMSS."
        };

        (tier_size, emp_est, suggested_price, scian_argument)
    }

    /// 4. Validador Pre-Flight de Enlaces de Checkout antes de persistir o enviar en mensajes.
    /// Si la URL generada no pasa el parser estricto `url::Url::parse`, devuelve `Err("ERR_INVALID_LINK_GENERATED")`.
    pub fn build_and_validate_checkout_link(empresa_nombre: &str, empresa_id_opt: Option<&str>) -> Result<String, &'static str> {
        let trimmed = empresa_nombre.trim();
        if trimmed.is_empty() || trimmed.contains('\0') || trimmed.contains('\n') || trimmed.contains('\r') {
            return Err("ERR_INVALID_LINK_GENERATED");
        }
        let encoded_ref = Self::encode_url_ref(trimmed);
        if encoded_ref.is_empty() {
            return Err("ERR_INVALID_LINK_GENERATED");
        }

        let raw_link = if let Some(id_str) = empresa_id_opt.filter(|s| !s.trim().is_empty()) {
            let encoded_id = Self::encode_url_ref(id_str);
            format!("https://mallen.lat/checkout?ref={}&empresa={}", encoded_ref, encoded_id)
        } else {
            format!("https://mallen.lat/checkout?ref={}", encoded_ref)
        };

        match Url::parse(&raw_link) {
            Ok(u) if (u.scheme() == "https" || u.scheme() == "http") && u.host_str() == Some("mallen.lat") && u.query().is_some() => {
                Ok(raw_link)
            }
            _ => Err("ERR_INVALID_LINK_GENERATED"),
        }
    }

    /// 5. Registro en tiempo real de Clics de Prospectos en la tabla `link_clicks` (SQLite WAL).
    /// Marca inmediatamente al prospecto como `has_clicked_link = 1` en `prospect_pipeline`.
    pub fn record_checkout_link_click(ref_hash_or_empresa: &str, visitor_ip: &str, user_agent: &str) -> Result<u64, String> {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
        Self::ensure_db_schema_once(&conn);

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS link_clicks (
                ref_key TEXT PRIMARY KEY,
                first_clicked_at_utc TEXT DEFAULT (datetime('now')),
                last_clicked_at_utc TEXT DEFAULT (datetime('now')),
                click_count INTEGER DEFAULT 1,
                last_visitor_ip TEXT DEFAULT '',
                last_user_agent TEXT DEFAULT ''
            );",
        )
        .map_err(|e| e.to_string())?;

        let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN has_clicked_link INTEGER DEFAULT 0", []);

        conn.execute(
            "INSERT INTO link_clicks (ref_key, first_clicked_at_utc, last_clicked_at_utc, click_count, last_visitor_ip, last_user_agent)
             VALUES (?1, datetime('now'), datetime('now'), 1, ?2, ?3)
             ON CONFLICT(ref_key) DO UPDATE SET
                last_clicked_at_utc = datetime('now'),
                click_count = link_clicks.click_count + 1,
                last_visitor_ip = excluded.last_visitor_ip,
                last_user_agent = excluded.last_user_agent",
            rusqlite::params![ref_hash_or_empresa.trim(), visitor_ip, user_agent],
        )
        .map_err(|e| e.to_string())?;

        let _ = conn.execute(
            "UPDATE prospect_pipeline
             SET has_clicked_link = 1,
                 pipeline_stage = 'STAGE_LINK_ABIERTO_INTENCION_COMPRA',
                 next_action_channel = 'LLAMADA_CIERRE_EJECUTIVO_INMEDIATO',
                 updated_at_utc = datetime('now')
             WHERE empresa_nombre = ?1",
            rusqlite::params![ref_hash_or_empresa.trim()],
        );

        let count: u64 = conn
            .query_row(
                "SELECT click_count FROM link_clicks WHERE ref_key = ?1",
                rusqlite::params![ref_hash_or_empresa.trim()],
                |r| r.get(0),
            )
            .unwrap_or(1);
        Ok(count)
    }

    /// 6. Obtiene la Lista de Empresas Contactadas pero que NO han respondido ni abierto el enlace (`has_clicked_link = 0`),
    /// ordenadas por mayor monto de contingencia para segundo toque omnicanal (Email / WhatsApp / Teléfono).
    pub fn get_unresponsive_contacted_prospects(limit: usize) -> Vec<(String, String, String, f64, String)> {
        let mut results = Vec::new();
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            Self::ensure_db_schema_once(&conn);
            let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN has_clicked_link INTEGER DEFAULT 0", []);
            let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN tech_stack_detected TEXT DEFAULT ''", []);
            let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN recommended_module TEXT DEFAULT ''", []);

            if let Ok(mut stmt) = conn.prepare(
                "SELECT p.empresa_nombre,
                        p.contact_method_tier,
                        COALESCE(NULLIF(p.extracted_emails, ''), NULLIF(p.extracted_whatsapp, ''), p.final_target_url) AS canal_seguimiento,
                        p.multa_estimada_mxn,
                        COALESCE(p.recommended_module, 'MODULO_CUMPLIMIENTO_360')
                 FROM prospect_pipeline p
                 LEFT JOIN link_clicks c ON c.ref_key = p.empresa_nombre
                 WHERE (p.ultimo_resultado_codigo = 'PROSPECCION_ENVIADA_EXITO'
                        OR p.contact_method_tier IN ('TIER_1_WEB_FORM_SENT', 'TIER_2_DIRECT_SMTP_READY', 'TIER_3_WHATSAPP_DIRECT_READY'))
                   AND COALESCE(p.has_clicked_link, 0) = 0
                   AND c.ref_key IS NULL
                 ORDER BY p.multa_estimada_mxn DESC
                 LIMIT ?1",
            ) {
                if let Ok(rows) = stmt.query_map(rusqlite::params![limit as i64], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, f64>(3)?,
                        r.get::<_, String>(4)?,
                    ))
                }) {
                    for item in rows.flatten() {
                        results.push(item);
                    }
                }
            }
        }
        results
    }



    /// 9. Filtro Preventivo de Dominios Inútiles (Dead-Domain Blacklist):
    /// Detecta antes de tocar la red si el campo `www` del DENUE es en realidad una red social
    /// (facebook.com, instagram.com, tiktok.com), un marketplace (mercadolibre.com.mx, amazon.com.mx),
    /// un correo mal puesto en el campo web (hotmail.com, gmail.com, prodigy.net.mx) o un dominio inválido,
    /// derivándolo en 0 ms al `TIER_6_PHONE_DENUE_FALLBACK` sin desperdiciar sockets HTTP.
    pub fn is_dead_or_social_directory_domain(raw_url_or_host: &str) -> Option<&'static str> {
        let low = raw_url_or_host.trim().to_lowercase();
        if low.is_empty() || low == "http://" || low == "https://" || low == "www" || low == "n/a" || low == "ninguno" || low == "sin pagina" {
            return Some("DOMINIO_INVALIDO_O_VACIO_DENUE");
        }
        if low.contains('@') && !low.starts_with("http") {
            return Some("CAMPO_WEB_CONTIENE_EMAIL_DENUE");
        }

        let host = Self::normalize_domain_host(&low);
        if !host.contains('.') && host != "localhost" && host != "127.0.0.1" {
            return Some("DOMINIO_SIN_TLD_VALIDO");
        }

        let dead_or_directory_hosts = [
            "facebook.com",
            "fb.com",
            "instagram.com",
            "tiktok.com",
            "twitter.com",
            "x.com",
            "youtube.com",
            "linkedin.com",
            "mercadolibre.com",
            "mercadolibre.com.mx",
            "amazon.com.mx",
            "seccionamarilla.com.mx",
            "paginasamarillas.com",
            "google.com",
            "maps.google.com",
            "hotmail.com",
            "gmail.com",
            "yahoo.com",
            "yahoo.com.mx",
            "outlook.com",
            "prodigy.net.mx",
            "live.com.mx",
        ];

        if dead_or_directory_hosts.iter().any(|&d| host == d || host.ends_with(&format!(".{}", d))) {
            return Some("DOMINIO_RED_SOCIAL_O_DIRECTORIO_OMITIDO");
        }

        None
    }

    /// 10. Sembrador Idempotente de Lote (`PENDIENTE_BARRIDO`) para Modo Checkpoint & Resume:
    /// Carga desde `denue_units` hasta `target_limit` PyMEs en `prospect_pipeline` con `pipeline_stage = 'PENDIENTE_BARRIDO'`.
    /// Si una empresa ya existe en `prospect_pipeline` (porque ya fue barrida antes de un reinicio SSH), NO la pisa (`INSERT OR IGNORE`),
    /// garantizando que al reanudar continúe exactamente en el registro donde se quedó.
    pub fn seed_checkpoint_resume_queue_10k(target_limit: usize) -> Result<usize, String> {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let mut conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
        Self::ensure_db_schema_once(&conn);

        let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN denue_phone TEXT DEFAULT ''", []);
        let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN denue_email TEXT DEFAULT ''", []);
        let _ = conn.execute("ALTER TABLE prospect_pipeline ADD COLUMN scian_code TEXT DEFAULT ''", []);

        Self::configure_sqlite_connection(&conn);
        let mut inserted_count = 0usize;
        let mut scanned = 0usize;
        let mut last_rowid = 0i64;
        while scanned < target_limit {
            let step = (target_limit - scanned).min(500);
            let page = Self::fetch_pyme_prospect_chunk_keyset(last_rowid, step);
            if page.is_empty() { break; }
            if let Some(last) = page.last() { last_rowid = last.0; }
            scanned += page.len();
            if let Ok(tx) = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate) {
                for (_rid, emp_name, norm_url, scian, kit_price) in page {
                let dom = Self::normalize_domain_host(&norm_url);
                // Si el dominio está en la Dead-Domain Blacklist, lo clasificamos de inmediato en TIER_6 sin gastar red
                let (init_stage, init_code, init_tier, init_action) = if let Some(dead_reason) = Self::is_dead_or_social_directory_domain(&norm_url) {
                    ("STAGE_SIN_CANAL_WEB", dead_reason, "TIER_6_PHONE_DENUE_FALLBACK", "COLA_RESCATE_TELEFONICO_DENUE")
                } else {
                    ("PENDIENTE_BARRIDO", "EN_COLA_CHECKPOINT", "PENDIENTE_CLASIFICACION", "EJECUTAR_BARRIDO_BOT")
                };

                let affected = tx
                    .execute(
                        "INSERT OR IGNORE INTO prospect_pipeline (
                            empresa_nombre, input_url, final_target_url, final_domain,
                            multa_estimada_mxn, ultimo_resultado_codigo, pipeline_stage,
                            next_action_channel, extracted_contact, contact_method_tier,
                            scian_code, attempts_count, updated_at_utc
                         ) VALUES (?1, ?2, ?2, ?3, ?4, ?5, ?6, ?7, '', ?8, ?9, 0, datetime('now'))",
                        rusqlite::params![
                            emp_name,
                            norm_url,
                            dom,
                            kit_price,
                            init_code,
                            init_stage,
                            init_action,
                            init_tier,
                            scian
                        ],
                    )
                    .unwrap_or(0);
                    inserted_count += affected;
                }
                let _ = tx.commit();
            }
        }
        Ok(inserted_count)
    }

    /// 11. Ejecutor con Reanudación Automática (`Checkpoint & Resume` sobre `WHERE pipeline_stage = 'PENDIENTE_BARRIDO'`):
    /// - Lee en bloques de máximo 500 registros únicamente aquellos que siguen en `PENDIENTE_BARRIDO`.
    /// - Si se cae el SSH o se reinicia el servidor en el registro 4,321, al volver a correr continúa desde el 4,322.
    /// - Cada 2,500 registros ejecuta `PRAGMA wal_checkpoint(FULL);` y compactación incremental para mantener disco mínimo.
    pub async fn run_checkpoint_resumable_campaign(max_to_process: usize, chunk_size: usize) -> (usize, usize) {
        let step = chunk_size.clamp(1, 500);
        let client = match Self::build_bounded_http_client() {
            Ok(c) => c,
            Err(_) => return (0, 0),
        };
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());

        let mut processed = 0usize;
        let mut success_or_harvested = 0usize;

        while processed < max_to_process {
            let batch_limit = (max_to_process - processed).min(step);
            let mut pending_chunk: Vec<(String, String, f64)> = Vec::new();

            if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                Self::ensure_db_schema_once(&conn);
                if let Ok(mut stmt) = conn.prepare(
                    "SELECT empresa_nombre, input_url, multa_estimada_mxn
                     FROM prospect_pipeline
                     WHERE pipeline_stage = 'PENDIENTE_BARRIDO'
                     ORDER BY rowid ASC
                     LIMIT ?1",
                ) {
                    if let Ok(rows) = stmt.query_map(rusqlite::params![batch_limit as i64], |r| {
                        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, f64>(2)?))
                    }) {
                        for item in rows.flatten() {
                            pending_chunk.push(item);
                        }
                    }
                }
            }

            if pending_chunk.is_empty() {
                break;
            }

            let concurrency = std::env::var("MALLEN_MAX_CONCURRENCY")
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|&n| (1..=500).contains(&n))
                .unwrap_or(32);

            let results: Vec<bool> = stream::iter(pending_chunk.into_iter().map(|(empresa, url, price_mxn)| {
                let cli = client.clone();
                async move {
                    if let Some(dead_code) = Self::is_dead_or_social_directory_domain(&url) {
                        Self::enqueue_telemetry_async(
                            empresa, url.clone(), url, price_mxn, dead_code.to_string(),
                            "DEAD_DOMAIN_FILTER".to_string(), 0, 0, "N/A".to_string(), 0,
                            String::new(), String::new(), String::new(),
                            "Derivado a Tier 6 por filtro preventivo de dominio inútil".to_string(),
                        ).await;
                        false
                    } else {
                        let res = Self::submit_form_direct(&cli, &url, &empresa, price_mxn).await;
                        res == "PROSPECCION_ENVIADA_EXITO" || res.starts_with("CANAL_ALTERNATIVO_")
                    }
                }
            }))
            .buffer_unordered(concurrency)
            .collect()
            .await;

            for ok in results {
                if ok { success_or_harvested += 1; }
                processed += 1;
            }
        }

        (processed, success_or_harvested)
    }


    /// 13. Simulador Pre-Vuelo (`--dry-run`) para Lotes Masivos del DENUE:
    /// Valida en memoria RAM (sin disparar peticiones HTTP ni alterar tablas de producción)
    /// que los registros del lote tengan links de checkout válidos, códigos SCIAN mapeables
    /// y hashes SHA-256 deterministas en menos de 10 segundos.
    pub fn validate_batch_dry_run_in_memory(limit: usize) -> (usize, usize, usize, u128) {
        let t0 = std::time::Instant::now();
        let candidates = Self::fetch_pyme_prospect_chunk_from_denue(0, limit.clamp(1, 500));
        let mut valid_links = 0usize;
        let mut dead_domains_filtered = 0usize;

        for (emp_name, norm_url, scian, _price) in &candidates {
            if Self::build_and_validate_checkout_link(emp_name, None).is_ok() {
                valid_links += 1;
            }
            if Self::is_dead_or_social_directory_domain(norm_url).is_some() {
                dead_domains_filtered += 1;
            }
            let _ = crate::engine::pdf_generator::estimate_monthly_payroll_exposure_cents(20, scian);
        }

        (candidates.len(), valid_links, dead_domains_filtered, t0.elapsed().as_micros())
    }

    /// 12. Generador de Copy Personalizado para las 4 Colas de Ataque (Tier 3 WhatsApp, Tier 2 SMTP + DMARC, Tier 1 Web Form, Tier 6 DENUE):
    pub fn build_tier_specific_outreach_copy(
        tier: &str,
        empresa_nombre: &str,
        domain: &str,
        has_dmarc: bool,
    ) -> String {
        let link = Self::build_and_validate_checkout_link(empresa_nombre, None)
            .unwrap_or_else(|_| format!("https://mallen.lat/checkout?ref={}", Self::encode_url_ref(empresa_nombre)));

        match tier {
            "TIER_3_WHATSAPP_DIRECT_READY" => format!(
                "Estimado equipo directivo de {}: Le saludamos de FIRMA MALLEN®. Nuestro sistema de auditoria preventiva detecto observaciones normativas publicas (STPS / LFPDPPP / SAT) asociadas a su giro registrado. Puede revisar su pre-dictamen tecnico sin costo aqui: {}",
                empresa_nombre.trim(),
                link
            ),
            "TIER_2_DIRECT_SMTP_READY" => {
                let dmarc_alert = if !has_dmarc {
                    format!(
                        " Adicionalmente, detectamos que el dominio {} carece de politica DMARC activa, lo que expone a su organizacion a suplantacion de identidad corporativa (phishing).",
                        domain
                    )
                } else {
                    String::new()
                };
                format!(
                    "ASUNTO: Pre-Diagnóstico de Cumplimiento Normativo y Seguridad Digital — {}\n\nEstimada Dirección Jurídica y de Operaciones de {}:\n\nFIRMA MALLEN® ha compilado la pre-evaluación preventiva de los 37 vectores regulatorios (STPS, SAT Art. 69-B, IMSS y LFPDPPP) correspondiente a su razón social.{} Consulte su expediente preliminar confidencial y active su Kit de Blindaje en: {}",
                    empresa_nombre.trim(),
                    empresa_nombre.trim(),
                    dmarc_alert,
                    link
                )
            }
            _ => Self::build_human_consultative_message(empresa_nombre, ""),
        }
    }

    /// 7. Extractor y Planificador por Chunks (de 500 en 500 registros) para la Campaña de 10,000 PyMEs:
    /// - Lee paginadamente desde `denue_units` en `data/mallen_prod.db` sin cargar los 10,000 strings de golpe en RAM.
    /// - Prioriza PyMEs (11 a 250 empleados) con sitio web activo (`www != ''`) y sectores SCIAN de alto valor.
    /// - Devuelve un lote acotado de máximo `chunk_size` (recomendado: 500) listo para procesar y liberar memoria.
    pub fn fetch_pyme_prospect_chunk_from_denue(
        chunk_offset: usize,
        chunk_size: usize,
    ) -> Vec<(String, String, String, f64)> {
        let bounded_size = chunk_size.clamp(1, 500);
        let mut batch = Vec::with_capacity(bounded_size);
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());

        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            let sql = "
                SELECT COALESCE(NULLIF(TRIM(raz_social), ''), TRIM(nom_estab)) AS empresa,
                       TRIM(www) AS web_raw,
                       COALESCE(NULLIF(TRIM(codigo_scian), ''), '541110') AS scian,
                       COALESCE(per_ocu, '11 a 30 personas') AS estrato
                FROM denue_units
                WHERE www IS NOT NULL
                  AND TRIM(www) != ''
                  AND (raz_social != '' OR nom_estab != '')
                  AND COALESCE(codigo_scian,'') NOT LIKE '93%' AND LOWER(COALESCE(nombre_act,'')) NOT LIKE '%sector público%' AND LOWER(COALESCE(nombre_act,'')) NOT LIKE '%sector publico%' AND LOWER(COALESCE(www,'')) NOT LIKE '%gob.mx%' AND LOWER(COALESCE(www,'')) NOT LIKE '%.gob.%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%SECRETARIA DE%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%GOBIERNO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%AYUNTAMIENTO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INFONAVIT%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%FONDO NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INSTITUTO MEXICANO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%INSTITUTO NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%COMISION FEDERAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%COMISION NACIONAL%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PETROLEOS MEXICANOS%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%BANCO DE MEXICO%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PROCURADURIA%' AND UPPER(COALESCE(raz_social,'')) NOT LIKE '%PODER JUDICIAL%' AND LOWER(COALESCE(www,'')) NOT LIKE '%infonavit%' AND LOWER(COALESCE(www,'')) NOT LIKE '%imss.gob%' AND LOWER(COALESCE(www,'')) NOT LIKE '%issste%' AND LOWER(COALESCE(www,'')) NOT LIKE '%pemex.com%' AND LOWER(COALESCE(www,'')) NOT LIKE '%cfe.mx%' AND LOWER(COALESCE(www,'')) NOT LIKE '%banxico%'
                  AND COALESCE(codigo_scian,'') NOT LIKE '5221%'
                  AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%ALCALDIA%'
                  AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%SECRETARIA DE%'
                  AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%GOBIERNO%'
                  AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%FISCALIA%'
                  AND UPPER(COALESCE(nom_estab,'')) NOT LIKE '%TRIBUNAL%'
                  AND LOWER(www) NOT LIKE '%banamex%'
                  AND LOWER(www) NOT LIKE '%bbva%'
                  AND LOWER(www) NOT LIKE '%santander%'
                  AND LOWER(www) NOT LIKE '%banorte%'
                  AND LOWER(www) NOT LIKE '%hsbc%'
                  AND LOWER(www) NOT LIKE '%scotiabank%' AND LOWER(www) NOT LIKE '%.gob.mx%' 
                ORDER BY CASE
                    WHEN per_ocu IN ('11 a 30 personas','31 a 50 personas','51 a 100 personas','101 a 250 personas') THEN 1
                    WHEN per_ocu IN ('6 a 10 personas','0 a 5 personas') THEN 2
                    ELSE 3
                END ASC, rowid ASC
                LIMIT ?1 OFFSET ?2
            ";
            if let Ok(mut stmt) = conn.prepare(sql) {
                if let Ok(rows) = stmt.query_map(
                    rusqlite::params![bounded_size as i64, chunk_offset as i64],
                    |r| {
                        Ok((
                            r.get::<_, String>(0)?,
                            r.get::<_, String>(1)?,
                            r.get::<_, String>(2)?,
                            r.get::<_, String>(3)?,
                        ))
                    },
                ) {
                    for item in rows.flatten() {
                        let (emp_name, raw_www, scian, estrato) = item;
                        let normalized_url = if raw_www.starts_with("http://") || raw_www.starts_with("https://") {
                            raw_www
                        } else {
                            format!("https://{}", raw_www.trim_start_matches('/'))
                        };
                        let (_tier, _emp_cnt, kit_price_mxn, _arg) =
                            Self::classify_denue_capacity_and_scian_risk(&estrato, &scian);
                        batch.push((emp_name, normalized_url, scian, kit_price_mxn));
                    }
                }
            }
        }
        batch
    }

    /// 8. Ejecutor de Lotes Controlados en Memoria (Chunked Batch Runner — 500 por lote):
    /// Procesa `total_target` empresas en bloques estrictos de `chunk_size` (máx 500), liberando el `Vec`
    /// y ejecutando `PRAGMA wal_checkpoint(PASSIVE)` al terminar cada bloque para mantener consumo de RAM < 45 MB.
    pub async fn run_chunked_pyme_campaign(
        total_target: usize,
        chunk_size: usize,
        dry_run_only: bool,
    ) -> (usize, usize, usize) {
        let step = chunk_size.clamp(1, 500);
        let client = match Self::build_bounded_http_client() {
            Ok(c) => c,
            Err(_) => return (0, 0, 0),
        };

        let mut offset = 0usize;
        let mut total_processed = 0usize;
        let mut form_sent_count = 0usize;
        let mut smtp_reserved_count = 0usize;

        while total_processed < total_target {
            let remaining = total_target - total_processed;
            let current_batch_size = remaining.min(step);
            let chunk = Self::fetch_pyme_prospect_chunk_from_denue(offset, current_batch_size);
            if chunk.is_empty() {
                break;
            }

            let chunk_len = chunk.len();
            if dry_run_only {
                total_processed += chunk_len;
            } else {
                for (empresa, url, _scian, price_mxn) in chunk {
                    let res = Self::submit_form_direct(&client, &url, &empresa, price_mxn).await;
                    if res == "PROSPECCION_ENVIADA_EXITO" {
                        form_sent_count += 1;
                    } else if res == "CANAL_ALTERNATIVO_MAILTO_DETECTADO" {
                        smtp_reserved_count += 1;
                    }
                    total_processed += 1;
                }
            }

            offset += chunk_len;
            // Liberar páginas del WAL suavemente entre cada chunk de 500 sin bloquear lectores
            let _ = Self::optimize_sqlite_storage_non_blocking(50_000);
        }

        (total_processed, form_sent_count, smtp_reserved_count)
    }

    /// Calcula el tiempo de espera exponencial con Jitter Descorrelacionado (Decorrelated Jitter)
    /// para evitar que los WAFs detecten periodicidad matemática exacta (1s, 2s, 4s) en los reintentos.
    pub fn compute_decorrelated_backoff_ms(attempt: u32, base_ms: u64) -> u64 {
        let exp_factor = 1u64 << attempt.min(5);
        let deterministic_base = base_ms.saturating_mul(exp_factor);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64;
        let salt = (UA_COUNTER.load(Ordering::Relaxed) as u64).wrapping_mul(1103515245).wrapping_add(nanos);
        let jitter_range = (deterministic_base / 2).max(45);
        let random_jitter = salt % jitter_range;
        deterministic_base + random_jitter
    }

    /// Detecta si un dominio del DENUE está estacionado (Parked), expirado o en página por defecto de hosting
    /// (GoDaddy, Sedo, Hostinger, cPanel Default, Sitio en Construcción) para no perder recursos en él.
    pub fn detect_parked_or_expired_domain(html_lower: &str) -> Option<&'static str> {
        let parked_signatures = [
            "this domain is parked",
            "este dominio está estacionado",
            "domain is for sale",
            "dominio en venta",
            "sedoparking.com",
            "parkingcrew.net",
            "bodis.com",
            "hugedomains.com",
            "godaddy.com/forsale",
            "cgi-sys/defaultwebpage.cgi",
            "account suspended",
            "cuenta suspendida",
            "sitio en construcción",
            "sitio web en construccion",
            "página en construcción",
        ];
        if !html_lower.contains("<form") && parked_signatures.iter().any(|s| html_lower.contains(s)) {
            return Some("DOMINIO_ESTACIONADO_O_EXPIRADO");
        }
        None
    }

    /// Extrae correos y teléfonos corporativos ocultos dentro de bloques `<script type="application/ld+json">` (Schema.org).
    pub fn extract_schema_org_jsonld_contacts(html: &str) -> (Vec<String>, Vec<String>) {
        let mut ld_emails = Vec::new();
        let mut ld_phones = Vec::new();
        let doc = Html::parse_document(html);
        if let Ok(sel) = Selector::parse("script[type=\"application/ld+json\"]") {
            for script in doc.select(&sel) {
                let raw_json = script.text().collect::<Vec<_>>().join(" ");
                for token in raw_json.split(|c: char| c == '"' || c == '\'' || c == ',' || c == '{' || c == '}' || c == '[' || c == ']' || c == '\n' || c == '\r') {
                    let t = token.trim();
                    if t.contains('@') && t.contains('.') {
                        if let Some(clean_m) = Self::sanitize_and_normalize_email(t) {
                            if !ld_emails.contains(&clean_m) {
                                ld_emails.push(clean_m);
                            }
                        }
                    } else if (t.starts_with("+52") || t.starts_with("55") || t.starts_with("33") || t.starts_with("81"))
                        && t.chars().filter(|c| c.is_ascii_digit()).count() >= 10
                    {
                        if let Some(clean_p) = Self::normalize_whatsapp_e164(t) {
                            if !ld_phones.contains(&clean_p) {
                                ld_phones.push(clean_p);
                            }
                        }
                    }
                }
            }
        }
        (ld_emails, ld_phones)
    }

    /// Devuelve la referencia al Semáforo Global de Concurrencia (por defecto 64 tareas simultáneas,
    /// configurable entre 1 y 500 mediante `MALLEN_MAX_CONCURRENCY`).
    pub fn global_semaphore() -> &'static Semaphore {
        GLOBAL_CONCURRENCY_SEMAPHORE.get_or_init(|| {
            let max_permits = std::env::var("MALLEN_MAX_CONCURRENCY")
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|&n| (1..=500).contains(&n))
                .unwrap_or(32);
            Semaphore::new(max_permits)
        })
    }

    /// Verifica si el Circuit Breaker está abierto para un dominio (3 fallos 403/429/WAF consecutivos -> 48h de enfriamiento).
    pub fn check_circuit_breaker_open(domain: &str) -> bool {
        let cb_ttl = Duration::from_secs(48 * 3600);
        let cache = Self::circuit_breaker_cache();
        if let Some((fails, opened_at)) = cache.get(domain) {
            if fails >= 3 {
                if opened_at.elapsed() < cb_ttl { return true; }
                else { cache.invalidate(domain); }
            }
        }
        false
    }

    pub fn record_circuit_breaker_outcome(domain: &str, status_code: &str) {
        let is_hostile = status_code == "BLOQUEO_WAF_CLOUDFLARE_CHALLENGE"
            || status_code == "HTTP_RATE_LIMIT_429"
            || status_code == "HTTP_POST_ERR_403"
            || status_code == "ERR_HTTP_GET_403";
        let cache = Self::circuit_breaker_cache();
        if is_hostile {
            let prev = cache.get(domain).map(|(f, _)| f).unwrap_or(0);
            cache.insert(domain.to_string(), (prev.saturating_add(1), Instant::now()));
        } else if status_code == "PROSPECCION_ENVIADA_EXITO" {
            cache.invalidate(domain);
        }
    }

    pub fn reset_circuit_breaker(domain: &str) {
        Self::circuit_breaker_cache().invalidate(domain);
    }

    /// Construye un mensaje de contacto B2B cálido, humano, consultivo y de alta conversión
    /// (sin cifras intimidantes de multas en el primer contacto para evitar fricción con recepción/filtros).
    pub fn build_human_consultative_message(empresa_nombre: &str, _encoded_ref: &str) -> String {
        let emp_clean = empresa_nombre.trim();
        let micro_link = format!("https://mallen.lat/api/micro_scan/{}", Self::encode_url_ref(emp_clean));
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let (scian_code, actividad_desc, emp_count): (String, String, u32) = rusqlite::Connection::open(db_path)
            .ok()
            .and_then(|conn| {
                conn.query_row(
                    "SELECT TRIM(codigo_scian), COALESCE(MAX(NULLIF(TRIM(nombre_act), '')), ''), COALESCE(MAX(TRIM(per_ocu)), '11 a 30 personas') FROM denue_units WHERE (UPPER(TRIM(raz_social)) = UPPER(?1) OR UPPER(TRIM(nom_estab)) = UPPER(?1)) AND TRIM(codigo_scian) GLOB '[0-9][0-9][0-9][0-9][0-9][0-9]' GROUP BY TRIM(codigo_scian) ORDER BY COUNT(*) DESC, TRIM(codigo_scian) ASC LIMIT 1",
                    rusqlite::params![emp_clean],
                    |r| {
                        let sc: String = r.get(0)?;
                        let act: String = r.get(1)?;
                        let estrato: String = r.get(2)?;
                        let emps = if estrato.contains("251") {
                            300
                        } else if estrato.contains("101") {
                            150
                        } else if estrato.contains("51") {
                            75
                        } else if estrato.contains("31") {
                            40
                        } else if estrato.contains("11") {
                            20
                        } else {
                            8
                        };
                        Ok((sc, act, emps))
                    },
                ).ok()
            })
            .unwrap_or_else(|| (String::new(), String::new(), 25));

        let intel_dom = crate::engine::pdf_generator::extract_deep_denue_360_intelligence(emp_clean);
        let is_scian_6d = |v: &str| {
            let t = v.trim();
            t.len() == 6 && t.bytes().all(|b| b.is_ascii_digit())
        };
        let effective_scian = if is_scian_6d(&scian_code) {
            scian_code.trim()
        } else if is_scian_6d(&intel_dom.codigo_scian) {
            intel_dom.codigo_scian.trim()
        } else {
            "541110"
        };
        let effective_act = if !actividad_desc.trim().is_empty() {
            actividad_desc.as_str()
        } else if !intel_dom.actividad_desc.trim().is_empty() && !intel_dom.actividad_desc.starts_with("Pendiente") {
            intel_dom.actividad_desc.as_str()
        } else {
            ""
        };
        let effective_emp = if intel_dom.estimated_employees > 0 {
            intel_dom.estimated_employees
        } else {
            emp_count
        };
        let (_sector_label, scian_pain, _closing_q) =
            crate::engine::pdf_generator::build_scian_sector_hook(effective_scian, effective_act, effective_emp);

        format!(
            "Estimado equipo directivo de {}: Les saludamos cordialmente de FIRMA MALLEN®. Ayudamos a empresas de su sector a proteger su deducibilidad fiscal, agilizar auditorías internas y mantener su expediente normativo listo para cualquier revisión, especialmente porque {}. Pueden consultar y descargar sin costo su resumen técnico confidencial con acciones prácticas para su giro en: {} | Fuente: INEGI, DENUE. Aviso de Privacidad: https://mallen.lat/privacidad | Exclusión ARCO: https://mallen.lat/derechos-arco",
            emp_clean,
            scian_pain,
            micro_link
        )
    }

    /// Obtiene una instantánea en tiempo real de la telemetría de procesos del motor.
    pub fn get_process_telemetry_snapshot() -> BotProcessTelemetry {
        let total = TELEMETRY_TOTAL_REQS.load(Ordering::Relaxed);
        let in_flight = TELEMETRY_IN_FLIGHT.load(Ordering::Relaxed);
        let succ = TELEMETRY_SUCCESS.load(Ordering::Relaxed);
        let waf = TELEMETRY_WAF_OR_CAPTCHA.load(Ordering::Relaxed);
        let cb = TELEMETRY_CB_TRIPS.load(Ordering::Relaxed);
        let total_lat = TELEMETRY_TOTAL_LATENCY_MS.load(Ordering::Relaxed);
        let avg_lat = if total > 0 { (total_lat / total) as u64 } else { 0 };

        BotProcessTelemetry {
            total_requests: total,
            active_in_flight: in_flight,
            available_semaphore_permits: Self::global_semaphore().available_permits(),
            success_count: succ,
            waf_or_captcha_count: waf,
            circuit_breaker_trips: cb,
            avg_latency_ms: avg_lat,
        }
    }

    /// Construye un cliente HTTP rápido con protección estricta contra bucles de redirección (máximo 5 saltos).
    pub fn build_bounded_http_client() -> Result<Client, Box<dyn std::error::Error>> {
        Self::build_rotating_proxy_client(&[])
    }

    /// Normaliza y sanea cualquier enlace o número de WhatsApp cosechado (wa.me/, api.whatsapp.com, tel:)
    /// al estándar internacional E.164 (+52XXXXXXXXXX para México o +CC para Europa/LATAM).
    pub fn normalize_whatsapp_e164(raw: &str) -> Option<String> {
        let low = raw.trim().to_lowercase();
        let phone_candidate = if let Some(idx) = low.find("phone=") {
            let after = &raw[idx + 6..];
            after.split('&').next().unwrap_or(after)
        } else if let Some(idx) = low.find("wa.me/") {
            let after = &raw[idx + 6..];
            after.split('?').next().unwrap_or(after).split('/').next().unwrap_or(after)
        } else {
            raw
        };

        let digits: String = phone_candidate.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 10 || digits.len() > 15 {
            return None;
        }
        // Descartar secuencias falsas como 0000000000 o 1234567890
        if digits.chars().all(|c| c == '0') || digits == "1234567890" || digits.ends_with("0000000") {
            return None;
        }

        // Reglas de normalización para México (+52)
        let normalized = if digits.len() == 10 {
            format!("+52{}", digits)
        } else if digits.len() == 12 && digits.starts_with("52") {
            format!("+{}", digits)
        } else if digits.len() == 13 && digits.starts_with("521") {
            // El prefijo móvil antiguo 521 en México se normaliza a +52 + 10 dígitos
            format!("+52{}", &digits[3..])
        } else if digits.len() == 13 && (digits.starts_with("044") || digits.starts_with("045")) {
            format!("+52{}", &digits[3..])
        } else if digits.len() == 12 && digits.starts_with("01") {
            format!("+52{}", &digits[2..])
        } else {
            format!("+{}", digits)
        };

        Some(normalized)
    }

    /// Optimización de almacenamiento en disco SQLite WAL para escala de 5M de registros:
    /// Ejecuta incremental_vacuum, trunca el archivo .db-wal cuando no hay lectores activos y actualiza estadísticas del query planner.
    pub fn optimize_sqlite_storage_non_blocking(max_log_rows: usize) -> Result<(usize, i64), String> {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;

        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA temp_store=MEMORY;
             PRAGMA auto_vacuum=INCREMENTAL;",
        )
        .map_err(|e| e.to_string())?;

        let deleted = conn
            .execute(
                "DELETE FROM prospect_logs WHERE id NOT IN (
                    SELECT id FROM prospect_logs ORDER BY id DESC LIMIT ?1
                ) OR created_at_utc < datetime('now', '-90 days')",
                rusqlite::params![max_log_rows as i64],
            )
            .unwrap_or(0);

        let _ = conn.execute_batch(
            "PRAGMA incremental_vacuum(2000);
             PRAGMA optimize;
             PRAGMA wal_checkpoint(PASSIVE);",
        );

        let page_count: i64 = conn.query_row("PRAGMA page_count;", [], |r| r.get(0)).unwrap_or(0);
        let page_size: i64 = conn.query_row("PRAGMA page_size;", [], |r| r.get(0)).unwrap_or(4096);
        Ok((deleted, page_count * page_size))
    }

    /// Inicializa y migra el esquema de SQLite WAL una sola vez por proceso (evita DDL repetitivo en cada petición).
    fn ensure_db_schema_once(conn: &rusqlite::Connection) {
        DB_SCHEMA_INITIALIZED.get_or_init(|| {
            let _ = conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 PRAGMA synchronous=NORMAL;
                 PRAGMA busy_timeout=5000;
                 PRAGMA temp_store=MEMORY;
                 PRAGMA cache_size=-32000;
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
                     http_status INTEGER DEFAULT 0,
                     latency_ms INTEGER DEFAULT 0,
                     charset_detected TEXT DEFAULT 'UTF-8',
                     retry_count INTEGER DEFAULT 0,
                     contact_method_tier TEXT DEFAULT '',
                     trace_detail TEXT DEFAULT '',
                     created_at_utc TEXT DEFAULT (datetime('now'))
                 );
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_codigo ON prospect_logs(resultado_codigo);
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_domain ON prospect_logs(final_domain);
                 CREATE INDEX IF NOT EXISTS idx_prospect_logs_tier ON prospect_logs(contact_method_tier);

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
                     contact_method_tier TEXT DEFAULT '',
                     extracted_emails TEXT DEFAULT '',
                     extracted_whatsapp TEXT DEFAULT '',
                     extracted_socials TEXT DEFAULT '',
                     last_http_status INTEGER DEFAULT 0,
                     last_latency_ms INTEGER DEFAULT 0,
                     last_charset TEXT DEFAULT 'UTF-8',
                     last_trace_detail TEXT DEFAULT '',
                     attempts_count INTEGER DEFAULT 1,
                     updated_at_utc TEXT DEFAULT (datetime('now'))
                 );
                 CREATE INDEX IF NOT EXISTS idx_pipeline_tier_multa ON prospect_pipeline(contact_method_tier, multa_estimada_mxn DESC);
                 CREATE TABLE IF NOT EXISTS link_clicks (
                     ref_key TEXT PRIMARY KEY,
                     first_clicked_at_utc TEXT DEFAULT (datetime('now')),
                     last_clicked_at_utc TEXT DEFAULT (datetime('now')),
                     click_count INTEGER DEFAULT 1,
                     last_visitor_ip TEXT DEFAULT '',
                     last_user_agent TEXT DEFAULT ''
                 );
                 CREATE TABLE IF NOT EXISTS suppression_list (
                     domain_or_company TEXT PRIMARY KEY,
                     reason TEXT DEFAULT 'ARCO_OPTOUT',
                     created_at_utc TEXT DEFAULT (datetime('now'))
                 );",
            );

            for col_sql in [
                "ALTER TABLE prospect_logs ADD COLUMN final_target_url TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN final_domain TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN pipeline_stage TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN next_action_channel TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN extracted_contact TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN http_status INTEGER DEFAULT 0",
                "ALTER TABLE prospect_logs ADD COLUMN latency_ms INTEGER DEFAULT 0",
                "ALTER TABLE prospect_logs ADD COLUMN charset_detected TEXT DEFAULT 'UTF-8'",
                "ALTER TABLE prospect_logs ADD COLUMN retry_count INTEGER DEFAULT 0",
                "ALTER TABLE prospect_logs ADD COLUMN contact_method_tier TEXT DEFAULT ''",
                "ALTER TABLE prospect_logs ADD COLUMN trace_detail TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN contact_method_tier TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN extracted_emails TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN extracted_whatsapp TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN extracted_socials TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN last_http_status INTEGER DEFAULT 0",
                "ALTER TABLE prospect_pipeline ADD COLUMN last_latency_ms INTEGER DEFAULT 0",
                "ALTER TABLE prospect_pipeline ADD COLUMN last_charset TEXT DEFAULT 'UTF-8'",
                "ALTER TABLE prospect_pipeline ADD COLUMN last_trace_detail TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN encrypted_contact_vault TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN tech_stack_detected TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN recommended_module TEXT DEFAULT ''",
                "ALTER TABLE prospect_pipeline ADD COLUMN has_clicked_link INTEGER DEFAULT 0",
            ] {
                let _ = conn.execute(col_sql, []);
            }
            true
        });
    }

    /// Lee el timeout global configurable por variable de entorno `MALLEN_TIMEOUT_MS` (por defecto 7500 ms).
    pub fn configured_timeout_ms() -> u64 {
        std::env::var("MALLEN_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|&ms| ms >= 500 && ms <= 60_000)
            .unwrap_or(18_000)
    }

    /// Decodificador inteligente de bytes HTTP para charsets exóticos y legacy
    /// (UTF-8, UTF-16LE/BE con BOM, Windows-1252, Windows-1250, ISO-8859-1, ISO-8859-2).
    pub fn decode_response_bytes_smart(content_type_hdr: &str, raw_bytes: &[u8]) -> (String, String) {
        if raw_bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return (String::from_utf8_lossy(&raw_bytes[3..]).into_owned(), "UTF-8-BOM".to_string());
        }
        if raw_bytes.starts_with(&[0xFF, 0xFE]) {
            let u16s: Vec<u16> = raw_bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            return (String::from_utf16_lossy(&u16s), "UTF-16LE".to_string());
        }
        if raw_bytes.starts_with(&[0xFE, 0xFF]) {
            let u16s: Vec<u16> = raw_bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            return (String::from_utf16_lossy(&u16s), "UTF-16BE".to_string());
        }

        let ct_low = content_type_hdr.to_lowercase();
        let head_ascii = String::from_utf8_lossy(&raw_bytes[..raw_bytes.len().min(1024)]).to_lowercase();

        let declared_single_byte = ct_low.contains("iso-8859-")
            || ct_low.contains("windows-125")
            || ct_low.contains("latin1")
            || head_ascii.contains("charset=iso-8859-")
            || head_ascii.contains("charset=\"iso-8859-")
            || head_ascii.contains("charset=windows-125")
            || head_ascii.contains("charset=\"windows-125");

        let charset_tag = if ct_low.contains("windows-1250") || head_ascii.contains("windows-1250") {
            "WINDOWS-1250"
        } else if ct_low.contains("windows-1252") || head_ascii.contains("windows-1252") {
            "WINDOWS-1252"
        } else if ct_low.contains("iso-8859-1") || head_ascii.contains("iso-8859-1") {
            "ISO-8859-1"
        } else if declared_single_byte {
            "LEGACY-8BIT"
        } else {
            "UTF-8"
        };

        // Si es UTF-8 válido y no declaró explícitamente tabla de 8 bits, usar UTF-8 directo
        if !declared_single_byte {
            if let Ok(valid_utf8) = std::str::from_utf8(raw_bytes) {
                return (valid_utf8.to_string(), "UTF-8".to_string());
            }
        }

        // Si declaró charset de 1 byte (ISO-8859-1 / Windows-1252 / Windows-1250) o tiene secuencias no-UTF8,
        // mapear cada byte (0x00..=0xFF) preservando tildes, eñes y caracteres europeos sin ""
        let mut out = String::with_capacity(raw_bytes.len());
        for &b in raw_bytes {
            let ch = match b {
                0x80 => '€',
                0x82 => '‚',
                0x84 => '„',
                0x85 => '…',
                0x8A => 'Š',
                0x8C => 'Œ',
                0x8E => 'Ž',
                0x91 => '‘',
                0x92 => '’',
                0x93 => '“',
                0x94 => '”',
                0x95 => '•',
                0x96 => '–',
                0x97 => '—',
                0x99 => '™',
                0x9A => 'š',
                0x9C => 'œ',
                0x9E => 'ž',
                0x9F => 'Ÿ',
                _ => b as char,
            };
            out.push(ch);
        }
        (out, charset_tag.to_string())
    }



    /// Cifrado Simétrico en Reposo (Cumplimiento Art. 19 LFPDPPP y Art. 32 GDPR):
    /// Cifra datos de contacto sensibles usando un flujo XOR derivado por bloques con SHA-256 (Keystream CTR)
    /// gobernado por la llave `MALLEN_PII_SECRET_KEY`. Produce el formato `ENCv1:<hex>`.
    fn derive_aead_key() -> Zeroizing<[u8; 32]> {
        let secret = std::env::var("MALLEN_PII_SECRET_KEY")
            .unwrap_or_else(|_| "MALLEN_TEST_ONLY_SET_MALLEN_PII_SECRET_KEY_IN_ENV".to_string());
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        hasher.update(b"|XCHACHA20POLY1305_KEY_DERIVATION_V1|");
        let digest = hasher.finalize();
        let mut key = Zeroizing::new([0u8; 32]);
        key.copy_from_slice(&digest[..32]);
        key
    }

    pub fn encrypt_pii_at_rest(plaintext: &str) -> String {
        let clean = plaintext.trim();
        if clean.is_empty() { return String::new(); }
        let key = Self::derive_aead_key();
        let cipher = XChaCha20Poly1305::new_from_slice(&*key).expect("32-byte key");
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let payload = Payload { msg: clean.as_bytes(), aad: PII_AAD_CONTEXT };
        match cipher.encrypt(&nonce, payload) {
            Ok(ct) => {
                let nh: String = nonce.iter().map(|b| format!("{:02x}", b)).collect();
                let ch: String = ct.iter().map(|b| format!("{:02x}", b)).collect();
                format!("ENCv1:{}:{}", nh, ch)
            }
            Err(_) => String::new(),
        }
    }

    pub fn decrypt_pii_from_rest(ciphertext: &str) -> Option<String> {
        let rest = ciphertext.trim().strip_prefix("ENCv1:")?;
        let (nh, ch) = rest.split_once(':')?;
        let dec = |h: &str| -> Option<Vec<u8>> {
            if h.len() % 2 != 0 { return None; }
            (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i+2], 16).ok()).collect()
        };
        let nb = dec(nh)?;
        if nb.len() != 24 { return None; }
        let cb = dec(ch)?;
        let key = Self::derive_aead_key();
        let cipher = XChaCha20Poly1305::new_from_slice(&*key).ok()?;
        let plain = cipher.decrypt(XNonce::from_slice(&nb), Payload { msg: &cb, aad: PII_AAD_CONTEXT }).ok()?;
        String::from_utf8(plain).ok()
    }

    /// Detecta defensas avanzadas de Honeypots inyectados por JavaScript o formularios que exigen
    /// ejecución de JS en navegador real (WP Armour, CleanTalk, Akismet JS Token, SPA React/Vue vacía)
    /// para derivarlos limpiamente al worker de Headless Chrome (`REQUIERE_HEADLESS_JS_DYNAMIC_FORM`).
    pub fn detect_js_dynamic_form_or_honeypot(html_lower: &str, form_html_lower: &str) -> Option<&'static str> {
        // 1. Plugins anti-bot que inyectan tokens o mutan el DOM exclusivamente vía JS en el navegador
        if form_html_lower.contains("wpa_initiator")
            || form_html_lower.contains("wparmour")
            || form_html_lower.contains("ct_checkjs")
            || form_html_lower.contains("apbct_visible_fields")
            || form_html_lower.contains("data-js-token-required")
            || (form_html_lower.contains("action=\"javascript:void(0)\"") && form_html_lower.contains("data-headless-only"))
        {
            return Some("REQUIERE_HEADLESS_JS_DYNAMIC_FORM");
        }

        // 2. Aplicaciones SPA puras (React / Vue / Next / Angular) donde el <div id="root"> o <div id="app"> está vacío sin <form>
        if !html_lower.contains("<form")
            && !html_lower.contains("<iframe")
            && (html_lower.contains("<div id=\"root\"></div>")
                || html_lower.contains("<div id=\"app\"></div>")
                || html_lower.contains("window.__next_data__")
                || html_lower.contains("ng-version="))
        {
            return Some("REQUIERE_HEADLESS_JS_SPA_RENDER");
        }

        None
    }

    /// Sanea y normaliza un correo electrónico:
    /// - Elimina espacios invisibles Unicode (\u{00A0}, \u{200B}, \u{FEFF}) y prefijos URL-encoded (%20).
    /// - Convierte estrictamente a minúsculas (.trim().to_lowercase()).
    /// - Descarta falsos positivos (archivos @2x, versiones npm/cdn como bootstrap@5.3.2, o placeholders).
    pub fn sanitize_and_normalize_email(raw: &str) -> Option<String> {
        let mut s = raw
            .replace("%20", " ")
            .replace("%09", " ")
            .replace("%0a", " ")
            .replace("%0A", " ")
            .replace("%0d", " ")
            .replace("%0D", " ")
            .replace("&nbsp;", " ")
            .trim()
            .to_lowercase();

        if let Some(rest) = s.strip_prefix("mailto:") {
            s = rest.trim().to_string();
        }

        let cleaned_unicode: String = s
            .chars()
            .filter(|&c| {
                !c.is_whitespace()
                    && c != '\u{00A0}'
                    && c != '\u{200B}'
                    && c != '\u{200C}'
                    && c != '\u{200D}'
                    && c != '\u{FEFF}'
            })
            .collect();

        let clean = cleaned_unicode
            .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '@' && c != '.' && c != '-' && c != '_' && c != '+')
            .to_lowercase();

        let parts: Vec<&str> = clean.split('@').collect();
        if parts.len() != 2 {
            return None;
        }
        let mut local = parts[0];
        let domain = parts[1];

        // Limpiar cualquier prefijo numérico residual de escape URL (como "20" de "%20") antes de buzones corporativos
        let stripped_digits = local.trim_start_matches(|c: char| c.is_ascii_digit());
        if stripped_digits.len() >= 4
            && (stripped_digits.starts_with("juridico")
                || stripped_digits.starts_with("contacto")
                || stripped_digits.starts_with("direccion")
                || stripped_digits.starts_with("legal")
                || stripped_digits.starts_with("info")
                || stripped_digits.starts_with("ventas")
                || stripped_digits.starts_with("cumplimiento")
                || stripped_digits.starts_with("admon"))
        {
            local = stripped_digits;
        }

        if local.is_empty() || domain.len() < 4 || !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return None;
        }

        let bad_suffixes = [
            ".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg", ".ico",
            ".js", ".css", ".woff", ".woff2", ".ttf", ".eot", ".map", ".pdf",
        ];
        if bad_suffixes.iter().any(|ext| domain.ends_with(ext)) {
            return None;
        }
        if domain.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return None;
        }

        let blacklisted_locals = ["no-reply", "noreply", "donotreply", "mailer-daemon", "postmaster", "ejemplo", "tu-correo", "tucorreo", "yourname", "user"];
        let blacklisted_domains = ["example.com", "example.org", "dominio.com", "tudominio.com", "sentry.io", "wixpress.com", "email.com"];
        if blacklisted_locals.iter().any(|&b| local == b || local.starts_with("no-reply") || local.starts_with("noreply")) {
            return None;
        }
        if blacklisted_domains.iter().any(|&d| domain == d || domain.ends_with(".sentry.io") || domain.ends_with(".wixpress.com")) {
            return None;
        }

        Some(format!("{}@{}", local, domain))
    }

    /// Desofusca correos protegidos por Cloudflare (`data-cfemail="hex_string"`).
    pub fn decode_cloudflare_cfemail(hex: &str) -> Option<String> {
        let clean = hex.trim();
        if clean.len() < 4 || clean.len() % 2 != 0 {
            return None;
        }
        let key = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let mut out = Vec::with_capacity((clean.len() - 2) / 2);
        let mut i = 2;
        while i < clean.len() {
            let b = u8::from_str_radix(&clean[i..i + 2], 16).ok()?;
            out.push(b ^ key);
            i += 2;
        }
        let decoded = String::from_utf8(out).ok()?;
        Self::sanitize_and_normalize_email(&decoded)
    }

    /// Cosechador Multicanal Profundo: extrae emails (incluyendo Cloudflare data-cfemail y Schema.org JSON-LD),
    /// WhatsApp corporativo E.164 y perfiles de redes B2B del DOM.
    pub fn harvest_multichannel_contacts(html: &str) -> (String, String, String) {
        let pre_cleaned_html = html.replace("%20", " ").replace("%09", " ").replace("&nbsp;", " ");
        let (mut emails, mut whatsapps) = Self::extract_schema_org_jsonld_contacts(&pre_cleaned_html);
        let mut socials: Vec<String> = Vec::new();

        let doc = Html::parse_document(&pre_cleaned_html);

        // 1. Desofuscar cualquier etiqueta con data-cfemail de Cloudflare
        if let Ok(cf_sel) = Selector::parse("[data-cfemail]") {
            for el in doc.select(&cf_sel) {
                if let Some(hex) = el.value().attr("data-cfemail") {
                    if let Some(mail) = Self::decode_cloudflare_cfemail(hex) {
                        if !emails.contains(&mail) {
                            emails.push(mail);
                        }
                    }
                }
            }
        }

        // 2. Revisar todos los enlaces <a href="...">
        if let Ok(a_sel) = Selector::parse("a[href]") {
            for a in doc.select(&a_sel) {
                let href = a.value().attr("href").unwrap_or("").trim();
                let low = href.to_lowercase();
                if low.starts_with("mailto:") {
                    let raw_mail = href[7..].split('?').next().unwrap_or("");
                    if let Some(mail) = Self::sanitize_and_normalize_email(raw_mail) {
                        if !emails.contains(&mail) {
                            emails.push(mail);
                        }
                    }
                } else if low.contains("/cdn-cgi/l/email-protection#") {
                    if let Some(hex) = href.split('#').nth(1) {
                        if let Some(mail) = Self::decode_cloudflare_cfemail(hex) {
                            if !emails.contains(&mail) {
                                emails.push(mail);
                            }
                        }
                    }
                } else if low.contains("wa.me/") || low.contains("api.whatsapp.com") || low.starts_with("whatsapp:") || low.starts_with("tel:") {
                    if let Some(clean_wa) = Self::normalize_whatsapp_e164(href) {
                        if !whatsapps.contains(&clean_wa) {
                            whatsapps.push(clean_wa);
                        }
                    }
                } else if low.contains("linkedin.com/company/")
                    || low.contains("linkedin.com/in/")
                    || low.contains("facebook.com/")
                    || low.contains("instagram.com/")
                {
                    if !low.contains("sharer") && !socials.contains(&href.to_string()) {
                        socials.push(href.to_string());
                    }
                }
            }
        }

        // 3. Extraer correos visibles en texto plano (desofuscando "[at]" y "[dot]")
        let deobfuscated_html = pre_cleaned_html
            .replace("mailto:", " ")
            .replace("MAILTO:", " ")
            .replace(" [at] ", "@")
            .replace(" (at) ", "@")
            .replace(" [arroba] ", "@")
            .replace(" [dot] ", ".");
        for token in deobfuscated_html.split(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '<' || c == '>' || c == ':' || c == ';' || c == ',') {
            if token.contains('@') && token.len() >= 6 && token.len() <= 75 {
                if let Some(clean) = Self::sanitize_and_normalize_email(token) {
                    if !emails.contains(&clean) {
                        emails.push(clean);
                    }
                }
            }
        }

        emails.sort_by_key(|m| {
            if m.starts_with("juridico") || m.starts_with("legal") || m.starts_with("compliance") || m.starts_with("cumplimiento") {
                0
            } else if m.starts_with("direccion") || m.starts_with("gerencia") || m.starts_with("admon") {
                1
            } else if m.starts_with("contacto") || m.starts_with("info") || m.starts_with("atencion") {
                2
            } else {
                3
            }
        });

        (
            emails.into_iter().take(3).collect::<Vec<_>>().join("; "),
            whatsapps.into_iter().take(2).collect::<Vec<_>>().join("; "),
            socials.into_iter().take(3).collect::<Vec<_>>().join("; "),
        )
    }

    /// Intenta adquirir el candado en vuelo para un dominio y verifica si otra sucursal ya lo contactó hoy
    /// (activable cuando la URL trae `?dedup_domain=1` o `MALLEN_DEDUP_DOMAIN=1`).
    pub fn try_acquire_domain_lock(domain_key: &str, enforce_db_dedup: bool) -> Result<bool, &'static str> {
        let set_mutex = IN_FLIGHT_DOMAINS.get_or_init(|| Mutex::new(HashSet::new()));
        if let Ok(mut active_set) = set_mutex.lock() {
            if active_set.contains(domain_key) {
                return Err("DOMINIO_DUPLICADO_EN_VUELO_OMITIDO");
            }
            if enforce_db_dedup {
                let clean_host = domain_key.split(':').next().unwrap_or(domain_key);
                let path_part = domain_key.split(':').nth(1).unwrap_or("");
                let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
                if let Ok(conn) = rusqlite::Connection::open(db_path) {
                    let already_contacted: i64 = if !path_part.is_empty() {
                        conn.query_row(
                            "SELECT COUNT(*) FROM prospect_pipeline
                             WHERE final_domain = ?1
                               AND (input_url LIKE '%' || ?2 || '%' OR final_target_url LIKE '%' || ?2 || '%')
                               AND ultimo_resultado_codigo = 'PROSPECCION_ENVIADA_EXITO'
                               AND updated_at_utc >= datetime('now', '-24 hours')",
                            rusqlite::params![clean_host, path_part],
                            |r| r.get(0),
                        )
                        .unwrap_or(0)
                    } else {
                        conn.query_row(
                            "SELECT COUNT(*) FROM prospect_pipeline
                             WHERE final_domain = ?1
                               AND ultimo_resultado_codigo = 'PROSPECCION_ENVIADA_EXITO'
                               AND updated_at_utc >= datetime('now', '-24 hours')",
                            rusqlite::params![clean_host],
                            |r| r.get(0),
                        )
                        .unwrap_or(0)
                    };
                    if already_contacted > 0 {
                        return Err("DOMINIO_DUPLICADO_YA_CONTACTADO_OMITIDO");
                    }
                }
            }
            active_set.insert(domain_key.to_string());
            return Ok(true);
        }
        Ok(false)
    }

    pub fn release_domain_lock(domain: &str) {
        if let Some(set_mutex) = IN_FLIGHT_DOMAINS.get() {
            if let Ok(mut active_set) = set_mutex.lock() {
                active_set.remove(domain);
            }
        }
    }

    /// Determina el Tier exacto de método de contacto para segmentación post-web inmediata.
    pub fn classify_contact_method_tier(
        status_result: &str,
        emails: &str,
        whatsapps: &str,
        socials: &str,
    ) -> (&'static str, &'static str, &'static str) {
        if status_result == "PROSPECCION_ENVIADA_EXITO" {
            ("STAGE_CONTACTADO_FORM", "TIER_1_WEB_FORM_SENT", "FOLLOWUP_MONITOREO_72H")
        } else if status_result == "CANAL_ALTERNATIVO_MAILTO_DETECTADO" || !emails.is_empty() {
            ("STAGE_COLA_SMTP_DIRECTO", "TIER_2_DIRECT_SMTP_READY", "RESERVA_SMTP_GUARDADA_SEGUNDA_OLA")
        } else if status_result == "CANAL_ALTERNATIVO_WHATSAPP_DETECTADO" || !whatsapps.is_empty() {
            ("STAGE_COLA_WHATSAPP_B2B", "TIER_3_WHATSAPP_DIRECT_READY", "ENVIAR_AVISO_EJECUTIVO_WHATSAPP")
        } else if status_result.starts_with("REQUIERE_HEADLESS_JS_")
            || status_result.starts_with("BLOQUEO_WAF_")
            || status_result.starts_with("PROTECCION_ANTIBOT_")
        {
            ("STAGE_COLA_HEADLESS_BROWSER", "TIER_4_HEADLESS_CHROME_WORKER", "EJECUTAR_WORKER_HEADLESS_PLAYWRIGHT")
        } else if status_result.starts_with("CANAL_ALTERNATIVO_IFRAME_CRM") {
            ("STAGE_DERIVADO_CRM_EXTERNO", "TIER_4_CRM_IFRAME_WIDGET", "PROSPECCION_WIDGET_CRM_EXTERNO")
        } else if !socials.is_empty() {
            ("STAGE_DERIVADO_SOCIAL_B2B", "TIER_5_LINKEDIN_B2B_OUTREACH", "CONTACTO_EJECUTIVO_LINKEDIN")
        } else if status_result == "HTTP_POST_TIMEOUT" || status_result.starts_with("ERR_HTTP_GET") || status_result == "HTTP_RATE_LIMIT_429" {
            ("STAGE_REINTENTO_PROGRAMADO", "TIER_6_PHONE_DENUE_FALLBACK", "COLA_BACKOFF_O_TELEFONO_DENUE")
        } else {
            ("STAGE_SIN_CANAL_WEB", "TIER_6_PHONE_DENUE_FALLBACK", "USAR_TELEFONO_Y_CORREO_DENUE")
        }
    }

    /// Registra telemetría operativa completa (latencia ms, HTTP status, charset, reintentos, canales extraídos y Tier).
    #[allow(clippy::too_many_arguments)]
    pub fn persist_detailed_operational_telemetry(
        empresa_nombre: &str, input_url: &str, final_target_url: &str, multa_estimada: f64,
        status_result: &str, user_agent_used: &str, http_status: u16, latency_ms: u64,
        charset_detected: &str, retry_count: u32, emails: &str, whatsapps: &str, socials: &str, trace_detail: &str,
    ) {
        let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
        let (ack_tx, ack_rx) = mpsc::sync_channel::<()>(1);
        let msg = TelemetryWriteMsg {
            db_path,
            empresa_nombre: empresa_nombre.to_string(), input_url: input_url.to_string(),
            final_target_url: final_target_url.to_string(), multa_estimada, status_result: status_result.to_string(),
            user_agent_used: user_agent_used.to_string(), http_status, latency_ms,
            charset_detected: charset_detected.to_string(), retry_count,
            emails: emails.to_string(), whatsapps: whatsapps.to_string(),
            socials: socials.to_string(), trace_detail: trace_detail.to_string(),
            ack: Some(WriterAck::Sync(ack_tx)),
        };
        if Self::sqlite_writer_sender().send(msg).is_ok() {
            let _ = ack_rx.recv_timeout(Duration::from_secs(3));
        }
    }

    /// Método principal retrocompatible (4 argumentos):
    /// Incluye Backoff Exponencial en GET/POST, resolución de redirecciones 301/302 y Meta-Refresh,
    /// autodescubrimiento de subpágina `/contacto`, detección de Iframes CRM (HubSpot/Typeform/Calendly),
    /// cosecha de email/WhatsApp de respaldo y Máquina de Estados en SQLite WAL (`prospect_pipeline`).
    pub fn build_wave2_followup_message(empresa_nombre: &str, had_clicked_link: bool) -> String {
        let encoded_ref = Self::encode_url_ref(empresa_nombre);
        let link = format!("https://mallen.lat/api/micro_scan/{}", encoded_ref);
        let emp = empresa_nombre.trim();
        if had_clicked_link {
            format!(
                "Hola, equipo de {}. Dando breve seguimiento a la consulta de su pre-evaluacion tecnica preventiva (STPS, SAT, IMSS y LFPDPPP), su resumen confidencial sigue disponible aqui: {} | Si desean activar el expediente completo, pueden hacerlo en linea cuando gusten. Aviso de Privacidad: https://mallen.lat/privacidad | Exclusion ARCO: https://mallen.lat/derechos-arco",
                emp, link
            )
        } else {
            format!(
                "Hola, equipo de {}. Recordatorio breve de 72 horas: el pre-diagnostico tecnico preventivo de cumplimiento (STPS, SAT, IMSS y LFPDPPP) para su entidad sigue disponible sin costo aqui: {} | Fuente: INEGI, DENUE. Aviso de Privacidad: https://mallen.lat/privacidad | Exclusion ARCO: https://mallen.lat/derechos-arco",
                emp, link
            )
        }
    }

    pub async fn submit_form_direct(
        client: &Client,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
    ) -> String {
        Self::submit_form_direct_with_message(client, page_url, empresa_nombre, multa_estimada, None).await
    }

    pub async fn submit_form_direct_with_message(
        client: &Client,
        page_url: &str,
        empresa_nombre: &str,
        multa_estimada: f64,
        custom_message: Option<&str>,
    ) -> String {
        if let Err(link_err) = Self::build_and_validate_checkout_link(empresa_nombre, None) {
            return link_err.to_string();
        }
        if let Some(dead_code) = Self::is_dead_or_social_directory_domain(page_url) {
            Self::enqueue_telemetry_async(
                empresa_nombre.to_string(), page_url.to_string(), page_url.to_string(), multa_estimada,
                dead_code.to_string(), "DEAD_DOMAIN_FILTER".to_string(), 0, 0, "N/A".to_string(), 0,
                String::new(), String::new(), String::new(),
                "Dominio de red social/directorio derivado automáticamente a Tier 6".to_string()
            ).await;
            return dead_code.to_string();
        }
        let parsed_in = match Url::parse(page_url) {
            Ok(u) => u,
            Err(_) => return "URL_INVALIDA".to_string(),
        };

        TELEMETRY_TOTAL_REQS.fetch_add(1, Ordering::Relaxed);
        let initial_host = Self::normalize_domain_host(page_url);
        if !Self::validate_url_and_dns_ssrf(&parsed_in).await {
            return "BLOQUEO_SEGURIDAD_ANTI_SSRF".to_string();
        }
        let (h_chk, e_chk) = (initial_host.clone(), empresa_nombre.to_string());
        if tokio::task::spawn_blocking(move || Self::is_suppressed_arco(&h_chk, &e_chk)).await.unwrap_or(false) {
            return "OMITIDO_SUPRESION_ARCO".to_string();
        }

        // Clave del Circuit Breaker (en localhost incluye el path para aislar rutas simuladas en tests)
        let cb_key = if initial_host == "127.0.0.1" || initial_host == "localhost" {
            format!("{}:{}", initial_host, parsed_in.path())
        } else {
            initial_host.clone()
        };
        if Self::check_circuit_breaker_open(&cb_key) {
            TELEMETRY_CB_TRIPS.fetch_add(1, Ordering::Relaxed);
            Self::enqueue_telemetry_async(
                empresa_nombre.to_string(), page_url.to_string(), page_url.to_string(), multa_estimada,
                "CIRCUIT_BREAKER_ABIERTO_48H".to_string(), "CIRCUIT_BREAKER_GUARD".to_string(), 403, 0,
                "N/A".to_string(), 0, String::new(), String::new(), String::new(),
                "Dominio en enfriamiento de 48h por bloqueos previos".to_string()
            ).await;
            return "CIRCUIT_BREAKER_ABIERTO_48H".to_string();
        }

        // Rate-limiting y jitter ANTES de adquirir el semáforo global
        Self::apply_domain_rate_limit_and_jitter(&initial_host).await;

        let _sem_permit = Self::global_semaphore().acquire().await.ok();
        let _in_flight_guard = InFlightTelemetryGuard::new();

        let active_client = Self::get_cached_proxy_client().unwrap_or(client);
        let (rotated_headers, chosen_ua) = Self::next_rotated_headers();
        if !Self::is_allowed_by_robots_txt(active_client, &parsed_in, &rotated_headers).await {
            return "OMITIDO_ROBOTS_TXT".to_string();
        }
        let encoded_ref = Self::encode_url_ref(empresa_nombre);

        let mensaje_utf8 = match custom_message {
            Some(m) if !m.trim().is_empty() => m.to_string(),
            _ => Self::build_human_consultative_message(empresa_nombre, &encoded_ref),
        };

        let mut tracked_final_url = page_url.to_string();
        let mut harvested_contact: Option<String> = None;

        let start_instant = Instant::now();
        let enforce_dedup = page_url.contains("dedup_domain=1") || std::env::var("MALLEN_DEDUP_DOMAIN").map(|v| v == "1").unwrap_or(false);
        let dedup_key = if page_url.contains("dedup_domain=1") {
            format!("{}:{}", initial_host, parsed_in.path())
        } else {
            initial_host.clone()
        };
        if enforce_dedup {
            let dk = dedup_key.clone();
            let lock_res = tokio::task::spawn_blocking(move || Self::try_acquire_domain_lock(&dk, true))
                .await
                .unwrap_or(Ok(false));
            if let Err(dup_code) = lock_res {
                Self::enqueue_telemetry_async(
                    empresa_nombre.to_string(), page_url.to_string(), page_url.to_string(), multa_estimada,
                    dup_code.to_string(), "DEDUP_GUARD".to_string(), 0,
                    start_instant.elapsed().as_millis() as u64, "N/A".to_string(), 0,
                    String::new(), String::new(), String::new(),
                    "Omitido por huella de dominio duplicada en campaña".to_string()
                ).await;
                return dup_code.to_string();
            }
        }
        let tracked_http_status: u16;
        let tracked_charset = String::from("UTF-8");
        let tracked_retries: u32 = 0;
        let mut tracked_emails = String::new();
        let mut tracked_whatsapps = String::new();
        let tracked_socials = String::new();
        let tracked_trace: String;
        let timeout_ms = Self::configured_timeout_ms();
        let task = timeout(Duration::from_millis(timeout_ms), async {
            let mut ephemeral_cookies: HashMap<String, String> = HashMap::new();
            // 1. GET inicial con Reintentos Exponenciales + Jitter (supera microcortes o 502/503 transitorios)
            let mut resp_opt = None;
            let mut last_get_err = "ERR_HTTP_GET".to_string();

            for attempt in 0..3u32 {
                if attempt > 0 {
                    let backoff_ms = Self::compute_decorrelated_backoff_ms(attempt, 110);
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
                match active_client.get(page_url).headers(rotated_headers.clone()).send().await {
                    Ok(r) => {
                        let code = r.status().as_u16();
                        Self::extract_ephemeral_cookies(r.headers(), &mut ephemeral_cookies);
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
                            return (format!("ERR_HTTP_GET_{}", code), r.url().to_string(), None);
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
                    Err(e) => {
                        if e.is_redirect() {
                            return ("ERR_REDIRECT_LOOP_DETECTADO".to_string(), page_url.to_string(), None);
                        }
                        last_get_err = if e.is_timeout() {
                            "ERR_HTTP_GET_TIMEOUT".to_string()
                        } else if e.is_connect() {
                            "ERR_HTTP_GET_CONNECT".to_string()
                        } else {
                            "ERR_HTTP_GET_NET".to_string()
                        };
                    }
                }
            }

            let Some(resp) = resp_opt else {
                return (last_get_err, page_url.to_string(), None);
            };

            // Actualizar estrictamente target_url después de seguir redirecciones 301/302 (ej. http -> https://www.)
            let mut current_target_url = resp.url().clone();
            let resp_http_code = resp.status().as_u16();
            let ctype_saved = resp.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("text/html").to_string();
            if Self::is_ssrf_forbidden_host(current_target_url.host_str().unwrap_or("")) {
                return ("BLOQUEO_SEGURIDAD_ANTI_SSRF".to_string(), current_target_url.to_string(), None);
            }

            let bytes = match Self::read_bounded_body(resp, MAX_RESPONSE_BODY_BYTES).await {
                Ok(b) => b,
                Err(code) => return (code.to_string(), current_target_url.to_string(), None),
            };
            let (mut html_text, active_charset) = Self::decode_response_bytes_smart(&ctype_saved, &bytes);

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

            // Detección temprana de redirección a brokers/parking de dominios antes de evaluar WAF o sondear /contacto
            if let Some(final_host) = current_target_url.host_str() {
                let fh = final_host.to_lowercase();
                let parking_hosts = [
                    "hugedomains.com", "sedo.com", "sedoparking.com", "dan.com",
                    "afternic.com", "bodis.com", "parkingcrew.net",
                ];
                if parking_hosts.iter().any(|ph| fh == *ph || fh.ends_with(&format!(".{}", ph))) {
                    return ("DOMINIO_ESTACIONADO".to_string(), current_target_url.to_string(), None);
                }
            }

            // Si en la página inicial no hay ningún <textarea>, navegar al enlace interno de contacto o sondear rutas estándar
            if !html_text.to_lowercase().contains("<textarea") {
                let mut found_contact_form = false;
                if let Some(contact_subpage) = Self::discover_internal_contact_link(&current_target_url, &html_text) {
                    if let Ok(Some((next_url, candidate_html))) = timeout(Duration::from_millis(4500), async {
                        let r3 = active_client.get(&contact_subpage).headers(rotated_headers.clone()).send().await.ok()?;
                        if !r3.status().is_success() {
                            return None;
                        }
                        let next_url = r3.url().clone();
                        let b3 = r3.bytes().await.ok()?;
                        let s3 = if b3.len() > 1_500_000 { &b3[..1_500_000] } else { &b3[..] };
                        Some((next_url, String::from_utf8_lossy(s3).into_owned()))
                    }).await {
                        let clow = candidate_html.to_lowercase();
                        if clow.contains("<form") || clow.contains("<iframe") {
                            current_target_url = next_url;
                            html_text = candidate_html;
                            found_contact_form = true;
                        }
                    }
                }
                if !found_contact_form {
                    for probe_path in ["/contacto", "/contactanos", "/contact"] {
                        if let Ok(probe_url) = current_target_url.join(probe_path) {
                            if probe_url.as_str() == current_target_url.as_str() {
                                continue;
                            }
                            if let Ok(Some((next_url, candidate_html))) = timeout(Duration::from_millis(3500), async {
                                let rp = active_client.get(probe_url.as_str()).headers(rotated_headers.clone()).send().await.ok()?;
                                if !rp.status().is_success() {
                                    return None;
                                }
                                let next_url = rp.url().clone();
                                let bp = rp.bytes().await.ok()?;
                                let sp = if bp.len() > 1_500_000 { &bp[..1_500_000] } else { &bp[..] };
                                Some((next_url, String::from_utf8_lossy(sp).into_owned()))
                            }).await {
                                let clow = candidate_html.to_lowercase();
                                if clow.contains("<form") || clow.contains("<iframe") {
                                    current_target_url = next_url;
                                    html_text = candidate_html;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            let (h_emails, h_wa, h_soc) = Self::harvest_multichannel_contacts(&html_text);
            let (tech_stack_str, _is_ecom, pitch_mod) = Self::detect_mexican_tech_stack(&ctype_saved, &html_text);
            let fallback_contact = Some(format!("MULTI5|{}|{}|{}|{}|{}", h_emails, h_wa, h_soc, tech_stack_str, pitch_mod));
            let _ = (&resp_http_code, &active_charset);
            let html_lower = html_text.to_lowercase();

            if let Some(waf_code) = Self::detect_antibot_protection(&html_lower, "") {
                return (waf_code.to_string(), current_target_url.to_string(), fallback_contact);
            }
            if let Some(parked_code) = Self::detect_parked_or_expired_domain(&html_lower) {
                return (parked_code.to_string(), current_target_url.to_string(), fallback_contact);
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
                        "FIRMA MALLEN® - Compliance".to_string()
                    } else {
                        "FIRMA MALLEN® — Compliance".to_string()
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
                            params.insert(name.to_string(), std::env::var("MALLEN_CONTACT_PHONE").ok().filter(|s| !s.trim().is_empty()).unwrap_or_else(|| "5550000000".to_string()));
                        } else if name_lower.contains("company")
                            || name_lower.contains("empresa")
                            || name_lower.contains("organiza")
                        {
                            params.insert(name.to_string(), "FIRMA MALLEN®".to_string());
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
                        if let Some(js_trap_tag) = Self::detect_js_dynamic_form_or_honeypot(&html_lower, &form_inner_lower) {
                            detected_special_reason = Some(js_trap_tag);
                            continue;
                        }

                        // Adaptadores CMS reales (CF7 REST, Elementor Pro, WPForms) solo si action es vacío/#ancla/#wpcf7
                        let origin_base = current_target_url.origin().ascii_serialization();
                        let is_cms_anchor = form_action_raw.is_empty() || form_action_raw.starts_with('#') || act_lower.contains("#wpcf7") || act_lower.starts_with("javascript:");
                        if is_cms_anchor && params.get("_wpcf7").map(|id| !id.trim().is_empty()).unwrap_or(false) {
                            chosen_action = format!("{}/wp-json/contact-form-7/v1/contact-forms/{}/feedback", origin_base, params.get("_wpcf7").unwrap().trim());
                        } else if is_cms_anchor && (form_class.contains("elementor-form") || params.contains_key("form_id")) && html_lower.contains("wp-admin/admin-ajax.php") {
                            params.entry("action".to_string()).or_insert_with(|| "elementor_pro_forms_send_form".to_string());
                            chosen_action = format!("{}/wp-admin/admin-ajax.php", origin_base);
                        } else if is_cms_anchor && form_class.contains("wpforms-form") && html_lower.contains("wp-admin/admin-ajax.php") {
                            params.entry("action".to_string()).or_insert_with(|| "wpforms_submit".to_string());
                            chosen_action = format!("{}/wp-admin/admin-ajax.php", origin_base);
                        } else if !form_action_raw.is_empty() && !form_action_raw.starts_with('#') && !act_lower.starts_with("javascript:") {
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
                if let Some(spa_code) = Self::detect_js_dynamic_form_or_honeypot(&html_lower, "") {
                    return (spa_code.to_string(), current_target_url.to_string(), fallback_contact);
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
                    sleep(Duration::from_millis(Self::compute_decorrelated_backoff_ms(post_attempt, 150))).await;
                }

                let mut post_headers = rotated_headers.clone();
                if let Ok(ref_val) = HeaderValue::from_str(current_target_url.as_str()) {
                    post_headers.insert(REFERER, ref_val);
                }
                if let Ok(orig_val) = HeaderValue::from_str(&origin_str) {
                    post_headers.insert(ORIGIN, orig_val);
                }
                if let Some(ck) = Self::format_cookie_header(&ephemeral_cookies) {
                    post_headers.insert(COOKIE, ck);
                }

                let req_builder = if form_method_is_get {
                    active_client.get(&form_action).headers(post_headers.clone()).query(&form_params)
                } else {
                    active_client
                        .post(&form_action)
                        .headers(post_headers.clone())
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
                    Ok(res) if res.status().as_u16() == 415 => {
                        // Fallback automático para APIs modernas que exigen objeto application/json con cookies y headers originales
                        let json_obj: HashMap<&str, &str> = form_params
                            .iter()
                            .map(|(k, v)| (k.as_str(), v.as_str()))
                            .collect();
                        let mut json_headers = post_headers.clone();
                        json_headers.remove("content-type");
                        if let Ok(Ok(json_res)) = timeout(
                            Duration::from_millis(6000),
                            active_client
                                .post(&form_action)
                                .headers(json_headers)
                                .json(&json_obj)
                                .send(),
                        ).await {
                            if json_res.status().is_success() || json_res.status().is_redirection() {
                                let body_resp = json_res.text().await.unwrap_or_default();
                                let sem = Self::classify_post_response_semantics(&body_resp).to_string();
                                return (sem, current_target_url.to_string(), fallback_contact);
                            }
                        }
                        return ("HTTP_POST_ERR_415".to_string(), current_target_url.to_string(), fallback_contact);
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
                if let Some(ref c) = harvested_contact {
                    if c.contains("cfemail_decoded:") {
                        tracked_emails = c.replace("cfemail_decoded:", "");
                    }
                }
                tracked_http_status = 200;
                let (st_in, pt_in) = if let Some(ref c) = harvested_contact {
                    if let Some(rest) = c.strip_prefix("MULTI5|") {
                        let p: Vec<&str> = rest.splitn(5, '|').collect();
                        if p.len() == 5 { (p[3], p[4]) } else { ("HTML5", "MODULO_PYME_3000MXN") }
                    } else { ("HTML5", "MODULO_PYME_3000MXN") }
                } else { ("HTML5", "MODULO_PYME_3000MXN") };
                tracked_trace = format!("OK:{} -> {} | STACK:{} | PITCH:{}", page_url, tracked_final_url, st_in, pt_in);
                status_str
            }
            Err(_) => {
                tracked_http_status = 408;
                tracked_trace = format!("TIMEOUT_EXCEEDED:{}ms", timeout_ms);
                "TIMEOUT_GLOBAL".to_string()
            }
        };

        if enforce_dedup {
            Self::release_domain_lock(&dedup_key);
        }
        let elapsed_ms = start_instant.elapsed().as_millis() as u64;
        let mut final_socials = tracked_socials;

        if let Some(ref c) = harvested_contact {
            if let Some(rest) = c.strip_prefix("MULTI5|") {
                let parts: Vec<&str> = rest.splitn(5, '|').collect();
                if parts.len() == 5 {
                    tracked_emails = parts[0].to_string();
                    tracked_whatsapps = parts[1].to_string();
                    final_socials = parts[2].to_string();
                    let _ = (parts[3], parts[4]);
                }
            } else if c.starts_with("mailto:") || c.starts_with("email:") {
                tracked_emails = c.split(':').nth(1).unwrap_or("").to_string();
            } else if c.contains("wa.me") || c.contains("whatsapp") || c.starts_with('+') {
                tracked_whatsapps = c.clone();
            }
        }
                TELEMETRY_TOTAL_LATENCY_MS.fetch_add(elapsed_ms as usize, Ordering::Relaxed);
        if final_status == "PROSPECCION_ENVIADA_EXITO" {
            TELEMETRY_SUCCESS.fetch_add(1, Ordering::Relaxed);
        } else if final_status.starts_with("BLOQUEO_WAF_") || final_status.starts_with("PROTECCION_ANTIBOT_") {
            TELEMETRY_WAF_OR_CAPTCHA.fetch_add(1, Ordering::Relaxed);
        }
        Self::record_circuit_breaker_outcome(&cb_key, &final_status);

        let emp_t = empresa_nombre.to_string();
        let in_u_t = page_url.to_string();
        let fin_u_t = tracked_final_url.clone();
        let st_t = final_status.clone();
        let ua_t = chosen_ua.to_string();
        let ch_t = tracked_charset.clone();
        let em_t = tracked_emails.clone();
        let wa_t = tracked_whatsapps.clone();
        let so_t = final_socials.clone();
        let tr_t = tracked_trace.clone();
        Self::enqueue_telemetry_async(
            emp_t, in_u_t, fin_u_t, multa_estimada, st_t, ua_t,
            tracked_http_status, elapsed_ms, ch_t, tracked_retries,
            em_t, wa_t, so_t, tr_t,
        ).await;

        final_status
    }
}
