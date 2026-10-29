use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DofRegulatoryAlert {
    pub fecha_dof: &'static str,
    pub organo: &'static str,
    pub precepto: &'static str,
    pub scian_afectados: &'static str,
    pub resumen_ejecutivo: &'static str,
    pub accion_preventiva: &'static str,
}

/// Devuelve los acuerdos y reformas vigentes del Diario Oficial de la Federación (DOF) filtrados por código SCIAN
pub fn get_active_dof_alerts_for_scian(scian: &str) -> Vec<DofRegulatoryAlert> {
    let mut alerts = vec![
        DofRegulatoryAlert {
            fecha_dof: "Vigente 2026 • INEGI / STPS / SAT",
            organo: "INEGI / CONASAMI / STPS",
            precepto: "Actualización de la Unidad de Medida y Actualización (UMA $113.14 MXN) y Tabulador PASL",
            scian_afectados: "TODOS LOS SECTORES",
            resumen_ejecutivo: "Incremento en la base cuantificadora de multas administrativas federales (STPS Arts. 992-1004 LFT, LFPDPPP Art. 64 e IMSS Art. 304-B).",
            accion_preventiva: "Actualizar matrices de riesgo y actas de Comisión Mixta NOM-019/030 al ejercicio en curso.",
        },
        DofRegulatoryAlert {
            fecha_dof: "DOF 05/08/2024 - Vigente 2026",
            organo: "IMSS • Consejo Técnico",
            precepto: "Acuerdo ACDO.AS2.HCT.230724/261.P.DIR — Lineamientos de Carácter General del Buzón IMSS",
            scian_afectados: "TODOS LOS REGISTROS PATRONALES",
            resumen_ejecutivo: "Notificación electrónica obligatoria de resoluciones, cédulas de liquidación, multas y opiniones 32-D a través del Buzón IMSS.",
            accion_preventiva: "Validar medios de contacto en Buzón IMSS con e.firma vigente para evitar notificaciones por estrados.",
        },
        DofRegulatoryAlert {
            fecha_dof: "DOF 08/06/2023 - Exigible 2026",
            organo: "STPS",
            precepto: "NOM-037-STPS-2023 — Teletrabajo: Condiciones de Seguridad y Salud en el Trabajo",
            scian_afectados: "54, 51, 52, 56 (Servicios, Corporativos, TI y Comercio)",
            resumen_ejecutivo: "Inspección de adendums contractuales, pago proporcional de electricidad/internet, desconexión digital y listas ergonómicas.",
            accion_preventiva: "Suscribir Política de Teletrabajo NOM-037 y convenio modificatorio individual.",
        },
    ];

    if scian.starts_with("72") || scian.starts_with("62") || scian.starts_with("311") {
        alerts.push(DofRegulatoryAlert {
            fecha_dof: "Vigente 2026 • COFEPRIS / DIGIPRIS",
            organo: "COFEPRIS / Secretaría de Salud",
            precepto: "NOM-251-SSA1-2009 y Plataforma Digital DIGIPRIS de Avisos de Funcionamiento",
            scian_afectados: "722 (Restaurantes/Alimentos), 621 (Clínicas/Salud), 311 (Industria Alimentaria)",
            resumen_ejecutivo: "Verificación de concordancia exacta entre la clave SCIAN del SAT y el Aviso de Funcionamiento DIGIPRIS, bitácoras de cadena de frío y control de plagas.",
            accion_preventiva: "Actualizar Aviso de Funcionamiento DIGIPRIS y bitácoras diarias de refrigeración y fumigación.",
        });
    }

    if scian.starts_with("48") || scian.starts_with("43") || scian.starts_with("46") || scian.starts_with("31") || scian.starts_with("32") || scian.starts_with("33") {
        alerts.push(DofRegulatoryAlert {
            fecha_dof: "DOF 17/07/2024 - Vigente 2026",
            organo: "SAT / SICT",
            precepto: "Complemento Carta Porte Versión 3.1 y Bitácora de Horas de Servicio NOM-087-SCT",
            scian_afectados: "484 (Autotransporte), 43/46 (Comercio Mayorista/Minorista), 31-33 (Manufactura)",
            resumen_ejecutivo: "Exigibilidad del CFDI con Complemento Carta Porte 3.1 con identificador CCP (IdCCP) en todo traslado federal de mercancías.",
            accion_preventiva: "Auditar emisión de CFDI de Ingreso/Traslado con Carta Porte 3.1 antes del despacho de unidades.",
        });
    }

    alerts
}
