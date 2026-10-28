//! Generador Determinístico de Instrumentos Jurídicos Corporativos.
//! Pre-llena automáticamente los datos registrales conocidos (DENUE/OSINT) y, cuando el usuario
//! deja en blanco campos variables (ej. Proveedor en Contrato Marco Art. 69-B o Vocales NOM-019),
//! inserta líneas jurídicas formales de llenado (`________________________________`) para uso multi-proveedor.

use crate::engine::models::{EmpresaB2B, LegalDossierInput};
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
        && !s.contains("Cédula")
    {
        return s;
    }
    blank
}

pub fn build_all_compliance_documents(empresa: &EmpresaB2B) -> Vec<GeneratedKitDocument> {
    build_all_compliance_documents_with_dossier(empresa, &LegalDossierInput::default())
}

pub fn build_all_compliance_documents_with_dossier(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
) -> Vec<GeneratedKitDocument> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let fecha = Utc::now().format("%d/%m/%Y").to_string();

    // Datos conocidos por RFC / DENUE / OSINT (Pre-llenados automáticamente)
    let razon_social = pick_known_or_blank(&dossier.razon_social_oficial, &empresa.nombre, BLANK_LINE);
    let domicilio = pick_known_or_blank(&dossier.domicilio_fiscal_completo, &empresa.direccion, BLANK_LINE);
    let cp_fiscal = pick_known_or_blank(&dossier.codigo_postal_fiscal, "", BLANK_SHORT);
    let jurisdiccion = pick_known_or_blank(&dossier.entidad_jurisdiccion, &empresa.estado, "Estados Unidos Mexicanos");
    let giro_scian = pick_known_or_blank(&dossier.giro_scian_descripcion, &empresa.actividad, BLANK_LINE);
    let email_arco = pick_known_or_blank(&dossier.email_arco_privacidad, &empresa.email, BLANK_LINE);
    let telefono_corp = pick_known_or_blank(&dossier.telefono_corporativo, &empresa.telefono, BLANK_SHORT);

    // Datos internos o multi-proveedor: si el cliente los deja en blanco, quedan con línea formal para llenado o uso múltiple
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
        "Lunes a Viernes conforme a jornada legal máxima de la LFT",
    );
    let apoyo_teletrabajo = pick_known_or_blank(&dossier.monto_apoyo_teletrabajo_mxn, "", "____________");

    let prov_nombre = pick_known_or_blank(&dossier.proveedor_principal_razon_social, "", BLANK_LINE);
    let prov_rfc = pick_known_or_blank(&dossier.proveedor_principal_rfc, "", BLANK_SHORT);
    let prov_objeto = pick_known_or_blank(&dossier.proveedor_principal_servicio_objeto, "", BLANK_LINE);
    let testigo_auditor = pick_known_or_blank(&dossier.testigo_auditor_interno, "", BLANK_LINE);

    let clase_prima_imss = pick_known_or_blank(
        &dossier.clase_prima_riesgo_imss,
        "",
        "Clase y Prima declarada conforme al Catálogo del Art. 196 RACERF",
    );
    let folio_repse = pick_known_or_blank(&dossier.folio_repse_activo, "", BLANK_SHORT);
    let dias_garantia = dossier.dias_garantia_comercial_profeco.unwrap_or(90).max(90);

    vec![
        GeneratedKitDocument {
            zip_relative_path: "01_SABG_INAI_Proteccion_Datos_LFPDPPP/01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.md".to_string(),
            disk_filename: "01_Aviso_de_Privacidad_Integral.md".to_string(),
            content: format!(
r#"# AVISO DE PRIVACIDAD INTEGRAL, CLÁUSULA DE CONSENTIMIENTO EXPRESO BIOMÉTRICO Y DESIGNACIÓN DE OFICIAL DE PRIVACIDAD
**Responsable del Tratamiento:** {razon} | **RFC:** {rfc}
**Domicilio Fiscal y de Notificaciones:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Oficial de Privacidad Designado (Art. 17 LFPDPPP):** {oficial} ({cargo})
**Canal Oficial de Atención a Derechos ARCO:** {email} | **Teléfono:** {tel}

---

### I. IDENTIDAD Y DOMICILIO DEL RESPONSABLE (ARTS. 8, 15 Y 16 LFPDPPP)
**{razon}** (con RFC **{rfc}**), representada legalmente por **{rep_legal}** en su carácter de **{cargo}** (Instrumento Público N° {escritura}), con domicilio en **{domicilio}, C.P. {cp}, {jurisdiccion}**, es responsable del tratamiento legítimo, controlado e informado de sus datos personales, financieros/patrimoniales y sensibles conforme a la Ley Federal de Protección de Datos Personales en Posesión de los Particulares vigente ante la Autoridad Garante Federal (SABG / Transparencia para el Pueblo).

### II. DEPARTAMENTO DE DATOS PERSONALES (ART. 17 Y 30 LFPDPPP)
Se designa formalmente a **{oficial}** como responsable del Departamento de Datos Personales de **{razon}**, encargado de custodiar el Documento de Seguridad Técnico, Físico y Administrativo (Art. 19 LFPDPPP) y dar trámite a las solicitudes de Derechos ARCO recibidas en **{email}** o en el domicilio físico antes señalado.

### III. CATEGORÍAS DE DATOS Y FINALIDADES PRIMARIAS
1. **Datos de Identificación, Laborales y Contacto:** Nombre completo, RFC, CURP, NSS, domicilio, teléfono y correo electrónico.
2. **Datos Patrimoniales y Financieros (Art. 8 LFPDPPP):** Cuentas CLABE, datos fiscales para emisión de CFDI 4.0 y referencias comerciales.
3. **Datos Personales Sensibles y Biométricos (Art. 9 LFPDPPP):** Huella dactilar, reconocimiento facial para control de asistencia o seguridad de acceso en **{domicilio}**, y certificados médicos requeridos por las Normas Oficiales Mexicanas de la STPS.

### IV. TRANSFERENCIAS (ARTS. 36 Y 37 LFPDPPP) Y EJERCICIO DE DERECHOS ARCO (ARTS. 22 A 35)
Sus datos sólo se transfieren a autoridades fiscalizadoras (SAT, IMSS, INFONAVIT, STPS) y encargados sujetos a este aviso. Para ejercer sus derechos de **Acceso, Rectificación, Cancelación u Oposición (ARCO)** o revocar su consentimiento, envíe su solicitud a **{email}**. Plazo máximo legal de respuesta: **20 días hábiles** (Art. 32 LFPDPPP).

---

### FORMATO DE CONSENTIMIENTO EXPRESO PARA DATOS SENSIBLES Y BIOMÉTRICOS (ART. 9 LFPDPPP)
Yo, _________________________________________________________, otorgo mi consentimiento expreso y autógrafo para que mis datos personales sensibles, biométricos y patrimoniales sean tratados por **{razon}** ({rfc}) conforme al presente Aviso de Privacidad Integral.

| **POR EL RESPONSABLE ({razon})** | **OFICIAL DE PRIVACIDAD** | **FIRMA AUTÓGRAFA DEL TITULAR** |
| :--- | :--- | :--- |
| **{rep_legal}** | **{oficial}** | ________________________________________ |
| Cargo: {cargo} | Canal ARCO: {email} | Fecha: {fecha} |

*Sello de Integridad Criptográfica SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, oficial = oficial_privacidad, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, rep_legal = rep_legal,
                escritura = escritura_num, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/02_Politica_Prevencion_Psicosocial_NOM035_STPS.md".to_string(),
            disk_filename: "02_Politica_Prevencion_NOM035_STPS.md".to_string(),
            content: format!(
r#"# POLÍTICA DE PREVENCIÓN DE RIESGOS PSICOSOCIALES Y PROTOCOLO DE ACTUACIÓN (NOM-035-STPS-2018)
**Patrón:** {razon} | **RFC:** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
**Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion} | **Giro SCIAN:** {giro}
**Fundamento Legal:** Art. 132 Fracc. XVI y 994 Fracc. V LFT; Numerales 5.1, 7.1, 8.1 y Guía de Referencia IV de la NOM-035-STPS-2018.

---

### 1. COMPROMISO PATRONAL DE PREVENCIÓN PSICOSOCIAL
La representación legal de **{razon}**, por conducto de **{rep_legal}** ({cargo}), establece como directriz obligatoria en este centro de trabajo la prevención de los factores de riesgo psicosocial, la erradicación de la violencia laboral y la promoción de un entorno organizacional favorable.

### 2. APLICACIÓN DE GUÍAS DE REFERENCIA Y CANAL CONFIDENCIAL
1. **Identificación de Acontecimientos Traumáticos Severos (Guía de Referencia I):** Aplicable a toda la plantilla bajo seguimiento del Coordinador de Seguridad **{csh_coord}**.
2. **Evaluación de Factores Psicosociales y Entorno Organizacional (Guías II y III):** Aplicación conforme al número de colaboradores del establecimiento y respeto a la jornada laboral ({horario}).
3. **Buzón Confidencial de Denuncia Interna (Numeral 8.1 b):** Todo trabajador cuenta con el canal confidencial **{email}** y con la representación laboral (**{csh_sec}**) para reportar actos opuestos al entorno organizacional favorable con garantía de cero represalias.

| **REPRESENTACIÓN PATRONAL ({razon})** | **REPRESENTACIÓN DE LOS TRABAJADORES** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_sec}** |
| Fecha de Emisión: {fecha} | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                giro = giro_scian, rep_legal = rep_legal, cargo = rep_cargo,
                csh_coord = csh_coord, horario = horario_jornada, email = email_arco,
                csh_sec = csh_sec, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.md".to_string(),
            disk_filename: "03_Acta_Comision_Mixta_NOM019_NOM030.md".to_string(),
            content: format!(
r#"# ACTA CONSTITUTIVA DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (NOM-019-STPS-2011) Y DESIGNACIÓN SST (NOM-030-STPS-2009)
**Fundamento Legal:** Arts. 509, 510 y 994 Fracc. V LFT; Numeral 7.4 de la NOM-019-STPS-2011 y Numeral 4.1 de la NOM-030-STPS-2009.

---

### I. DATOS OFICIALES DEL CENTRO DE TRABAJO (NUMERAL 7.4 NOM-019-STPS-2011)
* **Denominación o Razón Social:** {razon}
* **Registro Federal de Contribuyentes (RFC):** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
* **Domicilio del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
* **Actividad Económica (SCIAN):** {giro} | **Fecha de Constitución:** {fecha}

### II. INTEGRACIÓN DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE
Reunidos en el centro de trabajo, la representación patronal (**{rep_legal}** — {cargo}) y la representación de los trabajadores constituyen formalmente la Comisión Mixta de Seguridad e Higiene:

| **CARGO EN LA COMISIÓN** | **NOMBRE DEL INTEGRANTE DESIGNADO** | **SECTOR REPRESENTADO** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- | :--- |
| **Coordinador** | **{csh_coord}** | Representante Patronal | ________________________ |
| **Secretario** | **{csh_sec}** | Representante Trabajadores | ________________________ |
| **Vocal 1** | **{csh_vocal_pat}** | Vocal Patronal | ________________________ |
| **Vocal 2** | **{csh_vocal_trab}** | Vocal Trabajadores | ________________________ |

### III. DESIGNACIÓN DEL RESPONSABLE DE SERVICIOS PREVENTIVOS SST (NOM-030-STPS-2009)
En cumplimiento al Numeral 4.1 de la **NOM-030-STPS-2009**, se designa a **{csh_coord}** como **Responsable de Seguridad y Salud en el Trabajo**, a cargo del Diagnóstico Integral y del Programa Anual de Prevención (NOM-001, NOM-002, NOM-017, NOM-025, NOM-026 y NOM-029).

*Sello Criptográfico SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                giro = giro_scian, fecha = fecha, rep_legal = rep_legal, cargo = rep_cargo,
                csh_coord = csh_coord, csh_sec = csh_sec, csh_vocal_pat = csh_vocal_pat,
                csh_vocal_trab = csh_vocal_trab, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.md".to_string(),
            disk_filename: "04_Politica_Teletrabajo_NOM037.md".to_string(),
            content: format!(
r#"# POLÍTICA DE TELETRABAJO, DERECHO A LA DESCONEXIÓN Y CONDICIONES ERGONÓMICAS (NOM-037-STPS-2023)
**Empresa:** {razon} ({rfc}) | **Domicilio:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Fundamento Legal:** Artículos 330-A al 330-K de la Ley Federal del Trabajo y Numerales 5.2, 5.4 y 7.1 de la NOM-037-STPS-2023.

---

### 1. JORNADA LABORAL Y DERECHO A LA DESCONEXIÓN DIGITAL (ART. 330-E FRACC. VI LFT)
**{razon}** garantiza el **Derecho a la Desconexión Digital** al término de la jornada laboral pactada ({horario}), así como durante pausas de alimentos, días de descanso, incapacidades y periodos vacacionales.

### 2. COMPENSACIÓN PROPORCIONAL DE SERVICIOS E INSUMOS (ART. 330-E FRACC. III LFT)
Para el personal que desempeñe más del 40% de su jornada en modalidad remota, el patrón cubrirá mensualmente la cantidad de **${apoyo} MXN** por concepto de pago proporcional de internet y energía eléctrica, además de validar las condiciones ergonómicas mediante la Comisión Mixta.

| **POR EL PATRÓN ({razon})** | **CONFORMIDAD DEL TRABAJADOR EN TELETRABAJO** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | ________________________________________ |

*Fecha Cierta de Emisión:* {fecha} | *Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, horario = horario_jornada, apoyo = apoyo_teletrabajo,
                rep_legal = rep_legal, cargo = rep_cargo, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "03_SAT_Materialidad_Fiscal_69B/05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.md".to_string(),
            disk_filename: "05_Contrato_Materialidad_SAT_69B.md".to_string(),
            content: format!(
r#"# CONTRATO MARCO DE PRESTACIÓN DE SERVICIOS CON CLÁUSULA DE MATERIALIDAD FISCAL (ART. 69-B CFF), FECHA CIERTA SCJN Y BITÁCORA DE ENTREGABLES
**Fundamento Legal:** Arts. 27 Fracc. I LISR, 5 Fracc. I LIVA, 5-A, 29-A y 69-B del Código Fiscal de la Federación; Arts. 89 a 94 del Código de Comercio y Jurisprudencia **2a./J. 161/2019 (10a.) de la SCJN (Fecha Cierta)**.

---

### DECLARACIONES DE LAS PARTES
1. **Declara "EL CLIENTE" ({razon}):** Contribuyente activo con RFC **{rfc}**, con domicilio fiscal en **{domicilio}, C.P. {cp}, {jurisdiccion}**, representado por **{rep_legal}** en su carácter de **{cargo}** (Instrumento N° {escritura}).
2. **Declara "EL PROVEEDOR" ({prov_nombre}):** Contribuyente con RFC **{prov_rfc}**, con Opinión de Cumplimiento 32-D en sentido Positivo, domicilio fiscal Localizado y exento de los supuestos de los Artículos 69 y 69-B del CFF.

---

### CLÁUSULAS DE SUSTANCIA ECONÓMICA Y MATERIALIDAD
**PRIMERA (OBJETO ESPECÍFICO AUDITABLE):** EL PROVEEDOR (**{prov_nombre}** — RFC: **{prov_rfc}**) se obliga a prestar en favor de **{razon}** los servicios consistentes en: **{prov_objeto}**. Todo CFDI 4.0 emitido deberá referenciar los entregables de la bitácora anexa.

**SEGUNDA (EXPEDIENTE DE MATERIALIDAD ART. 69-B CFF Y RAZÓN DE NEGOCIOS ART. 5-A CFF):** Será requisito para cada pago integrar: (i) Orden de compra previa fechada; (ii) Evidencia material del entregable (informes, minutas, bitácoras o memoria fotográfica/técnica); (iii) Opinión 32-D vigente; y (iv) CFDI 4.0 y Complemento de Pago (REP).

**TERCERA (INDEMNIDAD FISCAL Y FECHA CIERTA DIGITAL):** En caso de publicación del proveedor en listados del Art. 69-B del CFF, éste indemnizará al 100% a **{razon}**. El presente instrumento cuenta con huella criptográfica SHA-256 (`{hash}`) de fecha **{fecha}**.

| **POR "EL CLIENTE": {razon} ({rfc})** | **POR "EL PROVEEDOR": {prov_nombre}** | **TESTIGO / AUDITOR DE MATERIALIDAD** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | Firma Representante Legal | **{testigo}** |
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, cargo = rep_cargo,
                escritura = escritura_num, prov_nombre = prov_nombre, prov_rfc = prov_rfc,
                prov_objeto = prov_objeto, testigo = testigo_auditor, hash = empresa.audit_hash, fecha = fecha
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "04_IMSS_INFONAVIT_Seguridad_Social_REPSE/06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.md".to_string(),
            disk_filename: "06_Protocolo_IMSS_REPSE_32D.md".to_string(),
            content: format!(
r#"# PROTOCOLO PATRONAL IMSS 32-D, PRIMA DE RIESGO Y BLINDAJE DE RESPONSABILIDAD SOLIDARIA REPSE (ICSOE / SISSUB)
**Patrón:** {razon} ({rfc}) | **Registro Patronal IMSS:** {reg_patronal}
**Clasificación de Riesgo (Arts. 71-75 LSS):** {clase_prima} | **Folio REPSE:** {repse}
**Domicilio:** {domicilio}, C.P. {cp}, {jurisdiccion} | **Representación Legal:** {rep_legal} ({cargo})

---

### 1. INTEGRACIÓN DEL EXPEDIENTE MENSUAL DE CONTRATISTAS ESPECIALIZADOS (ART. 15-A LSS Y ART. 27 FRACC. V LISR)
Para la deducibilidad de ISR y acreditamiento de IVA en servicios especializados (Contratista: **{prov_nombre}** — RFC: **{prov_rfc}**), se verifican y resguardan mensualmente: (1) Registro REPSE vigente ({repse}); (2) CFDI de nómina de los trabajadores asignados; (3) Comprobante bancario de retenciones ISR; (4) Declaración y pago mensual de IVA; (5) Cédula SUA y pago de cuotas IMSS; (6) Pago bimestral INFONAVIT; y (7) Acuses cuatrimestrales **ICSOE (IMSS)** y **SISSUB (INFONAVIT)**.

| **DIRECCIÓN PATRONAL ({razon})** | **AUDITORÍA DE CUMPLIMIENTO** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{testigo}** |

*Fecha de Emisión:* {fecha} | *Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                clase_prima = clase_prima_imss, repse = folio_repse, domicilio = domicilio,
                cp = cp_fiscal, jurisdiccion = jurisdiccion, rep_legal = rep_legal,
                cargo = rep_cargo, testigo = testigo_auditor, prov_nombre = prov_nombre,
                prov_rfc = prov_rfc, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "05_PROFECO_Comercio_Digital_LFPC/07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.md".to_string(),
            disk_filename: "07_Terminos_Ecommerce_PROFECO_76BIS.md".to_string(),
            content: format!(
r#"# TÉRMINOS Y CONDICIONES DE COMERCIO ELECTRÓNICO, DEVOLUCIONES Y PÓLIZA DE GARANTÍA (ART. 76 BIS Y 77 LFPC — PROFECO)
**Proveedor Oficial:** {razon} | **RFC:** {rfc}
**Domicilio Físico en Territorio Nacional (Art. 76 BIS Fracc. IV LFPC):** {domicilio}, C.P. {cp}, {jurisdiccion}
**Representante Legal:** {rep_legal} ({cargo}) | **Atención a Consumidores:** {email} | **Teléfono:** {tel}

---

### 1. TRANSPARENCIA DE PRECIOS TOTALES Y SEGURIDAD (ARTS. 7 BIS, 76 BIS Y 128 BIS LFPC)
Todo producto o servicio comercializado por **{razon}** exhibe su precio total en **Moneda Nacional (MXN)** con IVA e impuestos incluidos. Las transacciones electrónicas se encuentran protegidas bajo cifrado TLS y pasarelas certificadas PCI-DSS.

### 2. DERECHO DE REVOCACIÓN (5 DÍAS HÁBILES) Y PÓLIZA DE GARANTÍA ({garantia} DÍAS NATURALES)
1. **Revocación y Reembolso (Art. 56 LFPC):** El consumidor cuenta con **5 días hábiles** a partir de la entrega para revocar su compra a distancia solicitándolo en **{email}**.
2. **Garantía Legal Expresa (Arts. 77 y 78 LFPC):** **{razon}** otorga una garantía de **{garantia} días naturales** efectiva directamente en **{domicilio}, C.P. {cp}, {jurisdiccion}**.

| **EMISOR COMERCIAL ({razon})** | **ATENCIÓN AL CONSUMIDOR** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | Canal Oficial: {email} |

*Emitido el {fecha} | Sello Criptográfico SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, garantia = dias_garantia,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },
    ]
}

pub fn generate_compliance_kit(empresa: &EmpresaB2B) -> Result<String, Box<dyn std::error::Error>> {
    generate_compliance_kit_with_dossier(empresa, &LegalDossierInput::default())
}

pub fn generate_compliance_kit_with_dossier(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
) -> Result<String, Box<dyn std::error::Error>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let kit_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&kit_dir)?;

    let docs = build_all_compliance_documents_with_dossier(empresa, dossier);
    for doc in docs {
        let path = format!("{}/{}", kit_dir, doc.disk_filename);
        let mut f = File::create(&path)?;
        f.write_all(doc.content.as_bytes())?;
    }

    Ok(kit_dir)
}
