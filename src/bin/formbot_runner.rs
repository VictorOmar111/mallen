use std::collections::{BTreeMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use worker_inegi_engine::engine::http_form_bot::HttpFormBot;

struct LiveLockGuard {
    path: PathBuf,
}

impl LiveLockGuard {
    fn acquire(lock_path: &str) -> Result<Self, String> {
        let path = PathBuf::from(lock_path);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(old_pid) = content.trim().parse::<u32>() {
                    if std::path::Path::new(&format!("/proc/{}", old_pid)).exists() {
                        return Err(format!(
                            "Ya existe otra instancia --live ejecutándose (PID {}) en {}",
                            old_pid, lock_path
                        ));
                    }
                }
            }
            let _ = std::fs::remove_file(&path);
        }
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("No se pudo adquirir lock {}: {}", lock_path, e))?;
        let _ = writeln!(f, "{}", std::process::id());
        Ok(Self { path })
    }
}

impl Drop for LiveLockGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn load_dotenv_if_present() {
    if let Ok(content) = std::fs::read_to_string(".env") {
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let mut val = v.trim();
                if (val.starts_with('"') && val.ends_with('"') && val.len() >= 2)
                    || (val.starts_with('\'') && val.ends_with('\'') && val.len() >= 2)
                {
                    val = &val[1..val.len() - 1];
                }
                if !key.is_empty() && std::env::var(key).is_err() {
                    std::env::set_var(key, val);
                }
            }
        }
    }
}

fn normalize_domain_key(raw: &str) -> String {
    let lower = raw.trim().to_lowercase();
    let without_scheme = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))
        .unwrap_or(&lower);
    without_scheme
        .split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .trim_start_matches("www.")
        .trim()
        .to_string()
}

fn normalize_url_and_domain(raw_www: &str) -> Option<(String, String)> {
    let trimmed = raw_www.trim();
    if trimmed.is_empty()
        || trimmed.chars().any(|c| c.is_whitespace())
        || trimmed.contains('@')
    {
        return None;
    }
    let lower = trimmed.to_lowercase();
    for bad_prefix in ["mailto:", "tel:", "ftp:", "javascript:", "data:"] {
        if lower.starts_with(bad_prefix) {
            return None;
        }
    }
    if lower.contains("://") && !lower.starts_with("http://") && !lower.starts_with("https://") {
        return None;
    }
    let url = if lower.starts_with("http://") || lower.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed.trim_start_matches('/'))
    };
    let host = normalize_domain_key(&url);
    if host.is_empty()
        || !host.contains('.')
        || host == "localhost"
        || host == "127.0.0.1"
        || host.starts_with('.')
        || host.ends_with('.')
        || !host.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
    {
        return None;
    }
    Some((url, host))
}

fn is_excluded_entity(empresa: &str, domain: &str) -> bool {
    let e = empresa.to_uppercase();
    let d = domain.to_lowercase();
    let blocked_domains = [
        "facebook.com", "instagram.com", "twitter.com", "x.com", "linkedin.com",
        "tiktok.com", "youtube.com", "whatsapp.com", "wa.me", "gob.mx", "edu.mx",
        "blogspot.com", "wixsite.com", "google.com", "mercadolibre.com", "amazon.com",
        "seccionamarilla.com", "paginasamarillas", "yelp.com",
    ];
    if blocked_domains.iter().any(|b| d == *b || d.ends_with(&format!(".{}", b)) || d.contains(b)) {
        return true;
    }
    let blocked_names = [
        "GOBIERNO", "MUNICIPIO", "AYUNTAMIENTO", "SECRETARIA DE ", "IMSS", "ISSSTE",
        "PEMEX", "CFE ", "BANCO ", "BBVA", "BANAMEX", "SANTANDER", "HSBC", "SCOTIABANK",
        "BANORTE", "INBURSA", "UNIVERSIDAD ", "ESCUELA ", "COLEGIO ", "IGLESIA ",
    ];
    blocked_names.iter().any(|b| e.contains(b))
}

fn estimate_fine_from_estrato(estrato: &str) -> f64 {
    if estrato.contains("251") {
        1_850_000.0
    } else if estrato.contains("101") {
        1_250_000.0
    } else if estrato.contains("51") {
        890_000.0
    } else if estrato.contains("31") {
        640_000.0
    } else if estrato.contains("11") {
        420_000.0
    } else {
        280_000.0
    }
}

fn jittered_delay_ms(base_ms: u64) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    let span = (base_ms / 4).max(1);
    let offset = nanos % (span * 2 + 1);
    base_ms.saturating_sub(span).saturating_add(offset).max(200)
}

