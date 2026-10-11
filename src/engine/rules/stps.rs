use crate::engine::models::EmpresaB2B;

pub fn evaluate_stps(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let mut multa: f64 = 0.0;
    let scian = &empresa.actividad;
    let personal = &empresa.direccion;

    let es_manufactura = scian.starts_with("31") || scian.starts_with("32") || scian.starts_with("33");
    let es_construccion = scian.starts_with("23");
    let es_comercio_servicio = scian.starts_with("46") || scian.starts_with("54") || scian.starts_with("72");

    let tiene_mas_10 = personal.contains("11 a") || personal.contains("31 a") || personal.contains("51 a") || personal.contains("101 a") || personal.contains("251");
    let _tiene_mas_50 = personal.contains("51 a") || personal.contains("101 a") || personal.contains("251");

    // NOM-001-STPS-2008: Edificios, locales e instalaciones
    empresa.audit_flags.push("STPS_NOM_001_EDIFICIOS".to_string());
    multa += 500.0 * valor_uma;

    // NOM-002-STPS-2010: Prevención y protección contra incendios
    empresa.audit_flags.push("STPS_NOM_002_INCENDIOS".to_string());
    multa += 1000.0 * valor_uma;

    // NOM-017-STPS-2008: Equipo de Protección Personal (EPP)
    if es_manufactura || es_construccion {
        empresa.audit_flags.push("STPS_NOM_017_EPP".to_string());
        multa += 2000.0 * valor_uma;
    }

    // NOM-019-STPS-2011: Comisiones Mixtas de Seguridad e Higiene
    empresa.audit_flags.push("STPS_NOM_019_COMISIONES".to_string());
    multa += 1500.0 * valor_uma;

    // NOM-030-STPS-2009: Servicios preventivos de seguridad y salud
    empresa.audit_flags.push("STPS_NOM_030_SALUD_TRABAJO".to_string());
    multa += 1000.0 * valor_uma;

    // NOM-035-STPS-2018: Factores de Riesgo Psicosocial
    if tiene_mas_10 {
        empresa.audit_flags.push("STPS_NOM_035_PSICOSOCIAL".to_string());
        multa += 2500.0 * valor_uma;
    }

    // NOM-036-1-STPS-2018: Factores de Riesgo Ergonómico (Cargas)
    if es_manufactura || es_construccion || es_comercio_servicio {
        empresa.audit_flags.push("STPS_NOM_036_ERGONOMIA".to_string());
        multa += 1500.0 * valor_uma;
    }

    // NOM-037-STPS-2023: Teletrabajo (Home Office)
    if es_comercio_servicio {
        empresa.audit_flags.push("STPS_NOM_037_TELETRABAJO".to_string());
        multa += 1000.0 * valor_uma;
    }

    multa
}
