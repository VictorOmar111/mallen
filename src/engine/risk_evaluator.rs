//! Orquestador de evaluación de riesgo financiero y normativo 360°.
//! Calcula el score global, las bandas financieras y el desglose exacto por órgano regulador
//! (`AgencyFineBreakdown`) para que la página web y el PDF coincidan al centavo.

use crate::engine::models::{
    AgencyFineBreakdown, CalibrationAnswers, DEFAULT_UMA_VALUE_MXN, EmpresaB2B, RegulatoryAgency,
    RegulatoryFinding, RiskSeverity, UmaCalculator, WebOsintTelemetry,
};
use crate::engine::rules::{imss, inai, profeco, sat_cff, sectorial, stps};

pub fn evaluate_risk(empresa: &mut EmpresaB2B) {
    let valor_uma: f64 = DEFAULT_UMA_VALUE_MXN;

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

    let multa_stps = stps::evaluate_stps(empresa, valor_uma);
    let multa_inai = inai::evaluate_inai(empresa, valor_uma);
    let multa_profeco = profeco::evaluate_profeco(empresa, valor_uma);
    let multa_sat = sat_cff::evaluate_sat(empresa, valor_uma);
    let multa_imss = imss::evaluate_imss(empresa, valor_uma);
    let multa_sectorial = sectorial::evaluate_sectorial(empresa, valor_uma);

    let total_multa_mxn =
        ((multa_stps + multa_inai + multa_profeco + multa_sat + multa_imss + multa_sectorial)
            * 100.0)
            .round()
            / 100.0;

    empresa.audit_flags.append(&mut flags_scanner);
    empresa.estimated_fine_mxn = total_multa_mxn;

    let score = (total_multa_mxn / 2_500_000.0) * 100.0;
    empresa.risk_score = (score.min(99.0)) as u32;

    let precio_calculado = (total_multa_mxn * 0.01) / 18.0;
    empresa.shield_price_usd = ((precio_calculado.max(165.0)) * 100.0).round() / 100.0;
}

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

    all_findings.extend(stps::evaluate_stps_detailed(scian, employee_count, cal, &uma));
    all_findings.extend(inai::evaluate_inai_detailed(web, cal, &uma));
    all_findings.extend(sat_cff::evaluate_sat_detailed(
        scian,
        sat_69b_status,
        sat_69_status,
        web,
        cal,
        &uma,
    ));
    all_findings.extend(imss::evaluate_imss_detailed(
        scian,
        employee_count,
        repse_registered,
        cal,
        &uma,
    ));
    all_findings.extend(profeco::evaluate_profeco_detailed(scian, web, &uma));
    all_findings.extend(sectorial::evaluate_sectorial_detailed(
        scian,
        company_name,
        sfp_sanctioned,
        cal,
        &uma,
    ));

    all_findings
}

pub fn summarize_findings_financials(findings: &[RegulatoryFinding]) -> (u32, f64, f64, f64) {
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

    min_total = (min_total * 100.0).round() / 100.0;
    est_total = (est_total * 100.0).round() / 100.0;
    max_total = (max_total * 100.0).round() / 100.0;

    let mut score = ((est_total / 2_500_000.0) * 100.0).min(99.0) as u32;
    if has_critical_closure && score < 88 {
        score = 88;
    }

    (score, min_total, est_total, max_total)
}

/// Calcula los subtotales exactos por cada una de las 6 dependencias y sus puntos proporcionales
/// para que la tabla de la Sección 3 del Dictamen PDF sume exactamente el total mostrado en la Web.
pub fn compute_agency_breakdown(
    findings: &[RegulatoryFinding],
    total_score: u32,
) -> AgencyFineBreakdown {
    let mut b = AgencyFineBreakdown::default();

    for f in findings {
        match f.agency {
            RegulatoryAgency::STPS => b.stps_fine_mxn += f.estimated_fine_mxn,
            RegulatoryAgency::SAT => b.sat_fine_mxn += f.estimated_fine_mxn,
            RegulatoryAgency::SabgInai => b.sabg_inai_fine_mxn += f.estimated_fine_mxn,
            RegulatoryAgency::IMSSInfonavit => b.imss_fine_mxn += f.estimated_fine_mxn,
            RegulatoryAgency::PROFECO => b.profeco_fine_mxn += f.estimated_fine_mxn,
            _ => b.sectorial_pc_fine_mxn += f.estimated_fine_mxn,
        }
    }

    b.stps_fine_mxn = (b.stps_fine_mxn * 100.0).round() / 100.0;
    b.sat_fine_mxn = (b.sat_fine_mxn * 100.0).round() / 100.0;
    b.sabg_inai_fine_mxn = (b.sabg_inai_fine_mxn * 100.0).round() / 100.0;
    b.imss_fine_mxn = (b.imss_fine_mxn * 100.0).round() / 100.0;
    b.sectorial_pc_fine_mxn = (b.sectorial_pc_fine_mxn * 100.0).round() / 100.0;
    b.profeco_fine_mxn = (b.profeco_fine_mxn * 100.0).round() / 100.0;

    let grand_total = (b.stps_fine_mxn
        + b.sat_fine_mxn
        + b.sabg_inai_fine_mxn
        + b.imss_fine_mxn
        + b.sectorial_pc_fine_mxn
        + b.profeco_fine_mxn)
        .max(1.0);

    let s = total_score as f64;
    b.stps_points = ((b.stps_fine_mxn / grand_total) * s).round() as u32;
    b.sat_points = ((b.sat_fine_mxn / grand_total) * s).round() as u32;
    b.sabg_inai_points = ((b.sabg_inai_fine_mxn / grand_total) * s).round() as u32;
    b.imss_points = ((b.imss_fine_mxn / grand_total) * s).round() as u32;
    b.sectorial_pc_points = ((b.sectorial_pc_fine_mxn / grand_total) * s).round() as u32;
    b.profeco_points = total_score.saturating_sub(
        b.stps_points + b.sat_points + b.sabg_inai_points + b.imss_points + b.sectorial_pc_points,
    );

    b
}
