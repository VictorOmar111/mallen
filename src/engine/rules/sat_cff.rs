use crate::engine::models::EmpresaB2B;

pub fn evaluate_sat(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let mut multa: f64 = 0.0;
    let scian = &empresa.actividad;

    if scian.starts_with("54") || scian.starts_with("72") || scian.starts_with("46") {
        empresa.audit_flags.push("SAT_ART69B_MATERIALIDAD_ENTREGABLES".to_string());
        multa += 4000.0 * valor_uma;
    }

    empresa.audit_flags.push("SAT_BUZON_TRIBUTARIO_Y_DOMICILIO_FISCAL".to_string());
    multa += 1000.0 * valor_uma;

    multa
}
