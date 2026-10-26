//! Motor de Reglas de Protección al Consumidor y Comercio Electrónico (PROFECO - LFPC):
//! 1. Art. 76 BIS LFPC: Comercio Electrónico y portales digitales (Datos de contacto físicos, características y precios totales).
//! 2. Art. 76 BIS Fraccs. III y VII y Art. 77 LFPC: Políticas de Devolución, Cancelación, Reembolsos, Garantías y Reclamaciones.
//! 3. Arts. 85, 86 y 87 LFPC: Contratos de Adhesión obligatorios y registro público ante el RRPAC de PROFECO.
//! 4. Arts. 32 y 128 LFPC: Publicidad Veraz y prohibición de prácticas engañosas o abusivas.
//! 5. Art. 128 BIS LFPC: Seguridad en transacciones financieras y manejo de pagos en línea.

use crate::engine::models::{
    EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity, UmaCalculator, WebOsintTelemetry,
};

/// Función original 100% retrocompatible con `EmpresaB2B`.
pub fn evaluate_profeco(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let tiene_ecommerce = empresa
        .audit_flags
        .iter()
        .any(|f| f == "PROFECO_ECOMMERCE_DETECTADO");

    let web = WebOsintTelemetry {
        target_url: empresa.sitio_web.clone().unwrap_or_default(),
        http_status: 200,
        ssl_valid: true,
        has_ecommerce_cart: tiene_ecommerce,
        has_profeco_rrpac_number: false,
        ..Default::default()
    };

    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let findings = evaluate_profeco_detailed(&empresa.actividad, &web, &calc);

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("PROFECO_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de los 5 Vectores de la Ley Federal de Protección al Consumidor (LFPC).
/// Aplica la regla de "Prioridad de Hallazgo Digital": si el crawler detecta carrito/checkout en la web,
/// activa todo el marco de comercio electrónico aunque el DENUE indique solo tienda física.
pub fn evaluate_profeco_detailed(
    scian: &str,
    web: &WebOsintTelemetry,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();
    let prefix2 = if scian.len() >= 2 { &scian[..2] } else { "46" };

    // Prioridad de Hallazgo Digital (Hoja 1, Punto 3)
    let is_commercial_or_services =
        matches!(prefix2, "43" | "46" | "48" | "51" | "52" | "53" | "54" | "61" | "62" | "71" | "72" | "81");
    let operates_digital_commerce = web.has_ecommerce_cart || !web.target_url.is_empty();

    if !is_commercial_or_services && !operates_digital_commerce {
        return findings;
    }

    let mut push_profeco = |code: &str,
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
            agency: RegulatoryAgency::PROFECO,
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

    // 1. Art. 76 BIS y Art. 7 BIS LFPC: Transparencia en Comercio Electrónico, Domicilio Físico y Precios Totales
    if operates_digital_commerce {
        push_profeco(
            "LFPC-76BIS-ECOMMERCE-TRANSPARENCIA",
            "Obligaciones de transparencia en portales digitales: domicilio físico, teléfono y exhibición de monto total a pagar",
            "Arts. 7 BIS, 76 BIS Fraccs. IV y V, y Art. 127 LFPC / NMX-COE-001-SCFI-2018",
            RiskSeverity::High,
            "Todo portal que ofrezca bienes o servicios por medios electrónicos debe exhibir su razón social, domicilio físico en territorio nacional, números telefónicos y precios en moneda nacional con IVA, costos de envío y comisiones desglosadas antes del pago.",
            "Publicar Términos y Condiciones de Comercio Electrónico conforme al Art. 76 BIS LFPC y la norma NMX-COE-001-SCFI-2018.",
            250,
            35000,
            true,
        );
    }

    // 2. Art. 76 BIS Fraccs. III y VII, Arts. 56 y 77 LFPC: Políticas de Devolución, Cancelación, Garantías y Reclamaciones
    if web.has_ecommerce_cart || matches!(prefix2, "46" | "72") {
        push_profeco(
            "LFPC-76BIS-DEVOLUCIONES-GARANTIAS",
            "Mecanismos de cancelación, reembolso, derecho de revocación (5 días hábiles) y pólizas de garantía (mínimo 90 días)",
            "Arts. 56, 76 BIS Fraccs. III y VII, 77, 78 y 128 LFPC",
            RiskSeverity::High,
            "La ausencia de políticas claras de devolución, garantías menores al mínimo legal de 90 días o restricciones para revocar compras a distancia expone a quejas en Concilianet y multas directas.",
            "Incorporar Política de Garantías, Devoluciones y Derecho de Revocación de 5 días hábiles visible en el flujo de compra.",
            300,
            35000,
            web.has_ecommerce_cart,
        );
    }

    // 3. Arts. 85, 86 y 87 LFPC: Contratos de Adhesión y Registro Público (RRPAC)
    if !web.has_profeco_rrpac_number {
        push_profeco(
            "LFPC-85-87-CONTRATO-ADHESION-RRPAC",
            "Verificación de Contrato de Adhesión y registro obligatorio ante el RRPAC de PROFECO",
            "Arts. 85, 86, 87, 90 y 128 LFPC",
            RiskSeverity::Medium,
            "El escaneo no identificó un número de registro vigente ante el Registro Público de Contratos de Adhesión (RRPAC) ni cláusulas libres de prestaciones desproporcionadas o inequitativas (Art. 90 LFPC).",
            "Adecuar el contrato de prestación de servicios o términos comerciales al modelo tipo de PROFECO e inscribirlo en el RRPAC.",
            250,
            25000,
            true,
        );
    }

    // 4. Arts. 32 y 128 LFPC: Publicidad Veraz y Prohibición de Prácticas Engañosas
    push_profeco(
        "LFPC-32-128-PUBLICIDAD-VERAZ",
        "Auditoría de información comercial y publicidad veraz, comprobable y exenta de descripciones engañosas",
        "Arts. 32, 35, 128 y 128 TER LFPC",
        RiskSeverity::Medium,
        "El uso de leyendas absolutas ('el mejor', '100% garantizado', 'resultados inmediatos') o promociones sin vigencia, restricciones ni existencias claras constituye publicidad engañosa sancionable.",
        "Incluir leyendas de vigencia, piezas disponibles y soporte técnico documental de las afirmaciones publicitarias.",
        250,
        35000,
        false,
    );

    // 5. Art. 128 BIS y Art. 76 BIS Fracc. II LFPC: Seguridad en Transacciones Financieras y Pasarelas de Pago
    if web.has_ecommerce_cart && !web.ssl_valid {
        push_profeco(
            "LFPC-128BIS-SEGURIDAD-TRANSACCIONES",
            "Riesgo particularmente grave: Pasarela de comercio electrónico sin elementos técnicos de seguridad y confidencialidad",
            "Arts. 76 BIS Fracc. II y 128 BIS LFPC",
            RiskSeverity::CriticalClosure,
            "Operar cobros o captación de datos financieros de consumidores en un sitio con errores de certificado SSL/TLS faculta a PROFECO a imponer clausura temporal y multas para casos particularmente graves.",
            "Restablecer de inmediato cifrado TLS 1.3 extremo a extremo y delegar el tokenizado de tarjetas a pasarelas certificadas PCI-DSS Nivel 1.",
            1500,
            45000,
            true,
        );
    }

    findings
}
