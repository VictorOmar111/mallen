use crate::engine::models::EmpresaB2B;

pub fn evaluate_profeco(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let mut multa: f64 = 0.0;
    let scian = &empresa.actividad;
    let es_comercio = scian.starts_with("46") || scian.starts_with("72") || scian.starts_with("54");

    let tiene_ecommerce = empresa.audit_flags.iter().any(|f| f == "PROFECO_ECOMMERCE_DETECTADO");

    if es_comercio || tiene_ecommerce {
        empresa.audit_flags.push("PROFECO_ART76_BIS_LFPC_PRECIOS_Y_TERMINOS".to_string());
        multa += 1500.0 * valor_uma;

        if tiene_ecommerce {
            empresa.audit_flags.push("PROFECO_POLITICAS_DEVOLUCION_Y_ENVIO".to_string());
            multa += 2000.0 * valor_uma;
        }
    }

    multa
}
