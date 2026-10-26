//! Motor de los 9 Módulos Regulatorios Sectoriales Especializados (Activación dinámica por SCIAN, Texto y Calibración):
//! 1. CNBV / CONDUSEF / LFPIORPI: Sector Financiero, Fintech y Actividades Vulnerables Antilavado (Art. 17 LFPIORPI).
//! 2. COFEPRIS: Salud, Alimentos, Cosméticos, Restaurantes y Farmacia (Aviso de Funcionamiento, Licencia Sanitaria, NOM-051/251).
//! 3. PROTECCIÓN CIVIL (PIPC Estatal y Municipal): Programa Interno de Protección Civil, Dictamen Eléctrico, Estructural y Gas (>100 m²).
//! 4. SEMARNAT / ASEA / PROFEPA: Licencia Ambiental Única (LAU), COA, Impacto Ambiental y Residuos Peligrosos (NOM-052).
//! 5. SICT / CAPUFE: Permisos de Autotransporte Federal, Pesos y Dimensiones (NOM-012-SCT), Bitácoras de Horas de Servicio y Pólizas.
//! 6. REPSE / STPS / IMSS (Sector Servicios Especializados B2B): Vigencia del padrón y cumplimiento contractual.
//! 7. SSPC / DGSP (Seguridad Privada): Autorización federal DGSP, permiso estatal y registro CUIP.
//! 8. SEP (Educación y Capacitación): Reconocimiento de Validez Oficial de Estudios (RVOE) y plantillas académicas.
//! 9. SECTUR (Turismo y Hospitalidad): Registro Nacional de Turismo (RNT) y normas de clasificación hotelera/agencias.

use crate::engine::models::{
    CalibrationAnswers, EmpresaB2B, RegulatoryAgency, RegulatoryFinding, RiskSeverity,
    UmaCalculator,
};

/// Función original 100% retrocompatible con `EmpresaB2B`.
pub fn evaluate_sectorial(empresa: &mut EmpresaB2B, valor_uma: f64) -> f64 {
    let calc = UmaCalculator {
        uma_daily_mxn: valor_uma,
        fiscal_year: 2026,
    };
    let findings = evaluate_sectorial_detailed(
        &empresa.actividad,
        &empresa.nombre,
        false,
        &CalibrationAnswers::default(),
        &calc,
    );

    let mut total_multa = 0.0;
    for f in findings {
        empresa.audit_flags.push(format!("SECTORIAL_{}", f.code.replace('-', "_")));
        total_multa += f.estimated_fine_mxn;
    }
    total_multa
}

