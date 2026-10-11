use crate::engine::models::EmpresaB2B;
use crate::engine::rules::{stps, inai, profeco, sat_cff, imss, sectorial};

pub fn evaluate_risk(empresa: &mut EmpresaB2B) {
    let valor_uma: f64 = 108.57; // Valor UMA vigente

    // Limpiar banderas previas pero conservar hallazgos del Deep Scanner HTTP
    let mut flags_scanner: Vec<String> = empresa.audit_flags.iter()
        .filter(|f| f.starts_with("SSL_") || f.starts_with("LFPDPPP_AVISO_") || f.starts_with("PROFECO_ECOMMERCE_") || f.starts_with("SITIO_WEB_") || f.starts_with("INAI_SIN_AVISO"))
        .cloned()
        .collect();

    empresa.audit_flags.clear();

    // 1. Evaluación STPS (NOMs)
    let multa_stps = stps::evaluate_stps(empresa, valor_uma);
    
    // 2. Evaluación INAI (LFPDPPP)
    let multa_inai = inai::evaluate_inai(empresa, valor_uma);

    // 3. Evaluación PROFECO (LFPC)
    let multa_profeco = profeco::evaluate_profeco(empresa, valor_uma);

    // 4. Evaluación SAT (CFF 69-B)
    let multa_sat = sat_cff::evaluate_sat(empresa, valor_uma);

    // 5. Evaluación IMSS (REPSE)
    let multa_imss = imss::evaluate_imss(empresa, valor_uma);

    // 6. Evaluación Sectorial (COFEPRIS / Protección Civil / SCIAN)
    let multa_sectorial = sectorial::evaluate_sectorial(empresa, valor_uma);

    let total_multa_mxn = multa_stps + multa_inai + multa_profeco + multa_sat + multa_imss + multa_sectorial;

    // Reincorporar hallazgos del Deep Scanner
    empresa.audit_flags.append(&mut flags_scanner);
    empresa.estimated_fine_mxn = total_multa_mxn;

    // Score de riesgo calibrado de 0 a 100
    let score = (total_multa_mxn / 1_500_000.0) * 100.0;
    empresa.risk_score = (score.min(100.0)) as u32;

    // Tarificación dinámica: 1% del riesgo expuesto en USD o piso de $140 USD
    let precio_calculado = (total_multa_mxn * 0.01) / 18.0;
    empresa.shield_price_usd = precio_calculado.max(140.0);
}
