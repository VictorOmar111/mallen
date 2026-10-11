use crate::engine::models::EmpresaB2B;

pub fn evaluate_sectorial(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let actividad_lower = empresa.actividad.to_lowercase();
    let mut multa_sectorial: f64 = 0.0;

    // 1. Sector Financiero y Fintech (CNBV / CONDUSEF / LFPIORPI Anti-Lavado)
    if actividad_lower.contains("financiera") || actividad_lower.contains("banco") || actividad_lower.contains("credito") || actividad_lower.contains("inversion") {
        multa_sectorial += 5000.0 * valor_uma;
        empresa.audit_flags.push("REG_FINANCIERO_CNBV_PENDIENTE".to_string());
    } 
    // 2. Sector Salud, Alimentos y Hospitalidad (COFEPRIS)
    else if actividad_lower.contains("restaurante") || actividad_lower.contains("alimentos") || actividad_lower.contains("hospital") || actividad_lower.contains("farmacia") || actividad_lower.contains("medico") {
        multa_sectorial += 2500.0 * valor_uma;
        empresa.audit_flags.push("COFEPRIS_AVISO_FUNCIONAMIENTO_REQUERIDO".to_string());
    } 
    // 3. Industria Manufacturera, Química e Hidrocarburos (SEMARNAT / ASEA / STPS Pesada)
    else if actividad_lower.contains("fabrica") || actividad_lower.contains("manufactura") || actividad_lower.contains("industrial") || actividad_lower.contains("quimica") || actividad_lower.contains("metal") {
        multa_sectorial += 4000.0 * valor_uma;
        empresa.audit_flags.push("SEMARNAT_ASEA_IMPACTO_AMBIENTAL".to_string());
    } 
    // 4. Comercio Electrónico y Retail B2C (PROFECO / LFPC)
    else if actividad_lower.contains("comercio") || actividad_lower.contains("tienda") || actividad_lower.contains("electrodomesticos") || actividad_lower.contains("ropa") {
        multa_sectorial += 1500.0 * valor_uma;
        empresa.audit_flags.push("PROFECO_CONTRATO_ADHESION_PENDIENTE".to_string());
    } 
    // 5. Servicios Generales y Profesionales (Marco Estándar)
    else {
        multa_sectorial += 800.0 * valor_uma;
        empresa.audit_flags.push("REG_SECTORIAL_ESTANDAR".to_string());
    }

    multa_sectorial
}