/// Evaluación Pericial Completa de los 9 Módulos Regulatorios Sectoriales.
pub fn evaluate_sectorial_detailed(
    scian_or_act: &str,
    company_name: &str,
    sfp_sanctioned_in_db: bool,
    cal: &CalibrationAnswers,
    uma: &UmaCalculator,
) -> Vec<RegulatoryFinding> {
    let mut findings = Vec::new();
    let act_lower = format!("{} {}", scian_or_act, company_name).to_lowercase();
    let prefix2 = if scian_or_act.len() >= 2 {
        &scian_or_act[..2]
    } else {
        "46"
    };

    let mut push_sec = |code: &str,
                        agency: RegulatoryAgency,
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
            agency,
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

    // =========================================================================
    // MÓDULO 1: CNBV / CONDUSEF / LFPIORPI (Financiero, Fintech y Actividades Vulnerables PLD)
    // =========================================================================
    let is_fintech_or_pld = CalibrationAnswers::resolve_flag(
        cal.performs_vulnerable_activities_pld,
        prefix2 == "52"
            || prefix2 == "53"
            || act_lower.contains("financiera")
            || act_lower.contains("sofom")
            || act_lower.contains("fintech")
            || act_lower.contains("inmobiliaria")
            || act_lower.contains("arrendadora")
            || act_lower.contains("joyeria")
            || act_lower.contains("blindaje")
            || act_lower.contains("mutuo"),
    );
    if is_fintech_or_pld {
        push_sec(
            "CNBV-LFPIORPI-ART17-PLD",
            RegulatoryAgency::SectorialCnbvPld,
            "Cumplimiento en Prevención de Lavado de Dinero (LFPIORPI Art. 17) y regulación CNBV / CONDUSEF",
            "Arts. 17, 18, 53 y 54 LFPIORPI / Disposiciones de Carácter General CNBV / REDECO y SIPRES CONDUSEF",
            RiskSeverity::CriticalClosure,
            "La realización de Actividades Vulnerables (mutuo/préstamos, desarrollo o arrendamiento inmobiliario, comercialización de vehículos/joyas, activos virtuales) o servicios financieros exige alta en el Portal Antilavado SPPLD del SAT, expedientes KYC de Beneficiario Controlador y Avisos antes del día 17 de cada mes.",
            "Adoptar Manual de Cumplimiento PLD/FT, integrar expedientes únicos de identificación de clientes y automatizar alertas de umbrales de identificación y aviso en UMA.",
            200,
            65000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 2: COFEPRIS (Salud, Alimentos, Bebidas, Cosméticos, Restaurantes y Farmacia)
    // =========================================================================
    let is_cofepris = matches!(prefix2, "31" | "62" | "72")
        || scian_or_act.starts_with("464")
        || scian_or_act.starts_with("431")
        || act_lower.contains("restaurante")
        || act_lower.contains("alimentos")
        || act_lower.contains("farmacia")
        || act_lower.contains("clinica")
        || act_lower.contains("hospital")
        || act_lower.contains("medico")
        || act_lower.contains("cosmetico")
        || act_lower.contains("suplemento");

    if is_cofepris {
        push_sec(
            "COFEPRIS-LGS-AVISO-NOM251-051",
            RegulatoryAgency::SectorialCofepris,
            "Aviso de Funcionamiento / Licencia Sanitaria COFEPRIS, Responsable Sanitario y normas de higiene/etiquetado (NOM-251 / NOM-051)",
            "Arts. 198, 200 Bis, 376, 417 y 422 Ley General de Salud / NOM-251-SSA1-2009 / NOM-051-SCFI/SSA1-2010",
            RiskSeverity::CriticalClosure,
            "Operar establecimientos de alimentos, insumos para la salud, clínicas o cosméticos sin Aviso de Funcionamiento en DIGIPRIS, sin bitácoras de prácticas de higiene (NOM-251) o con etiquetado no conforme a sellos NOM-051 conlleva aseguramiento de producto y suspensión inmediata de trabajos.",
            "Tramitación/actualización de Aviso de Funcionamiento y Responsable Sanitario en DIGIPRIS e implementación de bitácoras sanitarias diarias.",
            1000,
            16000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 3: PROTECCIÓN CIVIL (PIPC Estatal y Municipal > 100 m² o Afluencia)
    // =========================================================================
    let area_sqm = cal.facility_area_sqm.unwrap_or(150.0);
    if area_sqm >= 100.0 || matches!(prefix2, "23" | "31" | "32" | "33" | "43" | "46" | "61" | "62" | "71" | "72") {
        push_sec(
            "PCIVIL-PIPC-DICTAMENES-ESTRUCTURAL-ELECTRICO",
            RegulatoryAgency::SectorialProteccionCivil,
            "Programa Interno de Protección Civil (PIPC), Visto Bueno y Dictámenes de Seguridad Estructural, Eléctrica y de Gas",
            "Arts. 39, 40 y 78 Ley General de Protección Civil y Leyes Estatales de Gestión Integral de Riesgos",
            RiskSeverity::CriticalClosure,
            "La falta de Programa Interno de Protección Civil (PIPC) elaborado por Tercer Acreditado / Consultor Registrado, póliza de seguro de responsabilidad civil vigente y dictámenes técnicos es la causa #1 de colocación de sellos de clausura municipal/estatal.",
            "Integrar carpeta técnica de PIPC anual, dictamen de instalaciones eléctricas (UVIE), dictamen estructural (DRO/Corresponsable) y póliza RC.",
            500,
            20000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 4: SEMARNAT / ASEA / PROFEPA (Impacto Ambiental, LAU y Residuos Peligrosos)
    // =========================================================================
    let is_environmental = CalibrationAnswers::resolve_flag(
        cal.handles_hazardous_chemicals,
        matches!(prefix2, "21" | "22" | "23" | "31" | "32" | "33" | "48" | "62")
            || act_lower.contains("manufactura")
            || act_lower.contains("quimica")
            || act_lower.contains("gasolinera")
            || act_lower.contains("hidrocarburo")
            || act_lower.contains("taller"),
    );
    if is_environmental {
        push_sec(
            "SEMARNAT-PROFEPA-LAU-RESIDUOS-NOM052",
            RegulatoryAgency::SectorialSemarnat,
            "Licencia Ambiental Única (LAU), Cédula de Operación Anual (COA) y Registro de Generador de Residuos Peligrosos",
            "Arts. 28, 109 Bis, 150 y 171 LGEEPA / Arts. 40 a 48 y 112 LGPGIR / NOM-052-SEMARNAT-2005",
            RiskSeverity::High,
            "Generar aceites gastados, solventes, envases impregnados, lodos o residuos biológico-infecciosos (RPBI NOM-087) sin registro de generador (Micro, Pequeño o Gran Generador), almacén temporal normado y manifiestos de entrega-transporte-recepción detona clausuras de PROFEPA/ASEA.",
            "Obtener Registro de Generador de Residuos Peligrosos, contratar recolectores autorizados SEMARNAT y presentar la COA en plazo.",
            300,
            50000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 5: SICT / CAPUFE (Transporte Federal, Fletes y Logística)
    // =========================================================================
    let is_transport = CalibrationAnswers::resolve_flag(
        cal.transports_goods_federal_highways,
        matches!(prefix2, "48" | "49")
            || act_lower.contains("transporte")
            || act_lower.contains("flete")
            || act_lower.contains("logistica")
            || act_lower.contains("autotransporte"),
    );
    if is_transport {
        push_sec(
            "SICT-AUTOTRANSPORTE-NOM012-NOM087",
            RegulatoryAgency::SectorialSict,
            "Permisos de Autotransporte Federal SICT, Pesos y Dimensiones (NOM-012-SCT), Bitácora de Horas de Servicio (NOM-087-SCT) y Pólizas",
            "Arts. 8, 35, 68 y 74 Ley de Caminos, Puentes y Autotransporte Federal / NOM-012-SCT-2-2017 / NOM-087-SCT-2-2017",
            RiskSeverity::High,
            "Circular en caminos de jurisdicción federal sin permiso SICT para carga general o especializada, sin verificación físico-mecánica/emisiones (NOM-068-SCT), bitácoras de fatiga o seguro de responsabilidad civil por mínimo 19,000 UMAs amerita detención de unidades y multas federales.",
            "Mantener expediente por unidad motriz: tarjeta de circulación federal SICT, dictámenes de condiciones físico-mecánicas, licencias federales digitales y bitácoras NOM-087.",
            500,
            5000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 6: SFP / COMPRANET (Proveedores y Contratistas Sancionados / B2G)
    // =========================================================================
    if sfp_sanctioned_in_db {
        push_sec(
            "SFP-LAASSP-PROVEEDOR-INHABILITADO",
            RegulatoryAgency::SectorialSfp,
            "ALERTA CRÍTICA: Registro activo en el Directorio de Proveedores y Contratistas Sancionados (SABG / ex-SFP)",
            "Arts. 50, 59 y 60 LAASSP / Arts. 78 y 81 LOPSRM / Ley General de Responsabilidades Administrativas",
            RiskSeverity::CriticalClosure,
            "La entidad presenta antecedentes de sanción o inhabilitación en el padrón oficial para celebrar contratos con dependencias gubernamentales y genera alerta roja de debida diligencia (Compliance B2B).",
            "Verificar vigencia de la resolución administrativa o promover juicio contencioso administrativo ante el TFJA.",
            1000,
            30000,
            true,
        );
    }

    // =========================================================================
    // MÓDULO 7: SSPC / DGSP (Seguridad Privada, Vigilancia y Custodia)
    // =========================================================================
    if scian_or_act.starts_with("5616")
        || act_lower.contains("seguridad privada")
        || act_lower.contains("vigilancia")
        || act_lower.contains("custodia")
    {
        push_sec(
            "SSPC-DGSP-LFSP-AUTORIZACION",
            RegulatoryAgency::SectorialSepSecturSspc,
            "Autorización Federal DGSP (SSPC), Permiso Estatal de Seguridad Privada y Registro Nacional de Personal (CUIP)",
            "Arts. 5, 15, 32 y 42 Ley Federal de Seguridad Privada",
            RiskSeverity::CriticalClosure,
            "Prestar servicios de seguridad privada, vigilancia intramuros o traslado de valores sin revalidación anual de la Dirección General de Seguridad Privada (DGSP) o sin inscripción de elementos en el RNPSP constituye delito e infracción administrativa grave.",
            "Mantener vigentes la autorización federal DGSP, permisos locales, registro REPSE y evaluaciones de control de confianza del personal operativo.",
            1000,
            15000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 8: SEP (Educación Privada, Colegios, Universidades y Capacitación)
    // =========================================================================
    if prefix2 == "61"
        || act_lower.contains("escuela")
        || act_lower.contains("colegio")
        || act_lower.contains("universidad")
        || act_lower.contains("instituto educativo")
    {
        push_sec(
            "SEP-LGE-RVOE-NOM237",
            RegulatoryAgency::SectorialSepSecturSspc,
            "Reconocimiento de Validez Oficial de Estudios (RVOE - SEP), otorgamiento de becas mínimas (5%) y Acuerdo de Comercialización Educativa (NOM-237)",
            "Arts. 146, 147, 149 y 170 Ley General de Educación / Acuerdo 17/11/17 SEP / NOM-237-SE-2021",
            RiskSeverity::High,
            "Impartir estudios sin mencionar expresamente el número de acuerdo RVOE (o su ausencia) en toda la publicidad, incumplir el porcentaje mínimo de becas (5% de la matrícula) o violar la NOM-237 de servicios educativos privados detona multas y retiro del reconocimiento oficial.",
            "Auditar expediente de cada programa RVOE, comité de asignación de becas, plantillas docentes e información de colegiaturas conforme a PROFECO/SEP.",
            1000,
            15000,
            false,
        );
    }

    // =========================================================================
    // MÓDULO 9: SECTUR (Turismo, Hoteles, Agencias de Viajes y Hospitalidad)
    // =========================================================================
    if scian_or_act.starts_with("721")
        || scian_or_act.starts_with("5615")
        || act_lower.contains("hotel")
        || act_lower.contains("agencia de viajes")
        || act_lower.contains("turismo")
        || act_lower.contains("resort")
    {
        push_sec(
            "SECTUR-LGT-RNT-CLASIFICACION",
            RegulatoryAgency::SectorialSepSecturSspc,
            "Inscripción obligatoria en el Registro Nacional de Turismo (RNT - SECTUR), Sistema de Clasificación Hotelera y Protocolos contra Explotación (CCNNA)",
            "Arts. 46, 48, 57, 58, 69 y 70 Ley General de Turismo",
            RiskSeverity::High,
            "Todo prestador de servicios turísticos (hoteles, moteles, agencias de viajes, operadoras, arrendadoras de autos y restaurantes turísticos) está obligado a inscribirse y renovar cada 2 años su certificado del Registro Nacional de Turismo (RNT).",
            "Tramitar o renovar el Certificado del Registro Nacional de Turismo (RNT), exhibirlo en lugar visible y cumplir las NOMs turísticas aplicables (NOM-07-TUR / NOM-010-TUR).",
            500,
            10000,
            false,
        );
    }

    findings
}
