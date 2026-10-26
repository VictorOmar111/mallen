//! Motor de Reglas de Seguridad Social y Vivienda (IMSS - LSS / INFONAVIT - LINFONAVIT / LFT):
//! 1. Arts. 15 LSS y 29 LINFONAVIT: Afiliación, Altas, Bajas y Salario Base de Cotización (SBC) real vs DENUE.
//! 2. Art. 15-A LSS / Art. 13 LFT: Subcontratación REPSE e informativas cuatrimestrales ICSOE (IMSS) y SISSUB (INFONAVIT).
//! 3. Arts. 71 al 75 LSS: Clasificación en Prima de Riesgo de Trabajo y revisión anual de Siniestralidad (Febrero).
//! 4. Art. 39 LSS y 32-D CFF: Opinión de Cumplimiento de Seguridad Social Positiva y conciliación EMA/EBA vs SUA.
//! 5. Art. 15-B LSS: Responsabilidad Solidaria e integración de expedientes B2B de contratistas especializados.
//! 6. Arts. 304-A y 304-B LSS: Infracciones y sanciones directas por omisión o extemporaneidad de avisos e informes.

use crate::engine::models::{
    CalibrationAnswers, EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity,
    UmaCalculator,
};
use crate::engine::rules::stps::parse_employee_count;