fn is_terminal_wave2_result(res: &str) -> bool {
    matches!(
        res,
        "PROSPECCION_ENVIADA_EXITO"
            | "OMITIDO_SUPRESION_ARCO"
            | "OMITIDO_ROBOTS_TXT"
            | "BLOQUEO_SEGURIDAD_ANTI_SSRF"
            | "SIN_FORMULARIO_CONTACTO"
            | "CANAL_ALTERNATIVO_MAILTO_DETECTADO"
            | "CANAL_ALTERNATIVO_IFRAME_CRM"
            | "URL_INVALIDA"
    ) || res.starts_with("DEAD_DOMAIN")
}

fn table_exists(conn: &rusqlite::Connection, table_name: &str) -> bool {
    conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
        rusqlite::params![table_name],
        |r| r.get::<_, i64>(0),
    )
    .unwrap_or(0)
        > 0
}

fn ensure_wave2_schema(conn: &rusqlite::Connection) {
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA busy_timeout=5000;
         CREATE TABLE IF NOT EXISTS formbot_wave2_log (
             empresa_nombre TEXT PRIMARY KEY,
             final_domain TEXT NOT NULL DEFAULT '',
             sent_at_utc TEXT NOT NULL DEFAULT (datetime('now')),
             result_code TEXT NOT NULL,
             had_clicked_link INTEGER NOT NULL DEFAULT 0
         );",
    )
    .expect("ERROR inicializando formbot_wave2_log en SQLite");
}

fn register_exclusion_value(
    raw: &str,
    excluded_names: &mut HashSet<String>,
    excluded_domains: &mut HashSet<String>,
) {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return;
    }
    excluded_names.insert(trimmed.to_uppercase());
    let dom = normalize_domain_key(trimmed);
    if dom.contains('.') && !dom.contains('@') {
        excluded_domains.insert(dom);
    }
}

fn load_exclusion_sets(
    conn: &rusqlite::Connection,
) -> (HashSet<String>, HashSet<String>) {
    let mut excluded_names: HashSet<String> = HashSet::new();
    let mut excluded_domains: HashSet<String> = HashSet::new();

    if table_exists(conn, "suppression_list") {
        let mut stmt = conn
            .prepare("SELECT domain_or_company FROM suppression_list")
            .expect("ERROR preparando consulta suppression_list");
        let rows = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .expect("ERROR leyendo suppression_list");
        for raw in rows.flatten() {
            register_exclusion_value(&raw, &mut excluded_names, &mut excluded_domains);
        }
    }

    if table_exists(conn, "audit_sessions") {
        let mut stmt = conn
            .prepare(
                "SELECT tax_id,
                        COALESCE(json_extract(master_json, '$.company_name'), ''),
                        COALESCE(json_extract(master_json, '$.domain'), '')
                 FROM audit_sessions
                 WHERE is_accredited = 1",
            )
            .expect("ERROR preparando consulta audit_sessions");
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .expect("ERROR leyendo audit_sessions");
        for (tax_id, company_name, domain) in rows.flatten() {
            register_exclusion_value(&tax_id, &mut excluded_names, &mut excluded_domains);
            register_exclusion_value(&company_name, &mut excluded_names, &mut excluded_domains);
            register_exclusion_value(&domain, &mut excluded_names, &mut excluded_domains);
        }
    }

    (excluded_names, excluded_domains)
}

fn main() {
    // Cargar .env en hilo único antes de arrancar el runtime de Tokio
    load_dotenv_if_present();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("ERROR iniciando runtime Tokio");
    rt.block_on(async_main());
}

