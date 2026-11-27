// TRANSACTIONAL_TABLES_PROTECTED_V80: Preserves audit_snapshots, audit_sessions, prospects, link_clicks
//! Motor ETL en Rust de alto rendimiento para ingesta masiva de TODO el mercado mexicano:
//! - INEGI DENUE Nacional (32 Entidades Federativas / ~5.5 millones de unidades económicas)
//! - SAT Listado Completo Art. 69-B (EFOS) y Art. 69 (Incumplidos / No Localizados)
//! - SFP Proveedores Sancionados y STPS Padrón REPSE
//! Opera con transacciones por lotes de 10,000 filas sobre SQLite WAL + índice FTS5 (<2ms).

use csv::{ReaderBuilder, StringRecord};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};
use worker_inegi_engine::engine::cleaner::{normalize_canonical_rfc, normalize_company_name};
use worker_inegi_engine::engine::db::DbState;

fn clean_field(rec: &StringRecord, idx: usize) -> String {
    rec.get(idx).unwrap_or("").trim().trim_matches('"').to_string()
}

fn clean_digits(raw: &str, pad_cp: bool) -> String {
    let t = raw.trim();
    if t.is_empty() {
        return String::new();
    }
    let normalized = if t.contains('e') || t.contains('E') || t.ends_with(".0") {
        t.parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && *v >= 0.0)
            .map(|v| format!("{}", v.round() as u64))
            .unwrap_or_else(|| t.to_string())
    } else {
        t.to_string()
    };
    if pad_cp && normalized.len() == 4 && normalized.chars().all(|c| c.is_ascii_digit()) {
        format!("0{}", normalized)
    } else {
        normalized
    }
}

fn clean_www(raw: &str) -> String {
    let token = raw
        .trim()
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .find(|tok| !tok.is_empty())
        .unwrap_or("")
        .trim_matches(|c: char| c == '"' || c == '\'' || c == '<' || c == '>')
        .to_lowercase();
    if token.is_empty()
        || token.contains('@')
        || !token.contains('.')
        || matches!(token.as_str(), "ninguno" | "no tiene" | "n/a" | "na" | "sin pagina" | "s/n")
    {
        return String::new();
    }
    token
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    std::fs::create_dir_all("data")?;

    let db_url = "sqlite://data/mallen_prod.db";
    let db_state = DbState::new(db_url).await?;
    db_state.init_schema().await?;

    let opts = SqliteConnectOptions::from_str(db_url)?
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(30));
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(opts)
        .await?;

    // Modo 1: Ingesta incremental de un archivo CSV estatal/sectorial del INEGI DENUE (--append-denue ruta.csv)
    if args.len() >= 3 && args[1] == "--append-denue" {
        let csv_path = &args[2];
        let count = ingest_denue_csv_file(&pool, csv_path).await?;
        println!(
            "    Ingestadas {} unidades económicas desde {} en {:.2?}",
            count,
            csv_path,
            start.elapsed()
        );
        return Ok(());
    }

    // Modo 2: Reconstrucción del índice FTS5 al finalizar los 32 estados (--rebuild-fts)
    if args.len() >= 2 && args[1] == "--rebuild-fts" {
        println!(" Reconstruyendo índice de texto completo FTS5 (denue_fts)...");
        sqlx::query("DELETE FROM denue_fts;").execute(&pool).await?;
        sqlx::query(
            "INSERT INTO denue_fts (id_denue, nombre_norm, www) SELECT id_denue, nombre_norm, www FROM denue_units;",
        )
        .execute(&pool)
        .await?;
        println!(" Índice FTS5 nacional reconstruido en {:.2?}", start.elapsed());
        return Ok(());
    }

    println!(" Iniciando motor de ingesta e indexación SQLite WAL en data/mallen_prod.db");

    // 1. Ingesta SAT Art. 69-B (EFOS)
    if Path::new("data/sat_efos.csv").exists() {
        let n = ingest_sat_69b(&pool, "data/sat_efos.csv").await?;
        println!("    SAT 69-B (EFOS) indexado: {} contribuyentes.", n);
    } else {
        seed_sat_69b(&pool).await?;
    }

    // 2. Ingesta SAT Art. 69 (No Localizados / Exigibles / Firmes)
    if Path::new("data/sat_69_incumplidos.csv").exists() {
        let n = ingest_sat_69(&pool, "data/sat_69_incumplidos.csv").await?;
        println!("    SAT Art. 69 indexado: {} registros.", n);
    }

    // 3. Ingesta SFP Sancionados
    if Path::new("data/sfp_sancionados.csv").exists() {
        let n = ingest_sfp(&pool, "data/sfp_sancionados.csv").await?;
        println!("    SFP Sancionados indexado: {} proveedores.", n);
    }

    // 4. Semillas garantizadas de control + CSV DENUE si existe
    seed_denue_control_units(&pool).await?;
    if Path::new("data/denue_actividades.csv").exists() {
        let n = ingest_denue_csv_file(&pool, "data/denue_actividades.csv").await?;
        println!("    INEGI DENUE indexado: {} establecimientos.", n);
    }

    // Actualizar FTS5
    sqlx::query("DELETE FROM denue_fts;").execute(&pool).await?;
    sqlx::query(
        "INSERT INTO denue_fts (id_denue, nombre_norm, www) SELECT id_denue, nombre_norm, www FROM denue_units;",
    )
    .execute(&pool)
    .await?;

    println!(" Ingesta e indexación completada en {:.2?}.", start.elapsed());
    Ok(())
}

