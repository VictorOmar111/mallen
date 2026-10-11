use crate::engine::models::EmpresaB2B;

pub fn evaluate_inai(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let mut multa: f64 = 0.0;

    let tiene_flag_sin_aviso = empresa.audit_flags.iter().any(|f| f == "INAI_SIN_AVISO_PRIVACIDAD" || f == "SIN_AVISO_PRIVACIDAD");
    let tiene_flag_ssl_inv = empresa.audit_flags.iter().any(|f| f == "SSL_INVALIDO" || f == "SIN_SSL_ENCRIPTACION");

    if empresa.sitio_web.is_none() || tiene_flag_sin_aviso {
        empresa.audit_flags.push("INAI_LFPDPPP_OMISION_AVISO_PRIVACIDAD".to_string());
        multa += 3000.0 * valor_uma;
    } else {
        empresa.audit_flags.push("INAI_LFPDPPP_REVISION_CLAUSULAS_ARCO".to_string());
        multa += 1000.0 * valor_uma;
    }

    if tiene_flag_ssl_inv {
        empresa.audit_flags.push("INAI_LFPDPPP_MEDIDAS_SEGURIDAD_TECNICAS_DEFICIENTES".to_string());
        multa += 5000.0 * valor_uma;
    }

    multa
}
