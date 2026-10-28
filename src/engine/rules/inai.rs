//! Motor de Reglas de Protección de Datos Personales (LFPDPPP vigente 2025/2026 - SABG / Transparencia para el Pueblo, ex-INAI):
//! 1. Arts. 8, 15 y 16 LFPDPPP: Aviso de Privacidad Integral, Simplificado y Corto.
//! 2. Arts. 22 a 35 LFPDPPP: Mecanismos y Procedimiento de Derechos ARCO.
//! 3. Arts. 9 y 16 LFPDPPP: Tratamiento de Datos Personales Sensibles y Biométricos (Consentimiento expreso y escrito).
//! 4. Arts. 19 y 63 LFPDPPP: Medidas de Seguridad Técnicas (SSL/TLS válido, vulnerabilidad en subdominios/servidores).
//! 5. Arts. 36 y 37 LFPDPPP: Cláusulas de Transferencia de Datos a Terceros (Proveedores, maquila de nómina, cloud).
//! 6. Art. 17 LFPDPPP: Designación del Oficial de Privacidad / Departamento de Datos Personales.

use crate::engine::models::{
    CalibrationAnswers, EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity,
    UmaCalculator, WebOsintTelemetry,
};

/// Función original 100% retrocompatible con `EmpresaB2B`.
pub fn evaluate_inai(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let tiene_flag_sin_aviso = empresa
        .audit_flags
        .iter()
        .any(|f| f == "INAI_SIN_AVISO_PRIVACIDAD" || f == "SIN_AVISO_PRIVACIDAD");
    let tiene_flag_ssl_inv = empresa
        .audit_flags
        .iter()
        .any(|f| f == "SSL_INVALIDO" || f == "SIN_SSL_ENCRIPTACION");

    let telemetry = WebOsintTelemetry {
        target_url: empresa.sitio_web.clone().unwrap_or_default(),
        http_status: empresa.status_web.unwrap_or(200),
        ssl_valid: !tiene_flag_ssl_inv,
        has_privacy_notice: empresa.sitio_web.is_some() && !tiene_flag_sin_aviso,
        mentions_arco_rights: false,
        ..Default::default()
    };

    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let findings = evaluate_inai_detailed(&telemetry, &CalibrationAnswers::default(), &calc);

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("INAI_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de los 6 Vectores LFPDPPP (Autoridad Garante Federal SABG / ex-INAI).
pub fn evaluate_inai_detailed(
    web: &WebOsintTelemetry,
    cal: &CalibrationAnswers,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();

    let mut push_inai = |code: &str,
                         title: &str,
                         basis: &str,
                         severity: RiskSeverity,
                         desc: &str,
                         remedy: &str,
                         min_u: u32,
                         max_u: u32,
                         osint: bool| {
        let (min_m, est_m, max_m) = uma.compute_fine_bands(min_u, max_u, 1, false);
        findings.push(RegulatoryFinding {
            code: code.to_string(),
            agency: RegulatoryAgency::SabgInai,
            title: title.to_string(),
            legal_basis: basis.to_string(),
            severity,
            description: desc.to_string(),
            remediation_action: remedy.to_string(),
            min_uma: min_u,
            max_uma: max_u,
            min_fine_mxn: min_m,
            estimated_fine_mxn: est_m,
            max_fine_mxn: max_m,
            verified_by_osint: osint,
        });
    };

    // 1. Arts. 8, 15 y 16 LFPDPPP: Aviso de Privacidad Integral, Simplificado y Corto
    if !web.has_privacy_notice {
        push_inai(
            "LFPDPPP-AVISO-OMISION",
            "Omisión o falta de visibilidad del Aviso de Privacidad Integral y Simplificado",
            "Arts. 8, 15, 16, 63 y 64 Fracc. II LFPDPPP (Autoridad Garante SABG / Transparencia para el Pueblo - ex INAI)",
            RiskSeverity::High,
            "El escaneo OSINT sobre el portal digital no detectó un Aviso de Privacidad Integral accesible previo a la captación de datos.",
            "Publicar de inmediato el Aviso de Privacidad Integral, Simplificado y Corto para clientes, proveedores y empleados (incluido en el Kit Mallen).",
            100,
            2500,
            true,
        );
    }

    // 2. Arts. 22 a 35 LFPDPPP: Mecanismos y Procedimiento de Derechos ARCO
    if !web.mentions_arco_rights {
        push_inai(
            "LFPDPPP-DERECHOS-ARCO",
            "Deficiencia u omisión en los mecanismos para el ejercicio de Derechos ARCO",
            "Arts. 22 al 35 y 63 Fracc. VII LFPDPPP (SABG / ex-INAI)",
            RiskSeverity::Medium,
            "Ausencia de procedimiento claro, plazos legales (20 días hábiles de respuesta y 15 de ejecución) y formulario para solicitudes de Acceso, Rectificación, Cancelación u Oposición.",
            "Incorporar cláusula expresa de procedimiento ARCO y habilitar buzón oficial de privacidad.",
            100,
            2500,
            true,
        );
    }

    // 3. Arts. 9 y 16 LFPDPPP: Tratamiento de Datos Personales Sensibles y Biométricos (Doble multa conforme a ley)
    if CalibrationAnswers::resolve_flag(cal.uses_biometrics_or_health_data, true) {
        push_inai(
            "LFPDPPP-DATOS-SENSIBLES-BIOMETRICOS",
            "Tratamiento de datos sensibles (biométricos de asistencia, salud o financieros/patrimoniales) sin consentimiento expreso y por escrito",
            "Arts. 9, 16 y 64 Fracc. IV LFPDPPP (Las sanciones se incrementan hasta el doble en datos sensibles)",
            RiskSeverity::CriticalClosure,
            "La captación de huella dactilar/reconocimiento facial en relojes checadores, exámenes médicos o cuentas bancarias exige firma autógrafa o firma electrónica expresa del titular.",
            "Implementar formato de Consentimiento Expreso y por Escrito para Datos Sensibles y Biométricos (incluido en el Kit Mallen).",
            200,
            4500,
            false,
        );
    }

    // 4. Arts. 19 y 63 LFPDPPP: Medidas de Seguridad Técnicas, Físicas y Administrativas (SSL/TLS y Subdominios)
    if !web.ssl_valid {
        let reason = web
            .ssl_error_reason
            .as_deref()
            .unwrap_or("Certificado TLS vencido, autofirmado o subdominios expuestos sin cifrado");
        push_inai(
            "LFPDPPP-SEGURIDAD-TECNICA-SSL",
            "Vulnerabilidad en medidas de seguridad técnicas para la protección de datos personales en tránsito",
            "Arts. 19, 63 Fracc. VIII y 64 Fracc. III LFPDPPP / Art. 57 Reglamento LFPDPPP",
            RiskSeverity::High,
            &format!("Hallazgo técnico OSINT: {}. Constituye infracción directa por falta de medidas técnicas de encriptación.", reason),
            "Renovar certificados TLS/SSL con HSTS activo, clausurar subdominios huérfanos y documentar el Documento de Seguridad Interno.",
            200,
            4500,
            true,
        );
    }

    // 5. Arts. 36 y 37 LFPDPPP: Cláusulas de Transferencia de Datos a Terceros
    if CalibrationAnswers::resolve_flag(cal.transfers_data_to_third_parties, true) {
        push_inai(
            "LFPDPPP-TRANSFERENCIA-TERCEROS",
            "Transferencia de datos personales a terceros sin cláusula de aceptación ni convenios de confidencialidad",
            "Arts. 36, 37 y 63 Fracc. XII LFPDPPP",
            RiskSeverity::Medium,
            "La remisión o transferencia de bases de datos a despachos contables, maquiladoras de nómina, CRMs o servicios en la nube requiere cláusulas contractuales de encargado.",
            "Suscribir Convenios de Encargado y Confidencialidad B2B con proveedores que traten datos por cuenta del responsable.",
            200,
            4500,
            false,
        );
    }

    // 6. Art. 17 y 30 LFPDPPP: Designación del Oficial de Privacidad / Departamento de Datos Personales
    push_inai(
        "LFPDPPP-OFICIAL-PRIVACIDAD",
        "Falta de designación formal del Oficial de Privacidad o Departamento de Datos Personales",
        "Arts. 17 y 30 LFPDPPP",
        RiskSeverity::Low,
        "Todo responsable está obligado a designar formalmente a una persona o departamento de datos personales encargado de dar trámite a las solicitudes de los titulares y fomentar la protección dentro de la organización.",
        "Emitir Acta de Designación interna del Oficial de Privacidad (incluida en el Kit Mallen).",
        100,
        1200,
        false,
    );

    findings
}
