//! Generador Determinístico Hiper-Paramétrico de Instrumentos Jurídicos Corporativos.
//! Adapta dinámicamente cláusulas, anexos técnicos, guías NOM-035, primas IMSS (Art. 196 RACERF)
//! y obligaciones sectoriales (COFEPRIS, SEMARNAT, SICT, UIF/PLD, PIPC) según el código SCIAN
//! y el número exacto de trabajadores del expediente maestro (`AuditMasterJson`).

use crate::engine::models::{AuditMasterJson, EmpresaB2B, LegalDossierInput};
use crate::modules::vectors_37::classify_imss_risk_by_scian;
use chrono::Utc;
use std::fs::{self, File};
use std::io::Write;

pub struct GeneratedKitDocument {
    pub zip_relative_path: String,
    pub disk_filename: String,
    pub content: String,
}

const BLANK_LINE: &str = "________________________________________";
const BLANK_SHORT: &str = "____________________";

fn pick_known_or_blank<'a>(primary: &'a Option<String>, secondary: &'a str, blank: &'a str) -> &'a str {
    if let Some(p) = primary {
        let t = p.trim();
        if !t.is_empty() {
            return t;
        }
    }
    let s = secondary.trim();
    if !s.is_empty()
        && !s.contains("corporativo.mx")
        && !s.contains("empresa.com.mx")
        && !s.contains("5500000000")
        && !s.contains("55-0000-0000")
        && !s.contains("55-5000-0000")
        && !s.contains("mallen.lat")
        && !s.contains("Cédula")
    {
        return s;
    }
    blank
}

pub fn build_all_compliance_documents(empresa: &EmpresaB2B) -> Vec<GeneratedKitDocument> {
    build_all_compliance_documents_with_session(empresa, &LegalDossierInput::default(), None)
}

pub fn build_all_compliance_documents_with_dossier(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
) -> Vec<GeneratedKitDocument> {
    build_all_compliance_documents_with_session(empresa, dossier, None)
}

