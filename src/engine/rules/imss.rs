use crate::engine::models::EmpresaB2B;

pub fn evaluate_imss(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let mut multa: f64 = 0.0;
    let scian = &empresa.actividad;

    if scian.starts_with("54") || scian.starts_with("23") || scian.starts_with("31") {
        empresa.audit_flags.push("IMSS_REPSE_CUMPLIMIENTO_SERVICIOS_ESPECIALIZADOS".to_string());
        multa += 3500.0 * valor_uma;
    }

    multa
}
