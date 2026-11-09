//! Modelos de dominio, estructuras de calibración pericial (12 preguntas),
//! Ficha de Personalización Jurídica Completa (`LegalDossierInput` — 23 campos para Cero Placeholders),
//! desglose matemático por dependencia (`AgencyFineBreakdown`) y calculadora UMA 2026.

use serde::{Deserialize, Serialize};

/// Valor oficial diario de la UMA en MXN (Ejercicio Fiscal 2026 INEGI)
pub const DEFAULT_UMA_VALUE_MXN: f64 = 113.14;

/// Estructura histórica base para compatibilidad total con los módulos existentes.
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CanalContacto {
    EmailDirecto(String),
    FormularioLimpio(String),
    FormularioConCaptcha { url: String, tipo: String },
    SinContactoUtil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluacionObjetivo {
    pub id_empresa: String,
    pub canal_optimo: CanalContacto,
    pub score_efectividad: u8,
    pub requiere_bypass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoAuditoria {
    pub id_empresa: String,
    pub estado_riesgo: String,
    pub score_cumplimiento: u8,
    pub hash_auditoria: String,
    pub merkle_root: Option<String>,
    pub timestamp: u64,
}

/// Ficha de Personalización Jurídica Completa (23 variables corporativas y legales).
/// Elimina al 100% cualquier línea en blanco (`____`) o dato supuesto en los Certificados y en los 7 Kits Legales.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalDossierInput {
    // 1. Identidad Corporativa y Fiscal (Certificados + CFDI 4.0)
    pub razon_social_oficial: Option<String>,
    pub nombre_comercial: Option<String>,
    pub giro_scian_descripcion: Option<String>,
    pub domicilio_fiscal_completo: Option<String>,
    pub codigo_postal_fiscal: Option<String>,
    pub entidad_jurisdiccion: Option<String>,
    pub regimen_fiscal_sat: Option<String>,
    pub uso_cfdi: Option<String>,

    // 2. Representación Legal y Gobernanza de Privacidad (LFPDPPP / SABG)
    pub representante_legal_nombre: Option<String>,
    pub representante_legal_cargo: Option<String>,
    pub instrumento_notarial_num: Option<String>,
    pub oficial_privacidad_nombre: Option<String>,
    pub email_arco_privacidad: Option<String>,
    pub telefono_corporativo: Option<String>,

    // 3. Comisión Mixta de Seguridad e Higiene (NOM-019), Salud (NOM-030) y Teletrabajo (NOM-037)
    pub registro_patronal_imss: Option<String>,
    pub comision_coordinador_patronal: Option<String>,
    pub comision_secretario_trabajadores: Option<String>,
    pub comision_vocal_patronal: Option<String>,
    pub comision_vocal_trabajadores: Option<String>,
    pub horario_jornada_laboral: Option<String>,
    pub monto_apoyo_teletrabajo_mxn: Option<String>,

    // 4. Materialidad Fiscal B2B (Art. 69-B CFF), REPSE (IMSS) y Garantías (PROFECO)
    pub proveedor_principal_razon_social: Option<String>,
    pub proveedor_principal_rfc: Option<String>,
    pub proveedor_principal_servicio_objeto: Option<String>,
    pub testigo_auditor_interno: Option<String>,
    pub clase_prima_riesgo_imss: Option<String>,
    pub folio_repse_activo: Option<String>,
    pub dias_garantia_comercial_profeco: Option<u32>,
}

/// Desglose matemático exacto por cada una de las 6 dependencias para sincronizar 1:1 la Web y la tabla del PDF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyFineBreakdown {
    pub stps_fine_mxn: f64,
    pub sat_fine_mxn: f64,
    pub sabg_inai_fine_mxn: f64,
    pub imss_fine_mxn: f64,
    pub sectorial_pc_fine_mxn: f64,
    pub profeco_fine_mxn: f64,
    pub stps_points: u32,
    pub sat_points: u32,
    pub sabg_inai_points: u32,
    pub imss_points: u32,
    pub sectorial_pc_points: u32,
    pub profeco_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub tax_id: String,
    pub domain: Option<String>,
    pub razon_social_override: Option<String>,
    pub email_contacto: Option<String>,
    #[serde(default)]
    pub calibration: CalibrationAnswers,
    pub partner_id: Option<String>,
}

/// Cuestionario de Certeza Pericial (12 bloques de riesgo + Ficha `LegalDossierInput` embebida con `#[serde(default)]`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationAnswers {
    pub exact_employee_count: Option<u32>,
    pub has_telework_over_40_percent: Option<bool>,
    pub uses_subcontracted_services_repse: Option<bool>,
    pub has_disabled_workers: Option<bool>,

    pub facility_area_sqm: Option<f64>,
    pub has_pressure_vessels_boilers: Option<bool>,
    pub has_forklifts_or_manual_loads: Option<bool>,
    pub has_heavy_machinery: Option<bool>,
    pub handles_hazardous_chemicals: Option<bool>,
    pub performs_height_work: Option<bool>,
    pub performs_welding_work: Option<bool>,
    pub has_confined_spaces: Option<bool>,
    pub performs_electrical_maintenance: Option<bool>,
    pub exposed_to_industrial_noise: Option<bool>,
    pub exposed_to_extreme_temperatures: Option<bool>,
    pub exposed_to_vibrations_or_radiation: Option<bool>,

    pub uses_biometrics_or_health_data: Option<bool>,
    pub transfers_data_to_third_parties: Option<bool>,

    pub pays_intangible_b2b_services: Option<bool>,
    pub transports_goods_federal_highways: Option<bool>,
    pub performs_vulnerable_activities_pld: Option<bool>,

    /// Datos explícitos de personalización jurídica proporcionados por el cliente (Cero Placeholders)
    #[serde(default)]
    pub dossier: LegalDossierInput,
}

impl CalibrationAnswers {
    pub fn resolve_flag(explicit: Option<bool>, inferred: bool) -> bool {
        explicit.unwrap_or(inferred)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegulatoryAgency {
    STPS,
    SAT,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    CriticalClosure,
}

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
                16..=50 => 1.35,
                51..=100 => 1.75,
                101..=250 => 2.25,
                _ => 3.0,
            }
        } else {
            1.0
        };

        let min_mxn = ((min_uma as f64) * self.uma_daily_mxn * 100.0).round() / 100.0;
        let max_mxn =
            ((max_uma as f64) * self.uma_daily_mxn * worker_multiplier * 100.0).round() / 100.0;
        let estimated_mxn = ((min_mxn + (max_mxn - min_mxn) * 0.18) * 100.0).round() / 100.0;

        (min_mxn, estimated_mxn, max_mxn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAcceptanceMetadata {
    pub accepted_at_utc: String,
    pub client_ip: String,
    pub user_agent: String,
    pub disclaimer_sha256: String,
    pub stripe_session_id: Option<String>,
    pub stripe_payment_intent: Option<String>,
    pub three_d_secure_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
