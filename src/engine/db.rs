//! Capa de persistencia y consultas de baja latencia (<2ms) sobre SQLite en modo WAL.
//! Gestiona prospectos, sesiones completas de auditoría (`audit_sessions` para sincronía 1:1 Web-PDF),
//! idempotencia de Stripe y cruces con DENUE (FTS5), SAT 69/69-B, REPSE y SFP.

use crate::engine::cleaner::{normalize_canonical_rfc, normalize_company_name};
use crate::engine::models::AuditMasterJson;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Row, SqlitePool,
};
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenueRecord {
    pub id_denue: String,
    pub nom_estab: String,
    pub raz_social: String,
    pub codigo_scian: String,
    pub nombre_act: String,
    pub per_ocu: String,
    pub direccion: String,
    pub entidad: String,
    pub www: String,
    pub email: String,
    pub telefono: String,
}

#[derive(Clone)]
pub struct DbState {
    pub pool: SqlitePool,
}

impl DbState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let connect_opts = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(10))
            .pragma("temp_store", "MEMORY")
            .pragma("cache_size", "-64000");

        let pool = SqlitePoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(connect_opts)
            .await?;

        Ok(Self { pool })
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        // 1. Tabla de prospectos
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS prospects (
                id TEXT PRIMARY KEY,
                tax_id TEXT NOT NULL,
                nombre TEXT NOT NULL,
                email TEXT,
                risk_score INTEGER NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_prospects_tax_id ON prospects(tax_id);
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 2. Tabla de Sesiones de Auditoría y Personalización Jurídica (Sincronía 100% Web <-> PDF <-> ZIP)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS audit_snapshots (
                    snapshot_uuid TEXT PRIMARY KEY,
                    tax_id_key TEXT NOT NULL,
                    empresa_key TEXT NOT NULL,
                    emp_count INTEGER NOT NULL,
                    total_cents INTEGER NOT NULL,
                    risk_score INTEGER NOT NULL,
                    master_json_blob TEXT NOT NULL,
                    created_at_utc TEXT DEFAULT (datetime('now'))
                );
                CREATE INDEX IF NOT EXISTS idx_audit_snapshots_tax_uuid ON audit_snapshots(tax_id_key, snapshot_uuid);
                CREATE TABLE IF NOT EXISTS audit_sessions (
                tax_id TEXT PRIMARY KEY,
                master_json TEXT NOT NULL,
                is_accredited INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 3. Tabla de Idempotencia para Webhooks de Stripe
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS processed_webhooks (
                event_id TEXT PRIMARY KEY,
                tax_id TEXT,
                processed_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 4. SAT Artículo 69-B (EFOS)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sat_69b_efos (
                rfc TEXT PRIMARY KEY,
                razon_social TEXT NOT NULL,
                situacion TEXT NOT NULL,
                oficio_global TEXT
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 5. SAT Artículo 69
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sat_69_incumplidos (
                rfc TEXT PRIMARY KEY,
                razon_social TEXT NOT NULL,
                supuesto TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 6. STPS Padrón REPSE
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS stps_repse (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                rfc TEXT,
                razon_social_norm TEXT NOT NULL,
                folio_repse TEXT NOT NULL,
                estado_registro TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_repse_rfc ON stps_repse(rfc);
            CREATE INDEX IF NOT EXISTS idx_repse_razon ON stps_repse(razon_social_norm);
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 7. SFP Sancionados
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sfp_sancionados (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                rfc TEXT,
                razon_social_norm TEXT NOT NULL,
                expediente TEXT,
                causa TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_sfp_rfc ON sfp_sancionados(rfc);
            CREATE INDEX IF NOT EXISTS idx_sfp_razon ON sfp_sancionados(razon_social_norm);
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 8. PROFECO Buró Comercial y RRPAC
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS profeco_buro (
                razon_social_norm TEXT PRIMARY KEY,
                quejas_totales INTEGER NOT NULL DEFAULT 0,
                registro_rrpac TEXT
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 9. INEGI DENUE + FTS5
        sqlx::query(
            r#"
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
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Guarda o actualiza la sesión completa de auditoría (`AuditMasterJson` + `LegalDossierInput`)
    /// para que el PDF y los 7 Kits Legales reflejen exactamente los mismos datos y montos que la Web.
    pub async fn save_audit_session(
        &self,
        tax_id: &str,
        master: &AuditMasterJson,
        is_accredited: bool,
    ) -> Result<(), sqlx::Error> {
        let clean_tax = tax_id.trim().to_uppercase();
        let json_str = serde_json::to_string(master).unwrap_or_default();
        let now = Utc::now().to_rfc3339();
        let acc_int = if is_accredited { 1 } else { 0 };

        sqlx::query(
            r#"
            INSERT INTO audit_sessions (tax_id, master_json, is_accredited, updated_at)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(tax_id) DO UPDATE SET
                master_json = excluded.master_json,
                is_accredited = CASE WHEN audit_sessions.is_accredited = 1 THEN 1 ELSE excluded.is_accredited END,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&clean_tax)
        .bind(&json_str)
        .bind(acc_int)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Recupera la sesión exacta de auditoría y si la empresa ya está acreditada/blindada (`is_accredited`).
    pub async fn get_audit_session(
        &self,
        tax_id: &str,
    ) -> Result<Option<(AuditMasterJson, bool)>, sqlx::Error> {
        let clean_tax = tax_id.trim().to_uppercase();
        let row = sqlx::query(
            "SELECT master_json, is_accredited FROM audit_sessions WHERE tax_id = ?1 LIMIT 1",
        )
        .bind(&clean_tax)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let json_str: String = r.get("master_json");
            let is_acc: i64 = r.get("is_accredited");
            if let Ok(parsed) = serde_json::from_str::<AuditMasterJson>(&json_str) {
                return Ok(Some((parsed, is_acc == 1)));
            }
        }
        Ok(None)
    }

    pub async fn insert_prospect(
        &self,
        tax_id: &str,
        nombre: &str,
        email: Option<&str>,
        risk_score: u32,
        status: &str,
    ) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO prospects (id, tax_id, nombre, email, risk_score, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(&id)
        .bind(tax_id)
        .bind(nombre)
        .bind(email)
        .bind(risk_score)
        .bind(status)
        .bind(&created_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn save_prospect(
        &self,
        tax_id: &str,
        nombre: &str,
        email: &str,
        risk_score: u32,
    ) -> Result<String, sqlx::Error> {
        let email_opt = if email.trim().is_empty() {
            None
        } else {
            Some(email.trim())
        };

        self.insert_prospect(
            tax_id,
            nombre,
            email_opt,
            risk_score,
            "LEAD_CAPTURED_FREE_PDF",
        )
        .await
    }

    pub async fn update_status_to_paid(&self, tax_id: &str) -> Result<(), sqlx::Error> {
        let clean_tax = tax_id.trim().to_uppercase();
        sqlx::query(
            r#"
            UPDATE prospects 
            SET status = 'PAID_CLIENT_KIT_DELIVERED' 
            WHERE tax_id = ?1
            "#,
        )
        .bind(&clean_tax)
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            UPDATE audit_sessions
            SET is_accredited = 1
            WHERE tax_id = ?1
            "#,
        )
        .bind(&clean_tax)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn register_webhook_once(
        &self,
        event_id: &str,
        tax_id: Option<&str>,
    ) -> Result<bool, sqlx::Error> {
        let now = Utc::now().to_rfc3339();
        let res = sqlx::query(
            r#"
            INSERT OR IGNORE INTO processed_webhooks (event_id, tax_id, processed_at)
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(event_id)
        .bind(tax_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() == 1)
    }

    pub async fn check_sat_69b_by_rfc(&self, raw_rfc: &str) -> Result<String, sqlx::Error> {
        let Some(canonical_rfc) = normalize_canonical_rfc(raw_rfc) else {
            return Ok("RFC_FORMATO_NO_CANONICO".to_string());
        };

        let row = sqlx::query("SELECT situacion FROM sat_69b_efos WHERE rfc = ?1 LIMIT 1")
            .bind(&canonical_rfc)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(r.get::<String, _>("situacion")),
            None => Ok("LIMPIO".to_string()),
        }
    }

    pub async fn check_sat_69_by_rfc(&self, raw_rfc: &str) -> Result<String, sqlx::Error> {
        let Some(canonical_rfc) = normalize_canonical_rfc(raw_rfc) else {
            return Ok("LIMPIO".to_string());
        };

        let row = sqlx::query("SELECT supuesto FROM sat_69_incumplidos WHERE rfc = ?1 LIMIT 1")
            .bind(&canonical_rfc)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(r.get::<String, _>("supuesto")),
            None => Ok("LIMPIO".to_string()),
        }
    }

    pub async fn check_repse_status(
        &self,
        raw_rfc: &str,
        razon_social: &str,
    ) -> Result<bool, sqlx::Error> {
        if let Some(rfc) = normalize_canonical_rfc(raw_rfc) {
            let row = sqlx::query("SELECT 1 FROM stps_repse WHERE rfc = ?1 LIMIT 1")
                .bind(rfc)
                .fetch_optional(&self.pool)
                .await?;
            if row.is_some() {
                return Ok(true);
            }
        }

        let norm_name = normalize_company_name(razon_social);
        if norm_name.len() >= 4 {
            let row = sqlx::query("SELECT 1 FROM stps_repse WHERE razon_social_norm = ?1 LIMIT 1")
                .bind(norm_name)
                .fetch_optional(&self.pool)
                .await?;
            return Ok(row.is_some());
        }

        Ok(false)
    }

    pub async fn check_sfp_sanctioned(
        &self,
        raw_rfc: &str,
        razon_social: &str,
    ) -> Result<bool, sqlx::Error> {
        if let Some(rfc) = normalize_canonical_rfc(raw_rfc) {
            let row = sqlx::query("SELECT 1 FROM sfp_sancionados WHERE rfc = ?1 LIMIT 1")
                .bind(rfc)
                .fetch_optional(&self.pool)
                .await?;
            if row.is_some() {
                return Ok(true);
            }
        }

        let norm_name = normalize_company_name(razon_social);
        if norm_name.len() >= 4 {
            let row = sqlx::query(
                "SELECT 1 FROM sfp_sancionados WHERE razon_social_norm = ?1 LIMIT 1",
            )
            .bind(norm_name)
            .fetch_optional(&self.pool)
            .await?;
            return Ok(row.is_some());
        }

        Ok(false)
    }

    pub async fn find_denue_by_name_or_domain(
        &self,
        domain: Option<&str>,
        company_name: Option<&str>,
    ) -> Result<Option<DenueRecord>, sqlx::Error> {
        if let Some(dom) = domain {
            let clean_dom = dom
                .trim()
                .to_lowercase()
                .replace("https://", "")
                .replace("http://", "")
                .replace("www.", "")
                .split('/')
                .next()
                .unwrap_or("")
                .to_string();

            if !clean_dom.is_empty() {
                let row = sqlx::query(
                    r#"
                    SELECT id_denue, nom_estab, raz_social, codigo_scian, nombre_act,
                           per_ocu, direccion, entidad, www, email, telefono
                    FROM denue_units
                    WHERE www LIKE ?1
                    LIMIT 1
                    "#,
                )
                .bind(format!("%{}%", clean_dom))
                .fetch_optional(&self.pool)
                .await?;

                if let Some(r) = row {
                    return Ok(Some(Self::map_denue_row(r)));
                }
            }
        }

        if let Some(name) = company_name {
            let norm = normalize_company_name(name);
            if norm.len() >= 3 {
                let row = sqlx::query(
                    r#"
                    SELECT id_denue, nom_estab, raz_social, codigo_scian, nombre_act,
                           per_ocu, direccion, entidad, www, email, telefono
                    FROM denue_units
                    WHERE nombre_norm = ?1
                    LIMIT 1
                    "#,
                )
                .bind(&norm)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(r) = row {
                    return Ok(Some(Self::map_denue_row(r)));
                }
            }
        }

        Ok(None)
    }

    fn map_denue_row(r: sqlx::sqlite::SqliteRow) -> DenueRecord {
        DenueRecord {
            id_denue: r.get("id_denue"),
            nom_estab: r.get("nom_estab"),
            raz_social: r.get("raz_social"),
            codigo_scian: r.get("codigo_scian"),
            nombre_act: r.get("nombre_act"),
            per_ocu: r.get("per_ocu"),
            direccion: r.get("direccion"),
            entidad: r.get("entidad"),
            www: r.get("www"),
            email: r.get("email"),
            telefono: r.get("telefono"),
        }
    }

    pub async fn process_arco_optout(
        &self,
        target: &str,
        email: &str,
        motivo: &str,
    ) -> String {
        let clean = target.trim().to_uppercase();
        let _ = sqlx::query("DELETE FROM audit_cache WHERE upper(tax_id) = ?1 OR master_json LIKE ?2")
            .bind(&clean)
            .bind(format!("%{}%", clean))
            .execute(&self.pool)
            .await;

        let folio = format!(
            "ARCO-MALLEN-{}",
            &crate::engine::cleaner::sha256_hex(&format!("{}:{}:{}", clean, email, motivo))[..10].to_uppercase()
        );
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO arco_optouts (folio, tax_id_or_domain, solicitante_email, motivo) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&folio)
        .bind(&clean)
        .bind(email.trim())
        .bind(motivo)
        .execute(&self.pool)
        .await;

        folio
    }

    pub async fn log_delivery_receipt(
        &self,
        tax_id: &str,
        artifact_type: &str,
        artifact_sha256: &str,
    ) {
        let receipt_id = format!("ACK-{}-{}", tax_id, &artifact_sha256[..8].to_uppercase());
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO delivery_receipts (receipt_id, tax_id, artifact_type, artifact_sha256) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&receipt_id)
        .bind(tax_id)
        .bind(artifact_type)
        .bind(artifact_sha256)
        .execute(&self.pool)
        .await;
    }

}