async fn async_main() {
    let args: Vec<String> = std::env::args().collect();
    let mut wave = String::from("1");
    let mut limit: usize = 25;
    let mut delay_ms: u64 = 1500;
    let mut live_mode = false;
    let mut db_override: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--wave" => {
                let Some(val) = args.get(i + 1) else {
                    eprintln!("ERROR: falta valor para --wave (usa 1 o 2-72h)");
                    std::process::exit(1);
                };
                wave = val.clone();
                i += 2;
            }
            "--limit" => {
                let Some(val) = args.get(i + 1) else {
                    eprintln!("ERROR: falta valor para --limit");
                    std::process::exit(1);
                };
                limit = val.parse::<usize>().unwrap_or_else(|_| {
                    eprintln!("ERROR: --limit inválido '{}'", val);
                    std::process::exit(1);
                });
                if limit == 0 || limit > 2000 {
                    eprintln!("ERROR: --limit debe estar entre 1 y 2000");
                    std::process::exit(1);
                }
                i += 2;
            }
            "--delay-ms" => {
                let Some(val) = args.get(i + 1) else {
                    eprintln!("ERROR: falta valor para --delay-ms");
                    std::process::exit(1);
                };
                delay_ms = val.parse::<u64>().unwrap_or_else(|_| {
                    eprintln!("ERROR: --delay-ms inválido '{}'", val);
                    std::process::exit(1);
                });
                if delay_ms < 200 || delay_ms > 30_000 {
                    eprintln!("ERROR: --delay-ms debe estar entre 200 y 30000");
                    std::process::exit(1);
                }
                i += 2;
            }
            "--db" => {
                let Some(val) = args.get(i + 1) else {
                    eprintln!("ERROR: falta valor para --db");
                    std::process::exit(1);
                };
                db_override = Some(val.clone());
                i += 2;
            }
            "--live" => {
                live_mode = true;
                i += 1;
            }
            "--dry-run" => {
                live_mode = false;
                i += 1;
            }
            "-h" | "--help" => {
                println!("Uso: formbot_runner --wave <1|2-72h> --limit <1..2000> [--delay-ms <200..30000>] [--db <ruta>] [--dry-run|--live]");
                return;
            }
            other => {
                eprintln!("ERROR: argumento desconocido '{}'", other);
                std::process::exit(1);
            }
        }
    }

    let _lock = if live_mode {
        match LiveLockGuard::acquire("/tmp/mallen_formbot.lock") {
            Ok(g) => Some(g),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let db_path = db_override.unwrap_or_else(|| {
        std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string())
    });

    let conn = if live_mode {
        let c = rusqlite::Connection::open(&db_path)
            .unwrap_or_else(|e| panic!("ERROR abriendo {} (RW): {}", db_path, e));
        ensure_wave2_schema(&c);
        c
    } else {
        let c = rusqlite::Connection::open_with_flags(
            &db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .unwrap_or_else(|e| panic!("ERROR abriendo {} (RO): {}", db_path, e));
        let _ = c.execute_batch("PRAGMA busy_timeout=5000;");
        c
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .connect_timeout(Duration::from_secs(8))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .expect("ERROR creando reqwest::Client");

    let t0 = Instant::now();
    println!(
        "=== FORMBOT RUNNER v160 | wave={} | mode={} | limit={} | delay_ms={}±25% | db={} ===",
        wave,
        if live_mode { "LIVE" } else { "DRY-RUN (RO)" },
        limit,
        delay_ms,
        db_path
    );

    if wave == "1" {
        run_wave1(&conn, &client, limit, delay_ms, live_mode).await;
    } else if wave == "2-72h" || wave == "2" {
        run_wave2_72h(&conn, &client, limit, delay_ms, live_mode).await;
    } else {
        eprintln!("ERROR: --wave inválido '{}'. Usa --wave 1 o --wave 2-72h", wave);
        std::process::exit(1);
    }

    if live_mode {
        let _ = conn.execute_batch("PRAGMA wal_checkpoint(PASSIVE);");
    }
    println!("✓ Corrida completada en {:.1}s", t0.elapsed().as_secs_f64());
}

async fn run_wave1(
    conn: &rusqlite::Connection,
    client: &reqwest::Client,
    limit: usize,
    delay_ms: u64,
    live_mode: bool,
) {
    let (mut existing_names, mut existing_domains) = load_exclusion_sets(conn);

    let mut pipe_stmt = conn
        .prepare("SELECT UPPER(TRIM(empresa_nombre)), LOWER(TRIM(final_domain)) FROM prospect_pipeline")
        .expect("ERROR preparando consulta prospect_pipeline");
    let pipe_rows = pipe_stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .expect("ERROR leyendo prospect_pipeline");
    for (n, d) in pipe_rows.flatten() {
        existing_names.insert(n);
        let dom = normalize_domain_key(&d);
        if !dom.is_empty() {
            existing_domains.insert(dom);
        }
    }

    let fetch_cap = (limit * 15).clamp(250, 12_000) as i64;
    let sql = "
        SELECT COALESCE(NULLIF(TRIM(raz_social), ''), TRIM(nom_estab)) AS empresa,
               TRIM(www) AS web_raw,
               COALESCE(NULLIF(TRIM(codigo_scian), ''), '541110') AS scian,
               COALESCE(per_ocu, '11 a 30 personas') AS estrato
        FROM denue_units
        WHERE www IS NOT NULL
          AND TRIM(www) != ''
          AND (TRIM(raz_social) != '' OR TRIM(nom_estab) != '')
        ORDER BY
          CASE
            WHEN per_ocu LIKE '%31 a 50%' OR per_ocu LIKE '%51 a 100%' OR per_ocu LIKE '%101 a 250%' THEN 1
            WHEN per_ocu LIKE '%11 a 30%' THEN 2
            WHEN per_ocu LIKE '%251%' THEN 3
            ELSE 4
          END,
          rowid ASC
        LIMIT ?1
    ";

    let mut candidates: Vec<(String, String, String, String, f64)> = Vec::new();
    let mut batch_domains: HashSet<String> = HashSet::new();

    let mut stmt = conn.prepare(sql).expect("ERROR preparando consulta denue_units");
    let rows = stmt
        .query_map(rusqlite::params![fetch_cap], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })
        .expect("ERROR leyendo denue_units");

    for (empresa, raw_www, scian, estrato) in rows.flatten() {
        if candidates.len() >= limit {
            break;
        }
        let emp_clean = empresa.trim().to_string();
        let emp_up = emp_clean.to_uppercase();
        if emp_clean.len() < 3 || existing_names.contains(&emp_up) {
            continue;
        }
        let Some((url, domain)) = normalize_url_and_domain(&raw_www) else {
            continue;
        };
        if existing_domains.contains(&domain) || batch_domains.contains(&domain) {
            continue;
        }
        if is_excluded_entity(&emp_clean, &domain) {
            continue;
        }
        batch_domains.insert(domain.clone());
        let multa = estimate_fine_from_estrato(&estrato);
        candidates.push((emp_clean, url, domain, scian, multa));
    }

    println!(
        "Candidatos frescos seleccionados (Ola 1): {} (excluidos previos/ARCO/pagados: {} nombres, {} dominios)",
        candidates.len(),
        existing_names.len(),
        existing_domains.len()
    );
    let mut stats: BTreeMap<String, usize> = BTreeMap::new();

    for (idx, (empresa, url, domain, scian, multa)) in candidates.iter().enumerate() {
        if !live_mode {
            println!(
                "  [DRY {:02}/{}] {} | {} | SCIAN {} | multa_est=${:.0}",
                idx + 1,
                candidates.len(),
                empresa,
                url,
                scian,
                multa
            );
            continue;
        }
        let res = HttpFormBot::submit_form_direct(client, url, empresa, *multa).await;
        println!(
            "  [LIVE {:02}/{}] {} ({}) -> {}",
            idx + 1,
            candidates.len(),
            empresa,
            domain,
            res
        );
        *stats.entry(res).or_insert(0) += 1;
        if idx + 1 < candidates.len() {
            tokio::time::sleep(Duration::from_millis(jittered_delay_ms(delay_ms))).await;
        }
    }

    if live_mode {
        println!("--- RESUMEN OLA 1 ---");
        for (code, count) in stats {
            println!("  {}: {}", code, count);
        }
    } else {
        let total_multa: f64 = candidates.iter().map(|c| c.4).sum();
        println!(
            "--- RESUMEN DRY-RUN OLA 1: {} candidatos | exposición estimada acumulada: ${:.0} MXN ---",
            candidates.len(),
            total_multa
        );
    }
}

async fn run_wave2_72h(
    conn: &rusqlite::Connection,
    client: &reqwest::Client,
    limit: usize,
    delay_ms: u64,
    live_mode: bool,
) {
    let (excluded_names, excluded_domains) = load_exclusion_sets(conn);
    let has_w2_table = table_exists(conn, "formbot_wave2_log");

    let sql = if has_w2_table {
        "SELECT p.empresa_nombre,
                p.final_target_url,
                p.final_domain,
                p.multa_estimada_mxn,
                COALESCE(p.has_clicked_link, 0) AS clicked
         FROM prospect_pipeline p
         LEFT JOIN formbot_wave2_log w2 ON UPPER(TRIM(w2.empresa_nombre)) = UPPER(TRIM(p.empresa_nombre))
         WHERE p.ultimo_resultado_codigo = 'PROSPECCION_ENVIADA_EXITO'
           AND p.updated_at_utc <= datetime('now', '-72 hours')
           AND LOWER(p.final_target_url) NOT LIKE '%127.0.0.1%'
           AND LOWER(p.final_target_url) NOT LIKE '%localhost%'
           AND w2.empresa_nombre IS NULL
         ORDER BY p.has_clicked_link DESC, p.multa_estimada_mxn DESC
         LIMIT ?1"
    } else {
        "SELECT p.empresa_nombre,
                p.final_target_url,
                p.final_domain,
                p.multa_estimada_mxn,
                COALESCE(p.has_clicked_link, 0) AS clicked
         FROM prospect_pipeline p
         WHERE p.ultimo_resultado_codigo = 'PROSPECCION_ENVIADA_EXITO'
           AND p.updated_at_utc <= datetime('now', '-72 hours')
           AND LOWER(p.final_target_url) NOT LIKE '%127.0.0.1%'
           AND LOWER(p.final_target_url) NOT LIKE '%localhost%'
         ORDER BY p.has_clicked_link DESC, p.multa_estimada_mxn DESC
         LIMIT ?1"
    };

    let fetch_cap = (limit * 6).max(200) as i64;
    let mut targets: Vec<(String, String, String, f64, bool)> = Vec::new();
    let mut stmt = conn.prepare(sql).expect("ERROR preparando consulta Ola 2");
    let rows = stmt
        .query_map(rusqlite::params![fetch_cap], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, f64>(3).unwrap_or(420_000.0),
                r.get::<_, i64>(4)? > 0,
            ))
        })
        .expect("ERROR leyendo candidatos Ola 2");

    for (empresa, raw_url, raw_domain, raw_multa, clicked) in rows.flatten() {
        if targets.len() >= limit {
            break;
        }
        let emp_clean = empresa.trim().to_string();
        let emp_up = emp_clean.to_uppercase();
        let Some((valid_url, valid_domain)) = normalize_url_and_domain(&raw_url) else {
            continue;
        };
        let stored_dom = normalize_domain_key(&raw_domain);
        if excluded_names.contains(&emp_up)
            || excluded_domains.contains(&valid_domain)
            || (!stored_dom.is_empty() && excluded_domains.contains(&stored_dom))
        {
            continue;
        }
        let safe_multa = if raw_multa.is_finite() && raw_multa >= 0.0 {
            raw_multa
        } else {
            420_000.0
        };
        targets.push((emp_clean, valid_url, valid_domain, safe_multa, clicked));
    }

    println!("Candidatos elegibles para Ola 2 (>=72h sin pago ni baja ARCO): {}", targets.len());
    let mut stats: BTreeMap<String, usize> = BTreeMap::new();

    for (idx, (empresa, url, domain, multa, clicked)) in targets.iter().enumerate() {
        let msg = HttpFormBot::build_wave2_followup_message(empresa, *clicked);
        if !live_mode {
            println!(
                "  [DRY-72H {:02}/{}] {} ({}) | clicked={} | multa_est=${:.0} | msg_len={}",
                idx + 1,
                targets.len(),
                empresa,
                domain,
                clicked,
                multa,
                msg.len()
            );
            continue;
        }
        let res = HttpFormBot::submit_form_direct_with_message(client, url, empresa, *multa, Some(&msg)).await;
        println!(
            "  [LIVE-72H {:02}/{}] {} ({}) | clicked={} -> {}",
            idx + 1,
            targets.len(),
            empresa,
            domain,
            clicked,
            res
        );
        if is_terminal_wave2_result(&res) {
            if let Err(e) = conn.execute(
                "INSERT OR IGNORE INTO formbot_wave2_log (empresa_nombre, final_domain, sent_at_utc, result_code, had_clicked_link)
                 VALUES (?1, ?2, datetime('now'), ?3, ?4)",
                rusqlite::params![empresa, domain, res, if *clicked { 1 } else { 0 }],
            ) {
                eprintln!("WARN guardando formbot_wave2_log para {}: {}", empresa, e);
            }
        } else {
            println!("    -> Resultado transitorio ({}); queda elegible para reintento.", res);
        }
        *stats.entry(res).or_insert(0) += 1;
        if idx + 1 < targets.len() {
            tokio::time::sleep(Duration::from_millis(jittered_delay_ms(delay_ms))).await;
        }
    }

    if live_mode {
        println!("--- RESUMEN OLA 2 (72h) ---");
        for (code, count) in stats {
            println!("  {}: {}", code, count);
        }
    } else {
        let clicked_cnt = targets.iter().filter(|t| t.4).count();
        let total_multa: f64 = targets.iter().map(|t| t.3).sum();
        println!(
            "--- RESUMEN DRY-RUN OLA 2: {} candidatos (clicked={}, sin_clic={}) | exposición: ${:.0} MXN ---",
            targets.len(),
            clicked_cnt,
            targets.len().saturating_sub(clicked_cnt),
            total_multa
        );
    }
}
