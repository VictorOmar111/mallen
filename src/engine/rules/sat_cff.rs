//! Motor de Reglas Fiscales SAT (Código Fiscal de la Federación, LISR y LIVA):
//! 1. Art. 69-B CFF: Materialidad y Sustancia Económica B2B (Fecha cierta, bitácoras, entregables y cruce EFOS/EDOS).
//! 2. Arts. 17-K y 86-C/86-D CFF: Buzón Tributario habilitado y medios de contacto actualizados.
//! 3. Arts. 29 y 29-A CFF: Facturación CFDI 4.0, Complemento Carta Porte, Complemento de Pago (REP) y Nómina.
//! 4. Art. 28 CFF: Contabilidad Electrónica (Balanza de comprobación mensual y catálogo de cuentas).
//! 5. Arts. 27, 17-H Bis y 32-D CFF: Domicilio Fiscal "Localizado", Opinión 32-D y riesgo de restricción de CSD.
//! 6. Arts. 113-A a 113-D LISR y Art. 18-J LIVA: Comercio Electrónico y Retenciones en Plataformas Digitales.

use crate::engine::models::{
    CalibrationAnswers, EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity,
    UmaCalculator, WebOsintTelemetry,
};

/// Función original 100% retrocompatible con `EmpresaB2B`.
pub fn evaluate_sat(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let web = WebOsintTelemetry::default();
    let cal = CalibrationAnswers::default();
    let findings = evaluate_sat_detailed(&empresa.actividad, "LIMPIO", "LIMPIO", &web, &cal, &calc);

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("SAT_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de los 6 Vectores del SAT.
pub fn evaluate_sat_detailed(
    scian: &str,
    sat_69b_status: &str,
    sat_69_status: &str,
    web: &WebOsintTelemetry,
    cal: &CalibrationAnswers,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();
    let prefix2 = if scian.len() >= 2 { &scian[..2] } else { "46" };

    let mut push_sat = |code: &str,
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
            agency: RegulatoryAgency::SAT,
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

    // 1A. Alerta Crítica si el propio RFC aparece en Listados 69-B (EFOS) o 69 (Créditos / No Localizado)
    if sat_69b_status != "LIMPIO" && sat_69b_status != "RFC_FORMATO_NO_CANONICO" {
        push_sat(
            "CFF-69B-LISTA-NEGRA-ACTIVA",
            &format!("ALERTA CRÍTICA: RFC listado en el padrón oficial del Art. 69-B CFF ({})", sat_69b_status),
            "Art. 69-B y Art. 83 Fracc. XVIII CFF",
            RiskSeverity::CriticalClosure,
            "El RFC auditado presenta coincidencia exacta en el listado público de presunción o definitividad de operaciones inexistentes del SAT.",
            "Presentar de inmediato pruebas de materialidad, activos, personal e infraestructura dentro del plazo legal o mediante acuerdo conclusivo ante PRODECON.",
            5000,
            25000,
            true,
        );
    }

    // 1B. Art. 69-B CFF: Materialidad, Sustancia Económica y Fecha Cierta (Jurisprudencia 2a./J. 161/2019 SCJN)
    if CalibrationAnswers::resolve_flag(cal.pays_intangible_b2b_services, true) {
        push_sat(
            "CFF-69B-MATERIALIDAD-FECHA-CIERTA",
            "Exposición por falta de expedientes de Materialidad B2B y Fecha Cierta en contratos mercantiles",
            "Art. 69-B CFF / Jurisprudencia 2a./J. 161/2019 (10a.) SCJN",
            RiskSeverity::High,
            "La sola exhibición del CFDI y la transferencia bancaria no acreditan la existencia de servicios intangibles ante una facultad de comprobación del SAT; se exige Fecha Cierta y bitácora de entregables.",
            "Implementar Contratos Marco con Fecha Cierta Criptográfica/NOM-151 y Actas de Recepción de Entregables (incluidos en el Kit Mallen).",
            1500,
            10000,
            false,
        );
    }

    // 2. Arts. 17-K y 86-C / 86-D CFF: Buzón Tributario
    push_sat(
        "CFF-17K-BUZON-TRIBUTARIO",
        "Verificación de habilitación y actualización de medios de contacto en Buzón Tributario",
        "Arts. 17-K, 86-C y 86-D CFF",
        RiskSeverity::Medium,
        "No habilitar el Buzón Tributario o no mantener actualizados al menos dos medios de contacto (correo y teléfono móvil) faculta al SAT a notificar por estrados y aplicar multa directa.",
        "Confirmar vigencia de correo electrónico y número celular en el portal del SAT cada trimestre.",
        30,
        100,
        false,
    );

    // 3. Arts. 29 y 29-A CFF: CFDI 4.0, Complemento Carta Porte, REP y Nómina
    let needs_carta_porte = CalibrationAnswers::resolve_flag(
        cal.transports_goods_federal_highways,
        matches!(prefix2, "31" | "32" | "33" | "43" | "48" | "49"),
    );
    if needs_carta_porte {
        push_sat(
            "CFF-29A-CARTA-PORTE-CFDI40",
            "Riesgo en emisión de CFDI 4.0 con Complemento Carta Porte 3.1 y Complementos de Pago (REP)",
            "Arts. 29, 29-A, 83 Fracc. VII y 84 Fracc. IV y XVI CFF",
            RiskSeverity::High,
            "El traslado de mercancías en tramos federales sin Complemento Carta Porte (o con errores de llenado) conlleva multas por cada CFDI, no deducibilidad del flete y presunción de contrabando (Art. 103 CFF).",
            "Implementar protocolo de validación previa de Carta Porte 3.1 y conciliación mensual de Complementos de Recepción de Pagos (REP antes del día 5 del mes siguiente).",
            400,
            6000,
            false,
        );
    }

    // 4. Art. 28 CFF: Contabilidad Electrónica (Balanza y Catálogo de Cuentas)
    push_sat(
        "CFF-28-CONTABILIDAD-ELECTRONICA",
        "Obligación de envío mensual de Contabilidad Electrónica (Catálogo de cuentas y Balanza de comprobación)",
        "Art. 28 Fracc. III y IV, Art. 81 Fracc. XLI y Art. 82 Fracc. XXXVIII CFF",
        RiskSeverity::Medium,
        "Omisión o envío extemporáneo de archivos XML del catálogo de cuentas con código agrupador del SAT y balanzas de comprobación mensuales.",
        "Automatizar el envío mensual de balanzas de comprobación a través del Buzón Tributario y conciliar contra CFDI emitidos/recibidos.",
        70,
        700,
        false,
    );

    // 5. Arts. 27, 17-H Bis y 32-D CFF: Domicilio Fiscal, Opinión 32-D y Restricción Temporal de CSD
    let dom_severity = if sat_69_status != "LIMPIO" {
        RiskSeverity::CriticalClosure
    } else {
        RiskSeverity::High
    };
    push_sat(
        "CFF-27-17HBIS-CSD-DOMICILIO",
        "Blindaje de Domicilio Fiscal 'Localizado', Opinión 32-D y prevención de restricción del Certificado de Sello Digital (CSD)",
        "Arts. 27, 17-H Bis y 32-D CFF",
        dom_severity,
        "Un estatus de 'No Localizado' o discrepancias entre ingresos declarados y CFDI timbrados provoca la restricción inmediata de los sellos para facturar (Art. 17-H Bis CFF).",
        "Mantener expediente de verificación de domicilio fiscal, monitorear mensualmente la Opinión 32-D y conciliar declaraciones contra visores de nómina e ingresos del SAT.",
        300,
        3000,
        sat_69_status != "LIMPIO",
    );

    // 6. Arts. 113-A a 113-D LISR / Art. 18-J LIVA: Comercio Electrónico y Plataformas Digitales
    if web.has_ecommerce_cart || matches!(prefix2, "46" | "51") {
        push_sat(
            "LISR-113A-LIVA-18J-ECOMMERCE",
            "Obligaciones fiscales en Comercio Electrónico, Pasarelas de Pago y Plataformas Digitales",
            "Arts. 113-A a 113-D LISR y Art. 18-J LIVA",
            RiskSeverity::Medium,
            "Se detectó operación de comercio electrónico o cobro digital; exige desglose correcto de IVA, emisión automatizada de factura al público en general o al cliente y control de retenciones en marketplaces.",
            "Configurar motor de autofacturación CFDI 4.0 vinculado a la pasarela de pagos y conciliar retenciones de plataformas tecnológicas.",
            500,
            4000,
            web.has_ecommerce_cart,
        );
    }

    findings
}
