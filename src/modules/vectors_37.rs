use serde::{Deserialize, Serialize};
use worker_inegi_engine_types::*;

pub mod worker_inegi_engine_types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EvaluatedVector {
        pub num: usize,
        pub organo: String,
        pub codigo: String,
        pub titulo: String,
        pub estatus: String, // "EXPOSICIÓN ACTIVA", "CUMPLIMIENTO VERIFICADO", "VIGILANCIA PREVENTIVA", "NO APLICABLE AL GIRO"
        pub severidad: String, // "CRÍTICA", "ALTA", "MEDIA", "OPTIMIZADO"
        pub uma_rango: String,
        pub multa_estimada_mxn: f64,
        pub hallazgo_especifico: String,
    }
}

const UMA_2026: f64 = 113.14;

/// Limpia direcciones corruptas del CSV del DENUE como "CALLE PRIVADA, COL. , C.P. COLONIA, ,"
pub fn sanitize_denue_address(raw_dir: &str, raw_ent: &str, id_denue: &str) -> String {
    let cleaned = raw_dir
        .replace("C.P. COLONIA", "")
        .replace("C.P. ,", "")
        .replace("COL. ,", "")
        .replace("COL. NINGUNO", "")
        .replace("CALLE NINGUNO", "")
        .replace("NINGUNO", "");

    let parts: Vec<String> = cleaned
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| {
            !s.is_empty()
                && s != "COL."
                && s != "C.P."
                && s != "CALLE"
                && s != "PRIVADA"
                && s != "CALLE PRIVADA"
                && s.len() > 2
        })
        .collect();

    let ent_clean = raw_ent.trim();
    if parts.is_empty() {
        if !ent_clean.is_empty() {
            format!(
                "Ubicación registrada en {} (Unidad Económica INEGI DENUE #{}) — Domicilio específico reservado en padrón público",
                ent_clean, id_denue
            )
        } else {
            format!(
                "Unidad Económica INEGI DENUE #{} — Domicilio vial reservado en padrón público (Complete en Cédula)",
                id_denue
            )
        }
    } else {
        let joined = parts.join(", ");
        if !ent_clean.is_empty() && !joined.to_uppercase().contains(&ent_clean.to_uppercase()) {
            format!("{}, {}", joined, ent_clean)
        } else {
            joined
        }
    }
}

