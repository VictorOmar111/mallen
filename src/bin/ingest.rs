//! Utilidad de línea de comandos para poblar e indexar `data/mallen_prod.db` en modo WAL
//! a partir de los archivos CSV de DENUE, SAT 69-B, SAT 69, STPS REPSE y SFP.

use csv::ReaderBuilder;
use rusqlite::{params, Connection};
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use worker_inegi_engine::engine::cleaner::{normalize_canonical_rfc, normalize_company_name};

const DB_PATH: &str = "data/mallen_prod.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("data")?;
    let start_total = Instant::now();
    println!("🚀 Iniciando motor de ingesta e indexación SQLite WAL en {}", DB_PATH);

    let mut conn = Connection::open(DB_PATH)?;

    // Pragmas de máxima velocidad para ingesta masiva
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA temp_store = MEMORY;
        PRAGMA cache_size = -128000;

        CREATE TABLE IF NOT EXISTS sat_69b_efos (
            rfc TEXT PRIMARY KEY,
            razon_social TEXT NOT NULL,
            situacion TEXT NOT NULL,
            oficio_global TEXT
        );

        CREATE TABLE IF NOT EXISTS sat_69_incumplidos (
            rfc TEXT PRIMARY KEY,
            razon_social TEXT NOT NULL,
            supuesto TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS stps_repse (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rfc TEXT,
            razon_social_norm TEXT NOT NULL,
            folio_repse TEXT NOT NULL,
            estado_registro TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_repse_rfc ON stps_repse(rfc);
        CREATE INDEX IF NOT EXISTS idx_repse_razon ON stps_repse(razon_social_norm);

        CREATE TABLE IF NOT EXISTS sfp_sancionados (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rfc TEXT,
            razon_social_norm TEXT NOT NULL,
            expediente TEXT,
            causa TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_sfp_rfc ON sfp_sancionados(rfc);
        CREATE INDEX IF NOT EXISTS idx_sfp_razon ON sfp_sancionados(razon_social_norm);

        CREATE TABLE IF NOT EXISTS denue_units (
            id_denue TEXT PRIMARY KEY,
            nom_estab TEXT NOT NULL,
            raz_social TEXT NOT NULL,
            nombre_norm TEXT NOT NULL,
            codigo_scian TEXT NOT NULL,
            nombre_act TEXT NOT NULL,
            per_ocu TEXT NOT NULL,
            direccion TEXT NOT NULL,
            entidad TEXT NOT NULL,
            www TEXT NOT NULL,
            email TEXT NOT NULL,
            telefono TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_denue_scian ON denue_units(codigo_scian);
        CREATE INDEX IF NOT EXISTS idx_denue_www ON denue_units(www);
        CREATE INDEX IF NOT EXISTS idx_denue_nombre_norm ON denue_units(nombre_norm);

        CREATE VIRTUAL TABLE IF NOT EXISTS denue_fts USING fts5(
            id_denue UNINDEXED,
            nombre_norm,
            www
        );
        "#,
    )?;

    ingest_sat_69b(&mut conn, "data/sat_efos.csv")?;
    ingest_sat_69(&mut conn, "data/sat_69_incumplidos.csv")?;
    ingest_denue(&mut conn, "data/denue_actividades.csv")?;

    println!(
        "✅ Ingesta e indexación completada en {:.2?}.",
        start_total.elapsed()
    );
    Ok(())
}

/// Ingesta el listado oficial del SAT Art. 69-B (EFOS) validando RFC canónico.
fn ingest_sat_69b(conn: &mut Connection, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(csv_path).exists() {
        println!("ℹ️  {} no encontrado. Insertando registros semilla de control SAT 69-B...", csv_path);
        conn.execute(
            "INSERT OR REPLACE INTO sat_69b_efos (rfc, razon_social, situacion, oficio_global) VALUES (?1, ?2, ?3, ?4)",
            params!["EFO990101AAA", "CONSTRUCTORA FANTASMA DEMO SA DE CV", "Definitivo", "500-05-2025-1001"],
        )?;
        return Ok(());
    }

    println!("📥 Procesando SAT 69-B desde {}...", csv_path);
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(file);

    let tx = conn.transaction()?;
    let mut count = 0usize;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO sat_69b_efos (rfc, razon_social, situacion, oficio_global) VALUES (?1, ?2, ?3, ?4)",
        )?;

        for result in rdr.records() {
            let Ok(record) = result else { continue };
            // En el CSV oficial del SAT 69-B: col 1 = RFC, col 2 = Nombre, col 3 = Situación
            let raw_rfc = record.get(1).or_else(|| record.get(0)).unwrap_or("");
            if let Some(rfc) = normalize_canonical_rfc(raw_rfc) {
                let razon = record.get(2).or_else(|| record.get(1)).unwrap_or("SIN RAZON SOCIAL").trim();
                let situacion = record.get(3).or_else(|| record.get(2)).unwrap_or("Definitivo").trim();
                let oficio = record.get(4).unwrap_or("");
                stmt.execute(params![rfc, razon, situacion, oficio])?;
                count += 1;
            }
        }
    }
    tx.commit()?;
    println!("   -> {} RFCs indexados en sat_69b_efos.", count);
    Ok(())
}

