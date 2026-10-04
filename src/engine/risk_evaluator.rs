use crate::engine::models::EmpresaB2B;

pub fn evaluate_risk(empresa: &mut EmpresaB2B) {
    let mut flags = Vec::new();
    let mut total_multa_mxn: f64 = 0.0;
    let valor_uma: f64 = 108.57; // Valor UMA diario de referencia

    let scian = &empresa.actividad;
    let es_manufactura = scian.starts_with("31") || scian.starts_with("32") || scian.starts_with("33");
    let es_servicio_profesional = scian.starts_with("54");
    let personal = &empresa.direccion;

    // Evaluaciones normativas (STPS)
    if es_manufactura || personal.contains("51 a") || personal.contains("101 a") {
        flags.push("STPS_NOM_019_OBLIGATORIA".to_string());
        flags.push("STPS_NOM_030_SALUD".to_string());
        total_multa_mxn += 2500.0 * valor_uma;
    }

    if personal.contains("11 a") || personal.contains("30") || personal.contains("51") {
        flags.push("STPS_NOM_035_PSICOSOCIAL".to_string());
        total_multa_mxn += 1000.0 * valor_uma;
    }

    // Evaluaciones normativas (SAT)
    if es_servicio_profesional {
        flags.push("SAT_ART69B_MATERIALIDAD".to_string());
        total_multa_mxn += 3000.0 * valor_uma;
    }

    // Evaluaciones de Presencia Digital e INAI
    if empresa.sitio_web.is_empty() || !empresa.sitio_web.contains("http") {
        flags.push("SIN_PRESENCIA_DIGITAL_INSPECCION".to_string());
        total_multa_mxn += 500.0 * valor_uma;
    } else {
        flags.push("LFPDPPP_REVISION_AVISO_PRIVACIDAD".to_string());
        total_multa_mxn += 1000.0 * valor_uma;
    }

    empresa.audit_flags = flags;
    empresa.estimated_fine_mxn = total_multa_mxn;

    // Risk score de 0 a 100%
    let base_score = (total_multa_mxn / 500000.0) * 100.0;
    empresa.risk_score = base_score.min(100.0) as u32;

    // Precio dinámico: 1% del riesgo en USD o Piso de $150 USD
    let calculated_price_usd = (total_multa_mxn * 0.01) / 18.0;
    empresa.shield_price_usd = calculated_price_usd.max(150.0);
}

pub fn calculate_dynamic_pricing(empresa: &mut EmpresaB2B) {
    // Redirecciona al evaluador de precisión
    evaluate_risk(empresa);
}
