use rusqlite::{params, Connection, Result};
use crate::engine::models::EmpresaB2B;
use std::sync::Mutex;

pub struct LocalDatabase {
    conn: Mutex<Connection>,
}

impl LocalDatabase {
    /// Inicializa la base de datos SQLite local protegida por Mutex para concurrencia segura
    pub fn init(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS empresas_auditadas (
                id TEXT PRIMARY KEY,
                tax_id TEXT,
                nombre TEXT,
                actividad TEXT,
                estado TEXT,
                risk_score INTEGER,
                estimated_fine_mxn REAL,
                shield_price_usd REAL,
                audit_hash TEXT,
                status_prospeccion TEXT,
                timestamp INTEGER
            )",
            params![],
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// Guarda o actualiza un expediente auditado de forma segura entre hilos
    pub fn save_empresa(&self, e: &EmpresaB2B, status_pros: &str, timestamp: u64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO empresas_auditadas 
             (id, tax_id, nombre, actividad, estado, risk_score, estimated_fine_mxn, shield_price_usd, audit_hash, status_prospeccion, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                e.id,
                e.tax_id.as_deref().unwrap_or(""),
                e.nombre,
                e.actividad,
                e.estado,
                e.risk_score,
                e.estimated_fine_mxn,
                e.shield_price_usd,
                e.audit_hash,
                status_pros,
                timestamp as i64
            ],
        )?;
        Ok(())
    }

    /// Verifica si un ID o RFC ya fue auditado previamente
    pub fn exists(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(1) FROM empresas_auditadas WHERE id = ?1")?;
        let count: i64 = stmt.query_row(params![id], |row| row.get(0))?;
        Ok(count > 0)
    }
}