/// Evalúa los 37 Vectores Oficiales del Directorio Normativo 360° personalizando multas según plantilla real, SCIAN y telemetría
pub fn evaluate_all_37_vectors(
    razon_social: &str,
    scian: &str,
    emp_count: u32,
    sat_69b_status: &str,
    sat_69_status: &str,
    repse_registered: bool,
    sfp_sanctioned: bool,
    has_ssl: bool,
    has_privacy_notice: bool,
    has_arco_mention: bool,
    has_profeco_mention: bool,
    has_telework: bool,
    uses_subcontracting: bool,
    has_boilers: bool,
    has_forklifts_loads: bool,
    has_chemicals: bool,
    uses_biometrics: bool,
    pays_intangibles: bool,
    transports_goods: bool,
    vulnerable_pld: bool,
) -> Vec<worker_inegi_engine_types::EvaluatedVector> {
    use worker_inegi_engine_types::EvaluatedVector;

    // Factor escalar real según plantilla de trabajadores (Art. 992 LFT y capacidad económica Art. 75 CFF)
    let emp_factor: f64 = if emp_count <= 5 {
        0.35
    } else if emp_count <= 15 {
        0.65
    } else if emp_count <= 50 {
        1.25
    } else if emp_count <= 100 {
        2.10
    } else {
        3.40
    };

    let is_food_or_health = scian.starts_with("722") || scian.starts_with("62") || scian.starts_with("311") || scian.starts_with("464");
    let is_industrial_or_workshop = scian.starts_with("31") || scian.starts_with("32") || scian.starts_with("33") || scian.starts_with("811") || scian.starts_with("23");
    let is_construction = scian.starts_with("23");
    let is_transport_or_wholesale = transports_goods || scian.starts_with("48") || scian.starts_with("49") || scian.starts_with("43");
    let is_pld_sector = vulnerable_pld || scian.starts_with("52") || scian.starts_with("53") || scian.starts_with("23");

    let mut v = Vec::with_capacity(37);

    let mut add = |organo: &str,
                   codigo: &str,
                   titulo: &str,
                   active_breach: bool,
                   applicable: bool,
                   severidad_if_breach: &str,
                   uma_min: u32,
                   uma_max: u32,
                   base_umas_calc: f64,
                   hallazgo: String| {
        let num = v.len() + 1;
        let (estatus, severidad, multa) = if !applicable {
            ("EXENTO POR GIRO SCIAN".to_string(), "OPTIMIZADO".to_string(), 0.0)
        } else if active_breach {
            let m = (base_umas_calc * UMA_2026 * emp_factor).round();
            ("EXPOSICIÓN ACTIVA".to_string(), severidad_if_breach.to_string(), m)
        } else {
            ("BLINDAJE VERIFICADO".to_string(), "OPTIMIZADO".to_string(), 0.0)
        };

        v.push(EvaluatedVector {
            num,
            organo: organo.to_string(),
            codigo: codigo.to_string(),
            titulo: titulo.to_string(),
            estatus,
            severidad,
            uma_rango: format!("{} a {} UMAs", uma_min, uma_max),
            multa_estimada_mxn: multa,
            hallazgo_especifico: hallazgo,
        });
    };

    // ==================== I. STPS (8 MÓDULOS CUBRIENDO LAS 39 NOMs) ====================
    let guia_035 = if emp_count > 50 {
        "Guías de Referencia I y III (Entorno Organizacional >50 empleados)"
    } else if emp_count >= 16 {
        "Guías de Referencia I y II (Identificación Psicosocial 16 a 50 empleados)"
    } else {
        "Política Escrita de Prevención Psicosocial y Guía I (1 a 15 empleados)"
    };
    add(
        "STPS",
        "NOM-035-STPS-2018 • Art. 994 Fracc. V LFT",
        "Factores de Riesgo Psicosocial en el Trabajo",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        420.0,
        format!("Obligatorio para {} ({} trabajadores): exhibir evidencia firmada de {}.", razon_social, emp_count, guia_035),
    );

    add(
        "STPS",
        "NOM-037-STPS-2023 • Art. 330-A a 330-K LFT",
        "Teletrabajo — Seguridad, Salud y Desconexión Digital",
        has_telework,
        true,
        "ALTA",
        250,
        5000,
        310.0,
        format!("Verificación de adendum contractual de teletrabajo, lista ergonómica y pago proporcional de servicios para {}.", razon_social),
    );

    add(
        "STPS",
        "NOM-036-1-STPS-2018 • Manejo Manual de Cargas",
        "Factores de Riesgo Ergonómico (>3 kg cotidianos)",
        has_forklifts_loads,
        true,
        "ALTA",
        250,
        5000,
        340.0,
        format!("Análisis ergonómico por levantamiento o transporte manual de cargas y límite de 25 kg/20 kg en giro SCIAN {}.", scian),
    );

    add(
        "STPS",
        "NOM-019-STPS-2011 y NOM-030-STPS-2009",
        "Comisiones Mixtas de Seguridad e Higiene y Servicios Preventivos SST",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        480.0,
        format!("Constitución de Comisión Mixta (Numeral 7.4 NOM-019), recorridos trimestrales y Diagnóstico Integral NOM-030 para {} trabajadores.", emp_count),
    );

    add(
        "STPS",
        "NOM-017, NOM-018, NOM-026 y NOM-028-STPS",
        "Organización: Matriz EPP, Químicos SGA, Señalética y Procesos",
        true,
        true,
        "MEDIA",
        250,
        5000,
        290.0,
        format!("Acuses firmados de entrega de EPP por puesto (NOM-017), hojas SGA (NOM-018) y señalización de seguridad (NOM-026) en {}.", razon_social),
    );

    add(
        "STPS",
        "12 NOMs de Seguridad (NOM-001, 002, 004, 005, 006, 009, 020, 022, 027, 029, 033, 034)",
        "Seguridad Física, Incendios, Electricidad, Tierras Físicas y Maquinaria",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        520.0 + if has_boilers { 350.0 } else { 0.0 } + if has_chemicals { 250.0 } else { 0.0 },
        format!("Estudio de Grado de Riesgo de Incendio (NOM-002), Diagrama Unifilar (NOM-029) y tierras físicas (NOM-022) exigibles al giro {}.", scian),
    );

    add(
        "STPS",
        "8 NOMs de Salud e Higiene (NOM-010 a 015, NOM-024 y NOM-025)",
        "Estudios de Iluminación (Luxes), Ruido, Estrés Térmico y Agentes",
        true,
        true,
        "MEDIA",
        250,
        5000,
        280.0,
        format!("Evaluación bianual de niveles de iluminación NOM-025 y condiciones térmicas/ruido aplicables a la actividad SCIAN {}.", scian),
    );

    add(
        "STPS",
        "7 NOMs Específicas de Sector (NOM-003, 007, 008, 016, 023, 031, 032)",
        "Obras de Construcción (NOM-031) y Ramas Especiales por SCIAN",
        is_construction,
        is_construction,
        "CRÍTICA",
        250,
        5000,
        600.0,
        format!("Evaluación de aplicabilidad sectorial por prefijo SCIAN {} (Construcción NOM-031 / Ramas Especiales).", scian),
    );

    // ==================== II. SAT / CFF (6 MÓDULOS FISCALES) ====================
    let is_69b_bad = !sat_69b_status.to_uppercase().contains("LIMPIO");
    add(
        "SAT",
        "Artículo 69-B y Art. 69 CFF • Jurisprudencia SCJN 2a./J. 161/2019",
        "Materialidad de Operaciones (EFOS/EDOS), Fecha Cierta y Listas Negras",
        pays_intangibles || is_69b_bad,
        true,
        "CRÍTICA",
        450,
        4500,
        if is_69b_bad { 2200.0 } else { 540.0 },
        format!("Estatus en Padrón SAT 69-B: [{}]. Exigibilidad de Contratos con Fecha Cierta y Bitácora de Entregables.", sat_69b_status),
    );

    add(
        "SAT",
        "Artículo 5-A del Código Fiscal de la Federación",
        "Cláusula Anti-Abuso: Razón de Negocios y Beneficio Económico",
        pays_intangibles,
        true,
        "ALTA",
        300,
        2500,
        320.0,
        format!("Documentación del beneficio económico cuantificable esperado en contrataciones de servicios de {}.", razon_social),
    );

    let is_69_bad = !sat_69_status.to_uppercase().contains("LIMPIO");
    add(
        "SAT",
        "Artículos 17-K, 86-C y 86-D del CFF",
        "Buzón Tributario Activo y Medios de Contacto Verificados",
        is_69_bad,
        true,
        "ALTA",
        35,
        102,
        102.0,
        format!("Estatus Art. 69 CFF: [{}]. Verificación de correo y teléfono móvil vigentes en Buzón Tributario.", sat_69_status),
    );

    add(
        "SAT",
        "Artículos 29 y 29-A CFF • Anexo 20 CFDI 4.0, REP y Carta Porte 3.1",
        "Facturación CFDI 4.0, Complemento de Pago (Día 5) y Carta Porte 3.1",
        true,
        true,
        "ALTA",
        170,
        995,
        if is_transport_or_wholesale { 590.0 } else { 210.0 },
        format!("Conciliación de CFDI 4.0, emisión puntual de REP y Carta Porte 3.1 en traslados de mercancía para giro {}.", scian),
    );

    add(
        "SAT",
        "Artículo 28 CFF y Arts. 32-B Ter a Quinquies • Anexo 24",
        "Contabilidad Electrónica y Expediente de Beneficiario Controlador",
        true,
        true,
        "CRÍTICA",
        1300,
        17600,
        680.0,
        format!("Integración del expediente fidedigno de Beneficiarios Controladores como parte de la contabilidad de {}.", razon_social),
    );

    add(
        "SAT",
        "Artículos 27, 17-H Bis y 32-D CFF",
        "Domicilio Fiscal Localizado, Aviso de Socios y Blindaje de Sellos (CSD)",
        is_69_bad,
        true,
        "ALTA",
        40,
        120,
        120.0,
        "Prevención de causales de restricción temporal de Certificado de Sello Digital (Art. 17-H Bis CFF) y actualización de socios.".to_string(),
    );

    // ==================== III. IMSS E INFONAVIT (6 MÓDULOS PATRONALES) ====================
    add(
        "IMSS",
        "Artículos 15 y 27 a 30 LSS • Art. 29 LINFONAVIT",
        "Conciliación de Plantilla DENUE vs. SUA y Salario Base de Cotización (SBC)",
        true,
        true,
        "CRÍTICA",
        20,
        350,
        290.0,
        format!("Cruce de plantilla estimada ({} trabajadores en giro {}) contra CFDI de Nómina y cédulas SUA para prevenir Capitales Constitutivos.", emp_count, scian),
    );

    add(
        "IMSS",
        "Artículo 15-A LSS • Arts. 12 a 15 y 1004-C LFT (REPSE / ICSOE / SISSUB)",
        "Subcontratación Especializada REPSE e Informativas Cuatrimestrales",
        uses_subcontracting && !repse_registered,
        uses_subcontracting,
        "CRÍTICA",
        500,
        50000,
        620.0,
        format!("Estatus en Padrón STPS REPSE: [{}]. Verificación de los 7 comprobantes mensuales del Art. 27 Fracc. V LISR.", if repse_registered { "REGISTRADO" } else { "VALIDAR PROVEEDORES / ICSOE" }),
    );

    add(
        "IMSS",
        "Artículos 71 a 75 LSS y Art. 196 RACERF",
        "Clasificación de Riesgo y Declaración Anual de Prima de Riesgos de Trabajo",
        true,
        true,
        "MEDIA",
        20,
        210,
        180.0,
        format!("Autoclasificación en el catálogo del Art. 196 RACERF acorde a la actividad SCIAN {} y revisión anual en febrero.", scian),
    );

    add(
        "IMSS",
        "Artículo 39 LSS, Art. 32-D CFF y Buzón IMSS Obligatorio",
        "Confronta Mensual EMA/EBA y Opinión 32-D de Seguridad Social",
        true,
        true,
        "ALTA",
        20,
        350,
        210.0,
        "Confronta preventiva de emisiones EMA y EBA antes del día 17 y habilitación de medios en Buzón IMSS.".to_string(),
    );

    add(
        "IMSS",
        "Artículo 15-A párrafo tercero y Art. 15 Fracc. VI LSS (SIROC)",
        "Blindaje de Responsabilidad Solidaria y Registro de Obras SIROC",
        uses_subcontracting || is_construction,
        uses_subcontracting || is_construction,
        "ALTA",
        20,
        350,
        260.0,
        "Cláusulas de retención y liberación de pago condicionadas a Opinión 32-D positiva y pago SUA del contratista.".to_string(),
    );

    add(
        "IMSS",
        "Artículos 304, 304-A, 304-B y 304-C de la LSS",
        "Auditoría de las 22 Infracciones LSS y Beneficio de Cumplimiento Espontáneo",
        false,
        true,
        "MEDIA",
        20,
        350,
        0.0,
        "Aplicación del beneficio de cumplimiento espontáneo sin multa conforme al Artículo 304-C de la LSS.".to_string(),
    );

    // ==================== IV. SABG / LFPDPPP (6 MÓDULOS DE PRIVACIDAD) ====================
    add(
        "SABG / LFPDPPP",
        "Artículos 8, 15 y 16 LFPDPPP • Lineamientos DOF 17/01/2013",
        "Aviso de Privacidad en sus 3 Modalidades (Integral, Simplificado y Corto)",
        !has_privacy_notice,
        true,
        "CRÍTICA",
        100,
        160000,
        460.0,
        format!("Telemetría sobre dominio web: Aviso de Privacidad Integral {}.", if has_privacy_notice { "DETECTADO" } else { "NO DETECTADO O INCOMPLETO EN DOM" }),
    );

    add(
        "SABG / LFPDPPP",
        "Artículos 22 al 35 LFPDPPP y Arts. 87 al 110 RLFPDPPP",
        "Procedimiento y Ventanilla de Atención a Derechos ARCO (Plazo 20 Días)",
        !has_arco_mention,
        true,
        "ALTA",
        200,
        320000,
        390.0,
        format!("Mención expresa de medios y plazos ARCO en portal web: {}.", if has_arco_mention { "VERIFICADA" } else { "OMISIÓN DETECTADA" }),
    );

    add(
        "SABG / LFPDPPP",
        "Artículos 9 y 64 Fracc. IV LFPDPPP",
        "Consentimiento Expreso y por Escrito para Datos Biométricos, Salud y Nómina",
        uses_biometrics,
        true,
        "CRÍTICA",
        400,
        640000,
        580.0,
        "Formato con firma autógrafa para uso de reloj checador biométrico (huella/rostro), expedientes médicos y cuentas bancarias.".to_string(),
    );

    add(
        "SABG / LFPDPPP",
        "Artículos 19 y 20 LFPDPPP • Arts. 57 a 66 RLFPDPPP",
        "Documento de Seguridad Interno, Inventario de Datos y Cifrado TLS/SSL",
        !has_ssl,
        true,
        "ALTA",
        200,
        320000,
        410.0,
        format!("Certificado de cifrado HTTPS/TLS en dominio: {}. Se requiere Documento de Seguridad interno actualizado.", if has_ssl { "ACTIVO" } else { "VULNERABLE / SIN SSL" }),
    );

    add(
        "SABG / LFPDPPP",
        "Artículos 36 y 37 LFPDPPP • Arts. 49 a 56 RLFPDPPP",
        "Cláusulas de Remisión a Encargados (Despachos Contables, Nómina y CRM)",
        true,
        true,
        "MEDIA",
        200,
        320000,
        240.0,
        "Convenios de confidencialidad y tratamiento de datos con despachos externos y proveedores tecnológicos.".to_string(),
    );

    add(
        "SABG / LFPDPPP",
        "Artículo 30 de la LFPDPPP",
        "Designación Formal del Oficial o Departamento de Datos Personales",
        !has_arco_mention,
        true,
        "MEDIA",
        100,
        160000,
        190.0,
        "Acta interna de nombramiento del Oficial de Privacidad de la organización.".to_string(),
    );

    // ==================== V. 9 SECTORIALES Y PLD (6 MÓDULOS POR GIRO SCIAN) ====================
    add(
        "SHCP / UIF",
        "Módulo Sectorial 01 • LFPIORPI Arts. 17, 18, 21, 32, 53 y 54",
        "Prevención de Lavado de Dinero y las 16 Actividades Vulnerables",
        is_pld_sector,
        is_pld_sector,
        "CRÍTICA",
        10000,
        65000,
        950.0,
        format!("Evaluación de umbrales del Art. 17 LFPIORPI, mutuos intercompañía, arrendamiento y restricción de efectivo para SCIAN {}.", scian),
    );

    add(
        "COFEPRIS",
        "Módulo Sectorial 02 • Arts. 198 y 200 Bis LGS • NOM-251-SSA1 y NOM-051",
        "Aviso de Funcionamiento DIGIPRIS, Higiene NOM-251 y Etiquetado NOM-051",
        is_food_or_health,
        is_food_or_health,
        "CRÍTICA",
        2000,
        16000,
        640.0,
        format!("Giro de Alimentos/Salud detectado (SCIAN {}): obligatorio Aviso DIGIPRIS, bitácoras de cadena de frío NOM-251 y control de plagas.", scian),
    );

    add(
        "PROTECCIÓN CIVIL",
        "Módulo Sectorial 03 • Arts. 39, 40 y 74 a 78 LGPC",
        "Programa Interno de Protección Civil (PIPC), Unidad Interna y Dictámenes",
        true,
        true,
        "CRÍTICA",
        1000,
        20000,
        510.0,
        format!("Carpeta PIPC, Acta de 4 Brigadas, Póliza RC y dictámenes eléctrico/gas/estructural para el inmueble de {}.", razon_social),
    );

    add(
        "SEMARNAT / PROFEPA",
        "Módulo Sectorial 04 • LGEEPA Arts. 109 Bis y 171 • LGPGIR Arts. 40 a 48",
        "Registro de Generador de Residuos Peligrosos / Manejo Especial y LAU",
        is_industrial_or_workshop || is_food_or_health,
        is_industrial_or_workshop || is_food_or_health,
        "ALTA",
        30,
        50000,
        430.0,
        format!("Manejo de residuos y manifiestos de recolección autorizada aplicables al giro SCIAN {}.", scian),
    );

    add(
        "SICT / CAPUFE",
        "Módulo Sectorial 05 • NOM-087-SCT-2-2017, NOM-012-SCT y NOM-068-SCT",
        "Autotransporte Federal, Flotillas de Reparto y Bitácora de Horas",
        is_transport_or_wholesale,
        is_transport_or_wholesale,
        "ALTA",
        100,
        500,
        310.0,
        "Bitácoras de horas de servicio NOM-087-SCT, dictamen físico-mecánico NOM-068 y seguro de carga.".to_string(),
    );

    add(
        "REPSE / SSPC / SEP / SECTUR",
        "Módulos Sectoriales 06 al 09 • Autorizaciones Federales Especiales",
        "Registros Especiales de Sector (REPSE, Seguridad DGSP, SEP RVOE, SECTUR RNT)",
        sfp_sanctioned || scian.starts_with("56") || scian.starts_with("721") || scian.starts_with("61"),
        true,
        "MEDIA",
        500,
        7000,
        270.0,
        format!("Cruce en SFP Sancionados: [{}]. Verificación de registros sectoriales según prefijo SCIAN {}.", if sfp_sanctioned { "ALERTA SANCIONADO" } else { "SIN SANCIONES" }, scian),
    );

    // ==================== VI. PROFECO Y ECONOMÍA (5 MÓDULOS COMERCIALES) ====================
    add(
        "PROFECO",
        "Artículo 76 BIS y 76 BIS 1 LFPC • NMX-COE-001-SCFI-2018",
        "Comercio Electrónico, Precios Totales con IVA y Domicilio en México",
        !has_profeco_mention,
        true,
        "ALTA",
        100,
        35000,
        320.0,
        "Publicación obligatoria de Razón Social, domicilio físico en México, teléfonos y precios totales con IVA incluido (Art. 7 BIS LFPC).".to_string(),
    );

    add(
        "PROFECO",
        "Artículos 56 y 77 a 82 de la LFPC",
        "Póliza de Garantía Mínima de 90 Días y Políticas de Reembolso / Revocación",
        !has_profeco_mention,
        true,
        "MEDIA",
        100,
        33900,
        240.0,
        "Adecuación de garantías al mínimo legal de 90 días naturales (Art. 77 LFPC) y derecho de revocación en 5 días hábiles (Art. 56 LFPC).".to_string(),
    );

    add(
        "PROFECO",
        "Artículos 85, 86, 87 y 90 de la LFPC (RRPAC)",
        "Contratos de Adhesión y Purga de Cláusulas Abusivas o Nulas (Art. 90)",
        !has_profeco_mention,
        true,
        "ALTA",
        100,
        33900,
        280.0,
        "Eliminación de cláusulas unilaterales prohibidas por el Artículo 90 de la LFPC en contratos con clientes.".to_string(),
    );

    add(
        "PROFECO",
        "Artículos 32 y 46 de la LFPC",
        "Publicidad Veraz, Comprobable y Reglas de Promociones Comerciales",
        false,
        true,
        "MEDIA",
        100,
        33900,
        0.0,
        "Cumplimiento de vigencias, restricciones claras y soporte comprobable en campañas publicitarias.".to_string(),
    );

    add(
        "ECONOMÍA / PROFECO",
        "Artículo 128 BIS LFPC • Código de Comercio Arts. 89–93 y NOM-151-SCFI-2016",
        "Seguridad en Transacciones Digitales y Conservación Criptográfica de Mensajes",
        !has_ssl,
        true,
        "ALTA",
        1700,
        48600,
        350.0,
        "Sellado criptográfico SHA-256 / Árbol de Merkle para dotar de integridad y Fecha Cierta a los contratos electrónicos.".to_string(),
    );

    v
}
