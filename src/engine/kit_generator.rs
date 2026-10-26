//! Generador Determinístico de Kits Legales Corporativos 100% personalizados (Cero Alucinación LLM).
//! Produce 7 instrumentos jurídicos listos para impresión y firma autógrafa del Representante Legal:
//! 1. Aviso de Privacidad Integral, Simplificado, Derechos ARCO y Consentimiento Biométrico (LFPDPPP / SABG).
//! 2. Política de Prevención de Riesgos Psicosociales (NOM-035-STPS-2018).
//! 3. Acta Constitutiva de Comisión Mixta de Seguridad e Higiene y Diagnóstico SST (NOM-019 y NOM-030-STPS).
//! 4. Política de Teletrabajo, Derecho a la Desconexión y Checklist Ergonómico (NOM-037-STPS-2023).
//! 5. Contrato Marco de Materialidad B2B con Cláusula de Fecha Cierta SCJN y Bitácora (Art. 69-B CFF).
//! 6. Protocolo de Blindaje Solidario Patronal IMSS 32-D y Expediente REPSE / ICSOE / SISSUB.
//! 7. Términos y Condiciones de Comercio Electrónico, Devoluciones y Garantías (Art. 76 BIS LFPC - PROFECO).

use crate::engine::models::EmpresaB2B;
use chrono::Utc;
use std::fs::{self, File};
use std::io::Write;

pub struct GeneratedKitDocument {
    pub zip_relative_path: String,
    pub disk_filename: String,
    pub content: String,
}

/// Construye en memoria los 7 instrumentos legales personalizados con la Razón Social, RFC, Domicilio y Sello SHA-256.
pub fn build_all_compliance_documents(empresa: &EmpresaB2B) -> Vec<GeneratedKitDocument> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let fecha = Utc::now().format("%d de %B de %Y").to_string();
    let email_arco = if empresa.email.trim().is_empty() {
        "privacidad@empresa.com.mx"
    } else {
        empresa.email.trim()
    };

    vec![
        // DOCUMENTO 1: LFPDPPP (SABG / ex-INAI) — Aviso Integral + Consentimiento Biométrico + Acta Oficial Privacidad
        GeneratedKitDocument {
            zip_relative_path: "01_SABG_INAI_Proteccion_Datos_LFPDPPP/01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.md".to_string(),
            disk_filename: "01_Aviso_de_Privacidad_Integral.md".to_string(),
            content: format!(
r#"# AVISO DE PRIVACIDAD INTEGRAL, CLÁUSULA DE CONSENTIMIENTO EXPRESO PARA DATOS BIOMÉTRICOS Y DESIGNACIÓN DE OFICIAL DE PRIVACIDAD
**(Cumplimiento estricto a los Artículos 8, 9, 15, 16, 17, 19, 22 al 37 de la Ley Federal de Protección de Datos Personales en Posesión de los Particulares vigente ante la Autoridad Garante Federal — SABG / Transparencia para el Pueblo)**

---

### I. IDENTIDAD Y DOMICILIO DEL RESPONSABLE
**{nombre}** (en lo sucesivo, "EL RESPONSABLE"), con Registro Federal de Contribuyentes **{rfc}** y domicilio convencional para oír y recibir notificaciones en **{domicilio}**, es el responsable del uso, tratamiento, almacenamiento y protección de sus datos personales, datos patrimoniales/financieros y datos personales sensibles.

### II. DEPARTAMENTO DE DATOS PERSONALES (OFICIAL DE PRIVACIDAD — ART. 17 LFPDPPP)
Por acuerdo de la administración de **{nombre}**, se designa formalmente al **Departamento de Protección de Datos Personales**, localizable en el correo electrónico oficial **{email}** y en el domicilio físico antes señalado, como el órgano interno encargado de dar trámite a las solicitudes de los titulares para el ejercicio de los Derechos ARCO y vigilar el cumplimiento del Documento de Seguridad Técnico, Administrativo y Físico (Art. 19 LFPDPPP).

### III. DATOS PERSONALES Y DATOS SENSIBLES SOMETIDOS A TRATAMIENTO
Para las finalidades señaladas en el presente Aviso de Privacidad, EL RESPONSABLE podrá recabar las siguientes categorías de datos:
1. **Datos de Identificación y Contacto:** Nombre completo, RFC, CURP, domicilio fiscal y particular, teléfono fijo/móvil, correo electrónico e identificación oficial vigente.
2. **Datos Financieros y Patrimoniales (Art. 8 párrafo cuarto LFPDPPP):** Cuentas bancarias, CLABE interbancaria, constancia de situación fiscal y referencias comerciales para facturación CFDI 4.0 y dispersión de pagos o nómina.
3. **Datos Personales Sensibles y Biométricos (Art. 9 LFPDPPP):** Huella dactilar, geometría facial para controles de asistencia laboral o acceso físico a instalaciones, así como certificados médicos o estado de salud requeridos por las Normas Oficiales Mexicanas de la STPS.

### IV. FINALIDADES PRIMARIAS Y SECUNDARIAS DEL TRATAMIENTO
* **Finalidades Primarias (Necesarias para la relación jurídica):**
  a) Verificar y confirmar su identidad, capacidad jurídica y situación fiscal (Art. 27 y 29-A CFF).
  b) Proveer los bienes, productos o servicios contratados y dar cumplimiento a obligaciones mercantiles o laborales.
  c) Emitir Comprobantes Fiscales Digitales por Internet (CFDI 4.0) y gestionar cobranza o pagos.
  d) Integrar expedientes laborales, control de asistencia y cumplimiento de seguridad social (IMSS/INFONAVIT/STPS).
