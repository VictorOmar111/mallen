//! Modelos de dominio, estructuras de calibración pericial (12 preguntas),
//! calculadora dinámica de multas en UMAs y contrato JSON maestro para Typst.

use serde::{Deserialize, Serialize};

/// Valor oficial diario de la UMA en MXN (Actualizable dinámicamente por ejercicio fiscal INEGI)
pub const DEFAULT_UMA_VALUE_MXN: f64 = 113.14;

/// Estructura histórica base para compatibilidad total con los módulos existentes de prospección y DB.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmpresaB2B {
    pub id: String,
    pub nombre: String,
    pub actividad: String,
    pub telefono: String,
    pub email: String,
    pub direccion: String,
    pub estado: String,
    pub sitio_web: Option<String>,
    pub status_web: Option<u16>,
    pub e_mail_verificado: Option<bool>,
    pub country_code: String,
    pub tax_id: Option<String>,
    pub language: Option<String>,
    pub audit_hash: String,
    pub risk_score: u32,
    pub audit_flags: Vec<String>,
    pub estimated_fine_mxn: f64,
    pub shield_price_usd: f64,
    pub accountant_signature: Option<String>,
}

/// Tipos de canal de contacto para el triage de prospección en `processor.rs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CanalContacto {
    EmailDirecto(String),
    FormularioLimpio(String),
    FormularioConCaptcha { url: String, tipo: String },
    SinContactoUtil,
}

/// Resultado del triage de contactabilidad de una unidad económica.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluacionObjetivo {
    pub id_empresa: String,
    pub canal_optimo: CanalContacto,
    pub score_efectividad: u8,
    pub requiere_bypass: bool,
}

/// Resultado resumido de auditoría y sellado criptográfico en `processor.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoAuditoria {
    pub id_empresa: String,
    pub estado_riesgo: String,
    pub score_cumplimiento: u8,
    pub hash_auditoria: String,
    pub merkle_root: Option<String>,
    pub timestamp: u64,
}

/// Petición de entrada en la Landing (Fricción cero: RFC + Dominio) o con Calibración Pericial.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub tax_id: String,
    pub domain: Option<String>,
    pub razon_social_override: Option<String>,
    pub email_contacto: Option<String>,
    /// Respuestas opcionales del Cuestionario de Certeza Pericial (None en Teaser rápido)
    #[serde(default)]
    pub calibration: CalibrationAnswers,
    /// Identificador opcional de Despacho Contable para modo White-Label (B2B2B)
    pub partner_id: Option<String>,
}

/// Cuestionario de Certeza Pericial (12 bloques de variables internas invisibles por OSINT).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalibrationAnswers {
    // Bloque 1: Personal, Modalidad y Seguridad Social (STPS + IMSS)
    pub exact_employee_count: Option<u32>,
    pub has_telework_over_40_percent: Option<bool>,       // Activa NOM-037-STPS-2023
    pub uses_subcontracted_services_repse: Option<bool>,  // Activa Art. 15-A/15-B LSS (REPSE/ICSOE/SISSUB)
    pub has_disabled_workers: Option<bool>,               // Activa NOM-034-STPS

    // Bloque 2: Instalaciones, Maquinaria y Riesgos Físicos (NOMs de Seguridad y Salud)
    pub facility_area_sqm: Option<f64>,                   // >100 m² activa PIPC Protección Civil
    pub has_pressure_vessels_boilers: Option<bool>,       // Activa NOM-020-STPS
    pub has_forklifts_or_manual_loads: Option<bool>,      // Activa NOM-006-STPS y NOM-036-1-STPS
    pub has_heavy_machinery: Option<bool>,                // Activa NOM-004-STPS
    pub handles_hazardous_chemicals: Option<bool>,        // Activa NOM-005, NOM-010 y NOM-018-STPS
    pub performs_height_work: Option<bool>,               // Activa NOM-009-STPS
    pub performs_welding_work: Option<bool>,              // Activa NOM-027-STPS
    pub has_confined_spaces: Option<bool>,                // Activa NOM-033-STPS
    pub performs_electrical_maintenance: Option<bool>,    // Activa NOM-029 y NOM-022-STPS
    pub exposed_to_industrial_noise: Option<bool>,        // Activa NOM-011-STPS
    pub exposed_to_extreme_temperatures: Option<bool>,    // Activa NOM-015-STPS
    pub exposed_to_vibrations_or_radiation: Option<bool>, // Activa NOM-024 / NOM-012 / NOM-013-STPS

    // Bloque 3: Privacidad y Datos Sensibles (LFPDPPP / Autoridad Garante SABG ex-INAI)
    pub uses_biometrics_or_health_data: Option<bool>,     // Activa Arts. 9 y 16 LFPDPPP (Consentimiento expreso)
    pub transfers_data_to_third_parties: Option<bool>,    // Activa Arts. 36 y 37 LFPDPPP (Cláusulas transferencia)

    // Bloque 4: Fiscal, Materialidad y Operaciones Vulnerables (SAT + LFPIORPI)
    pub pays_intangible_b2b_services: Option<bool>,       // Activa Kit Materialidad Art. 69-B CFF
    pub transports_goods_federal_highways: Option<bool>,  // Activa Complemento Carta Porte CFDI 4.0 + SICT
    pub performs_vulnerable_activities_pld: Option<bool>, // Activa LFPIORPI / CNBV Antilavado
}

