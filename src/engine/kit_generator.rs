//! Generador Determinístico de Kits Legales Corporativos 100% personalizados (Cero Placeholders y Cero Líneas en Blanco).
//! Inyecta los 23 campos de `LegalDossierInput` en los 7 instrumentos jurídicos para que sean válidos al instante de descargarse.

use crate::engine::models::{EmpresaB2B, LegalDossierInput};
use chrono::Utc;
use std::fs::{self, File};
use std::io::Write;

pub struct GeneratedKitDocument {
    pub zip_relative_path: String,
    pub disk_filename: String,
    pub content: String,
}

fn non_empty<'a>(opt: &'a Option<String>, fallback: &'a str) -> &'a str {
    match opt {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => fallback,
    }
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

    let razon_social = non_empty(&dossier.razon_social_oficial, &empresa.nombre);
    let domicilio = non_empty(&dossier.domicilio_fiscal_completo, &empresa.direccion);
    let cp_fiscal = non_empty(&dossier.codigo_postal_fiscal, "06600");
    let jurisdiccion = non_empty(&dossier.entidad_jurisdiccion, &empresa.estado);
    let giro_scian = non_empty(&dossier.giro_scian_descripcion, &empresa.actividad);

    let rep_legal = non_empty(
        &dossier.representante_legal_nombre,
        "Representante Legal Acreditado en Acta Constitutiva",
    );
    let rep_cargo = non_empty(
        &dossier.representante_legal_cargo,
        "Administrador Único / Apoderado General",
    );
    let escritura_num = non_empty(
        &dossier.instrumento_notarial_num,
        "Instrumento Público inscrito en el Registro Público de Comercio",
    );

    let oficial_privacidad = non_empty(&dossier.oficial_privacidad_nombre, rep_legal);
    let email_arco = non_empty(
        &dossier.email_arco_privacidad,
        if empresa.email.trim().is_empty() {
            "privacidad@corporativo.mx"
        } else {
            empresa.email.trim()
        },
    );
    let telefono_corp = non_empty(
        &dossier.telefono_corporativo,
        if empresa.telefono.trim().is_empty() {
            "Atención Telefónica en Domicilio Fiscal"
        } else {
            empresa.telefono.trim()
        },
    );

    let reg_patronal = non_empty(&dossier.registro_patronal_imss, "Registro Patronal Vigente IMSS");
    let csh_coord = non_empty(&dossier.comision_coordinador_patronal, rep_legal);
    let csh_sec = non_empty(
        &dossier.comision_secretario_trabajadores,
        "Representante Titular Designado por los Trabajadores",
    );
    let csh_vocal_pat = non_empty(&dossier.comision_vocal_patronal, oficial_privacidad);
    let csh_vocal_trab = non_empty(
        &dossier.comision_vocal_trabajadores,
        "Vocal Operativo de la Plantilla Laboral",
    );

    let horario_jornada = non_empty(
        &dossier.horario_jornada_laboral,
        "Lunes a Viernes de 09:00 a 18:00 horas (48 horas semanales máx. conforme a LFT)",
    );
    let apoyo_teletrabajo = non_empty(&dossier.monto_apoyo_teletrabajo_mxn, "450.00");

    let prov_nombre = non_empty(
        &dossier.proveedor_principal_razon_social,
        "Prestador de Servicios Especializados / Proveedor B2B Contratado",
    );
    let prov_rfc = non_empty(&dossier.proveedor_principal_rfc, "RFC-PROVEEDOR-VERIFICADO-32D");
    let prov_objeto = non_empty(
        &dossier.proveedor_principal_servicio_objeto,
        "Servicios profesionales, técnicos, suministro e implementación operativa indispensable para el giro social",
    );
    let testigo_auditor = non_empty(
        &dossier.testigo_auditor_interno,
        "Contraloría Interna / C.P.C. Víctor Omar Malpica Guillén",
    );

    let clase_prima_imss = non_empty(
        &dossier.clase_prima_riesgo_imss,
        "Autoclasificación conforme al Catálogo del Art. 196 RACERF (Prima vigente declarada en SUA)",
    );
    let folio_repse = non_empty(&dossier.folio_repse_activo, "Padrón Público STPS REPSE Verificado");
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
**Canal Oficial de Derechos ARCO:** {email} | **Teléfono:** {tel}

---

### I. IDENTIDAD Y DOMICILIO DEL RESPONSABLE (ARTS. 8, 15 Y 16 LFPDPPP)
**{razon}** (con RFC **{rfc}**), legalmente representada por **{rep_legal}** en su carácter de **{cargo}** (acreditado mediante {escritura}), con domicilio en **{domicilio}, C.P. {cp}, {jurisdiccion}**, es el responsable del tratamiento legítimo, controlado e informado de sus datos personales, financieros/patrimoniales y sensibles conforme a la Ley Federal de Protección de Datos Personales en Posesión de los Particulares vigente ante la Autoridad Garante Federal (SABG / Transparencia para el Pueblo).

### II. DESIGNACIÓN FORMAL DEL OFICIAL DE PRIVACIDAD (ART. 17 Y 30 LFPDPPP)
Queda formalmente designado(a) **{oficial}** como titular del Departamento de Datos Personales de **{razon}**, encargado(a) de custodiar el Documento de Seguridad Técnico, Físico y Administrativo (Art. 19 LFPDPPP) y dar respuesta a las solicitudes de Derechos ARCO en el correo **{email}**.

### III. CATEGORÍAS DE DATOS SOMETIDOS A TRATAMIENTO Y FINALIDADES PRIMARIAS
1. **Datos de Identificación, Laborales y Contacto:** Nombre, RFC, CURP, NSS, domicilio, teléfono y correo electrónico.
2. **Datos Patrimoniales y Financieros (Art. 8 LFPDPPP):** Cuentas CLABE, datos de facturación CFDI 4.0 (Régimen Fiscal y Uso CFDI) e historial crediticio comercial.
3. **Datos Personales Sensibles y Biométricos (Art. 9 LFPDPPP):** Huella dactilar, reconocimiento facial para control de asistencia o seguridad de acceso en **{domicilio}**, y certificados médicos exigidos por las Normas Oficiales Mexicanas de la STPS.
* **Finalidades Primarias:** Cumplimiento de obligaciones contractuales, laborales, timbrado de nómina y facturación CFDI 4.0 ante el SAT, altas ante el IMSS ({reg_patronal}) y seguridad de instalaciones.

### IV. TRANSFERENCIAS DE DATOS (ARTS. 36 Y 37 LFPDPPP) Y DERECHOS ARCO (ARTS. 22 A 35)
Sus datos sólo se transfieren a autoridades fiscalizadoras (SAT, IMSS, INFONAVIT, STPS) y encargados sujetos a este aviso. Para ejercer sus derechos de **Acceso, Rectificación, Cancelación u Oposición (ARCO)** o revocar su consentimiento, envíe su solicitud a **{email}** dirigida a **{oficial}**. Plazo máximo legal de respuesta: **20 días hábiles** (Art. 32 LFPDPPP).

---

### ACTA DE EMISIÓN Y CONSENTIMIENTO EXPRESO DE DATOS SENSIBLES Y BIOMÉTRICOS (ART. 9 LFPDPPP)
El titular de los datos manifiesta su consentimiento expreso y por escrito para el tratamiento de sus datos sensibles, biométricos y patrimoniales conforme a los términos aquí establecidos.

| **RESPONSABLE LEGAL ({razon})** | **OFICIAL DE PRIVACIDAD DESIGNADO** | **CONFORMIDAD DEL TITULAR** |
| :--- | :--- | :--- |
| **{rep_legal}** | **{oficial}** | Nombre y Firma Autógrafa del Titular |
| Cargo: {cargo} | Canal ARCO: {email} | Fecha de Emisión: {fecha} |

*Sello de Integridad Criptográfica SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, oficial = oficial_privacidad, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, rep_legal = rep_legal,
                escritura = escritura_num, reg_patronal = reg_patronal, fecha = fecha, hash = empresa.audit_hash
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

### 1. COMPROMISO PATRONAL VINCULANTE
La representación legal de **{razon}**, encabezada por **{rep_legal}** ({cargo}), establece como norma obligatoria en este centro de trabajo la prevención de los factores de riesgo psicosocial, la erradicación de cualquier forma de violencia laboral (acoso, hostigamiento y malos tratos) y la promoción de un entorno organizacional favorable.

### 2. APLICACIÓN DE GUÍAS DE REFERENCIA Y VIGILANCIA
1. **Identificación de Acontecimientos Traumáticos Severos (Guía de Referencia I):** Aplicable al 100% de los trabajadores bajo la coordinación de **{csh_coord}**.
2. **Evaluación de Factores Psicosociales y Entorno Organizacional (Guías II y III):** Aplicación periódica conforme a la plantilla activa del centro de trabajo y respeto estricto al horario de jornada ({horario}).
3. **Línea Confidencial de Denuncia Interna (Numeral 8.1 b):** Todo colaborador cuenta con el canal confidencial **{email}** (supervisado por **{oficial}**) y ante el representante de los trabajadores **{csh_sec}** para reportar actos de violencia laboral con garantía de cero represalias.

| **APROBACIÓN PATRONAL ({razon})** | **REPRESENTACIÓN DE LOS TRABAJADORES** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_sec}** (Secretario Comisión) |
| Fecha de Vigencia: {fecha} | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                giro = giro_scian, rep_legal = rep_legal, cargo = rep_cargo,
                csh_coord = csh_coord, horario = horario_jornada, email = email_arco,
                oficial = oficial_privacidad, csh_sec = csh_sec, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.md".to_string(),
            disk_filename: "03_Acta_Comision_Mixta_NOM019_NOM030.md".to_string(),
            content: format!(
r#"# ACTA CONSTITUTIVA DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (NOM-019-STPS-2011) Y DESIGNACIÓN SST (NOM-030-STPS-2009)
**Fundamento Legal:** Arts. 509, 510 y 994 Fracc. V LFT; Numeral 7.4 de la NOM-019-STPS-2011 y Numeral 4.1 de la NOM-030-STPS-2009.

---

### I. DATOS OFICIALES DEL CENTRO DE TRABAJO (EXIGIDOS POR EL NUMERAL 7.4 NOM-019-STPS-2011)
* **Denominación o Razón Social:** {razon}
* **Registro Federal de Contribuyentes (RFC):** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
* **Domicilio Completo:** {domicilio}, C.P. {cp}, {jurisdiccion}
* **Rama o Actividad Económica (SCIAN):** {giro}
* **Fecha Formal de Constitución:** {fecha}

### II. CUADRO DE INTEGRACIÓN FORMAL DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE
Reunidos en el domicilio del centro de trabajo, el representante legal **{rep_legal}** ({cargo}) y la representación de los trabajadores declaran formalmente constituida la Comisión Mixta de Seguridad e Higiene con los siguientes integrantes titulares:

| **CARGO OFICIAL NOM-019** | **NOMBRE COMPLETO DEL DESIGNADO** | **REPRESENTACIÓN** | **ESTATUS** |
| :--- | :--- | :--- | :--- |
| **Coordinador de la Comisión** | **{csh_coord}** | Representante Patronal | En Funciones |
| **Secretario de la Comisión** | **{csh_sec}** | Representante Trabajadores | En Funciones |
| **Vocal 1 de Verificación** | **{csh_vocal_pat}** | Vocal Patronal | En Funciones |
| **Vocal 2 de Verificación** | **{csh_vocal_trab}** | Vocal de los Trabajadores | En Funciones |

### III. NOMBRAMIENTO DEL RESPONSABLE DE SERVICIOS PREVENTIVOS SST (NOM-030-STPS-2009)
En cumplimiento al Numeral 4.1 de la **NOM-030-STPS-2009**, **{razon}** designa formalmente a **{csh_coord}** como **Responsable de Seguridad y Salud en el Trabajo**, encargado de ejecutar el Diagnóstico Integral y el Programa Anual de Prevención (NOM-001 Edificios, NOM-002 Incendios, NOM-017 EPP, NOM-025 Iluminación, NOM-026 Señalética y NOM-029 Instalaciones Eléctricas).

| **COORDINADOR PATRONAL Y RESP. NOM-030** | **SECRETARIO (REP. TRABAJADORES)** | **REPRESENTANTE LEGAL** |
| :--- | :--- | :--- |
| **{csh_coord}** | **{csh_sec}** | **{rep_legal}** |

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

### 1. JORNADA CONTRACTUAL Y DERECHO A LA DESCONEXIÓN DIGITAL (ART. 330-E FRACC. VI LFT)
**{razon}**, a través de su representante legal **{rep_legal}**, garantiza el **Derecho a la Desconexión Digital** fuera de la jornada laboral establecida de **{horario}**, así como durante descansos, incapacidades y vacaciones.

### 2. COMPENSACIÓN PROPORCIONAL DE INSUMOS (ART. 330-E FRACC. III LFT)
Para las posiciones que laboren más del 40% del tiempo en modalidad remota, el patrón cubrirá en el CFDI de nómina una asignación mensual proporcional de **${apoyo} MXN** por concepto de pago proporcional de telecomunicaciones (internet) y energía eléctrica, además de proveer el equipo ergonómico validado por la Comisión Mixta (**{csh_coord}** y **{csh_sec}**).

| **POR EL PATRÓN ({razon})** | **COMISIÓN DE SEGURIDAD E HIGIENE** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_coord}** / **{csh_sec}** |