* **Finalidades Secundarias:** Envío de comunicaciones comerciales, encuestas de calidad y boletines informativos. Si usted no desea que sus datos sean tratados para finalidades secundarias, puede manifestarlo enviando un correo a **{email}**.

### V. TRANSFERENCIA DE DATOS PERSONALES A TERCEROS (ARTS. 36 Y 37 LFPDPPP)
Sus datos personales únicamente serán transferidos sin requerir consentimiento adicional en los supuestos del Artículo 37 de la LFPDPPP: (i) Al Servicio de Administración Tributaria (SAT) mediante Proveedores Autorizados de Certificación (PAC); (ii) Al IMSS, INFONAVIT y STPS por mandato legal; y (iii) A encargados que presten servicios de procesamiento contable, jurídico o alojamiento en la nube bajo cláusulas estrictas de confidencialidad.

### VI. PROCEDIMIENTO Y MECANISMOS PARA EL EJERCICIO DE DERECHOS ARCO (ARTS. 22 A 35 LFPDPPP)
Usted tiene derecho a conocer qué datos personales tenemos de usted (**Acceso**), solicitar su corrección (**Rectificación**), que los eliminemos de nuestras bases (**Cancelación**) u oponerse a su uso (**Oposición**), así como revocar su consentimiento.
1. **Presentación de Solicitud:** Envíe un escrito libre al correo **{email}** acompañando copia digital de su identificación oficial (INE/Pasaporte) o instrumento notarial en caso de representante legal, describiendo de forma clara el derecho ARCO a ejercer.
2. **Plazos Legales Obligatorios (Art. 32 LFPDPPP):** EL RESPONSABLE comunicará la determinación adoptada en un plazo máximo de **20 (veinte) días hábiles** contados desde la recepción de la solicitud. De resultar procedente, se hará efectiva dentro de los **15 (quince) días hábiles** siguientes.

---

### ANEXO OBLIGATORIO: FORMATO DE CONSENTIMIENTO EXPRESO Y POR ESCRITO PARA DATOS SENSIBLES, BIOMÉTRICOS Y PATRIMONIALES (ARTS. 8 Y 9 LFPDPPP)

> Yo, _________________________________________________________, por mi propio derecho, manifiesto que previo al tratamiento de mis datos personales me fue puesto a disposición el Aviso de Privacidad Integral de **{nombre}** ({rfc}), y en términos del **Artículo 9 de la LFPDPPP**, otorgo mi **consentimiento expreso y por escrito** para que mis **datos personales sensibles (estado de salud y datos biométricos de huella dactilar/reconocimiento facial)** y mis **datos patrimoniales/financieros** sean tratados exclusivamente para las finalidades primarias de la relación jurídica que nos vincula.

**Lugar y Fecha:** {domicilio}, a {fecha}.

| **NOMBRE Y FIRMA AUTÓGRAFA DEL TITULAR** | **HUELLA DACTILAR** | **POR EL RESPONSABLE ({nombre})** |
| :--- | :---: | :--- |
| _______________________________________ | `[       ]` | _______________________________________ |
| Firma Autógrafa | Índice Derecho | Oficial de Privacidad / Representante Legal |