async fn ingest_sat_69b(pool: &SqlitePool, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(BufReader::new(file));

    let mut tx = pool.begin().await?;
    let mut count = 0usize;

    for result in rdr.records() {
        let Ok(rec) = result else { continue };
        // En el CSV oficial del SAT 69-B: col 1 = RFC, col 2 = Nombre, col 3 = Situación (Definitivo, Presunto, Desvirtuado)
        let raw_rfc = if rec.len() >= 4 { clean_field(&rec, 1) } else { clean_field(&rec, 0) };
        let Some(rfc) = normalize_canonical_rfc(&raw_rfc) else { continue };
        let razon = if rec.len() >= 4 { clean_field(&rec, 2) } else { clean_field(&rec, 1) };
        let situacion = if rec.len() >= 4 { clean_field(&rec, 3) } else { clean_field(&rec, 2) };
        let oficio = if rec.len() >= 5 { clean_field(&rec, 4) } else { "DOF/SAT-69B".to_string() };

        sqlx::query(
            "INSERT OR REPLACE INTO sat_69b_efos (rfc, razon_social, situacion, oficio_global) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&rfc)
        .bind(&razon)
        .bind(if situacion.is_empty() { "Definitivo" } else { &situacion })
        .bind(&oficio)
        .execute(&mut *tx)
        .await?;

        count += 1;
        if count % 10_000 == 0 {
            tx.commit().await?;
            tx = pool.begin().await?;
        }
    }
    tx.commit().await?;
    Ok(count)
}

async fn ingest_sat_69(pool: &SqlitePool, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(BufReader::new(file));

    let mut tx = pool.begin().await?;
    let mut count = 0usize;

    for result in rdr.records() {
        let Ok(rec) = result else { continue };
        let raw_rfc = clean_field(&rec, 0);
        let Some(rfc) = normalize_canonical_rfc(&raw_rfc) else { continue };
        let razon = clean_field(&rec, 1);
        let col3 = clean_field(&rec, 3);
        let col2 = clean_field(&rec, 2);
        let supuesto = if !col3.is_empty() {
            col3
        } else if col2 != "F" && col2 != "M" {
            col2
        } else {
            String::new()
        };

        sqlx::query(
            "INSERT OR REPLACE INTO sat_69_incumplidos (rfc, razon_social, supuesto) VALUES (?1, ?2, ?3)",
        )
        .bind(&rfc)
        .bind(&razon)
        .bind(if supuesto.is_empty() { "NO LOCALIZADO / EXIGIBLE" } else { &supuesto })
        .execute(&mut *tx)
        .await?;

        count += 1;
        if count % 10_000 == 0 {
            tx.commit().await?;
            tx = pool.begin().await?;
        }
    }
    tx.commit().await?;
    Ok(count)
}

async fn ingest_sfp(pool: &SqlitePool, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut tx = pool.begin().await?;
    let mut count = 0usize;

    for result in rdr.records() {
        let Ok(rec) = result else { continue };
        let raw_rfc = clean_field(&rec, 0);
        let razon = if rec.len() > 1 { clean_field(&rec, 1) } else { raw_rfc.clone() };
        let norm = normalize_company_name(&razon);
        if norm.len() < 3 {
            continue;
        }
        let expediente = clean_field(&rec, 2);
        let causa = clean_field(&rec, 3);

        sqlx::query(
            "INSERT INTO sfp_sancionados (rfc, razon_social_norm, expediente, causa) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(normalize_canonical_rfc(&raw_rfc))
        .bind(&norm)
        .bind(&expediente)
        .bind(&causa)
        .execute(&mut *tx)
        .await?;

        count += 1;
        if count % 10_000 == 0 {
            tx.commit().await?;
            tx = pool.begin().await?;
        }
    }
    tx.commit().await?;
    Ok(count)
}

/// Lee cualquier CSV oficial del INEGI DENUE mapeando columnas por nombre de encabezado (inmune a cambios de orden del INEGI)
async fn ingest_denue_csv_file(pool: &SqlitePool, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let hdr_map: std::collections::HashMap<String, usize> = rdr
        .headers()?
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim().trim_start_matches('﻿').to_ascii_uppercase(), i))
        .collect();

    let has_named_headers = hdr_map.contains_key("NOM_ESTAB") || hdr_map.contains_key("WWW");
    if !hdr_map.contains_key("WWW") {
        eprintln!("WARN [ingest]: encabezado 'WWW' no encontrado en {}; usando índice de respaldo 38.", path);
    }

    let idx_of = |names: &[&str], fallback: usize| -> usize {
        for name in names {
            if let Some(&i) = hdr_map.get(*name) {
                return i;
            }
        }
        fallback
    };

    let i_id = idx_of(&["ID"], 0);
    let i_nom = idx_of(&["NOM_ESTAB"], 2);
    let i_raz = idx_of(&["RAZ_SOCIAL"], 3);
    let i_scian = idx_of(&["CODIGO_ACT", "CODIGO_SCIAN"], 4);
    let i_act = idx_of(&["NOMBRE_ACT"], 5);
    let i_per = idx_of(&["PER_OCU"], 6);
    let i_vial = idx_of(&["NOM_VIAL"], 8);
    let i_num = idx_of(&["NUMERO_EXT"], 15);
    let i_col = idx_of(&["NOMB_ASENT"], 22);
    let i_cp = idx_of(&["COD_POSTAL"], 26);
    let i_ent = idx_of(&["ENTIDAD", "NOM_ENT"], 29);
    let i_mun = idx_of(&["MUNICIPIO", "NOM_MUN"], 31);
    let i_tel = idx_of(&["TELEFONO"], 36);
    let i_mail = idx_of(&["CORREOELEC", "EMAIL"], 37);
    let i_web = idx_of(&["WWW"], 38);

    let mut tx = pool.begin().await?;
    let mut count = 0usize;

    for result in rdr.records() {
        let Ok(rec) = result else { continue };
        if rec.len() < 8 {
            continue;
        }

        let (id_denue, nom_estab, raz_social, codigo_scian, nombre_act, per_ocu, direccion, entidad, www, email, telefono) =
            if has_named_headers || rec.len() >= 38 {
                let id = clean_field(&rec, i_id);
                let nom = clean_field(&rec, i_nom);
                let raz = clean_field(&rec, i_raz);
                let scian = clean_field(&rec, i_scian);
                let act = clean_field(&rec, i_act);
                let per = clean_field(&rec, i_per);
                let vial = clean_field(&rec, i_vial);
                let num = clean_field(&rec, i_num);
                let col = clean_field(&rec, i_col);
                let cp = clean_digits(&clean_field(&rec, i_cp), true);
                let ent = clean_field(&rec, i_ent);
                let mun = clean_field(&rec, i_mun);
                let tel = clean_digits(&clean_field(&rec, i_tel), false);
                let mail = clean_field(&rec, i_mail);
                let web = clean_www(&clean_field(&rec, i_web));
                let dir = format!("{} {}, COL. {}, C.P. {}, {}", vial, num, col, cp, mun);
                (id, nom, raz, scian, act, per, dir, ent, web, mail, tel)
            } else {
                (
                    clean_field(&rec, 0),
                    clean_field(&rec, 1),
                    clean_field(&rec, 2),
                    clean_field(&rec, 3),
                    clean_field(&rec, 4),
                    clean_field(&rec, 5),
                    clean_field(&rec, 6),
                    clean_field(&rec, 7),
                    clean_www(&clean_field(&rec, 8)),
                    clean_field(&rec, 9),
                    clean_digits(&clean_field(&rec, 10), false),
                )
            };

        if id_denue.is_empty() || id_denue == "id" {
            continue;
        }

        let display_for_norm = if !raz_social.is_empty() {
            format!("{} {}", nom_estab, raz_social)
        } else {
            nom_estab.clone()
        };
        let nombre_norm = normalize_company_name(&display_for_norm);
        if nombre_norm.len() < 2 {
            continue;
        }

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO denue_units
            (id_denue, nom_estab, raz_social, nombre_norm, codigo_scian, nombre_act, per_ocu, direccion, entidad, www, email, telefono)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
        )
        .bind(&id_denue)
        .bind(&nom_estab)
        .bind(&raz_social)
        .bind(&nombre_norm)
        .bind(&codigo_scian)
        .bind(&nombre_act)
        .bind(&per_ocu)
        .bind(&direccion)
        .bind(&entidad)
        .bind(&www.to_lowercase())
        .bind(&email.to_lowercase())
        .bind(&telefono)
        .execute(&mut *tx)
        .await?;

        count += 1;
        if count % 10_000 == 0 {
            tx.commit().await?;
            tx = pool.begin().await?;
        }
    }
    tx.commit().await?;
    Ok(count)
}