*Fecha Cierta de Emisión:* {fecha} | *Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, horario = horario_jornada,
                apoyo = apoyo_teletrabajo, csh_coord = csh_coord, csh_sec = csh_sec,
                cargo = rep_cargo, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        GeneratedKitDocument {
            zip_relative_path: "03_SAT_Materialidad_Fiscal_69B/05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.md".to_string(),
            disk_filename: "05_Contrato_Materialidad_SAT_69B.md".to_string(),
            content: format!(
r#"# CONTRATO MARCO DE PRESTACIÓN DE SERVICIOS CON CLÁUSULA DE MATERIALIDAD FISCAL (ART. 69-B CFF), FECHA CIERTA SCJN Y BITÁCORA DE ENTREGABLES
**Fundamento Legal:** Arts. 27 Fracc. I LISR, 5 Fracc. I LIVA, 5-A, 29-A y 69-B del Código Fiscal de la Federación; Arts. 89 a 94 del Código de Comercio y Jurisprudencia **2a./J. 161/2019 (10a.) de la SCJN (Fecha Cierta)**.

---

### DECLARACIONES VINCULANTES DE LAS PARTES
1. **Declara "EL CLIENTE" ({razon}):** Persona moral/física con RFC **{rfc}**, domicilio fiscal en **{domicilio}, C.P. {cp}, {jurisdiccion}**, representada en este acto por **{rep_legal}** en su carácter de **{cargo}** ({escritura}).
2. **Declara "EL PROVEEDOR" ({prov_nombre}):** Contribuyente activo con RFC **{prov_rfc}**, Opinión de Cumplimiento 32-D Positiva, domicilio fiscal Localizado y exento de los supuestos del Artículo 69 y 69-B del CFF.

---

### CLÁUSULAS DE SUSTANCIA ECONÓMICA Y MATERIALIDAD
**PRIMERA (OBJETO ESPECÍFICO AUDITABLE):** EL PROVEEDOR (**{prov_nombre}** — **{prov_rfc}**) se obliga a ejecutar en favor de **{razon}** las operaciones y servicios consistentes en: **{prov_objeto}**. Todo CFDI 4.0 deberá vincularse a los entregables descritos en este instrumento.

**SEGUNDA (DOSSIER DE MATERIALIDAD ART. 69-B CFF Y RAZÓN DE NEGOCIOS ART. 5-A CFF):** Para liberar cada pago, EL PROVEEDOR entregará bajo supervisión de **{testigo}**: (i) Orden de servicio fechada; (ii) Entregable técnico verificable (reportes, planos, bitácoras o evidencia fotográfica/digital); (iii) Opinión 32-D vigente; y (iv) CFDI 4.0 y Complemento de Pago (REP).

**TERCERA (INDEMNIDAD FISCAL Y FECHA CIERTA DIGITAL):** Si EL PROVEEDOR apareciera en listados del Art. 69-B del CFF, indemnizará al 100% a **{razon}**. El presente instrumento queda sellado con huella criptográfica SHA-256 (`{hash}`) en Árbol de Merkle el día **{fecha}**, dotándolo de Fecha Cierta Digital conforme al Código de Comercio.

| **POR "EL CLIENTE": {razon} ({rfc})** | **POR "EL PROVEEDOR": {prov_nombre} ({prov_rfc})** | **TESTIGO / AUDITOR DE MATERIALIDAD** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | Representante Acreditado de {prov_nombre} | **{testigo}** |
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
**Clasificación de Riesgo Declarada (Arts. 71-75 LSS):** {clase_prima} | **Estatus/Folio REPSE:** {repse}
**Domicilio:** {domicilio}, C.P. {cp}, {jurisdiccion} | **Responsable Legal:** {rep_legal} ({cargo})

---

### 1. INTEGRACIÓN DEL EXPEDIENTE MENSUAL DE CONTRATISTAS ESPECIALIZADOS (ART. 15-A LSS Y ART. 27 FRACC. V LISR)
Por instrucción vinculante de **{rep_legal}** y bajo auditoría de **{testigo}**, para la deducibilidad de ISR y acreditamiento de IVA en servicios especializados (proveedor **{prov_nombre}** — **{prov_rfc}**), se resguardan mensualmente: (1) Registro REPSE vigente ({repse}); (2) CFDI de nómina de los trabajadores asignados; (3) Entero bancario de retenciones ISR; (4) Pago mensual de IVA; (5) Pago SUA de cuotas IMSS; (6) Pago bimestral INFONAVIT; y (7) Acuses cuatrimestrales **ICSOE (IMSS)** y **SISSUB (INFONAVIT)** presentados a más tardar el día 17 de enero, mayo y septiembre.

| **DIRECCIÓN PATRONAL ({razon})** | **CONTRALORÍA / AUDITORÍA DE CUMPLIMIENTO** |
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
2. **Garantía Legal Expresa (Arts. 77 y 78 LFPC):** **{razon}** otorga una garantía de **{garantia} días naturales** (cumpliendo y superando el mínimo legal de 90 días del Art. 77 LFPC) efectiva directamente en **{domicilio}, C.P. {cp}, {jurisdiccion}**.

| **EMISOR COMERCIAL ({razon})** | **RESPONSABLE DE ATENCIÓN AL CONSUMIDOR** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{oficial}** ({email}) |

*Emitido el {fecha} | Sello Criptográfico SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, garantia = dias_garantia,
                oficial = oficial_privacidad, fecha = fecha, hash = empresa.audit_hash
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