impl CalibrationAnswers {
    /// Resuelve un flag booleano priorizando la respuesta explícita del usuario sobre la inferencia OSINT/SCIAN.
    pub fn resolve_flag(explicit: Option<bool>, inferred: bool) -> bool {
        explicit.unwrap_or(inferred)
    }
}

/// Órgano regulador o materia del hallazgo normativo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegulatoryAgency {
    STPS,
    SAT,
    /// Secretaría Anticorrupción y de Buen Gobierno / Transparencia para el Pueblo (ex-INAI) - LFPDPPP
    SabgInai,
    IMSSInfonavit,
    PROFECO,
    SectorialCofepris,
    SectorialProteccionCivil,
    SectorialSemarnat,
    SectorialCnbvPld,
    SectorialSict,
    SectorialSfp,
    SectorialSepSecturSspc,
}

/// Nivel de criticidad para el semáforo del Dictamen Ejecutivo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    CriticalClosure,
}

/// Hallazgo individual auditado con fundamento legal y cuantificación en UMAs y MXN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFinding {
    pub code: String,
    pub agency: RegulatoryAgency,
    pub title: String,
    pub legal_basis: String,
    pub severity: RiskSeverity,
    pub description: String,
    pub remediation_action: String,
    pub min_uma: u32,
    pub max_uma: u32,
    pub min_fine_mxn: f64,
    pub estimated_fine_mxn: f64,
    pub max_fine_mxn: f64,
    pub verified_by_osint: bool,
}

/// Calculadora dinámica de sanciones de UMA a Pesos Mexicanos (MXN).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UmaCalculator {
    pub uma_daily_mxn: f64,
    pub fiscal_year: u16,
}

impl Default for UmaCalculator {
    fn default() -> Self {
        Self {
            uma_daily_mxn: DEFAULT_UMA_VALUE_MXN,
            fiscal_year: 2026,
        }
    }
}

impl UmaCalculator {
    pub fn compute_fine_bands(
        &self,
        min_uma: u32,
        max_uma: u32,
        employee_count: u32,
        scale_by_workers: bool,
    ) -> (f64, f64, f64) {
        let worker_multiplier = if scale_by_workers {
            match employee_count {
                0..=15 => 1.0,
                16..=50 => 2.5,
                51..=100 => 5.0,
                101..=250 => 10.0,
                _ => 20.0,
            }
        } else {
            1.0
        };

        let min_mxn = (min_uma as f64) * self.uma_daily_mxn;
        let max_mxn = (max_uma as f64) * self.uma_daily_mxn * worker_multiplier;
        let estimated_mxn = min_mxn + (max_mxn - min_mxn) * 0.35;

        (min_mxn, estimated_mxn, max_mxn)
    }
}

/// Evidencia criptográfica de aceptación del Disclaimer Pericial y blindaje Anti-Chargeback (Stripe 3DS).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalAcceptanceMetadata {
    pub accepted_at_utc: String,
    pub client_ip: String,
    pub user_agent: String,
    pub disclaimer_sha256: String,
    pub stripe_session_id: Option<String>,
    pub stripe_payment_intent: Option<String>,
    pub three_d_secure_status: Option<String>,
}

/// Estado técnico capturado por el crawler de doble capa sobre el dominio del cliente.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebOsintTelemetry {
    pub target_url: String,
    pub http_status: u16,
    pub ssl_valid: bool,
    pub ssl_error_reason: Option<String>,
    pub is_spa_rendered: bool,
    pub has_privacy_notice: bool,
    pub mentions_arco_rights: bool,
    pub has_ecommerce_cart: bool,
    pub has_profeco_rrpac_number: bool,
    pub orphan_subdomains: Vec<String>,
}

/// Contrato JSON Maestro que alimenta a `typst_runner.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMasterJson {
    pub audit_id: String,
    pub generated_at_utc: String,
    pub tax_id: String,
    pub razon_social: String,
    pub nombre_comercial: String,
    pub codigo_scian: String,
    pub actividad_scian_desc: String,
    pub domicilio_fiscal_o_denue: String,
    pub employee_stratum: String,
    pub estimated_employees: u32,

    pub sat_69b_status: String,
    pub sat_69_status: String,
    pub stps_repse_registered: bool,
    pub sfp_sanctioned: bool,

    pub web_telemetry: WebOsintTelemetry,
    pub calibration_applied: CalibrationAnswers,

    pub usa_datos_biometricos: bool,
    pub realiza_transferencias_datos: bool,
    pub requiere_nom_035: bool,
    pub requiere_nom_037_teletrabajo: bool,
    pub requiere_pipc_proteccion_civil: bool,
    pub opera_ecommerce: bool,

    pub findings: Vec<RegulatoryFinding>,
    pub risk_score: u32,
    pub total_min_fine_mxn: f64,
    pub total_estimated_fine_mxn: f64,
    pub total_max_fine_mxn: f64,

    pub audit_sha256_hash: String,
    pub merkle_root_hex: Option<String>,
    pub l2_tx_hash: Option<String>,
    pub legal_acceptance: Option<LegalAcceptanceMetadata>,
    pub partner_whitelabel_name: Option<String>,
    pub partner_logo_base64: Option<String>,
}
