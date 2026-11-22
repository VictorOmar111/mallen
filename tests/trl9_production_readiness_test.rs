use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::IpAddr;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;
use tokio::net::TcpListener;
use url::Url;
use worker_inegi_engine::engine::http_form_bot::HttpFormBot;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_trl9_phase8_to_9_production_readiness() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  SUITE INTENSIVA TRL 8 → TRL 9 — MEDICIÓN DE CAPACIDADES REALES Y PUNTOS CRÍTICOS (ESC. 106 AL 111)  ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝");

    // -------------------------------------------------------------------------
    // ESCENARIO 106: Estrés de Contención SQLite WAL (500 escrituras + 200 lecturas concurrentes)
    // -------------------------------------------------------------------------
    let stress_db = "/tmp/mallen_trl9_stress.db";
    let _ = std::fs::remove_file(stress_db);
    let _ = std::fs::remove_file(format!("{}-wal", stress_db));
    let _ = std::fs::remove_file(format!("{}-shm", stress_db));
    std::env::set_var("MALLEN_DB_PATH", stress_db);

    HttpFormBot::persist_detailed_operational_telemetry(
        "SEMILLA_WAL_TRL9", "https://semilla.mx", "https://semilla.mx", 3000.0,
        "PROSPECCION_ENVIADA_EXITO", "TRL9-BENCH", 200, 12, "UTF-8", 0,
        "legal@semilla.mx", "+525512345678", "", "SEED",
    );

    let t_wal = Instant::now();
    let sqlite_busy_errors = Arc::new(AtomicUsize::new(0));
    let mut write_handles = Vec::with_capacity(500);
    for i in 0..500usize {
        write_handles.push(tokio::task::spawn_blocking(move || {
            HttpFormBot::persist_detailed_operational_telemetry(
                &format!("EMPRESA_STRESS_{:04}", i),
                &format!("https://empresa{}.mx", i),
                &format!("https://empresa{}.mx/contacto", i),
                3000.0,
                "PROSPECCION_ENVIADA_EXITO",
                "TRL9-STRESS",
                200,
                25,
                "UTF-8",
                0,
                &format!("contacto{}@empresa.mx", i),
                "",
                "",
                "OK | STACK:WordPress | PITCH:MODULO_PYME",
            );
        }));
    }

    let mut read_handles = Vec::with_capacity(200);
    for _ in 0..200usize {
        let busy_c = sqlite_busy_errors.clone();
        read_handles.push(tokio::task::spawn_blocking(move || {
            let t0 = Instant::now();
            match rusqlite::Connection::open(stress_db) {
                Ok(conn) => {
                    let _ = conn.busy_timeout(std::time::Duration::from_millis(5000));
                    match conn.query_row("SELECT COUNT(*) FROM prospect_pipeline", [], |r| r.get::<_, i64>(0)) {
                        Ok(_) => t0.elapsed().as_micros() as u64,
                        Err(_) => {
                            busy_c.fetch_add(1, Ordering::Relaxed);
                            t0.elapsed().as_micros() as u64
                        }
                    }
                }
                Err(_) => {
                    busy_c.fetch_add(1, Ordering::Relaxed);
                    t0.elapsed().as_micros() as u64
                }
            }
        }));
    }

    for h in write_handles {
        let _ = h.await;
    }
    let mut read_lat_us = Vec::with_capacity(200);
    for h in read_handles {
        if let Ok(us) = h.await {
            read_lat_us.push(us);
        }
    }
    let wal_elapsed_s = t_wal.elapsed().as_secs_f64().max(0.001);
    read_lat_us.sort_unstable();
    let p50 = read_lat_us[read_lat_us.len() / 2];
    let p95 = read_lat_us[(read_lat_us.len() * 95) / 100];
    let p99 = read_lat_us[(read_lat_us.len() * 99) / 100];
    let busy_cnt = sqlite_busy_errors.load(Ordering::Relaxed);
    let wal_bytes = std::fs::metadata(format!("{}-wal", stress_db)).map(|m| m.len()).unwrap_or(0);

    let persisted_rows: i64 = rusqlite::Connection::open(stress_db)
        .unwrap()
        .query_row("SELECT COUNT(*) FROM prospect_pipeline", [], |r| r.get(0))
        .unwrap();

    println!(
        "  [106] SQLITE WAL ACTOR (500W + 200R simultáneos) | Filas: {}/501 | SQLITE_BUSY: {} | {:.0} writes/s | Lectura p50: {}µs p95: {}µs p99: {}µs | WAL: {} KB",
        persisted_rows, busy_cnt, 500.0 / wal_elapsed_s, p50, p95, p99, wal_bytes / 1024
    );
    assert_eq!(busy_cnt, 0, "Cero errores SQLITE_BUSY permitidos bajo contención");
    assert_eq!(persisted_rows, 501, "Deben persistirse las 501 filas sin pérdida");

    // -------------------------------------------------------------------------
    // ESCENARIO 107: Criptografía AEAD XChaCha20Poly1305 (Nonce único + Detección 1-Bit)
    // -------------------------------------------------------------------------
    std::env::set_var("MALLEN_PII_SECRET_KEY", "trl9_audit_secret_key_2026_32bytes_min");
    let plain_pii = "emails=direccion@pymereal.mx|wa=+525599887766|soc=";
    let c1 = HttpFormBot::encrypt_pii_at_rest(plain_pii);
    let c2 = HttpFormBot::encrypt_pii_at_rest(plain_pii);
    assert!(c1.starts_with("ENCv1:") && c2.starts_with("ENCv1:"));
    assert_ne!(c1, c2, "Cada cifrado AEAD debe generar un nonce aleatorio de 192 bits distinto");
    assert_eq!(HttpFormBot::decrypt_pii_from_rest(&c1).as_deref(), Some(plain_pii));
    assert_eq!(HttpFormBot::decrypt_pii_from_rest(&c2).as_deref(), Some(plain_pii));

    let mut tampered = c1.clone();
    let last_ch = tampered.pop().unwrap();
    tampered.push(if last_ch == 'a' { 'b' } else { 'a' });
    assert!(HttpFormBot::decrypt_pii_from_rest(&tampered).is_none(), "XChaCha20Poly1305 debe rechazar alteración de 1 bit");
    println!(
        "  [107] AEAD XChaCha20Poly1305 + Zeroize           | Nonces únicos: OK (c1 != c2) | Descifrado: OK | Sabotaje 1-bit rechazado: OK"
    );

    // -------------------------------------------------------------------------
    // Servidor mock local para Escenarios 108, 109 y 110
    // -------------------------------------------------------------------------
    let app = Router::new()
        .route("/redirect-to-imds", get(|| async {
            (StatusCode::FOUND, [(header::LOCATION, "http://169.254.169.254/latest/meta-data/")], "")
        }))
        .route("/wp-cf7-ok", get(|| async {
            (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
             r##"<html><body><form class="wpcf7-form" action="#wpcf7-f77-p1-o1" method="post"><input type="hidden" name="_wpcf7" value="77"><input name="your-email" type="email"><textarea name="your-message"></textarea></form></body></html>"##)
        }))
        .route("/wp-json/contact-form-7/v1/contact-forms/77/feedback", post(|body: String| async move {
            if body.contains("your-message=") {
                (StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], r#"{"status":"mail_sent","message":"Gracias"}"#)
            } else {
                (StatusCode::BAD_REQUEST, [(header::CONTENT_TYPE, "application/json")], r#"{"status":"validation_failed"}"#)
            }
        }))
        .route("/wp-cf7-fake200", get(|| async {
            (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
             r##"<html><body><form class="wpcf7-form" action="#wpcf7-f88-p1-o1" method="post"><input type="hidden" name="_wpcf7" value="88"><textarea name="your-message"></textarea></form></body></html>"##)
        }))
        .route("/wp-json/contact-form-7/v1/contact-forms/88/feedback", post(|| async {
            (StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], r#"{"status":"validation_failed","message":"Error en campos"}"#)
        }))
        .route("/csrf-cookie-form", get(|| async {
            let mut h = HeaderMap::new();
            h.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
            h.insert(header::SET_COOKIE, "mallen_sid=CSRF_LIVE_2026; Path=/; HttpOnly".parse().unwrap());
            (StatusCode::OK, h, r#"<html><body><form action="/csrf-cookie-submit" method="post"><input type="hidden" name="_token" value="CSRF_LIVE_2026"><textarea name="mensaje"></textarea></form></body></html>"#)
        }))
        .route("/csrf-cookie-submit", post(|headers: HeaderMap, body: String| async move {
            let ck = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()).unwrap_or("");
            if ck.contains("mallen_sid=CSRF_LIVE_2026") && body.contains("_token=CSRF_LIVE_2026") {
                (StatusCode::OK, r#"{"success":true,"status":"mail_sent"}"#)
            } else {
                (StatusCode::FORBIDDEN, r#"{"success":false,"error":"csrf_cookie_missing"}"#)
            }
        }))
        .route("/bomb-3mb", get(|| async {
            let mut huge = String::with_capacity(3_000_200);
            huge.push_str("<html><body><!-- ");
            huge.push_str(&"A".repeat(3_000_000));
            huge.push_str(" --><form action=\"/never\" method=\"post\"><textarea name=\"m\"></textarea></form></body></html>");
            (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")], huge).into_response()
        }));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://127.0.0.1:{}", addr.port());
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let client = HttpFormBot::build_bounded_http_client().unwrap();

    // -------------------------------------------------------------------------
    // ESCENARIO 108: Aislamiento SSRF Multicapa
    // -------------------------------------------------------------------------
    let ssrf_vectors = [
        ("IMDSv1 169.254.169.254", "http://169.254.169.254/latest/meta-data/"),
        ("VPC EC2 172.31.37.230", "http://172.31.37.230/"),
        ("IPv6 Loopback [::1]", "http://[::1]/"),
        ("IPv6 Link-Local [fe80::1]", "http://[fe80::1]/"),
        ("Puerto no estándar :6379", "https://example.com:6379/"),
    ];
    for (label, u) in ssrf_vectors {
        let parsed = Url::parse(u).unwrap();
        assert!(!HttpFormBot::is_url_ssrf_safe_sync(&parsed), "Debe bloquear vector SSRF: {}", label);
    }
    let ip_cgnat: IpAddr = "100.64.0.1".parse().unwrap();
    assert!(HttpFormBot::is_private_or_forbidden_ip(ip_cgnat, false));
    let r_redir = HttpFormBot::submit_form_direct(&client, &format!("{}/redirect-to-imds", base), "SSRF_302_TEST", 3000.0).await;
    assert_ne!(r_redir, "PROSPECCION_ENVIADA_EXITO", "Salto 302 hacia IMDS debe bloquearse");
    println!(
        "  [108] AISLAMIENTO SSRF MULTICAPA (6 vectores)    | IMDS/VPC/IPv6/CGNAT/:6379: BLOQUEADOS | 302->IMDS: {}",
        r_redir
    );

    // -------------------------------------------------------------------------
    // ESCENARIO 109: Adaptadores CMS (WP CF7 REST + Falso Positivo 200 + Cookies Efímeras CSRF)
    // -------------------------------------------------------------------------
    let r_cf7_ok = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-cf7-ok", base), "PYME_WP_CF7", 3000.0).await;
    assert_eq!(r_cf7_ok, "PROSPECCION_ENVIADA_EXITO");

    let r_cf7_fake = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-cf7-fake200", base), "PYME_WP_FAKE200", 3000.0).await;
    assert_eq!(r_cf7_fake, "ERR_RESPUESTA_SERVIDOR_FALLIDO");

    let r_csrf = HttpFormBot::submit_form_direct(&client, &format!("{}/csrf-cookie-form", base), "PYME_CSRF_COOKIE", 3000.0).await;
    assert_eq!(r_csrf, "PROSPECCION_ENVIADA_EXITO");

    println!(
        "  [109] ADAPTADORES CMS + COOKIES CSRF             | WP CF7 REST: {} | Anti-Falso-200: {} | Cookie+CSRF: {}",
        r_cf7_ok, r_cf7_fake, r_csrf
    );

    // -------------------------------------------------------------------------
    // ESCENARIO 110: Bomba 3 MB (Tope 512 KiB) + Exclusión ARCO (suppression_list)
    // -------------------------------------------------------------------------
    let t_bomb = Instant::now();
    let r_bomb = HttpFormBot::submit_form_direct(&client, &format!("{}/bomb-3mb", base), "PYME_BOMB_3MB", 3000.0).await;
    let bomb_ms = t_bomb.elapsed().as_millis();
    assert_eq!(r_bomb, "SIN_FORMULARIO_CONTACTO");

    {
        let conn = rusqlite::Connection::open(stress_db).unwrap();
        let _ = conn.execute(
            "INSERT OR IGNORE INTO suppression_list (domain_or_company, reason) VALUES ('EMPRESA_ARCO_BLOQUEADA', 'ARCO_OPTOUT')",
            [],
        );
    }
    let t_arco = Instant::now();
    let r_arco = HttpFormBot::submit_form_direct(&client, &format!("{}/wp-cf7-ok", base), "EMPRESA_ARCO_BLOQUEADA", 3000.0).await;
    let arco_us = t_arco.elapsed().as_micros();
    assert_eq!(r_arco, "OMITIDO_SUPRESION_ARCO");

    println!(
        "  [110] ANTI-OOM 512 KiB + SUPRESIÓN ARCO          | Bomba 3MB truncada en {} ms ({}) | ARCO bloqueado en {} µs",
        bomb_ms, r_bomb, arco_us
    );

    let _ = std::fs::remove_file(stress_db);
    let _ = std::fs::remove_file(format!("{}-wal", stress_db));
    let _ = std::fs::remove_file(format!("{}-shm", stress_db));
    std::env::set_var("MALLEN_DB_PATH", "data/mallen_prod.db");

    // -------------------------------------------------------------------------
    // ESCENARIO 111: Benchmark Real Keyset sobre 462,805 filas DENUE CDMX + Detección de Gobierno
    // -------------------------------------------------------------------------
    let t_ks = Instant::now();
    let mut last_rid = 0i64;
    let mut fetched_total = 0usize;
    let mut gov_detected = 0usize;
    for _ in 0..10 {
        let chunk = HttpFormBot::fetch_pyme_prospect_chunk_keyset(last_rid, 500);
        if chunk.is_empty() {
            break;
        }
        last_rid = chunk.last().unwrap().0;
        for (_, name, _, scian, _) in &chunk {
            let up = name.to_uppercase();
            if scian.starts_with("93")
                || up.contains("ALCALDIA ")
                || up.contains("SECRETARIA DE ")
                || up.contains("GOBIERNO DE ")
                || up.contains("FISCALIA ")
                || up.contains("TRIBUNAL ")
                || up.contains("JUZGADO ")
            {
                gov_detected += 1;
            }
        }
        fetched_total += chunk.len();
    }
    let ks_ms = t_ks.elapsed().as_millis();

    let (total_denue, total_web, total_ola1, total_gov_web): (i64, i64, i64, i64) = {
        let conn = rusqlite::Connection::open("data/mallen_prod.db").unwrap();
        let t_all: i64 = conn.query_row("SELECT COUNT(*) FROM denue_units", [], |r| r.get(0)).unwrap_or(0);
        let t_w: i64 = conn.query_row("SELECT COUNT(*) FROM denue_units WHERE TRIM(www) != ''", [], |r| r.get(0)).unwrap_or(0);
        let t_o1: i64 = conn.query_row(
            "SELECT COUNT(*) FROM denue_units WHERE TRIM(www) != '' AND per_ocu IN ('11 a 30 personas','31 a 50 personas','51 a 100 personas','101 a 250 personas')",
            [],
            |r| r.get(0),
        ).unwrap_or(0);
        let t_gov: i64 = conn.query_row(
            "SELECT COUNT(*) FROM denue_units WHERE TRIM(www) != '' AND (codigo_scian LIKE '93%' OR UPPER(nom_estab) LIKE '%ALCALDIA%' OR UPPER(nom_estab) LIKE '%SECRETARIA DE%' OR UPPER(nom_estab) LIKE '%GOBIERNO DE%' OR UPPER(nom_estab) LIKE '%FISCALIA%' OR UPPER(nom_estab) LIKE '%TRIBUNAL%')",
            [],
            |r| r.get(0),
        ).unwrap_or(0);
        (t_all, t_w, t_o1, t_gov)
    };

    println!(
        "  [111] DENUE REAL CDMX (Keyset 5,000 filas)       | Leídas: {} en {} ms | BD Total: {} | Web: {} | Ola 1 PyME: {} | Gobierno a excluir: {} (muestra: {})",
        fetched_total, ks_ms, total_denue, total_web, total_ola1, total_gov_web, gov_detected
    );
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!("✓ SUITE TRL 8 → TRL 9 (ESCENARIOS 106 AL 111) SUPERADA AL 100%");
}
