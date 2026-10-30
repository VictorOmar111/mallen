use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedVector {
    pub num: usize,
    pub organo: String,
    pub codigo: String,
    pub titulo: String,
    pub estatus: String,
    pub severidad: String,
    pub uma_rango: String,
    pub multa_estimada_mxn: f64,
    pub hallazgo_especifico: String,
}

const UMA_2026: f64 = 113.14;

/// Clasifica automáticamente la Clase y Prima Media de Riesgo de Trabajo IMSS (Art. 73 LSS y Art. 196 RACERF) según el giro SCIAN
pub fn classify_imss_risk_by_scian(scian: &str) -> (&'static str, &'static str, &'static str) {
    let s = scian.trim();
    if s.starts_with("23") || s.starts_with("21") || s.starts_with("484") || s.starts_with("5616") {
        (
            "Clase V (Riesgo Máximo — Art. 196 RACERF)",
            "7.58875%",
            "Construcción, edificación, autotransporte de carga pesada o seguridad privada con alta siniestralidad laboral",
        )
    } else if s.starts_with("31") || s.starts_with("32") || s.starts_with("33") || s.starts_with("811") {
        (
            "Clase IV (Riesgo Alto — Art. 196 RACERF)",
            "4.65325%",
            "Industria manufacturera, transformación, talleres mecánicos automotrices y mantenimiento de maquinaria",
        )
    } else if s.starts_with("722") || s.starts_with("721") || s.starts_with("622") || s.starts_with("43") || s.starts_with("488") {
        (
            "Clase III (Riesgo Medio — Art. 196 RACERF)",
            "2.59840%",
            "Restaurantes, preparación de alimentos, hospitalidad, hospitales, logística aduanal y almacenes mayoristas",
        )
    } else if s.starts_with("46") || s.starts_with("621") || s.starts_with("61") || s.starts_with("56") {
        (
            "Clase II (Riesgo Bajo-Ordinario — Art. 196 RACERF)",
            "1.13065%",
            "Comercio minorista, consultorios médicos, instituciones educativas y servicios de apoyo empresarial",
        )
    } else {
        (
            "Clase I (Riesgo Ordinario Mínimo — Art. 196 RACERF)",
            "0.54355%",
            "Servicios profesionales, corporativos, financieros, aseguradoras y tecnología de la información",
        )
    }
}

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
                "Ubicación registrada en {} (Unidad Económica INEGI DENUE #{}) — Domicilio vial reservado en padrón público",
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

/// Escapa caracteres especiales de Typst para evitar cualquier error de compilación en el PDF
pub fn escape_typst(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "(")
        .replace(']', ")")
        .replace('@', "\\@")
        .replace('<', "(")
        .replace('>', ")")
}

pub fn format_mxn(val: f64) -> String {
    let whole = val.trunc() as i64;
    let cents = ((val.fract().abs() * 100.0).round() as i64).min(99);
    let s = whole.to_string();
    let mut out = String::new();
    for (count, ch) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 && ch != '-' {
            out.push(',');
        }
        out.push(ch);
    }
    let formatted_whole: String = out.chars().rev().collect();
    format!("{}.{:02}", formatted_whole, cents)
}