*Instrumento certificado con Sello Criptográfico SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                email = email_arco,
                fecha = fecha,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 2: STPS NOM-035-STPS-2018 — Política de Prevención de Riesgos Psicosociales
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/02_Politica_Prevencion_Psicosocial_NOM035_STPS.md".to_string(),
            disk_filename: "02_Politica_Prevencion_NOM035_STPS.md".to_string(),
            content: format!(
r#"# POLÍTICA DE PREVENCIÓN DE RIESGOS PSICOSOCIALES Y PROTOCOLO DE ACTUACIÓN (NOM-035-STPS-2018)
**Razón Social del Patrón:** {nombre}
**RFC:** {rfc} | **Ubicación del Centro de Trabajo:** {domicilio}
**Fundamento Legal:** Artículos 132 fracción XVI y 994 fracción V de la Ley Federal del Trabajo; Numerales 5.1, 7.1, 8.1 y Guía de Referencia IV de la NOM-035-STPS-2018.

---

### 1. DECLARACIÓN EXPRESA DE COMPROMISO PATRONAL
La Dirección General y Representación Legal de **{nombre}** asume el compromiso ineludible de:
1. **Prevención de Factores de Riesgo Psicosocial:** Identificar, analizar y mitigar cargas de trabajo excesivas, falta de control sobre el trabajo, jornadas superiores a las previstas en la Ley Federal del Trabajo e interferencia en la relación trabajo-familia.
2. **Prevención de la Violencia Laboral:** Cero tolerancia ante actos de acoso psicológico, hostigamiento, malos tratos o discriminación por origen étnico, género, edad, discapacidad o condición social.
3. **Promoción de un Entorno Organizacional Favorable:** Fomentar el sentido de pertenencia, la definición precisa de responsabilidades mediante descripciones de puesto, la comunicación proactiva y el reconocimiento al desempeño.

### 2. OBLIGACIONES ESCALONADAS SEGÚN EL TAMAÑO DEL CENTRO DE TRABAJO
* **Obligación Universal (1 a 15 trabajadores y superiores — Numeral 5.1 y 5.5):** Difusión permanente de esta Política, prevención de violencia laboral y aplicación del cuestionario de la **Guía de Referencia I** para identificar a los trabajadores que fueron sujetos a **Acontecimientos Traumáticos Severos** (accidentes graves, asaltos, actos violentos) durante o con motivo del trabajo, canalizándolos a valoración clínica (IMSS o médico privado).
* **Centros de 16 a 50 trabajadores (Numeral 5.2):** Aplicación bienal de los cuestionarios de la **Guía de Referencia II** (Identificación y análisis de los factores de riesgo psicosocial).
* **Centros de más de 50 trabajadores (Numeral 5.3):** Aplicación bienal de la **Guía de Referencia III** (Identificación y análisis de los factores de riesgo psicosocial y evaluación del entorno organizacional).

### 3. MECANISMO SEGURO Y CONFIDENCIAL DE RECEPCIÓN DE QUEJAS (NUMERAL 8.1 b)
Se habilita formalmente el buzón confidencial **{email}** y el buzón físico situado en el área de Recursos Humanos de **{domicilio}** para que cualquier colaborador denuncie prácticas opuestas al entorno organizacional favorable o actos de violencia laboral, garantizando la prohibición estricta de represalias.

---

### CONSTANCIA DE APROBACIÓN, DIFUSIÓN Y EXHIBICIÓN FÍSICA

| **REPRESENTANTE LEGAL DEL PATRÓN** | **REPRESENTANTE DE LOS TRABAJADORES** |
| :--- | :--- |
| ___________________________________________ | ___________________________________________ |
| **{nombre}** ({rfc}) | Comisión de Seguridad e Higiene |

*Fecha Cierta de Emisión:* {fecha} | *Sello Criptográfico SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                email = email_arco,
                fecha = fecha,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 3: STPS NOM-019-STPS-2011 y NOM-030-STPS-2009 — Acta Constitutiva Comisión Mixta + Diagnóstico SST
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.md".to_string(),
            disk_filename: "03_Acta_Comision_Mixta_NOM019_NOM030.md".to_string(),
            content: format!(
r#"# ACTA CONSTITUTIVA DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (NOM-019-STPS-2011) Y DESIGNACIÓN DEL RESPONSABLE DE SERVICIOS PREVENTIVOS (NOM-030-STPS-2009)
**Fundamento Legal:** Artículos 509, 510 y 994 Fracción V de la Ley Federal del Trabajo; Numerales 5.1, 7.1, 7.3 y 7.4 de la NOM-019-STPS-2011; Numerales 4.1 y 5.1 de la NOM-030-STPS-2009.

---

### I. DATOS OFICIALES DEL CENTRO DE TRABAJO (NUMERAL 7.4 NOM-019-STPS-2011)
* **Nombre, Denominación o Razón Social:** {nombre}
* **Registro Federal de Contribuyentes (RFC):** {rfc}
* **Domicilio Completo del Centro de Trabajo:** {domicilio}
* **Rama Industrial o Actividad Económica (SCIAN):** {actividad}
* **Fecha de Integración de la Comisión:** {fecha}

### II. CONSTITUCIÓN E INTEGRACIÓN DE LA COMISIÓN MIXTA
Siendo las 10:00 horas de la fecha señalada, reunidos en las instalaciones de **{nombre}**, los representantes del patrón y de los trabajadores acuerdan constituir formalmente la **Comisión de Seguridad e Higiene**, cuya función será investigar las causas de los accidentes y enfermedades de trabajo, proponer medidas para prevenirlos y vigilar que se cumplan mediante recorridos trimestrales de verificación:

| **CARGO EN LA COMISIÓN** | **NOMBRE COMPLETO DEL INTEGRANTE** | **RFC / CURP** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- | :--- |
| **Coordinador (Representante Patronal)** | ____________________________________ | ________________ | __________________ |
| **Secretario (Representante Trabajadores)** | ____________________________________ | ________________ | __________________ |
| **Vocal 1 (Representante Patronal)** | ____________________________________ | ________________ | __________________ |
| **Vocal 2 (Representante Trabajadores)** | ____________________________________ | ________________ | __________________ |

### III. DESIGNACIÓN DEL RESPONSABLE DE SEGURIDAD Y SALUD EN EL TRABAJO (NOM-030-STPS-2009)
En cumplimiento al Numeral 4.1 de la **NOM-030-STPS-2009**, el patrón **{nombre}** designa al Coordinador Patronal arriba firmante como **Responsable de los Servicios Preventivos de Seguridad y Salud en el Trabajo**, quien tendrá a su cargo elaborar el Diagnóstico Integral de Seguridad y Salud y dar seguimiento al Programa Anual de Prevención (PASST) respecto a las normas de edificio (NOM-001), prevención de incendios (NOM-002), equipo de protección personal (NOM-017), iluminación (NOM-025), colores y señales (NOM-026) y mantenimiento eléctrico (NOM-029).

### IV. CALENDARIO ANUAL DE RECORRIDOS TRIMESTRALES DE VERIFICACIÓN
1. **Primer Recorrido Trimestral:** Primeros 10 días hábiles de Enero (Inspección de instalaciones, extintores y botiquín).
2. **Segundo Recorrido Trimestral:** Primeros 10 días hábiles de Abril (Revisión de instalaciones eléctricas, tierras físicas y EPP).
3. **Tercer Recorrido Trimestral:** Primeros 10 días hábiles de Julio (Simulacro de evacuación y revisión estructural).
4. **Cuarto Recorrido Trimestral:** Primeros 10 días hábiles de Octubre (Evaluación anual de factores psicosociales y ergonómicos).

*Sello Criptográfico de Integridad SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                actividad = empresa.actividad,
                fecha = fecha,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 4: STPS NOM-037-STPS-2023 — Política de Teletrabajo y Derecho a la Desconexión
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.md".to_string(),
            disk_filename: "04_Politica_Teletrabajo_NOM037.md".to_string(),
            content: format!(
r#"# POLÍTICA DE TELETRABAJO, DERECHO A LA DESCONEXIÓN Y LISTA DE VERIFICACIÓN DE CONDICIONES DE SEGURIDAD (NOM-037-STPS-2023)
**Empresa:** {nombre} ({rfc}) | **Domicilio:** {domicilio}
**Fundamento Legal:** Artículos 330-A al 330-K de la Ley Federal del Trabajo y Numerales 5.2, 5.4 y 7.1 de la NOM-037-STPS-2023.

---

### 1. OBJETO Y ÁMBITO DE APLICACIÓN
La presente Política rige las condiciones de seguridad y salud para las personas trabajadoras de **{nombre}** que desarrollen más del **40% (cuarenta por ciento)** de su jornada laboral semanal en un domicilio distinto al centro de trabajo utilizando tecnologías de la información y comunicación.

### 2. DERECHO A LA DESCONEXIÓN DIGITAL (ART. 330-E FRACC. VI LFT)
**{nombre}** garantiza de manera irrestricta el **Derecho a la Desconexión** al término de la jornada laboral pactada, durante pausas de alimentos, fines de semana, días de descanso obligatorio, incapacidades y periodos vacacionales. Ningún trabajador en modalidad de teletrabajo estará obligado a responder correos, mensajes o llamadas fuera de su horario contractual, salvo casos de fuerza mayor debidamente justificados.

### 3. INSUMOS, COSTOS PROPORCIONALES Y REVERSIBILIDAD (ARTS. 330-E Y 330-G LFT)
1. El patrón proporcionará y dará mantenimiento al equipo de cómputo, silla ergonómica y herramientas necesarias.
2. En los recibos de nómina CFDI 4.0 se desglosará el concepto de pago proporcional por consumo de servicios de telecomunicaciones (internet) y suministro de energía eléctrica.
3. El cambio a la modalidad de teletrabajo es voluntario y **reversible** hacia la modalidad presencial cuando así lo acuerden las partes o se actualicen causas operativas justificadas.

### 4. LISTA DE VERIFICACIÓN DE CONDICIONES DEL LUGAR DE TELETRABAJO (AUTO-INSPECCIÓN)
| **CRITERIO ERGONÓMICO, FÍSICO Y ELÉCTRICO (NOM-037)** | **CUMPLE (SÍ/NO)** | **OBSERVACIONES** |
| :--- | :---: | :--- |
| 1. Área de trabajo limpia, ordenada, iluminada y con ventilación natural o artificial adecuada. | `[   ]` | |
| 2. Escritorio o mesa a una altura de 70-75 cm que permite apoyar antebrazos a 90° sin tensión. | `[   ]` | |
| 3. Silla ergonómica con soporte lumbar y ajuste de altura que permite apoyar los pies en el piso. | `[   ]` | |
| 4. Instalación eléctrica sin cables expuestos, sin sobrecarga de multicontactos y con supresor de picos. | `[   ]` | |
| 5. Pasillos libres de obstáculos hacia la salida del inmueble ante sismo o incendio. | `[   ]` | |

**Firma de Conformidad del Trabajador en Teletrabajo:** _____________________________________
*Sello Criptográfico SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 5: SAT ART. 69-B CFF — Contrato Marco de Materialidad B2B con Fecha Cierta SCJN + Bitácora
        GeneratedKitDocument {
            zip_relative_path: "03_SAT_Materialidad_Fiscal_69B/05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.md".to_string(),
            disk_filename: "05_Contrato_Materialidad_SAT_69B.md".to_string(),
            content: format!(
r#"# CONTRATO MARCO DE PRESTACIÓN DE SERVICIOS ESPECIALIZADOS CON CLÁUSULA DE MATERIALIDAD FISCAL (ART. 69-B CFF), FECHA CIERTA Y ACTA DE RECEPCIÓN DE ENTREGABLES
**Fundamento Legal:** Artículos 27 Fracción I de la LISR, 5 Fracción I de la LIVA, 29-A y 69-B del Código Fiscal de la Federación, Artículos 89 a 94 del Código de Comercio y Jurisprudencia **2a./J. 161/2019 (10a.) de la Suprema Corte de Justicia de la Nación (Fecha Cierta)**.

---

### DECLARACIONES DE LAS PARTES
**I. Declara "EL CLIENTE" ({nombre}):**
a) Ser una persona moral/física legalmente constituida conforme a las leyes mexicanas, con Registro Federal de Contribuyentes **{rfc}** y domicilio fiscal en **{domicilio}**.
b) Que requiere contratar los servicios descritos en el presente instrumento por ser **estrictamente indispensables** para la consecución de su objeto social y generación de ingresos gravados en términos del Artículo 27 Fracción I de la Ley del Impuesto sobre la Renta.

**II. Declara "EL PROVEEDOR" (_________________________________________________):**
a) Contar con RFC ________________________, Opinión de Cumplimiento 32-D en sentido **POSITIVO**, domicilio fiscal con estatus **LOCALIZADO** ante el SAT, y no encontrarse listado de manera presunta ni definitiva en los supuestos del **Artículo 69 ni 69-B del Código Fiscal de la Federación**.
b) Contar con los activos, personal propio dado de alta ante el IMSS, infraestructura técnica y capacidad material directa para ejecutar los servicios objeto de este contrato.

---

### CLÁUSULAS DE BLINDAJE FISCAL Y SUSTANCIA ECONÓMICA

**PRIMERA (OBJETO ESPECÍFICO Y PROHIBICIÓN DE CONCEPTOS GENÉRICOS):** EL PROVEEDOR se obliga a prestar en favor de **{nombre}** los servicios consistentes en: ________________________________________________________________________________. Queda estrictamente prohibido emitir CFDI 4.0 con descripciones genéricas (tales como *"Asesoría"*, *"Servicios profesionales"* o *"Comisiones"*); todo comprobante fiscal deberá referenciar el folio de la **Bitácora de Entregables** anexa a este contrato.

**SEGUNDA (EXPEDIENTE DE MATERIALIDAD Y RAZÓN DE NEGOCIOS — ART. 5-A Y 69-B CFF):** Para efectos de acreditar ante la autoridad hacendaria la efectiva realización de las operaciones, será requisito indispensable para liberar cualquier pago que EL PROVEEDOR entregue a **{nombre}** el **Dossier Mensual de Materialidad**, integrado por:
1. Cotización u Orden de Compra previa autorizada con sello de tiempo.
2. Evidencia tangible del servicio (informes técnicos, código fuente, planos, fotografías geolocalizadas, minutas de reuniones con lista de asistencia y correos de seguimiento).
3. Acta de Entrega-Recepción firmada por el responsable operativo de **{nombre}**.
4. Opinión del Cumplimiento de Obligaciones Fiscales (Art. 32-D CFF) vigente del mes de facturación.

**TERCERA (GARANTÍA DE INDEMNIDAD POR ART. 69-B CFF):** En caso de que EL PROVEEDOR sea publicado por el SAT en el Diario Oficial de la Federación en el listado preliminar o definitivo del Artículo 69-B del CFF dentro de los 5 años posteriores a la firma de este contrato, se obliga a comparecer dentro de los 15 días legales para desvirtuar los hechos e indemnizar al 100% a **{nombre}** por cualquier crédito fiscal, actualización o recargo derivado de sus comprobantes.

**CUARTA (SELLADO DE FECHA CIERTA DIGITAL Y CONSERVACIÓN NOM-151):** Las partes acuerdan que el presente contrato y sus bitácoras de entregables se firman y resguardan mediante huella criptográfica SHA-256 anclada en registro distribuido inmutable, surtiendo plenos efectos probatorios conforme a los Artículos 89 Bis y 93 del Código de Comercio y Artículo 210-A del Código Federal de Procedimientos Civiles.

---

### ANEXO A: ACTA DE ENTREGA-RECEPCIÓN Y BITÁCORA DE MATERIALIDAD (ART. 69-B CFF)

| **FOLIO CFDI 4.0 VINCULADO** | **DESCRIPCIÓN TÉCNICA DEL ENTREGABLE RECIBIDO** | **BENEFICIO ECONÓMICO / RAZÓN DE NEGOCIOS (ART. 5-A CFF)** | **FECHA Y FIRMA DE CONFORMIDAD** |
| :--- | :--- | :--- | :--- |
| UUID: ______________________ | | | |
| UUID: ______________________ | | | |

| **POR "EL CLIENTE": {nombre} ({rfc})** | **POR "EL PROVEEDOR"** | **TESTIGO DE ASISTENCIA / AUDITOR** |
| :--- | :--- | :--- |
| _____________________________________ | _____________________________________ | _____________________________________ |
| Representante Legal | Representante Legal | Nombre y Firma |

*Fecha Cierta de Generación:* {fecha} | *Sello Criptográfico SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                fecha = fecha,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 6: IMSS / INFONAVIT / STPS — Protocolo de Blindaje Solidario REPSE, ICSOE y SISSUB
        GeneratedKitDocument {
            zip_relative_path: "04_IMSS_INFONAVIT_Seguridad_Social_REPSE/06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.md".to_string(),
            disk_filename: "06_Protocolo_IMSS_REPSE_32D.md".to_string(),
            content: format!(
r#"# PROTOCOLO INTERNO DE AUDITORÍA IMSS 32-D, INTEGRACIÓN SALARIAL (SBC) Y BLINDAJE DE RESPONSABILIDAD SOLIDARIA EN SUBCONTRATACIÓN (REPSE / ICSOE / SISSUB)
**Patrón / Contratante:** {nombre} ({rfc}) | **Domicilio:** {domicilio}
**Fundamento Legal:** Artículos 12 a 15 de la Ley Federal del Trabajo; Artículos 15, 15-A, 27, 39-C, 71 a 75 y 304-A de la Ley del Seguro Social; Artículo 29 Bis de la Ley del INFONAVIT; Artículo 27 Fracción V párrafo tercero de la LISR y Artículo 5 Fracción II de la LIVA.

---

### 1. REGLA DE ORO PARA LA DEDUCIBILIDAD DE SERVICIOS ESPECIALIZADOS (ART. 27 LISR / ART. 5 LIVA)
Queda estrictamente prohibido al departamento de Cuentas por Pagar de **{nombre}** liberar el pago o acreditar el IVA de cualquier proveedor que ponga trabajadores a disposición en nuestras instalaciones (limpieza, vigilancia, mantenimiento industrial, construcción, soporte en sitio), sin haber integrado previamente en la bóveda mensual los **7 documentos obligatorios del Artículo 27 Fracción V de la LISR**:

| **N°** | **DOCUMENTO OBLIGATORIO DEL CONTRATISTA REPSE (MENSUAL)** | **FUNDAMENTO LEGAL** | **VERIFICADO** |
| :---: | :--- | :--- | :---: |
| **1** | Copia vigente del Registro ante la STPS (**Padrón Público REPSE**) cuyo folio coincida con el contrato. | Art. 15 LFT / Art. 27 Fracc. V LISR | `[   ]` |
| **2** | Copia de los **CFDI 4.0 de Nómina** de cada trabajador asignado al servicio en **{nombre}**. | Art. 27 Fracc. V LISR | `[   ]` |
| **3** | Recibo bancario de pago del entero de las **Retenciones de ISR de salarios** de dichos trabajadores. | Art. 27 Fracc. V LISR | `[   ]` |
| **4** | Declaración mensual de **IVA** del contratista y su acuse de recibo de pago bancario del periodo. | Art. 5 Fracc. II LIVA | `[   ]` |
| **5** | Cédula de determinación **SUA** y comprobante de pago de cuotas obrero-patronales al **IMSS**. | Art. 15-A LSS / Art. 27 LISR | `[   ]` |
| **6** | Comprobante de pago de las aportaciones y amortizaciones al **INFONAVIT** (Bimestral). | Art. 29 Bis LINFONAVIT | `[   ]` |
| **7** | Acuses de las informativas cuatrimestrales **ICSOE (IMSS)** y **SISSUB (INFONAVIT)** (Enero, Mayo y Septiembre). | Art. 15-A LSS / Art. 304-A Fracc. XXII | `[   ]` |

### 2. CALENDARIO DE CONTROL PATRONAL INTERNO ({nombre})
* **Día 3 de cada mes:** Descarga de Opinión de Cumplimiento **32-D del IMSS** y Constancia de Situación Fiscal del **INFONAVIT**, conciliando la Emisión Mensual Anticipada (EMA) contra los CFDI de nómina timbrados en el Visor del SAT.
* **Del 1 al 28 de Febrero de cada año:** Presentación obligatoria de la **Determinación Anual de la Prima en el Seguro de Riesgos de Trabajo** (Arts. 72 y 74 LSS) derivada de los casos de riesgos de trabajo terminados (ST-7 y ST-2).

*Aprobado por la Administración de {nombre} ({rfc}) — Sello SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                hash = empresa.audit_hash
            ),
        },

        // DOCUMENTO 7: PROFECO LFPC ART. 76 BIS — Términos y Condiciones de Comercio Electrónico y Póliza de Garantía
        GeneratedKitDocument {
            zip_relative_path: "05_PROFECO_Comercio_Digital_LFPC/07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.md".to_string(),
            disk_filename: "07_Terminos_Ecommerce_PROFECO_76BIS.md".to_string(),
            content: format!(
r#"# TÉRMINOS Y CONDICIONES DE COMERCIO ELECTRÓNICO, POLÍTICA DE DEVOLUCIONES Y PÓLIZA DE GARANTÍA (ART. 76 BIS Y 77 LFPC — PROFECO)
**Proveedor Responsable:** {nombre}
**RFC:** {rfc} | **Domicilio Físico en Territorio Nacional:** {domicilio}
**Correo y Teléfono de Atención a Clientes:** {email} | Tel: {telefono}
**Fundamento Legal:** Artículos 7, 7 BIS, 32, 56, 76 BIS, 76 BIS 1, 77, 78, 85 y 128 BIS de la Ley Federal de Protección al Consumidor (LFPC) y Norma Mexicana **NMX-COE-001-SCFI-2018**.

---

### 1. IDENTIDAD DEL PROVEEDOR Y TRANSPARENCIA DE PRECIOS (ART. 7 BIS Y 76 BIS FRACC. IV LFPC)
Todas las transacciones celebradas a través del portal digital de **{nombre}** se rigen por la legislación mercantil y de protección al consumidor de los Estados Unidos Mexicanos. Todos los precios exhibidos se expresan en **Moneda Nacional (Pesos Mexicanos — MXN)** e incluyen de manera visible el Impuesto al Valor Agregado (IVA) y cualquier otro impuesto aplicable, desglosando previo al cierre de la compra el costo de envío y el monto total a pagar.

### 2. CONFIDENCIALIDAD Y SEGURIDAD EN TRANSACCIONES ELECTRÓNICAS (ART. 76 BIS FRACC. II Y 128 BIS LFPC)
**{nombre}** utiliza certificados de cifrado criptográfico TLS/SSL en tránsito y procesa los pagos exclusivamente a través de pasarelas certificadas bajo el estándar internacional **PCI-DSS**, sin almacenar códigos de seguridad (CVV/CVC) en servidores propios.

### 3. DERECHO DE REVOCACIÓN, CANCELACIÓN Y DEVOLUCIONES (ART. 56 Y 76 BIS FRACC. III LFPC)
1. **Plazo Legal de Revocación:** En ventas a distancia o por medios electrónicos, el consumidor cuenta con un plazo improrrogable de **5 (cinco) días hábiles** contados a partir de la entrega del bien o de la firma del contrato para revocar su consentimiento sin responsabilidad alguna, siempre que el bien no haya sido utilizado ni sufrido deterioro imputable al consumidor.
2. **Procedimiento de Reembolso:** Para ejercer la devolución o cancelación, el consumidor deberá notificarlo al correo **{email}** indicando su número de pedido. **{nombre}** efectuará el reembolso mediante el mismo medio de pago utilizado en la compra dentro de los **15 días hábiles** siguientes a la recepción física de la devolución.

### 4. PÓLIZA DE GARANTÍA LEGAL MÍNIMA (ARTS. 77 Y 78 LFPC)
Sin perjuicio de las garantías extendidas del fabricante, todo producto o servicio comercializado por **{nombre}** cuenta con una **garantía mínima legal de 90 (noventa) días naturales** contados a partir de su entrega contra cualquier defecto de fabricación, vicio oculto o incumplimiento de las especificaciones ofrecidas, la cual podrá hacerse efectiva directamente en **{domicilio}** o a través del canal **{email}**, cubriendo el proveedor los gastos de transportación que deriven del cumplimiento de la garantía (Art. 79 LFPC).

*Emitido el {fecha} | Sello de Integridad Criptográfica SHA-256:* `{hash}`
"#,
                nombre = empresa.nombre,
                rfc = tax_id,
                domicilio = empresa.direccion,
                email = email_arco,
                telefono = if empresa.telefono.trim().is_empty() { "55-0000-0000" } else { &empresa.telefono },
                fecha = fecha,
                hash = empresa.audit_hash
            ),
        },
    ]
}

/// Función original 100% retrocompatible: guarda en disco los 7 instrumentos dentro de `kits_blindaje/{tax_id}`.
pub fn generate_compliance_kit(empresa: &EmpresaB2B) -> Result<String, Box<dyn std::error::Error>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let kit_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&kit_dir)?;

    let docs = build_all_compliance_documents(empresa);
    for doc in docs {
        let path = format!("{}/{}", kit_dir, doc.disk_filename);
        let mut f = File::create(&path)?;
        f.write_all(doc.content.as_bytes())?;
    }

    Ok(kit_dir)
}
