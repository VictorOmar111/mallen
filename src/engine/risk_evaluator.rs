//! Orquestador de evaluación de riesgo financiero y normativo 360°.
//! Mantiene `evaluate_risk(&mut EmpresaB2B)` 100% compatible y expone `build_full_regulatory_findings`
//! para ensamblar el vector completo de hallazgos de las 6 materias (STPS, SAT, SABG/INAI, IMSS, PROFECO y Sectoriales).

use crate::engine::models::{
    CalibrationAnswers, DEFAULT_UMA_VALUE_MXN, EmpresaB2B, RegulatoryFinding, RiskSeverity,
    UmaCalculator, WebOsintTelemetry,
};
use crate::engine::rules::{imss, inai, profeco, sat_cff, sectorial, stps};

pub fn evaluate_risk(empresa: &mut EmpresaB2B) {
    let valor_uma: f64 = DEFAULT_UMA_VALUE_MXN;

    // Conservar hallazgos previos capturados por el scanner HTTP
    let mut flags_scanner: Vec<String> = empresa
        .audit_flags
        .iter()
        .filter(|f| {
            f.starts_with("SSL_")
                || f.starts_with("LFPDPPP_AVISO_")
                || f.starts_with("PROFECO_ECOMMERCE_")
                || f.starts_with("SITIO_WEB_")
                || f.starts_with("INAI_SIN_AVISO")
        })
        .cloned()
        .collect();

    empresa.audit_flags.clear();

    // 1. Evaluación STPS (39 NOMs)
    let multa_stps = stps::evaluate_stps(empresa, valor_uma);

    // 2. Evaluación SABG / ex-INAI (LFPDPPP)
    let multa_inai = inai::evaluate_inai(empresa, valor_uma);

    // 3. Evaluación PROFECO (LFPC)
    let multa_profeco = profeco::evaluate_profeco(empresa, valor_uma);

    // 4. Evaluación SAT (CFF / LISR / LIVA)
    let multa_sat = sat_cff::evaluate_sat(empresa, valor_uma);

    // 5. Evaluación IMSS / INFONAVIT (LSS / REPSE)
    let multa_imss = imss::evaluate_imss(empresa, valor_uma);

    // 6. Evaluación de los 9 Módulos Sectoriales (COFEPRIS, CNBV, PIPC, SEMARNAT, SICT, etc.)
    let multa_sectorial = sectorial::evaluate_sectorial(empresa, valor_uma);

    let total_multa_mxn =
        multa_stps + multa_inai + multa_profeco + multa_sat + multa_imss + multa_sectorial;

    empresa.audit_flags.append(&mut flags_scanner);
    empresa.estimated_fine_mxn = total_multa_mxn;

    // Score de riesgo calibrado de 0 a 100
    let score = (total_multa_mxn / 2_500_000.0) * 100.0;
    empresa.risk_score = (score.min(100.0)) as u32;

    // Tarificación dinámica B2B
    let precio_calculado = (total_multa_mxn * 0.01) / 18.0;
    empresa.shield_price_usd = precio_calculado.max(165.0);
}

/// Ejecuta la evaluación pericial completa de las 6 materias cruzando SCIAN, Padrones DB,
/// Telemetría OSINT Web y las 12 preguntas del Cuestionario de Certeza Pericial.
#[allow(clippy::too_many_arguments)]
pub fn build_full_regulatory_findings(
    scian: &str,
    company_name: &str,
    employee_count: u32,
    sat_69b_status: &str,
    sat_69_status: &str,
    repse_registered: bool,
    sfp_sanctioned: bool,
    web: &WebOsintTelemetry,
    cal: &CalibrationAnswers,
) -> Vec<RegulatoryFinding> {
    let uma = UmaCalculator::default();
    let mut all_findings = Vec::with_capacity(48);

    // 1. STPS (39 NOMs)
    all_findings.extend(stps::evaluate_stps_detailed(scian, employee_count, cal, &uma));

    // 2. SABG / Transparencia para el Pueblo (ex-INAI) - LFPDPPP
    all_findings.extend(inai::evaluate_inai_detailed(web, cal, &uma));

    // 3. SAT (CFF / LISR / LIVA)
    all_findings.extend(sat_cff::evaluate_sat_detailed(
        scian,
        sat_69b_status,
        sat_69_status,
        web,
        cal,
        &uma,
    ));

    // 4. IMSS e INFONAVIT (LSS / LINFONAVIT / REPSE)
    all_findings.extend(imss::evaluate_imss_detailed(
        scian,
        employee_count,
        repse_registered,
        cal,
        &uma,
    ));

    // 5. PROFECO (LFPC)
    all_findings.extend(profeco::evaluate_profeco_detailed(scian, web, &uma));

    // 6. Los 9 Módulos Sectoriales Especializados
    all_findings.extend(sectorial::evaluate_sectorial_detailed(
        scian,
        company_name,
        sfp_sanctioned,
        cal,
        &uma,
    ));

    all_findings
}

/// Calcula el score global (0-100) y las 3 bandas financieras (Mínima, Estimada, Máxima en MXN)
/// a partir de un vector de `RegulatoryFinding`.
pub fn summarize_findings_financials(
    findings: &[RegulatoryFinding],
) -> (u32, f64, f64, f64) {
    let mut min_total = 0.0;
    let mut est_total = 0.0;
    let mut max_total = 0.0;
    let mut has_critical_closure = false;

    for f in findings {
        min_total += f.min_fine_mxn;
        est_total += f.estimated_fine_mxn;
        max_total += f.max_fine_mxn;
        if f.severity == RiskSeverity::CriticalClosure {
            has_critical_closure = true;
        }
    }

    let mut score = ((est_total / 2_500_000.0) * 100.0).min(99.0) as u32;
    if has_critical_closure && score < 88 {
        score = 88;
    }

    (score, min_total, est_total, max_total)
}