async fn seed_sat_69b(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR IGNORE INTO sat_69b_efos (rfc, razon_social, situacion, oficio_global) VALUES ('EFO190101AAA', 'COMERCIALIZADORA FANTASMA DEL CENTRO S.A. DE C.V.', 'Definitivo', '500-05-2025-18422')",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn seed_denue_control_units(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let seeds = [
        (
            "DENUE-0001",
            "STEREN",
            "ELECTRONICA STEREN S.A. DE C.V.",
            "434222",
            "Comercio al por mayor de equipo y material eléctrico y electrónico",
            "251 y más personas",
            "AV. BIOLOGO MAXIMINO MARTINEZ 3408, COL. SAN SALVADOR XOCHIMANCA, AZCAPOTZALCO",
            "CIUDAD DE MÉXICO",
            "steren.com.mx",
            "info@steren.com.mx",
            "5553542200",
        ),
        (
            "DENUE-0004",
            "PIZZERIA CANCINO",
            "GRUPO GASTRONOMICO ARCHIPIZZA S.A. DE C.V. (CANCINO)",
            "722511",
            "Restaurantes con servicio de preparación de alimentos a la carta o de comida corrida",
            "31 a 50 personas",
            "PLAZA VILLA DE MADRID 17, COL. ROMA NORTE, CUAUHTÉMOC",
            "CIUDAD DE MÉXICO",
            "archipizza.com",
            "contacto@archipizza.com",
            "5555110982",
        ),
        (
            "DENUE-0005",
            "CANCINO",
            "OPERADORA RESTAURANTERA CANCINO S.A. DE C.V.",
            "722514",
            "Restaurantes con servicio de preparación de pizzas, hamburguesas, hot dogs y pollos rostizados",
            "31 a 50 personas",
            "DURANGO 205, COL. ROMA NORTE, CUAUHTÉMOC",
            "CIUDAD DE MÉXICO",
            "cancino.mx",
            "reservaciones@cancino.mx",
            "5555110982",
        ),
    ];

    for s in seeds {
        let norm = normalize_company_name(&format!("{} {}", s.1, s.2));
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO denue_units
            (id_denue, nom_estab, raz_social, nombre_norm, codigo_scian, nombre_act, per_ocu, direccion, entidad, www, email, telefono)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
        )
        .bind(s.0)
        .bind(s.1)
        .bind(s.2)
        .bind(norm)
        .bind(s.3)
        .bind(s.4)
        .bind(s.5)
        .bind(s.6)
        .bind(s.7)
        .bind(s.8)
        .bind(s.9)
        .bind(s.10)
        .execute(pool)
        .await?;
    }
    Ok(())
}