/// Función original 100% retrocompatible con `EmpresaB2B`.
pub fn evaluate_imss(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let emp_count = parse_employee_count(&empresa.direccion, None);
    let findings = evaluate_imss_detailed(
        &empresa.actividad,
        emp_count,
        false,
        &CalibrationAnswers::default(),
        &calc,
    );

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("IMSS_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de los 6 Vectores de IMSS e INFONAVIT.
pub fn evaluate_imss_detailed(
    scian: &str,
    employee_count: u32,
    repse_registered_in_db: bool,
    cal: &CalibrationAnswers,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();
    let prefix2 = if scian.len() >= 2 { &scian[..2] } else { "46" };

    let mut push_imss = |code: &str,
                         title: &str,
                         basis: &str,
                         severity: RiskSeverity,
                         desc: &str,
                         remedy: &str,
                         min_u: u32,
                         max_u: u32,
                         scale_workers: bool,
                         osint: bool| {
        let (min_m, est_m, max_m) =
            uma.compute_fine_bands(min_u, max_u, employee_count, scale_workers);
        findings.push(RegulatoryFinding {
            code: code.to_string(),
            agency: RegulatoryAgency::IMSSInfonavit,
            title: title.to_string(),
            legal_basis: basis.to_string(),
            severity,
            description: desc.to_string(),
            remediation_action: remedy.to_string(),
            min_uma: min_u,
            max_uma: max_u,
            min_fine_mxn: min_m,
            estimated_fine_mxn: est_m,
            max_fine_mxn: max_m,
            verified_by_osint: osint,
        });
    };

    // 1. Arts. 15 LSS y 29 LINFONAVIT: Afiliación, Altas/Bajas e Integración Real del SBC vs Estrato DENUE
    push_imss(
        "LSS-15-LINFONAVIT-29-SBC-DENUE",
        "Auditoría de conciliación entre plantilla visible en INEGI DENUE, CFDI de Nómina y Salario Base de Cotización (SBC)",
        "Art. 15 Fraccs. I a III, Art. 27, Art. 304-A Fraccs. I a IV y 304-B Fracc. IV LSS / Art. 29 LINFONAVIT",
        RiskSeverity::High,
        &format!(
            "El registro público estima una plantilla de ~{} colaboradores. El IMSS cruza automáticamente los CFDI de nómina (bonos, comisiones, previsión social) contra las cédulas SUA para fincar capitales constitutivos por subintegración de SBC.",
            employee_count
        ),
        "Ejecutar conciliación mensual entre Visor de Nómina del SAT, SUA e IDSE para prevenir créditos fiscales por diferencias en integración salarial.",
        20,
        350,
        true,
        true,
    );

    // 2. Art. 15-A LSS / Art. 13 LFT: Reforma de Subcontratación REPSE, ICSOE (IMSS) y SISSUB (INFONAVIT)
    let uses_repse = CalibrationAnswers::resolve_flag(
        cal.uses_subcontracted_services_repse,
        matches!(prefix2, "23" | "31" | "32" | "33" | "48" | "54" | "56"),
    );
    if uses_repse || repse_registered_in_db {
        push_imss(
            "LSS-15A-REPSE-ICSOE-SISSUB",
            "Obligación de presentar informativas cuatrimestrales ICSOE (IMSS) y SISSUB (INFONAVIT) por Servicios Especializados (REPSE)",
            "Arts. 13 a 15 LFT / Art. 15-A, Art. 304-A Fracc. XXII y Art. 304-B Fracc. V LSS / Art. 29 Bis LINFONAVIT",
            RiskSeverity::CriticalClosure,
            "La omisión o presentación extemporánea de los reportes cuatrimestrales ICSOE y SISSUB (enero, mayo y septiembre) genera multas directas de 500 a 2,000 UMAs y causal de cancelación del padrón REPSE.",
            "Presentar informativas cuatrimestrales (incluso 'Sin Información') antes del día 17 de enero, mayo y septiembre en portales ICSOE y SISSUB.",
            500,
            2000,
            false,
            repse_registered_in_db,
        );
    }

    // 3. Arts. 71 al 75 LSS: Clasificación de Prima de Riesgo de Trabajo y Declaración Anual en Febrero
    push_imss(
        "LSS-71-75-PRIMA-RIESGO-TRABAJO",
        "Verificación de Autoclasificación de Riesgo de Trabajo (Clases I a V) y Determinación Anual de Siniestralidad",
        "Arts. 71, 72, 73, 74, 75, 304-A Fracc. XV y 304-B Fracc. III LSS / Art. 196 RACERF",
        RiskSeverity::Medium,
        "Una clasificación errónea respecto a la actividad real del SCIAN o no presentar en febrero la Determinación Anual de la Prima en el Seguro de Riesgos de Trabajo faculta al IMSS a rectificar la clase al máximo legal con recargos.",
        "Validar correspondencia entre actividad SCIAN y fracción del Catálogo del Art. 196 RACERF, y presentar declaración anual de prima en febrero.",
        20,
        210,
        false,
        false,
    );

    // 4. Art. 39 LSS y 32-D CFF: Opinión de Cumplimiento de Seguridad Social Positiva y diferencias EMA/EBA
    push_imss(
        "LSS-39-OPINION-32D-EMA-EBA",
        "Vigilancia de Opinión de Cumplimiento 32-D en materia de Seguridad Social y conciliación de emisiones EMA y EBA",
        "Arts. 39, 39-C LSS y Art. 32-D CFF",
        RiskSeverity::High,
        "La existencia de diferencias no aclaradas entre la Emisión Mensual Anticipada (EMA), Emisión Bimestral (EBA) y el pago SUA cambia la Opinión 32-D a Negativa, bloqueando contrataciones B2B y licitaciones.",
        "Habilitar Buzón IMSS y verificar mensualmente la Opinión de Cumplimiento 32-D del IMSS y Constancia de Situación Fiscal del INFONAVIT.",
        50,
        350,
        false,
        false,
    );

    // 5. Art. 15-B LSS / Art. 27 Fracc. V LISR: Responsabilidad Solidaria en contratación de terceros
    if uses_repse {
        push_imss(
            "LSS-15B-RESPONSABILIDAD-SOLIDARIA-B2B",
            "Riesgo de Responsabilidad Solidaria Patronal e improcedencia de deducción fiscal por proveedores de servicios en sitio",
            "Art. 14 LFT / Art. 15-A párrafo tercero LSS / Art. 27 Fracc. V LISR y Art. 5 Fracc. II LIVA",
            RiskSeverity::High,
            "Si un proveedor de limpieza, mantenimiento, seguridad o construcción incumple el pago de cuotas IMSS/INFONAVIT de su personal, el contratante responde solidariamente al 100% y pierde la deducibilidad de las facturas.",
            "Exigir mensualmente a cada contratista su expediente de materialidad REPSE: CFDI de nómina, acuses de pago SUA, declaraciones de IVA/ISR y acuses ICSOE/SISSUB.",
            350,
            2000,
            false,
            false,
        );
    }

    // 6. Art. 304-A y 304-B LSS: Listado general de infracciones por avisos afiliatorios y controles de asistencia
    push_imss(
        "LSS-304A-CONTROLES-NOMINA-ASISTENCIA",
        "Conservación de listas de raya, controles de asistencia y presentación oportuna de movimientos afiliatorios (5 días hábiles)",
        "Art. 15 Fracc. II y Art. 304-A Fraccs. II, VI y IX LSS",
        RiskSeverity::Medium,
        "No conservar nóminas y listas de asistencia durante los 5 años de caducidad fiscal o presentar avisos de alta/modificación salarial fuera del plazo de 5 días hábiles detona multas y cobro de capitales constitutivos (Art. 77 LSS) en caso de accidente.",
        "Automatizar el timbrado de recibos de nómina y presentar movimientos en IDSE previo al ingreso físico del trabajador.",
        20,
        210,
        false,
        false,
    );

    findings
}
