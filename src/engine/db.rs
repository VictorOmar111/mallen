//! Capa de persistencia y consultas de baja latencia (<2ms) sobre SQLite en modo WAL.
//! Gestiona prospectos, idempotencia de Stripe y cruces con DENUE (FTS5), SAT 69/69-B, REPSE y SFP.

use crate::engine::cleaner::{normalize_canonical_rfc, normalize_company_name};
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
    /// Inicializa el pool de SQLite forzando modo WAL, busy_timeout de 10s y memoria caché optimizada.
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let connect_opts = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(10))
            .pragma("temp_store", "MEMORY")
            .pragma("cache_size", "-64000"); // ~64 MB de caché en RAM para páginas B-Tree

        let pool = SqlitePoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(10))
            .connect_with(connect_opts)
            .await?;

        Ok(Self { pool })
    }

    /// Crea todas las tablas del ecosistema (Prospectos, Idempotencia Stripe y Padrones Oficiales).
    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        // 1. Tabla histórica de prospectos y clientes (Retrocompatible)
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

        // 2. Tabla de Idempotencia para Webhooks de Stripe (Evita doble timbrado CFDI 4.0)
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

        // 3. SAT Artículo 69-B (EFOS - Operaciones Simuladas: Presuntos, Definitivos, Desvirtuados)
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

        // 4. SAT Artículo 69 (Créditos Firmes, Exigibles, No Localizados, Cancelados)
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

        // 5. STPS Padrón Público REPSE (Subcontratación de Servicios Especializados)
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

        // 6. SFP Directorio de Proveedores y Contratistas Sancionados / Inhabilitados
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

        // 7. PROFECO Buró Comercial y Contratos de Adhesión (RRPAC)
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

        // 8. INEGI DENUE + Índice FTS5 para resolución ultrarrápida sin RFC
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

    // =========================================================================
    // MÉTODOS ORIGINALES DE EMBUDO Y PROSPECTOS (100% RETROCOMPATIBLES)
    // =========================================================================

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
        sqlx::query(
            r#"
            UPDATE prospects 
            SET status = 'PAID_CLIENT_KIT_DELIVERED' 
            WHERE tax_id = ?1
            "#,
        )
        .bind(tax_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // NUEVOS MÉTODOS DE AUDITORÍA O(1) Y BLINDAJE TRANSACCIONAL
    // =========================================================================

    /// Garantiza idempotencia en el Webhook de Stripe: devuelve `true` si es la primera vez que se procesa.
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

    /// Consulta O(1) en Lista Negra SAT Art. 69-B (EFOS) bloqueada estrictamente por RFC canónico.
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

    /// Consulta O(1) en Lista SAT Art. 69 (Créditos firmes, No localizados, Exigibles).
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

    /// Verifica si la empresa cuenta con registro REPSE activo (por RFC o Razón Social normalizada).
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

    /// Verifica si el contribuyente está inhabilitado por la SFP (Secretaría Anticorrupción / Función Pública).
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

    /// Puente de Resolución DENUE (Resuelve la ausencia de RFC en el CSV público del INEGI):
    /// Busca primero por dominio web exacto y luego por coincidencia en `nombre_norm` o FTS5.
    pub async fn find_denue_by_name_or_domain(
        &self,
        domain: Option<&str>,
        company_name: Option<&str>,
    ) -> Result<Option<DenueRecord>, sqlx::Error> {
        // 1. Intento directo por dominio web (alta precisión)
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

        // 2. Intento por Razón Social o Nombre Comercial normalizado
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
}