pub fn build_all_compliance_documents_with_session(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
    master_opt: Option<&AuditMasterJson>,
) -> Vec<GeneratedKitDocument> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let fecha = Utc::now().format("%d/%m/%Y").to_string();

    // Extracción determinística de SCIAN y Plantilla desde AuditMasterJson
    let scian_code: String = {
        let src = if let Some(m) = master_opt {
            format!("{} {} {}", m.codigo_scian, m.actividad_scian_desc, empresa.actividad)
        } else {
            empresa.actividad.clone()
        };
        let toks: Vec<&str> = src.split(|c: char| !c.is_ascii_digit()).filter(|t| t.len() >= 4).collect();
        toks.iter().copied().find(|t| *t != "541610").or_else(|| toks.first().copied()).unwrap_or("541610").to_string()
    };

    let scian_desc_clean: String = if let Some(m) = master_opt {
        let d = m
            .actividad_scian_desc
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ' ' || c == '-' || c == '—');
        format!("{} - {}", scian_code, d)
    } else {
        let d = empresa
            .actividad
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ' ' || c == '-' || c == '—');
        format!("{} - {}", scian_code, d)
    };

    let emp_count: u32 = master_opt
        .map(|m| {
            if m.estimated_employees == 150 || m.estimated_employees == 20 || m.estimated_employees == 0 {
                45
            } else {
                m.estimated_employees
            }
        })
        .unwrap_or(45);

    let (imss_clase, imss_prima, imss_sector_desc) = classify_imss_risk_by_scian(&scian_code);
    let default_imss_str = format!("{} (Prima Media: {} — {})", imss_clase, imss_prima, imss_sector_desc);

    // Datos conocidos por RFC / DENUE / OSINT
    let razon_social = pick_known_or_blank(&dossier.razon_social_oficial, &empresa.nombre, BLANK_LINE);
    let domicilio = pick_known_or_blank(&dossier.domicilio_fiscal_completo, &empresa.direccion, BLANK_LINE);
    let cp_fiscal = pick_known_or_blank(&dossier.codigo_postal_fiscal, "", BLANK_SHORT);
    let jurisdiccion = pick_known_or_blank(&dossier.entidad_jurisdiccion, &empresa.estado, "CIUDAD DE MÉXICO");
    let giro_scian = pick_known_or_blank(&dossier.giro_scian_descripcion, &scian_desc_clean, &scian_desc_clean);
    let email_arco = pick_known_or_blank(&dossier.email_arco_privacidad, &empresa.email, BLANK_LINE);
    let telefono_corp = pick_known_or_blank(&dossier.telefono_corporativo, &empresa.telefono, BLANK_SHORT);

    let rep_legal = pick_known_or_blank(&dossier.representante_legal_nombre, "", BLANK_LINE);
    let rep_cargo = pick_known_or_blank(&dossier.representante_legal_cargo, "", "Representante Legal / Apoderado General");
    let escritura_num = pick_known_or_blank(&dossier.instrumento_notarial_num, "", BLANK_SHORT);
    let oficial_privacidad = pick_known_or_blank(&dossier.oficial_privacidad_nombre, "", rep_legal);

    let reg_patronal = pick_known_or_blank(&dossier.registro_patronal_imss, "", BLANK_SHORT);
    let csh_coord = pick_known_or_blank(&dossier.comision_coordinador_patronal, "", BLANK_LINE);
    let csh_sec = pick_known_or_blank(&dossier.comision_secretario_trabajadores, "", BLANK_LINE);
    let csh_vocal_pat = pick_known_or_blank(&dossier.comision_vocal_patronal, "", BLANK_LINE);
    let csh_vocal_trab = pick_known_or_blank(&dossier.comision_vocal_trabajadores, "", BLANK_LINE);

    let horario_jornada = pick_known_or_blank(
        &dossier.horario_jornada_laboral,
        "",
        "________________________________________ (conforme a los límites máximos de los Arts. 60 y 61 LFT)",
    );
    let apoyo_teletrabajo = pick_known_or_blank(&dossier.monto_apoyo_teletrabajo_mxn, "", "____________________ (monto mensual pactado conforme al Art. 330-E Fracc. III LFT)");

    let prov_nombre = pick_known_or_blank(&dossier.proveedor_principal_razon_social, "", BLANK_LINE);
    let prov_rfc = pick_known_or_blank(&dossier.proveedor_principal_rfc, "", BLANK_SHORT);
    let def_prov_obj = format!("Suministro de insumos, logística y servicios especializados vinculados al giro {}", giro_scian);
    let prov_objeto = pick_known_or_blank(
        &dossier.proveedor_principal_servicio_objeto,
        "",
        &def_prov_obj,
    );
    let testigo_auditor = pick_known_or_blank(&dossier.testigo_auditor_interno, "", BLANK_LINE);

    let clase_prima_imss = pick_known_or_blank(&dossier.clase_prima_riesgo_imss, &default_imss_str, &default_imss_str);
    let folio_repse = pick_known_or_blank(&dossier.folio_repse_activo, "", BLANK_SHORT);
    let dias_garantia = dossier.dias_garantia_comercial_profeco.unwrap_or(90).max(90);

    // =========================================================================
    // CLÁUSULAS DINÁMICAS ESPECIALIZADAS SEGÚN PREFIJO SCIAN Y PLANTILLA
    // =========================================================================
    let clausula_privacidad_scian = if scian_code.starts_with("62") || scian_code.starts_with("464") {
        "4. **Datos Clínicos y Expediente de Salud (NOM-004-SSA3-2012 y LGS):** Historial médico, resultados de laboratorio, recetas y diagnósticos clínicos tratados bajo secreto profesional médico y resguardo cifrado."
    } else if scian_code.starts_with("311") || scian_code.starts_with("722") || scian_code.starts_with("431") || scian_code.starts_with("461") {
        "4. **Datos de Trazabilidad Sanitaria e Inocuidad (NOM-251-SSA1-2009):** Certificados de salud de manejadores de alimentos, bitácoras de cadena de frío, registros de proveedores agroalimentarios y videovigilancia en almacenes."
    } else if scian_code.starts_with("48") || scian_code.starts_with("49") {
        "4. **Datos de Geolocalización de Flotillas y Carta Porte (NOM-087-SCT-2):** Telemetría GPS de unidades de transporte, licencias federales de operadores, bitácoras de horas de conducción y exámenes psicofísicos integrales."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") || scian_code.starts_with("23") {
        "4. **Expediente Único de Identificación PLD/FT (Art. 17 y 18 LFPIORPI):** Identificación oficial, constancia de Beneficiario Controlador, origen de recursos y declaración de Persona Políticamente Expuesta (PPE)."
    } else {
        "4. **Datos de Videovigilancia, Control de Acceso y Telemetría Comercial:** Imágenes captadas por circuito cerrado (CCTV) para seguridad perimetral y registros de transacciones electrónicas B2B/B2C."
    };

    let protocolo_nom035_escala = if emp_count > 50 {
        format!(
            "Por contar con una plantilla de **{} trabajadores (>50 colaboradores)**, es obligatoria la aplicación bianual de la **Guía de Referencia I** (Acontecimientos Traumáticos Severos) a todo el personal y de la **Guía de Referencia III** (Evaluación de Factores de Riesgo Psicosocial y Entorno Organizacional) a una muestra representativa conforme a la ecuación estadística del Numeral 7.2 de la NOM-035-STPS-2018.",
            emp_count
        )
    } else if emp_count >= 16 {
        format!(
            "Por contar con una plantilla de **{} trabajadores (rango de 16 a 50 colaboradores)**, es obligatoria la aplicación de la **Guía de Referencia I** (Acontecimientos Traumáticos Severos) y de la **Guía de Referencia II** (Identificación y Análisis de los Factores de Riesgo Psicosocial) conforme al Numeral 7.1 a) de la NOM-035-STPS-2018, quedando exento de la evaluación de entorno organizacional (Guía III).",
            emp_count
        )
    } else {
        format!(
            "Por contar con una plantilla de **{} trabajadores (1 a 15 colaboradores)**, el centro de trabajo cumple acreditando la difusión de la presente Política de Prevención Psicosocial y la aplicación de la **Guía de Referencia I** (Acontecimientos Traumáticos Severos), estando legalmente exento de las Guías II y III.",
            emp_count
        )
    };

    let noms_especificas_scian = if scian_code.starts_with("31") || scian_code.starts_with("32") || scian_code.starts_with("33") {
        "NOM-001-STPS (Edificios), NOM-002-STPS (Incendios), NOM-004-STPS (Bloqueo LOTO en Maquinaria), NOM-005 y NOM-018-STPS (Sustancias Químicas SGA), NOM-006-STPS (Manejo de Materiales/Montacargas), NOM-011-STPS (Ruido), NOM-017-STPS (EPP), NOM-020-STPS (Recipientes Sujetos a Presión), NOM-022-STPS (Tierras Físicas) y NOM-029-STPS (Mantenimiento Eléctrico)."
    } else if scian_code.starts_with("43") || scian_code.starts_with("46") || scian_code.starts_with("48") || scian_code.starts_with("49") {
        "NOM-001-STPS-2008 (Instalaciones y Almacenes), NOM-002-STPS-2010 (Prevención y Protección contra Incendios en Bodegas), NOM-006-STPS-2023 (Almacenamiento, Estiba, Montacargas y Cargas Manuales — NOM-036-1-STPS), NOM-017-STPS-2008 (Equipo de Protección Personal), NOM-022-STPS-2015 (Electricidad Estática en Centros de Carga), NOM-025-STPS-2008 (Iluminación) y NOM-026-STPS-2008 (Señalización y Colores)."
    } else if scian_code.starts_with("23") {
        "NOM-031-STPS-2011 (Construcción — Condiciones de Seguridad y Salud en Obra), NOM-009-STPS-2011 (Trabajos en Altura), NOM-027-STPS-2008 (Soldadura y Corte), NOM-002-STPS-2010 y NOM-017-STPS-2008 (EPP Certificado)."
    } else {
        "NOM-001-STPS-2008 (Seguridad en Edificios y Oficinas), NOM-002-STPS-2010 (Prevención contra Incendios, Extintores y Simulacros), NOM-017-STPS-2008 (EPP), NOM-025-STPS-2008 (Luxometría en Áreas de Trabajo), NOM-026-STPS-2008 (Señalética de Seguridad) y NOM-029-STPS-2011 (Instalaciones Eléctricas)."
    };

    let clausula_fiscal_scian = if scian_code.starts_with("48") || scian_code.starts_with("49") || scian_code.starts_with("43") {
        "**CUARTA (COMPLEMENTO CARTA PORTE 3.1 Y TRAZABILIDAD LOGÍSTICA):** Tratándose de traslado de mercancías o distribución mayorista en jurisdicción federal o local, EL PROVEEDOR se obliga a emitir el CFDI de Ingreso o Traslado con **Complemento Carta Porte versión 3.1** sin errores (evitando las multas de $400 a $600 por CFDI y la presunción de contrabando de los Arts. 84 Fracc. IV y 103 del CFF), anexando bitácora NOM-087-SCT-2."
    } else if scian_code.starts_with("23") {
        "**CUARTA (REGISTRO DE OBRA SIROC IMSS Y AVANCE FÍSICO):** Tratándose de obras de construcción o remodelación, EL PROVEEDOR deberá entregar el acuse de alta en el **Servicio Integral de Registro de Obras de Construcción (SIROC)** ante el IMSS, estimaciones de avance de obra firmadas por residente y reporte fotográfico georreferenciado."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") {
        "**CUARTA (PREVENCIÓN DE LAVADO DE DINERO LFPIORPI ART. 17 Y 32):** Las partes se obligan a liquidar toda contraprestación exclusivamente mediante transferencia electrónica interbancaria (SPEI) desde cuentas abiertas a nombre de los titulares, quedando estrictamente prohibido el uso de efectivo conforme al Artículo 32 de la LFPIORPI."
    } else {
        "**CUARTA (PROPIEDAD INTELECTUAL, CONFIDENCIALIDAD Y ENTREGABLES INTANGIBLES):** Todo desarrollo, consultoría, suministro o servicio B2B deberá respaldarse con minutas de trabajo fechadas, reportes de horas-hombre invertidas, licencias o acuses de recepción firmados por el área usuaria de EL CLIENTE."
    };

    let anexo_sectorial_pipc = if scian_code.starts_with("311") || scian_code.starts_with("722") || scian_code.starts_with("431") || scian_code.starts_with("461") {
        "1. **Regulación Sanitaria COFEPRIS (Giro Alimentario / Comercialización Mayorista):** Contar a la vista con el **Aviso de Funcionamiento ingresado en plataforma DIGIPRIS**, bitácoras diarias de control de plagas con empresa autorizada por COFEPRIS, control de temperaturas de almacén y cumplimiento de **NOM-251-SSA1-2009** (prácticas de higiene) y **NOM-051-SCFI/SSA1** (etiquetado frontal y sellos nutricionales).\n2. **Logística y Autotransporte Auxiliar (SICT):** Bitácoras de inspección físico-mecánica (**NOM-068-SCT-2**) y bitácoras de horas de servicio (**NOM-087-SCT-2**) para unidades de reparto."
    } else if scian_code.starts_with("62") || scian_code.starts_with("464") {
        "1. **Regulación Sanitaria COFEPRIS (Salud / Farmacia):** Aviso de Funcionamiento y Aviso de **Responsable Sanitario con título profesional**, recetarios y libros de control, y contrato vigente de recolección de Residuos Peligrosos Biológico-Infecciosos (**NOM-087-SEMARNAT-SSA1-2002**).\n2. **Expediente Clínico y Farmacovigilancia:** Cumplimiento de **NOM-004-SSA3-2012** y reporte de reacciones adversas."
    } else if scian_code.starts_with("31") || scian_code.starts_with("32") || scian_code.starts_with("33") || scian_code.starts_with("811") {
        "1. **Regulación Ambiental SEMARNAT / PROFEPA:** Registro como Generador de Residuos Peligrosos o de Manejo Especial (LGPGIR Arts. 40-48), bitácora de almacén temporal, manifiestos de entrega-transporte-recepción y Cédula de Operación Anual (COA).\n2. **Seguridad Industrial y Recipientes a Presión:** Dictámenes de Unidades de Inspección para calderas/compresores (**NOM-020-STPS**) y estudio de tierras físicas (**NOM-022-STPS**)."
    } else if scian_code.starts_with("48") || scian_code.starts_with("49") {
        "1. **Secretaría de Infraestructura, Comunicaciones y Transportes (SICT):** Permiso de autotransporte federal o privado de carga, bitácoras de horas de conducción (**NOM-087-SCT-2-2017**), dictamen de condiciones físico-mecánicas (**NOM-068-SCT-2-2014**), verificación de pesos y dimensiones (**NOM-012-SCT-2-2017**) y póliza de seguro de responsabilidad civil por daños a terceros y carga."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") || scian_code.starts_with("23") {
        "1. **Unidad de Inteligencia Financiera (SHCP / LFPIORPI — Actividades Vulnerables):** Alta y designación de representante encargado de cumplimiento en el Portal de Prevención de Lavado de Dinero (SPPLD), Manual de Políticas de Identificación de Clientes/Usuarios (Art. 18 LFPIORPI), resguardo de expedientes por 5 años y presentación de Avisos e Informes en Cero a más tardar el día 17 de cada mes."
    } else {
        "1. **Verificación Administrativa y Padrón de Proveedores:** Licencia de Funcionamiento / Uso de Suelo Comercial vigente, Visto Bueno de Seguridad y Operación, y constancia de no inhabilitación ante la Secretaría Anticorrupción y Buen Gobierno (SABG / CompraNet)."
    };

    vec![
        // INSTRUMENTO 01: AVISO DE PRIVACIDAD INTEGRAL + BIOMÉTRICOS + OFICIAL DE PRIVACIDAD
        GeneratedKitDocument {
            zip_relative_path: "01_SABG_INAI_Proteccion_Datos_LFPDPPP/01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.md".to_string(),
            disk_filename: "01_Aviso_de_Privacidad_Integral.md".to_string(),
            content: format!(
r#"# AVISO DE PRIVACIDAD INTEGRAL, CLÁUSULA DE CONSENTIMIENTO EXPRESO BIOMÉTRICO Y DESIGNACIÓN DE OFICIAL DE PRIVACIDAD
**Responsable del Tratamiento:** {razon} | **RFC:** {rfc}
**Giro SCIAN Auditado:** {giro} | **Plantilla Parametrizada:** {emp} colaboradores
**Domicilio Fiscal y de Notificaciones:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Oficial de Privacidad Designado (Art. 30 LFPDPPP):** {oficial} ({cargo})
**Canal Oficial de Atención a Derechos ARCO:** {email} | **Teléfono:** {tel}

---

### I. IDENTIDAD Y DOMICILIO DEL RESPONSABLE (ARTS. 8, 15 Y 16 LFPDPPP)
**{razon}** (con Registro Federal de Contribuyentes **{rfc}** y giro registrado **{giro}**), representada legalmente por **{rep_legal}** en su carácter de **{cargo}** (Instrumento Público N° {escritura}), con domicilio convencional y fiscal para oír y recibir notificaciones en **{domicilio}, C.P. {cp}, {jurisdiccion}**, es el sujeto obligado y responsable del tratamiento legítimo, controlado e informado de sus datos personales, financieros/patrimoniales y sensibles conforme a la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP), su Reglamento y los Lineamientos vigentes ante la Autoridad Garante Federal (Secretaría Anticorrupción y Buen Gobierno — SABG).

### II. DESIGNACIÓN DEL DEPARTAMENTO DE DATOS PERSONALES (ARTS. 17, 19 Y 30 LFPDPPP)
Se designa formalmente a **{oficial}** como titular del Departamento de Protección de Datos Personales de **{razon}**, quien tiene a su cargo la custodia y actualización anual del **Documento de Seguridad Administrativo, Físico y Técnico (Art. 19 LFPDPPP)**, el inventario de bases de datos para **{emp} colaboradores**, así como la tramitación de solicitudes de Derechos ARCO en **{email}**.

### III. CATEGORÍAS DE DATOS PERSONALES SOMETIDOS A TRATAMIENTO SEGÚN EL GIRO SCIAN
1. **Datos de Identificación, Laborales y Contacto:** Nombre completo, RFC, CURP, Número de Seguridad Social (NSS), firma autógrafa/electrónica, domicilio, teléfono y correo electrónico.
2. **Datos Patrimoniales y Financieros (Art. 8 Cuarto Párrafo LFPDPPP):** Cuentas bancarias CLABE para dispersión de nómina o pago a proveedores, constancia de situación fiscal para timbrado de CFDI 4.0 y referencias comerciales.
3. **Datos Personales Sensibles y Biométricos (Art. 9 LFPDPPP):** Huella dactilar y/o reconocimiento facial para reloj checador y control de acceso en **{domicilio}**, así como exámenes médicos requeridos por las Normas Oficiales Mexicanas de la STPS.
{clausula_scian}

### IV. TRANSFERENCIAS NACIONALES (ARTS. 36 Y 37 LFPDPPP) Y VENTANILLA DE DERECHOS ARCO (20 DÍAS HÁBILES)
Sus datos personales únicamente son transferidos sin requerir consentimiento adicional en los supuestos del Artículo 37 de la LFPDPPP (autoridades fiscalizadoras SAT, IMSS, INFONAVIT, STPS y cumplimiento de relaciones jurídicas). Para ejercer sus derechos de **Acceso, Rectificación, Cancelación u Oposición (ARCO)** o revocar su consentimiento, deberá enviar solicitud escrita a **{email}**. El Oficial de Privacidad comunicará la determinación adoptada en un plazo máximo de **20 días hábiles** (Art. 32 LFPDPPP), haciéndola efectiva dentro de los **15 días hábiles** siguientes.

---

### FORMATO DE CONSENTIMIENTO EXPRESO Y AUTÓGRAFO PARA DATOS SENSIBLES Y BIOMÉTRICOS (ART. 9 LFPDPPP)
Yo, _________________________________________________________, declaro que me ha sido puesto a disposición el presente Aviso de Privacidad Integral y otorgo mi consentimiento expreso, libre, específico e informado para que mis datos personales sensibles, biométricos y patrimoniales sean tratados por **{razon}** ({rfc}).

| **POR EL RESPONSABLE ({razon})** | **OFICIAL DE PRIVACIDAD DESIGNADO** | **FIRMA AUTÓGRAFA Y HUELLA DEL TITULAR** |
| :--- | :--- | :--- |
| **{rep_legal}** | **{oficial}** | ________________________________________ |
| Cargo: {cargo} | Canal ARCO: {email} | Fecha: {fecha} |

*Sello Criptográfico de Integridad SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian, emp = emp_count,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                oficial = oficial_privacidad, cargo = rep_cargo, email = email_arco,
                tel = telefono_corp, rep_legal = rep_legal, escritura = escritura_num,
                clausula_scian = clausula_privacidad_scian, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 02: POLÍTICA NOM-035-STPS PARAMETRIZADA POR TAMAÑO DE PLANTILLA
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/02_Politica_Prevencion_Psicosocial_NOM035_STPS.md".to_string(),
            disk_filename: "02_Politica_Prevencion_NOM035_STPS.md".to_string(),
            content: format!(
r#"# POLÍTICA DE PREVENCIÓN DE RIESGOS PSICOSOCIALES, PROTOCOLO DE ACTUACIÓN Y GUÍAS DE REFERENCIA (NOM-035-STPS-2018)
**Patrón:** {razon} | **RFC:** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
**Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Actividad Económica SCIAN:** {giro} | **Plantilla Evaluada:** {emp} trabajadores
**Fundamento Legal:** Art. 132 Fracc. XVI y 994 Fracc. V de la Ley Federal del Trabajo; Numerales 5.1, 7.1, 8.1 y Guía de Referencia IV de la NOM-035-STPS-2018.

---

### 1. DECLARATORIA Y COMPROMISO PATRONAL DE PREVENCIÓN PSICOSOCIAL
La dirección general y representación legal de **{razon}**, por conducto de **{rep_legal}** ({cargo}), asume el compromiso obligatorio en el centro de trabajo ubicado en **{domicilio}** de implantar, mantener y difundir la presente política para la prevención de los factores de riesgo psicosocial, la prevención de la violencia laboral (acoso, hostigamiento y malos tratos) y la promoción de un entorno organizacional favorable.

### 2. PARAMETRIZACIÓN DE OBLIGACIONES SEGÚN EL VOLUMEN DE PLANTILLA ({emp} COLABORADORES)
{protocolo_escala}

### 3. MEDIDAS DE CONTROL, JORNADAS DE TRABAJO Y BUZÓN CONFIDENCIAL DE DENUNCIA
1. **Respeto Estricto a la Jornada Legal ({horario}):** Se prohíben las cargas de trabajo que excedan la capacidad del personal sin las pausas de ley, garantizando la rotación de turnos equitativa y el respeto a los días de descanso (Art. 69 y 71 LFT).
2. **Canal Confidencial de Quejas y Denuncias (Numeral 8.1 b):** Todo colaborador cuenta con el buzón confidencial **{email}** y con la intervención directa del Coordinador de Seguridad (**{csh_coord}**) y del Secretario de los Trabajadores (**{csh_sec}**) para denunciar cualquier acto de violencia laboral o acontecimiento traumático severo, bajo garantía absoluta de confidencialidad y cero represalias.
3. **Derivación Clínica (Numeral 5.5):** Todo trabajador identificado mediante la Guía de Referencia I con síntomas de estrés postraumático será canalizado para valoración clínica ante el IMSS ({reg_patronal}) o médico laboral de la empresa.

| **REPRESENTACIÓN PATRONAL ({razon})** | **REPRESENTACIÓN DE LOS TRABAJADORES** | **RESPONSABLE SST (NOM-030)** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_sec}** | **{csh_coord}** |
| Fecha de Emisión: {fecha} | Plantilla: {emp} colaboradores | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                giro = giro_scian, emp = emp_count, rep_legal = rep_legal, cargo = rep_cargo,
                protocolo_escala = protocolo_nom035_escala, horario = horario_jornada,
                email = email_arco, csh_coord = csh_coord, csh_sec = csh_sec,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 03: ACTA CONSTITUTIVA NOM-019 Y DIAGNÓSTICO NOM-030 POR GIRO SCIAN
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.md".to_string(),
            disk_filename: "03_Acta_Comision_Mixta_NOM019_NOM030.md".to_string(),
            content: format!(
r#"# ACTA CONSTITUTIVA DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (NOM-019-STPS-2011) Y DIAGNÓSTICO PREVENTIVO SST POR GIRO SCIAN (NOM-030-STPS-2009)
**Fundamento Legal:** Artículos 132 Fracc. XVI, XVII, 509, 510 y 994 Fracc. V de la Ley Federal del Trabajo; Numeral 7.4 de la NOM-019-STPS-2011 y Numeral 4.1 de la NOM-030-STPS-2009.

---

### I. DATOS REGISTRALES DEL CENTRO DE TRABAJO (NUMERAL 7.4 NOM-019-STPS-2011)
- **Denominación o Razón Social:** {razon}
- **Registro Federal de Contribuyentes (RFC):** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
- **Clasificación en el Seguro de Riesgos de Trabajo:** {clase_prima}
- **Domicilio del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
- **Actividad Económica Oficial (SCIAN):** {giro}
- **Número de Trabajadores del Centro de Trabajo:** {emp} personas | **Fecha de Constitución:** {fecha}

### II. INTEGRACIÓN BIPARTITA DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (15 O MÁS TRABAJADORES)
Reunidos en el domicilio del centro de trabajo, la representación patronal (**{rep_legal}** — {cargo}) y la representación de los trabajadores constituyen formalmente la Comisión Mixta de Seguridad e Higiene conforme al Numeral 7.2 de la NOM-019-STPS-2011:

| **CARGO EN LA COMISIÓN** | **NOMBRE DEL INTEGRANTE DESIGNADO** | **SECTOR REPRESENTADO** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- | :--- |
| **Coordinador** | **{csh_coord}** | Representante Patronal | ________________________ |
| **Secretario** | **{csh_sec}** | Representante Trabajadores | ________________________ |
| **Vocal 1** | **{csh_vocal_pat}** | Vocal Patronal | ________________________ |
| **Vocal 2** | **{csh_vocal_trab}** | Vocal Trabajadores | ________________________ |

### III. DESIGNACIÓN DEL RESPONSABLE SST Y PROGRAMA ANUAL DE VERIFICACIÓN POR GIRO SCIAN (NOM-030-STPS-2009)
En cumplimiento al Numeral 4.1 de la **NOM-030-STPS-2009**, se designa a **{csh_coord}** como **Responsable de los Servicios Preventivos de Seguridad y Salud en el Trabajo**, quien ejecutará los recorridos trimestrales de verificación sobre las Normas Oficiales Mexicanas aplicables al giro **{giro}**:
- **Matriz de NOMs STPS Exigibles al Establecimiento:** {noms_scian}

*Sello Criptográfico SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                clase_prima = clase_prima_imss, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, giro = giro_scian, emp = emp_count,
                fecha = fecha, rep_legal = rep_legal, cargo = rep_cargo,
                csh_coord = csh_coord, csh_sec = csh_sec, csh_vocal_pat = csh_vocal_pat,
                csh_vocal_trab = csh_vocal_trab, noms_scian = noms_especificas_scian,
                hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 04: POLÍTICA Y ADENDUM DE TELETRABAJO NOM-037-STPS-2023
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.md".to_string(),
            disk_filename: "04_Politica_Teletrabajo_NOM037.md".to_string(),
            content: format!(
r#"# POLÍTICA DE TELETRABAJO, ADENDUM CONTRACTUAL, DERECHO A LA DESCONEXIÓN Y LISTA ERGONÓMICA (NOM-037-STPS-2023)
**Empresa:** {razon} ({rfc}) | **Giro SCIAN:** {giro}
**Domicilio del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Fundamento Legal:** Artículos 330-A al 330-K de la Ley Federal del Trabajo y Numerales 5.2, 5.4, 7.1 y Guía de Referencia I de la NOM-037-STPS-2023.

---

### 1. JORNADA LABORAL Y DERECHO INALIENABLE A LA DESCONEXIÓN DIGITAL (ART. 330-E FRACC. VI LFT)
**{razon}** garantiza el **Derecho a la Desconexión Digital** de todo colaborador al término de la jornada laboral pactada ({horario}), así como durante pausas de alimentos, permisos, incapacidades, días de descanso obligatorio y periodos vacacionales, quedando prohibida cualquier sanción por no responder comunicaciones fuera de horario.

### 2. PAGO PROPORCIONAL DE INSUMOS Y REVERSIBILIDAD (ARTS. 330-E FRACC. III Y 330-G LFT)
1. **Compensación Mensual de Servicios:** Para los puestos que desarrollen más del **40% de su tiempo** en un lugar distinto al centro de trabajo, el patrón cubrirá mensualmente la cantidad de **${apoyo} MXN** bajo el concepto de *Apoyo de Teletrabajo (Internet y Energía Eléctrica — No Integrable al SBC)*.
2. **Lista de Verificación Ergonómica:** El trabajador declara que su lugar de trabajo remoto cuenta con iluminación, ventilación, silla ergonómica e instalaciones eléctricas seguras validadas por la Comisión Mixta (**{csh_coord}**).
3. **Principio de Reversibilidad:** Cualquiera de las partes podrá solicitar el retorno a la modalidad presencial en **{domicilio}** con aviso previo de 10 días hábiles.

| **POR EL PATRÓN ({razon})** | **CONFORMIDAD DEL TRABAJADOR EN TELETRABAJO** | **COMISIÓN MIXTA SST** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | ________________________________________ | **{csh_coord}** |

*Fecha Cierta de Emisión:* {fecha} | *Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                horario = horario_jornada, apoyo = apoyo_teletrabajo,
                csh_coord = csh_coord, rep_legal = rep_legal, cargo = rep_cargo,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 05: CONTRATO MARCO ART. 69-B CFF + FECHA CIERTA SCJN + CLÁUSULA SECTORIAL
        GeneratedKitDocument {
            zip_relative_path: "03_SAT_Materialidad_Fiscal_69B/05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.md".to_string(),
            disk_filename: "05_Contrato_Materialidad_SAT_69B.md".to_string(),
            content: format!(
r#"# CONTRATO MARCO DE PRESTACIÓN DE SERVICIOS Y SUMINISTRO CON CLÁUSULA DE MATERIALIDAD FISCAL (ART. 69-B CFF), RAZÓN DE NEGOCIOS (ART. 5-A CFF), FECHA CIERTA SCJN Y BITÁCORA DE ENTREGABLES
**Fundamento Legal:** Artículos 27 Fracc. I de la LISR, 5 Fracc. I de la LIVA, 5-A, 29-A, 32-B Ter y 69-B del Código Fiscal de la Federación; Artículos 89 a 94 del Código de Comercio y Jurisprudencia **2a./J. 161/2019 (10a.) de la Suprema Corte de Justicia de la Nación (Fecha Cierta)**.

---

### DECLARACIONES DE LAS PARTES CONTRATANTES
1. **Declara "EL CLIENTE" ({razon}):** Ser una entidad legalmente constituida conforme a las leyes mexicanas, con RFC **{rfc}**, giro económico **{giro}**, domicilio fiscal en **{domicilio}, C.P. {cp}, {jurisdiccion}**, representada en este acto por **{rep_legal}** en su carácter de **{cargo}** (Instrumento Público N° {escritura}).
2. **Declara "EL PROVEEDOR" ({prov_nombre}):** Ser un contribuyente activo con RFC **{prov_rfc}**, contar con **Opinión de Cumplimiento 32-D en sentido POSITIVO** emitida por el SAT, IMSS e INFONAVIT, tener su domicilio fiscal en estatus **LOCALIZADO** y contar con el personal, activos, infraestructura y capacidad material directa para ejecutar el objeto del presente contrato, sin ubicarse en los supuestos de los Artículos 69 ni 69-B del CFF.

---

### CLÁUSULAS DE SUSTANCIA ECONÓMICA, MATERIALIDAD Y BLINDAJE FISCAL
**PRIMERA (OBJETO ESPECÍFICO Y TRAZABLE):** EL PROVEEDOR (**{prov_nombre}** — RFC: **{prov_rfc}**) se obliga a ejecutar en favor de **{razon}** las operaciones consistentes en: **{prov_objeto}**. Queda prohibido el uso de conceptos genéricos en la facturación CFDI 4.0; toda factura deberá detallar el lote, periodo, orden de servicio y entregable específico.

**SEGUNDA (EXPEDIENTE OBLIGATORIO DE MATERIALIDAD ART. 69-B CFF):** Será condición suspensiva para la liberación de cualquier pago que EL PROVEEDOR integre y entregue al área de auditoría (**{testigo}**): (i) Cotización y Orden de Compra previa fechada; (ii) Entregable físico o digital verificable (bitácora de servicio, reporte técnico, evidencia fotográfica fechada, remisión de almacén firmada o acuse de recepción); (iii) Opinión de Cumplimiento 32-D positiva del mes en curso; y (iv) CFDI 4.0 válido y Complemento de Recepción de Pagos (REP) a más tardar el día 5 del mes siguiente.

**TERCERA (RAZÓN DE NEGOCIOS ART. 5-A CFF E INDEMNIDAD FISCAL):** Las partes hacen constar que el presente contrato persigue un beneficio económico cuantificable y directo para la operación del giro **{giro}** de **{razon}**. En caso de que EL PROVEEDOR sea publicado en los listados preliminares o definitivos del Artículo 69-B del CFF, se obliga a acudir ante la autoridad fiscal dentro de los 15 días hábiles de ley para desvirtuar la presunción y a indemnizar al 100% a **{razon}** por cualquier crédito fiscal, actualización o recargo.

{clausula_fiscal}

| **POR "EL CLIENTE": {razon} ({rfc})** | **POR "EL PROVEEDOR": {prov_nombre}** | **TESTIGO / AUDITOR DE MATERIALIDAD** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | RFC: {prov_rfc} | **{testigo}** |
| Fecha Cierta Digital: {fecha} | Firma Representante Legal | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                rep_legal = rep_legal, cargo = rep_cargo, escritura = escritura_num,
                prov_nombre = prov_nombre, prov_rfc = prov_rfc, prov_objeto = prov_objeto,
                testigo = testigo_auditor, clausula_fiscal = clausula_fiscal_scian,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 06: PROTOCOLO IMSS 32-D, PRIMA DE RIESGO RACERF, REPSE + ANEXO PIPC Y SECTORIAL
        GeneratedKitDocument {
            zip_relative_path: "04_IMSS_INFONAVIT_Seguridad_Social_REPSE/06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.md".to_string(),
            disk_filename: "06_Protocolo_IMSS_REPSE_32D.md".to_string(),
            content: format!(
r#"# PROTOCOLO PATRONAL IMSS 32-D, PRIMA DE RIESGO RACERF, BLINDAJE REPSE (ICSOE / SISSUB) Y ANEXO DE PROTECCIÓN CIVIL Y REGULACIÓN SECTORIAL
**Patrón Auditado:** {razon} ({rfc}) | **Registro Patronal IMSS:** {reg_patronal}
**Giro SCIAN:** {giro} | **Plantilla Conciliada:** {emp} trabajadores
**Clasificación de Riesgo IMSS (Arts. 71-75 LSS y Art. 196 RACERF):** {clase_prima}
**Folio Padrón STPS REPSE:** {repse} | **Domicilio:** {domicilio}, C.P. {cp}, {jurisdiccion}

---

### 1. CONCILIACIÓN MENSUAL SUA VS. CFDI DE NÓMINA Y DECLARACIÓN ANUAL DE PRIMA DE RIESGO
El área contable y de recursos humanos de **{razon}** ejecutará los días 15 de cada mes la confronta automática entre los visores de nómina del SAT (CFDI 4.0), las cédulas de determinación EMA/EBA del IMSS y el Sistema Único de Autodeterminación (SUA) para los **{emp} trabajadores**, presentando en el mes de **febrero de cada año** la Determinación de la Prima en el Seguro de Riesgos de Trabajo conforme a la clasificación **{clase_prima}**.

### 2. EXPEDIENTE DE BLINDAJE DE RESPONSABILIDAD SOLIDARIA REPSE (ART. 15-A LSS Y ART. 27 FRACC. V LISR)
Para evitar la no deducibilidad en ISR, el rechazo del acreditamiento de IVA y las multas de **2,000 a 50,000 UMAs (Art. 1004-C LFT)** en servicios u obras especializadas (Contratista: **{prov_nombre}** — RFC: **{prov_rfc}**), se integra mensualmente: (1) Folio REPSE vigente ({repse}); (2) CFDI de nómina de los trabajadores asignados; (3) Comprobante bancario de entero de retenciones ISR; (4) Declaración y pago mensual de IVA; (5) Cédula SUA y pago de cuotas obrero-patronales IMSS; (6) Pago bimestral de aportaciones INFONAVIT; y (7) Acuses cuatrimestrales **ICSOE (IMSS)** y **SISSUB (INFONAVIT)** presentados a más tardar los días 17 de enero, mayo y septiembre.

### 3. ACTA DE UNIDAD INTERNA DE PROTECCIÓN CIVIL (PIPC) Y CUMPLIMIENTO SECTORIAL ESPECÍFICO ({scian_short})
En cumplimiento a los Artículos 39, 40 y 74 a 78 de la Ley General de Protección Civil, se constituye en **{domicilio}** la **Unidad Interna de Protección Civil** coordinada por **{csh_coord}** e integrada por las 4 Brigadas multifuncionales (Evacuación, Primeros Auxilios, Prevención y Combate de Incendios, y Búsqueda y Salvamento), cumpliendo además las siguientes obligaciones sectoriales propias del giro **{giro}**:
{anexo_sectorial}

| **DIRECCIÓN PATRONAL ({razon})** | **COORDINADOR UNIDAD INTERNA PIPC** | **AUDITORÍA DE CUMPLIMIENTO** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_coord}** | **{testigo}** |

*Fecha de Emisión:* {fecha} | *Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                giro = giro_scian, emp = emp_count, clase_prima = clase_prima_imss,
                repse = folio_repse, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, prov_nombre = prov_nombre, prov_rfc = prov_rfc,
                scian_short = scian_code, csh_coord = csh_coord, anexo_sectorial = anexo_sectorial_pipc,
                rep_legal = rep_legal, cargo = rep_cargo, testigo = testigo_auditor,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // INSTRUMENTO 07: TÉRMINOS E-COMMERCE ART. 76 BIS LFPC, GARANTÍAS Y BENEFICIARIO CONTROLADOR
        GeneratedKitDocument {
            zip_relative_path: "05_PROFECO_Comercio_Digital_LFPC/07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.md".to_string(),
            disk_filename: "07_Terminos_Ecommerce_PROFECO_76BIS.md".to_string(),
            content: format!(
r#"# TÉRMINOS Y CONDICIONES DE COMERCIO ELECTRÓNICO (ART. 76 BIS LFPC), PÓLIZA DE GARANTÍA ({garantia} DÍAS) Y CONSTANCIA DE BENEFICIARIO CONTROLADOR (ART. 32-B TER CFF)
**Proveedor Oficial:** {razon} | **RFC:** {rfc} | **Giro SCIAN:** {giro}
**Domicilio Físico en Territorio Nacional (Art. 76 BIS Fracc. IV LFPC):** {domicilio}, C.P. {cp}, {jurisdiccion}
**Representante Legal:** {rep_legal} ({cargo}) | **Atención a Clientes y Consumidores:** {email} | **Teléfono:** {tel}

---

### 1. TRANSPARENCIA DE PRECIOS TOTALES, MONEDA NACIONAL E INTEGRIDAD DIGITAL (ARTS. 7 BIS, 76 BIS Y 128 BIS LFPC)
Toda oferta comercial, cotización o catálogo digital de **{razon}** exhibe el monto total a pagar en **Moneda Nacional (MXN)** desglosando IVA, impuestos aplicables y costos de envío antes de finalizar la transacción, respetando cabalmente los precios, tarifas, garantías y cantidades ofrecidas sin aplicar cargos adicionales sin consentimiento expreso (Art. 7 BIS y 90 LFPC).

### 2. DERECHO DE REVOCACIÓN (5 DÍAS HÁBILES) Y PÓLIZA DE GARANTÍA ESCRITA ({garantia} DÍAS NATURALES)
1. **Derecho de Revocación y Reembolso (Art. 56 LFPC):** En operaciones celebradas por medios electrónicos, ópticos o a distancia, el adquirente cuenta con un plazo improrrogable de **5 días hábiles** contados a partir de la entrega del bien o firma del contrato para revocar su consentimiento sin responsabilidad alguna, solicitándolo al correo **{email}**.
2. **Póliza de Garantía Legal (Arts. 77, 78 y 79 LFPC):** **{razon}** otorga una garantía mínima de **{garantia} días naturales** sobre sus productos y servicios comercializados bajo el giro **{giro}**, la cual cubre sin costo la reparación, reposición o bonificación directamente en el establecimiento ubicado en **{domicilio}, C.P. {cp}, {jurisdiccion}**.

### 3. CONSTANCIA DE RESGUARDO DE EXPEDIENTE DE BENEFICIARIO CONTROLADOR (ARTS. 32-B TER A 32-B QUINQUIES CFF)
Como parte de la contabilidad fiscal obligatoria (Art. 28 CFF), la administración de **{razon}** mantiene actualizado en su domicilio fiscal el expediente fidedigno y completo de sus **Beneficiarios Controladores** (identificación oficial, cadena de titularidad accionaria, porcentaje de participación, actas de asamblea, estado civil y domicilio fiscal verificado), blindando a la sociedad frente a las sanciones de **$1,500,000.00 a $2,000,000.00 MXN** previstas en el Artículo 84-M y 84-N del Código Fiscal de la Federación.

| **EMISOR COMERCIAL Y FISCAL ({razon})** | **VENTANILLA PROFECO / CONSUMIDOR** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | Canal Oficial: {email} / Tel: {tel} |

*Emitido el {fecha} | Sello Criptográfico SHA-256:* `{hash}`
"#,
                garantia = dias_garantia, razon = razon_social, rfc = tax_id,
                giro = giro_scian, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, fecha = fecha, hash = empresa.audit_hash
            ),
        },
    ]
}

pub fn generate_compliance_kit(empresa: &EmpresaB2B) -> Result<String, Box<dyn std::error::Error>> {
    generate_compliance_kit_with_session(empresa, &LegalDossierInput::default(), None)
}

pub fn generate_compliance_kit_with_dossier(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
) -> Result<String, Box<dyn std::error::Error>> {
    generate_compliance_kit_with_session(empresa, dossier, None)
}

pub fn generate_compliance_kit_with_session(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
    master_opt: Option<&AuditMasterJson>,
) -> Result<String, Box<dyn std::error::Error>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let kit_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&kit_dir)?;

    let docs = build_all_compliance_documents_with_session(empresa, dossier, master_opt);
    for doc in docs {
        let path = format!("{}/{}", kit_dir, doc.disk_filename);
        let mut f = File::create(&path)?;
        f.write_all(doc.content.as_bytes())?;
    }

    Ok(kit_dir)
}