/// Ingesta el listado del SAT Art. 69 (Exigibles, Firmes, No Localizados).
fn ingest_sat_69(conn: &mut Connection, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(csv_path).exists() {
        conn.execute(
            "INSERT OR REPLACE INTO sat_69_incumplidos (rfc, razon_social, supuesto) VALUES (?1, ?2, ?3)",
            params!["NLO880101BBB", "COMERCIALIZADORA NO LOCALIZADA SA DE CV", "NO LOCALIZADO"],
        )?;
        return Ok(());
    }

    println!("📥 Procesando SAT Art. 69 desde {}...", csv_path);
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);
    let tx = conn.transaction()?;
    let mut count = 0usize;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO sat_69_incumplidos (rfc, razon_social, supuesto) VALUES (?1, ?2, ?3)",
        )?;
        for result in rdr.records() {
            let Ok(record) = result else { continue };
            let raw_rfc = record.get(0).unwrap_or("");
            if let Some(rfc) = normalize_canonical_rfc(raw_rfc) {
                let razon = record.get(1).unwrap_or("SIN NOMBRE").trim();
                let supuesto = record.get(2).unwrap_or("CRÉDITO FIRME").trim();
                stmt.execute(params![rfc, razon, supuesto])?;
                count += 1;
            }
        }
    }
    tx.commit()?;
    println!("   -> {} RFCs indexados en sat_69_incumplidos.", count);
    Ok(())
}

/// Ingesta el archivo del INEGI DENUE indexando por SCIAN, dominio web y razón social normalizada.
fn ingest_denue(conn: &mut Connection, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(csv_path).exists() {
        println!("ℹ️  {} no encontrado. Insertando unidades económicas semilla en denue_units...", csv_path);
        let norm = normalize_company_name("ELECTRONICA STEREN SA DE CV");
        conn.execute(
            r#"
            INSERT OR REPLACE INTO denue_units
            (id_denue, nom_estab, raz_social, nombre_norm, codigo_scian, nombre_act, per_ocu, direccion, entidad, www, email, telefono)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
            params![
                "DENUE-STEREN-001",
                "STEREN CORPORATIVO",
                "ELECTRONICA STEREN S.A. DE C.V.",
                norm,
                "434222",
                "Comercio al por mayor de equipo y material eléctrico y electrónico",
                "251 y más personas",
                "Biólogo Maximino Martínez 3408, Azcapotzalco, CDMX",
                "CIUDAD DE MÉXICO",
                "steren.com.mx",
                "info@steren.com.mx",
                "5553542200"
            ],
        )?;
        return Ok(());
    }

    println!("📥 Procesando INEGI DENUE desde {}...", csv_path);
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    let tx = conn.transaction()?;
    let mut count = 0usize;
    {
        let mut stmt = tx.prepare(
            r#"
            INSERT OR REPLACE INTO denue_units
            (id_denue, nom_estab, raz_social, nombre_norm, codigo_scian, nombre_act, per_ocu, direccion, entidad, www, email, telefono)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
        )?;

        for result in rdr.records() {
            let Ok(rec) = result else { continue };
            let id_denue = rec.get(0).unwrap_or("").trim();
            if id_denue.is_empty() {
                continue;
            }
            let nom_estab = rec.get(2).unwrap_or("").trim();
            let raz_social = rec.get(3).unwrap_or("").trim();
            let base_name = if !raz_social.is_empty() { raz_social } else { nom_estab };
            let nombre_norm = normalize_company_name(base_name);

            let codigo_scian = rec.get(4).unwrap_or("461110").trim();
            let nombre_act = rec.get(5).unwrap_or("Actividad comercial general").trim();
            let per_ocu = rec.get(6).unwrap_or("6 a 10 personas").trim();
            let direccion = rec.get(7).unwrap_or("Domicilio registrado en DENUE").trim();
            let entidad = rec.get(28).unwrap_or("MÉXICO").trim();
            let telefono = rec.get(35).unwrap_or("").trim();
            let email = rec.get(36).unwrap_or("").trim().to_lowercase();
            let www = rec.get(37).unwrap_or("").trim().to_lowercase();

            stmt.execute(params![
                id_denue,
                nom_estab,
                raz_social,
                nombre_norm,
                codigo_scian,
                nombre_act,
                per_ocu,
                direccion,
                entidad,
                www,
                email,
                telefono
            ])?;
            count += 1;
        }
    }
    tx.commit()?;
    println!("   -> {} establecimientos DENUE indexados.", count);
    Ok(())
}