/// Genera el código Typst de la Tabla Forense con los 37 Vectores Evaluados para la Página 2 del Dictamen PDF
pub fn render_vectors_37_typst_table(vectors: &[EvaluatedVector]) -> String {
    let mut out = String::from(
        "#table(\n  columns: (5%, 16%, 29%, 34%, 16%),\n  inset: (x: 4pt, y: 2.8pt),\n  stroke: 0.35pt + rgb(\"#cbd5e1\"),\n  fill: (x, y) => if y == 0 { rgb(\"#0f172a\") } else if calc.odd(y) { rgb(\"#f8fafc\") } else { rgb(\"#ffffff\") },\n  [#text(fill: white, weight: \"bold\", size: 5.8pt)[\\#]],\n  [#text(fill: white, weight: \"bold\", size: 5.8pt)[ÓRGANO / ESTATUS]],\n  [#text(fill: white, weight: \"bold\", size: 5.8pt)[PRECEPTO Y VECTOR NORMATIVO]],\n  [#text(fill: white, weight: \"bold\", size: 5.8pt)[DICTAMINACIÓN ESPECÍFICA AL GIRO AUDITADO]],\n  [#text(fill: white, weight: \"bold\", size: 5.8pt)[CONTINGENCIA]],\n",
    );

    for v in vectors {
        let is_active = v.multa_estimada_mxn > 0.0;
        let color_hex = if is_active { "#b91c1c" } else { "#047857" };
        let fine_label = if is_active {
            format!("\\$ {} MXN", format_mxn(v.multa_estimada_mxn))
        } else {
            format!("{} (\\$0.00)", escape_typst(&v.estatus))
        };

        out.push_str(&format!(
            "  [#text(weight: \"bold\", size: 5.6pt)[{:02}]],\n  [#text(weight: \"bold\", size: 5.5pt, fill: rgb(\"{}\"))[{} • {}]],\n  [#text(weight: \"bold\", size: 5.6pt)[{} — {}]],\n  [#text(size: 5.4pt)[{}]],\n  [#text(weight: \"bold\", size: 5.6pt, fill: rgb(\"{}\"))[{} ({})]],\n",
            v.num,
            color_hex,
            escape_typst(&v.organo),
            escape_typst(&v.severidad),
            escape_typst(&v.codigo),
            escape_typst(&v.titulo),
            escape_typst(&v.hallazgo_especifico),
            color_hex,
            fine_label,
            escape_typst(&v.uma_rango)
        ));
    }

    out.push_str(")\n");
    out
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
) -> Vec<EvaluatedVector> {
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

    let (imss_clase, imss_prima, _imss_desc) = classify_imss_risk_by_scian(scian);
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
            ("EXENTO SCIAN".to_string(), "EXENTO".to_string(), 0.0)
        } else if active_breach {
            let m = (base_umas_calc * UMA_2026 * emp_factor).round();
            ("EXPOSICIÓN ACTIVA".to_string(), severidad_if_breach.to_string(), m)
        } else {
            ("BLINDADO".to_string(), "OPTIMIZADO".to_string(), 0.0)
        };

        v.push(EvaluatedVector {
            num,
            organo: organo.to_string(),
            codigo: codigo.to_string(),
            titulo: titulo.to_string(),
            estatus,
            severidad,
            uma_rango: format!("{}-{} UMAs", uma_min, uma_max),
            multa_estimada_mxn: multa,
            hallazgo_especifico: hallazgo,
        });
    };

    // ==================== I. STPS (8 MÓDULOS CUBRIENDO LAS 39 NOMs) ====================
    let guia_035 = if emp_count > 50 {
        "Guías I y III (Entorno Organizacional >50 trabajadores)"
    } else if emp_count >= 16 {
        "Guías I y II (Factores Psicosociales 16 a 50 trabajadores)"
    } else {
        "Política de Prevención Psicosocial y Guía I (1 a 15 trabajadores)"
    };
    add(
        "STPS",
        "NOM-035-STPS-2018",
        "Factores de Riesgo Psicosocial",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        420.0,
        format!("Exigible para {} trabajadores en {}: acreditar {}.", emp_count, razon_social, guia_035),
    );

    add(
        "STPS",
        "NOM-037-STPS-2023",
        "Teletrabajo y Desconexión Digital",
        has_telework,
        true,
        "ALTA",
        250,
        5000,
        310.0,
        format!("Adendum de teletrabajo (Art. 330-B LFT), lista ergonómica y pago proporcional de internet/luz en giro {}.", scian),
    );

    add(
        "STPS",
        "NOM-036-1-STPS-2018",
        "Riesgo Ergonómico en Manejo de Cargas",
        has_forklifts_loads,
        true,
        "ALTA",
        250,
        5000,
        340.0,
        format!("Análisis ergonómico por levantamiento/arrastre manual de cargas (>3 kg) y vigilancia médica en giro {}.", scian),
    );

    add(
        "STPS",
        "NOM-019 y NOM-030-STPS",
        "Comisión Mixta SST y Diagnóstico Preventivo",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        480.0,
        format!("Acta constitutiva NOM-019, recorridos trimestrales y Programa Anual NOM-030 para plantilla de {} personas.", emp_count),
    );

    add(
        "STPS",
        "NOM-017, 018, 026 y 028-STPS",
        "EPP por Puesto, Químicos SGA y Señalética",
        true,
        true,
        "MEDIA",
        250,
        5000,
        290.0,
        format!("Matriz de Equipo de Protección Personal firmada (NOM-017), hojas SGA (NOM-018) y colores de seguridad (NOM-026) en {}.", scian),
    );

    add(
        "STPS",
        "12 NOMs de Seguridad (NOM-001 a 034)",
        "Incendios NOM-002, Electricidad NOM-029 y Maquinaria",
        true,
        true,
        "CRÍTICA",
        250,
        5000,
        520.0 + if has_boilers { 350.0 } else { 0.0 } + if has_chemicals { 250.0 } else { 0.0 },
        format!("Estudio de Grado de Riesgo de Incendio NOM-002, diagrama unifilar NOM-029 y tierras físicas NOM-022 para giro {}.", scian),
    );

    add(
        "STPS",
        "8 NOMs de Salud (NOM-010 a 025)",
        "Estudios de Iluminación, Ruido y Agentes",
        true,
        true,
        "MEDIA",
        250,
        5000,
        280.0,
        format!("Reconocimiento y evaluación de niveles de iluminación en luxes (NOM-025) y ruido laboral (NOM-011) en {}.", razon_social),
    );

    add(
        "STPS",
        "7 NOMs Específicas (NOM-003 a 032)",
        "Construcción NOM-031 y Ramas Especiales",
        is_construction,
        is_construction,
        "CRÍTICA",
        250,
        5000,
        600.0,
        format!("Evaluación sectorial por prefijo SCIAN {} (Obras de construcción NOM-031 / Ramas especiales).", scian),
    );

    // ==================== II. SAT / CFF (6 MÓDULOS FISCALES) ====================
    let is_69b_bad = !sat_69b_status.to_uppercase().contains("LIMPIO");
    add(
        "SAT",
        "Art. 69-B CFF • SCJN 2a./J. 161/2019",
        "Materialidad B2B, Fecha Cierta y Listas 69-B",
        pays_intangibles || is_69b_bad,
        true,
        "CRÍTICA",
        450,
        4500,
        if is_69b_bad { 2200.0 } else { 540.0 },
        format!("Compulsa SAT 69-B: [{}]. Exigibilidad de contratos con Fecha Cierta y dossier de entregables tangibles.", sat_69b_status),
    );

    add(
        "SAT",
        "Art. 5-A del CFF",
        "Razón de Negocios y Beneficio Económico",
        pays_intangibles,
        true,
        "ALTA",
        300,
        2500,
        320.0,
        "Memoria económica que acredite que el beneficio cuantificable esperado es mayor al beneficio fiscal.".to_string(),
    );

    let is_69_bad = !sat_69_status.to_uppercase().contains("LIMPIO");
    add(
        "SAT",
        "Arts. 17-K, 86-C y 86-D CFF",
        "Buzón Tributario y Medios de Contacto",
        is_69_bad,
        true,
        "ALTA",
        35,
        102,
        102.0,
        format!("Estatus Art. 69 CFF: [{}]. Verificación de correo electrónico y teléfono móvil activos en Buzón Tributario.", sat_69_status),
    );

    add(
        "SAT",
        "Arts. 29 y 29-A CFF • CFDI 4.0 / Carta Porte",
        "CFDI 4.0, REP (Día 5) y Carta Porte 3.1",
        true,
        true,
        "ALTA",
        170,
        995,
        if is_transport_or_wholesale { 590.0 } else { 210.0 },
        format!("Conciliación de CFDI 4.0, Complemento de Recepción de Pagos y Carta Porte 3.1 aplicable al giro {}.", scian),
    );

    add(
        "SAT",
        "Art. 28 y Arts. 32-B Ter a Quinquies CFF",
        "Contabilidad Electrónica y Beneficiario Controlador",
        true,
        true,
        "CRÍTICA",
        1300,
        17600,
        680.0,
        format!("Expediente notarizado e identificado de Beneficiarios Controladores de {} (multa de hasta $2,000,000 por beneficiario).", razon_social),
    );

    add(
        "SAT",
        "Arts. 27, 17-H Bis y 32-D CFF",
        "Domicilio Localizado y Blindaje de Sellos CSD",
        is_69_bad,
        true,
        "ALTA",
        40,
        120,
        120.0,
        "Prevención de las 12 causales de restricción temporal de Certificado de Sello Digital (Art. 17-H Bis CFF) y aviso de socios.".to_string(),
    );

    // ==================== III. IMSS E INFONAVIT (6 MÓDULOS PATRONALES) ====================
    add(
        "IMSS",
        "Arts. 15 y 27 a 30 LSS • Art. 29 LINFONAVIT",
        "Conciliación DENUE vs. SUA y Salario Base (SBC)",
        true,
        true,
        "CRÍTICA",
        20,
        350,
        290.0,
        format!("Confronta de {} trabajadores estimados en giro {} contra CFDI de Nómina y SUA para evitar Capitales Constitutivos.", emp_count, scian),
    );

    add(
        "IMSS",
        "Art. 15-A LSS • Art. 1004-C LFT (REPSE)",
        "Subcontratación REPSE, ICSOE y SISSUB",
        uses_subcontracting && !repse_registered,
        uses_subcontracting,
        "CRÍTICA",
        500,
        50000,
        620.0,
        format!("Padrón REPSE: [{}]. Verificación mensual de los 7 comprobantes del Art. 27 Fracc. V LISR e informativas ICSOE/SISSUB.", if repse_registered { "ACTIVO" } else { "VALIDAR PROVEEDORES" }),
    );

    add(
        "IMSS",
        "Arts. 71 a 75 LSS y Art. 196 RACERF",
        "Clasificación y Prima de Riesgos de Trabajo",
        true,
        true,
        "MEDIA",
        20,
        210,
        180.0,
        format!("Por giro SCIAN {}, corresponde {} con Prima Media del {} (revisión anual obligatoria en febrero).", scian, imss_clase, imss_prima),
    );

    add(
        "IMSS",
        "Art. 39 LSS, Art. 32-D CFF y Buzón IMSS",
        "Confronta EMA/EBA y Opinión 32-D Seguridad Social",
        true,
        true,
        "ALTA",
        20,
        350,
        210.0,
        "Descarga mensual de Opinión 32-D positiva del IMSS/INFONAVIT y confronta de cédulas EMA/EBA antes del día 17.".to_string(),
    );

    add(
        "IMSS",
        "Art. 15-A párr. 3° y Art. 15 Fracc. VI LSS",
        "Responsabilidad Solidaria y Registro SIROC",
        uses_subcontracting || is_construction,
        uses_subcontracting || is_construction,
        "ALTA",
        20,
        350,
        260.0,
        "Cláusula de retención de pagos a contratistas condicionada a la entrega de pago SUA y alta de obra SIROC.".to_string(),
    );

    add(
        "IMSS",
        "Arts. 304-A, 304-B y 304-C LSS",
        "Cumplimiento Espontáneo sin Multa (Art. 304-C)",
        false,
        true,
        "OPTIMIZADO",
        20,
        350,
        0.0,
        "Protocolo de regularización espontánea previa a orden de visita domiciliaria conforme al Art. 304-C LSS.".to_string(),
    );

    // ==================== IV. SABG / LFPDPPP (6 MÓDULOS DE PRIVACIDAD) ====================
    add(
        "LFPDPPP",
        "Arts. 8, 15 y 16 LFPDPPP • Lineamientos DOF",
        "Aviso de Privacidad Integral, Simplificado y Corto",
        !has_privacy_notice,
        true,
        "CRÍTICA",
        100,
        160000,
        460.0,
        format!("Escaneo DOM web: Aviso de Privacidad Integral [{}].", if has_privacy_notice { "LOCALIZADO" } else { "OMISIÓN DETECTADA" }),
    );

    add(
        "LFPDPPP",
        "Arts. 22 al 35 LFPDPPP • Arts. 87-110 RLFPDPPP",
        "Ventanilla y Protocolo de Derechos ARCO (20 Días)",
        !has_arco_mention,
        true,
        "ALTA",
        200,
        320000,
        390.0,
        format!("Mención expresa de canal y plazos ARCO en portal digital: [{}].", if has_arco_mention { "VERIFICADA" } else { "NO DETECTADA" }),
    );

    add(
        "LFPDPPP",
        "Arts. 9 y 64 Fracc. IV LFPDPPP",
        "Consentimiento Expreso para Biométricos y Salud",
        uses_biometrics,
        true,
        "CRÍTICA",
        400,
        640000,
        580.0,
        "Autorización autógrafa obligatoria para uso de reloj checador biométrico (huella/rostro), estados de salud y cuentas bancarias.".to_string(),
    );

    add(
        "LFPDPPP",
        "Arts. 19 y 20 LFPDPPP • Arts. 57-66 RLFPDPPP",
        "Documento de Seguridad Interno y Cifrado TLS",
        !has_ssl,
        true,
        "ALTA",
        200,
        320000,
        410.0,
        format!("Cifrado TLS/SSL en dominio: [{}]. Se requiere Documento de Seguridad con inventario de bases de datos.", if has_ssl { "TLS ACTIVO" } else { "VULNERABLE" }),
    );

    add(
        "LFPDPPP",
        "Arts. 36 y 37 LFPDPPP • Arts. 49-56 RLFPDPPP",
        "Contratos de Remisión a Encargados (Contabilidad/Cloud)",
        true,
        true,
        "MEDIA",
        200,
        320000,
        240.0,
        "Cláusulas de encargo de tratamiento de datos con despachos contables externos, maquila de nómina y proveedores CRM.".to_string(),
    );

    add(
        "LFPDPPP",
        "Art. 30 LFPDPPP",
        "Designación Formal del Oficial de Privacidad",
        !has_arco_mention,
        true,
        "MEDIA",
        100,
        160000,
        190.0,
        "Acta societaria interna de designación del Oficial o Departamento de Protección de Datos Personales.".to_string(),
    );

    // ==================== V. 9 SECTORIALES Y PLD (6 MÓDULOS POR GIRO SCIAN) ====================
    add(
        "SHCP / UIF",
        "Sectorial 01 • LFPIORPI Arts. 17, 18, 32 y 53",
        "Prevención de Lavado de Dinero (Actividades Vulnerables)",
        is_pld_sector,
        is_pld_sector,
        "CRÍTICA",
        10000,
        65000,
        950.0,
        format!("Giro SCIAN {} sujeto a verificación de umbrales del Art. 17 LFPIORPI, alta en SPPLD y prohibición de efectivo (Art. 32).", scian),
    );

    add(
        "COFEPRIS",
        "Sectorial 02 • LGS Art. 200 Bis • NOM-251 / NOM-051",
        "Aviso DIGIPRIS, Higiene NOM-251 y Etiquetado",
        is_food_or_health,
        is_food_or_health,
        "CRÍTICA",
        2000,
        16000,
        640.0,
        format!("Giro Alimentos/Salud (SCIAN {}): obligatorio Aviso de Funcionamiento DIGIPRIS y bitácoras sanitarias NOM-251-SSA1.", scian),
    );

    add(
        "PROT. CIVIL",
        "Sectorial 03 • LGPC Arts. 39, 40 y 74 a 78",
        "Programa Interno de Protección Civil (PIPC) y Brigadas",
        true,
        true,
        "CRÍTICA",
        1000,
        20000,
        510.0,
        format!("Carpeta PIPC firmada por Tercer Acreditado, Acta de 4 Brigadas, Póliza RC y dictámenes eléctrico/gas en {}.", razon_social),
    );

    add(
        "SEMARNAT",
        "Sectorial 04 • LGEEPA Art. 171 • LGPGIR Arts. 40-48",
        "Registro de Generador de Residuos y Manifiestos",
        is_industrial_or_workshop || is_food_or_health,
        is_industrial_or_workshop || is_food_or_health,
        "ALTA",
        30,
        50000,
        430.0,
        format!("Registro de generador de residuos peligrosos/manejo especial, COA y bitácoras aplicables al giro SCIAN {}.", scian),
    );

    add(
        "SICT",
        "Sectorial 05 • NOM-087-SCT-2, NOM-012 y NOM-068",
        "Autotransporte Federal, Flotillas y Bitácora Horas",
        is_transport_or_wholesale,
        is_transport_or_wholesale,
        "ALTA",
        100,
        500,
        310.0,
        format!("Bitácora de horas de conducción NOM-087-SCT, dictamen físico-mecánico NOM-068 y logística para SCIAN {}.", scian),
    );

    add(
        "SECTORIAL",
        "Sectoriales 06 al 09 • REPSE / SSPC / SEP / SECTUR",
        "Padrones Federales Especiales y Compulsa SFP",
        sfp_sanctioned || scian.starts_with("56") || scian.starts_with("721") || scian.starts_with("61"),
        true,
        "MEDIA",
        500,
        7000,
        270.0,
        format!("Directorio SFP Inhabilitados: [{}]. Verificación de autorizaciones sectoriales por giro {}.", if sfp_sanctioned { "ALERTA SANCIONADO" } else { "SIN SANCIONES" }, scian),
    );

    // ==================== VI. PROFECO Y ECONOMÍA (5 MÓDULOS COMERCIALES) ====================
    add(
        "PROFECO",
        "Arts. 76 BIS y 7 BIS LFPC • NMX-COE-001",
        "Comercio Digital, Precios Totales con IVA y Domicilio",
        !has_profeco_mention,
        true,
        "ALTA",
        100,
        35000,
        320.0,
        "Exhibición de Razón Social, domicilio físico en México, teléfonos y precios totales con IVA incluido (Art. 7 BIS LFPC).".to_string(),
    );

    add(
        "PROFECO",
        "Arts. 56 y 77 a 82 de la LFPC",
        "Garantía Mínima de 90 Días y Derecho de Revocación",
        !has_profeco_mention,
        true,
        "MEDIA",
        100,
        33900,
        240.0,
        "Póliza de garantía escrita no inferior a 90 días naturales (Art. 77 LFPC) y plazo de revocación de 5 días hábiles.".to_string(),
    );

    add(
        "PROFECO",
        "Arts. 85, 86, 87 y 90 LFPC (RRPAC)",
        "Contratos de Adhesión y Purga de Cláusulas Nulas",
        !has_profeco_mention,
        true,
        "ALTA",
        100,
        33900,
        280.0,
        "Purga de cláusulas leoninas o modificaciones unilaterales prohibidas por el Artículo 90 de la LFPC.".to_string(),
    );

    add(
        "PROFECO",
        "Arts. 32 y 46 de la LFPC",
        "Publicidad Veraz, Comprobable y Promociones",
        false,
        true,
        "OPTIMIZADO",
        100,
        33900,
        0.0,
        "Veracidad comprobable de ofertas comerciales, vigencias explícitas y exclusión de leyendas ambiguas.".to_string(),
    );

    add(
        "ECONOMÍA",
        "Art. 128 BIS LFPC • CCom Arts. 89-93 y NOM-151",
        "Seguridad Transaccional e Integridad Criptográfica",
        !has_ssl,
        true,
        "ALTA",
        1700,
        48600,
        350.0,
        "Sellado criptográfico SHA-256 / Árbol de Merkle para dotar de integridad inmutable y Fecha Cierta a contratos electrónicos.".to_string(),
    );

    v
}
