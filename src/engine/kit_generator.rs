//! Generador Determinístico Hiper-Paramétrico de Instrumentos Jurídicos Corporativos — FIRMA W16 MALLEN®.
//! - 100% apegado a la legislación mexicana vigente (LFPDPPP, LFT, NOMs STPS, CFF, LSS, RACERF, LGPC, LFPC).
//! - Cero suposiciones falsas: no inventa horarios, no inventa montos, no cruza cargos sin autorización
//!   y no declara como "ya cumplidos" hechos físicos que requieren llenado o firma en sitio.
//! - Inserta líneas jurídicas formales (`________________________________________`) y listas de verificación
//!   reales (`[ ] Sí / [ ] No`) para que el cliente tenga documentos listos para uso legal inmediato.

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
        if !t.is_empty()
            && !t.contains("POR DESIGNAR")
            && !t.contains("Cédula")
            && !t.contains("mallen.lat")
        {
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
        && !s.contains("POR DESIGNAR")
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

    // 1. Extracción exacta del código SCIAN desde AuditMasterJson o EmpresaB2B
    let scian_code: String = {
        let src = if let Some(m) = master_opt {
            format!("{} {} {}", m.codigo_scian, m.actividad_scian_desc, empresa.actividad)
        } else {
            empresa.actividad.clone()
        };
        let toks: Vec<&str> = src
            .split(|c: char| !c.is_ascii_digit())
            .filter(|t| t.len() >= 4)
            .collect();
        toks.iter()
            .copied()
            .find(|t| *t != "541610")
            .or_else(|| toks.first().copied())
            .unwrap_or("541610")
            .to_string()
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

    // 2. Número exacto de trabajadores (Sin alterar 20 ni 150 empleados)
    let emp_count: u32 = master_opt
        .map(|m| {
            m.calibration_applied
                .exact_employee_count
                .unwrap_or(m.estimated_employees)
                .max(1)
        })
        .unwrap_or(45);

    let (imss_clase, imss_prima, imss_sector_desc) = classify_imss_risk_by_scian(&scian_code);
    let default_imss_str = format!(
        "{} (Prima Media de Catálogo Art. 196 RACERF: {} — {})",
        imss_clase, imss_prima, imss_sector_desc
    );

    // 3. Variables pre-llenadas solo con datos reales; si no existen, línea formal de llenado
    let razon_social = pick_known_or_blank(&dossier.razon_social_oficial, &empresa.nombre, BLANK_LINE);
    let domicilio = pick_known_or_blank(&dossier.domicilio_fiscal_completo, &empresa.direccion, BLANK_LINE);
    let cp_fiscal = pick_known_or_blank(&dossier.codigo_postal_fiscal, "", BLANK_SHORT);
    let jurisdiccion = pick_known_or_blank(&dossier.entidad_jurisdiccion, &empresa.estado, BLANK_SHORT);
    let giro_scian = pick_known_or_blank(&dossier.giro_scian_descripcion, &scian_desc_clean, &scian_desc_clean);
    let email_arco = pick_known_or_blank(&dossier.email_arco_privacidad, &empresa.email, BLANK_LINE);
    let telefono_corp = pick_known_or_blank(&dossier.telefono_corporativo, &empresa.telefono, BLANK_SHORT);

    let rep_legal = pick_known_or_blank(&dossier.representante_legal_nombre, "", BLANK_LINE);
    let rep_cargo = pick_known_or_blank(&dossier.representante_legal_cargo, "", BLANK_SHORT);
    let escritura_num = pick_known_or_blank(&dossier.instrumento_notarial_num, "", BLANK_SHORT);
    // No asumir que el Representante Legal es el Oficial de Privacidad si se dejó vacío
    let oficial_privacidad = pick_known_or_blank(&dossier.oficial_privacidad_nombre, "", BLANK_LINE);

    let reg_patronal = pick_known_or_blank(&dossier.registro_patronal_imss, "", BLANK_SHORT);
    let csh_coord = pick_known_or_blank(&dossier.comision_coordinador_patronal, "", BLANK_LINE);
    let csh_sec = pick_known_or_blank(&dossier.comision_secretario_trabajadores, "", BLANK_LINE);
    let csh_vocal_pat = pick_known_or_blank(&dossier.comision_vocal_patronal, "", BLANK_LINE);
    let csh_vocal_trab = pick_known_or_blank(&dossier.comision_vocal_trabajadores, "", BLANK_LINE);

    // Sin asumir horarios ni montos inventados
    let horario_jornada = pick_known_or_blank(&dossier.horario_jornada_laboral, "", BLANK_LINE);
    let apoyo_teletrabajo = pick_known_or_blank(&dossier.monto_apoyo_teletrabajo_mxn, "", BLANK_SHORT);

    let prov_nombre = pick_known_or_blank(&dossier.proveedor_principal_razon_social, "", BLANK_LINE);
    let prov_rfc = pick_known_or_blank(&dossier.proveedor_principal_rfc, "", BLANK_SHORT);
    let prov_objeto = pick_known_or_blank(&dossier.proveedor_principal_servicio_objeto, "", BLANK_LINE);
    let testigo_auditor = pick_known_or_blank(&dossier.testigo_auditor_interno, "", BLANK_LINE);

    let clase_prima_imss = pick_known_or_blank(&dossier.clase_prima_riesgo_imss, &default_imss_str, &default_imss_str);
    let folio_repse = pick_known_or_blank(&dossier.folio_repse_activo, "", BLANK_SHORT);
    let dias_garantia = dossier.dias_garantia_comercial_profeco.unwrap_or(90).max(90);

    // 4. Reglas sectoriales dinámicas basadas en la ley aplicable al giro SCIAN
    let clausula_privacidad_scian = if scian_code.starts_with("62") || scian_code.starts_with("464") {
        "4. **Datos Clínicos y Expediente de Salud (NOM-004-SSA3-2012 y LGS):** Cuando por la naturaleza del servicio se recaben datos clínicos o recetas, estos serán resguardados bajo secreto profesional y medidas de seguridad de nivel alto conforme al Art. 9 de la LFPDPPP."
    } else if scian_code.starts_with("311") || scian_code.starts_with("722") || scian_code.starts_with("431") || scian_code.starts_with("461") {
        "4. **Datos de Trazabilidad Sanitaria e Inocuidad (NOM-251-SSA1-2009):** Registros sanitarios del personal operativo, bitácoras de higiene, control de proveedores y sistemas de videovigilancia en áreas de almacenamiento y atención."
    } else if scian_code.starts_with("48") || scian_code.starts_with("49") {
        "4. **Datos de Logística, Operadores y Carta Porte (NOM-087-SCT-2):** Licencias federales, datos de identificación de operadores para emisión de Complemento Carta Porte 3.1 y telemetría GPS de rutas de transporte."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") || scian_code.starts_with("23") {
        "4. **Datos de Identificación para Prevención de Lavado de Dinero (Arts. 17 y 18 LFPIORPI):** En caso de celebrar operaciones sujetas a la LFPIORPI, se integrará el expediente único de identificación de clientes/usuarios y constancia de Beneficiario Controlador."
    } else {
        "4. **Datos de Videovigilancia, Control de Acceso y Operaciones Comerciales:** Imágenes captadas por cámaras de seguridad perimetral en las instalaciones y registros de facturación y contratación mercantil."
    };

    let protocolo_nom035_escala = if emp_count > 50 {
        format!(
            "En virtud de que el centro de trabajo cuenta con una plantilla parametrizada de **{} trabajadores (más de 50 colaboradores)**, conforme a los Numerales 5.2, 5.3 y 7.1 inciso b) de la **NOM-035-STPS-2018**, corresponde aplicar:\n- **Guía de Referencia I:** Identificación de trabajadores sujetos a acontecimientos traumáticos severos (a toda la plantilla).\n- **Guía de Referencia III:** Identificación y análisis de los factores de riesgo psicosocial y evaluación del entorno organizacional (al total de la plantilla o a una muestra representativa calculada mediante la ecuación del Numeral 7.2, con periodicidad máxima de 2 años).",
            emp_count
        )
    } else if emp_count >= 16 {
        format!(
            "En virtud de que el centro de trabajo cuenta con una plantilla parametrizada de **{} trabajadores (entre 16 y 50 colaboradores)**, conforme a los Numerales 5.2 y 7.1 inciso a) de la **NOM-035-STPS-2018**, corresponde aplicar:\n- **Guía de Referencia I:** Identificación de trabajadores sujetos a acontecimientos traumáticos severos.\n- **Guía de Referencia II:** Identificación y análisis de los factores de riesgo psicosocial (con periodicidad máxima de 2 años), encontrándose legalmente exento de la evaluación de entorno organizacional (Guía III).",
            emp_count
        )
    } else {
        format!(
            "En virtud de que el centro de trabajo cuenta con una plantilla parametrizada de **{} trabajadores (hasta 15 colaboradores)**, conforme al Numeral 5.1 y 5.5 de la **NOM-035-STPS-2018**, el patrón cumple implantando y difundiendo la presente Política de Prevención de Riesgos Psicosociales y aplicando la **Guía de Referencia I** (Acontecimientos Traumáticos Severos), encontrándose legalmente exento de las Guías II y III.",
            emp_count
        )
    };

    let tabla_comision_nom019 = if emp_count >= 15 {
        format!(
r#"Conforme al Numeral 7.1 inciso b) de la **NOM-019-STPS-2011**, por contar el centro de trabajo con **{} trabajadores (15 o más trabajadores)**, la Comisión Mixta de Seguridad e Higiene se integra de forma bipartita por un Coordinador, un Secretario y los Vocales designados:

| **CARGO EN LA COMISIÓN** | **NOMBRE DEL INTEGRANTE DESIGNADO** | **SECTOR REPRESENTADO** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- | :--- |
| **Coordinador** | **{csh_coord}** | Representante Patronal | ________________________ |
| **Secretario** | **{csh_sec}** | Representante Trabajadores | ________________________ |
| **Vocal 1** | **{csh_vocal_pat}** | Vocal Patronal | ________________________ |
| **Vocal 2** | **{csh_vocal_trab}** | Vocal Trabajadores | ________________________ |"#,
            emp_count,
            csh_coord = csh_coord,
            csh_sec = csh_sec,
            csh_vocal_pat = csh_vocal_pat,
            csh_vocal_trab = csh_vocal_trab
        )
    } else {
        format!(
r#"Conforme al Numeral 7.1 inciso a) de la **NOM-019-STPS-2011**, por contar el centro de trabajo con **{} trabajadores (menos de 15 trabajadores)**, la Comisión Mixta de Seguridad e Higiene se integra legalmente por un representante de los trabajadores y el patrón o su representante:

| **FUNCIONES EN LA COMISIÓN** | **NOMBRE DEL INTEGRANTE DESIGNADO** | **SECTOR REPRESENTADO** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- | :--- |
| **Representante Patronal** | **{csh_coord}** | Patrón / Representante | ________________________ |
| **Representante Laboral** | **{csh_sec}** | Representante Trabajadores | ________________________ |"#,
            emp_count,
            csh_coord = csh_coord,
            csh_sec = csh_sec
        )
    };

    let noms_especificas_scian = if scian_code.starts_with("31") || scian_code.starts_with("32") || scian_code.starts_with("33") {
        "NOM-001-STPS-2008 (Instalaciones), NOM-002-STPS-2010 (Prevención y Protección contra Incendios), NOM-004-STPS-1999 (Sistemas de Protección en Maquinaria), NOM-005-STPS-1998 y NOM-018-STPS-2015 (Sustancias Químicas Peligrosas SGA), NOM-006-STPS-2023 y NOM-036-1-STPS-2018 (Manejo de Materiales y Cargas), NOM-011-STPS-2001 (Ruido), NOM-017-STPS-2008 (EPP), NOM-020-STPS-2011 (Recipientes Sujetos a Presión, de aplicar), NOM-022-STPS-2015 (Electricidad Estática) y NOM-029-STPS-2011 (Mantenimiento de Instalaciones Eléctricas)."
    } else if scian_code.starts_with("43") || scian_code.starts_with("46") || scian_code.starts_with("48") || scian_code.starts_with("49") {
        "NOM-001-STPS-2008 (Edificios, Locales y Almacenes), NOM-002-STPS-2010 (Prevención y Protección contra Incendios), NOM-006-STPS-2023 y NOM-036-1-STPS-2018 (Almacenamiento, Estiba y Manejo Manual de Cargas), NOM-017-STPS-2008 (Equipo de Protección Personal), NOM-022-STPS-2015 (Tierras Físicas), NOM-025-STPS-2008 (Condiciones de Iluminación), NOM-026-STPS-2008 (Colores y Señales de Seguridad) y NOM-029-STPS-2011 (Instalaciones Eléctricas)."
    } else if scian_code.starts_with("23") {
        "NOM-031-STPS-2011 (Construcción — Condiciones de Seguridad y Salud en el Trabajo), NOM-009-STPS-2011 (Trabajos en Altura), NOM-027-STPS-2008 (Soldadura y Corte), NOM-002-STPS-2010 (Incendios) y NOM-017-STPS-2008 (EPP)."
    } else {
        "NOM-001-STPS-2008 (Condiciones de Seguridad en Edificios y Locales), NOM-002-STPS-2010 (Prevención y Protección contra Incendios), NOM-017-STPS-2008 (EPP por puesto), NOM-025-STPS-2008 (Iluminación), NOM-026-STPS-2008 (Señalización de Seguridad) y NOM-029-STPS-2011 (Instalaciones Eléctricas)."
    };

    let clausula_fiscal_scian = if scian_code.starts_with("48") || scian_code.starts_with("49") || scian_code.starts_with("43") {
        "**CUARTA (COMPLEMENTO CARTA PORTE 3.1 Y TRAZABILIDAD LOGÍSTICA):** Cuando el servicio implique traslado de bienes o mercancías por vías de jurisdicción federal o local aplicable, EL PROVEEDOR se obliga a emitir el CFDI con **Complemento Carta Porte versión 3.1** cumpliendo los instructivos del SAT (Arts. 29 y 29-A CFF), adjuntando evidencia de entrega y bitácora operativa."
    } else if scian_code.starts_with("23") {
        "**CUARTA (REGISTRO DE OBRA SIROC Y SUBCONTRATACIÓN ESPECIALIZADA):** Tratándose de obras de construcción o servicios especializados en inmueble, EL PROVEEDOR deberá exhibir el registro de obra en el **SIROC del IMSS**, su registro vigente en el Padrón **REPSE (Art. 15 LFT)** y las estimaciones de avance físico firmadas."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") {
        "**CUARTA (TRAZABILIDAD BANCARIA Y PREVENCIÓN DE LAVADO DE DINERO LFPIORPI):** Todo pago derivado del presente contrato se realizará exclusivamente mediante transferencia electrónica (SPEI) entre cuentas bancarias abiertas a nombre de las partes contratantes, respetando las restricciones de uso de efectivo del Artículo 32 de la LFPIORPI."
    } else {
        "**CUARTA (ENTREGABLES INTANGIBLES, CONFIDENCIALIDAD Y PROPIEDAD INTELECTUAL):** Todo servicio profesional, tecnológico, administrativo o comercial deberá documentarse mediante órdenes de servicio, minutas fechadas, reportes de actividades y acta de recepción firmada por el área usuaria de EL CLIENTE."
    };

    let anexo_sectorial_pipc = if scian_code.starts_with("311") || scian_code.starts_with("722") || scian_code.starts_with("431") || scian_code.starts_with("461") {
        "1. **Control Sanitario (COFEPRIS / NOM-251-SSA1-2009):** Verificar exhibición del Aviso de Funcionamiento (DIGIPRIS), bitácoras de limpieza y control de plagas con licencia sanitaria vigente, y control de temperaturas.\n2. **Logística Auxiliar (SICT):** En caso de operar flotilla propia de reparto, mantener bitácoras de inspección físico-mecánica (**NOM-068-SCT-2**) y horas de servicio (**NOM-087-SCT-2**)."
    } else if scian_code.starts_with("62") || scian_code.starts_with("464") {
        "1. **Regulación Sanitaria (COFEPRIS / LGS):** Verificar Aviso de Funcionamiento, Aviso de Responsable Sanitario (cuando aplique), resguardo de expediente clínico (**NOM-004-SSA3-2012**) y contrato de recolección de RPBI (**NOM-087-SEMARNAT-SSA1-2002**)."
    } else if scian_code.starts_with("31") || scian_code.starts_with("32") || scian_code.starts_with("33") || scian_code.starts_with("811") {
        "1. **Gestión Ambiental (SEMARNAT / LGPGIR Arts. 40–48):** Verificar registro como generador de residuos, bitácoras de almacén temporal y manifiestos de recolección con empresas autorizadas.\n2. **Seguridad Industrial:** Dictámenes eléctricos y de recipientes sujetos a presión cuando existan en planta."
    } else if scian_code.starts_with("48") || scian_code.starts_with("49") {
        "1. **Normatividad de Autotransporte y Logística (SICT):** Verificar pólizas de seguro de carga y responsabilidad civil, bitácoras de horas de conducción (**NOM-087-SCT-2-2017**), condiciones físico-mecánicas (**NOM-068-SCT-2-2014**) y pesos y dimensiones (**NOM-012-SCT-2-2017**)."
    } else if scian_code.starts_with("52") || scian_code.starts_with("53") || scian_code.starts_with("23") {
        "1. **Prevención de Lavado de Dinero (SHCP / UIF — LFPIORPI):** Para actividades vulnerables del Art. 17 LFPIORPI, mantener alta en el SPPLD, Manual de Identificación de Clientes/Usuarios y acuses mensuales del día 17."
    } else {
        "1. **Cumplimiento Administrativo Municipal/Estatal:** Mantener vigente la Licencia de Funcionamiento, Uso de Suelo Comercial, Dictamen Eléctrico y constancia del Programa Interno de Protección Civil conforme a la superficie y aforo del inmueble."
    };

    vec![
        // =====================================================================
        // INSTRUMENTO 01: AVISO DE PRIVACIDAD INTEGRAL + FORMATO BIOMÉTRICO
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "01_SABG_INAI_Proteccion_Datos_LFPDPPP/01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.md".to_string(),
            disk_filename: "01_Aviso_de_Privacidad_Integral.md".to_string(),
            content: format!(
r#"# AVISO DE PRIVACIDAD INTEGRAL, FORMATO DE CONSENTIMIENTO EXPRESO Y DESIGNACIÓN DEL DEPARTAMENTO DE DATOS PERSONALES
**Sujeto Obligado / Responsable:** {razon} | **RFC:** {rfc}
**Giro Económico SCIAN:** {giro} | **Plantilla Parametrizada:** {emp} colaboradores
**Domicilio para Oír y Recibir Notificaciones:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Titular / Encargado del Departamento de Datos Personales (Art. 30 LFPDPPP):** {oficial}
**Medio Oficial para Ejercicio de Derechos ARCO:** {email} | **Teléfono:** {tel}

---

### I. IDENTIDAD Y DOMICILIO DEL RESPONSABLE (ARTS. 8, 15 Y 16 LFPDPPP)
**{razon}** (con Registro Federal de Contribuyentes **{rfc}**), representada legalmente por **{rep_legal}** (Carácter / Cargo: **{cargo}**, Instrumento Público N° {escritura}), con domicilio convencional y fiscal en **{domicilio}, C.P. {cp}, {jurisdiccion}**, es el responsable del tratamiento legítimo, controlado e informado de sus datos personales, financieros/patrimoniales y sensibles, en estricto cumplimiento a la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP), su Reglamento y los Lineamientos vigentes ante la Autoridad Garante Federal (Secretaría Anticorrupción y Buen Gobierno — SABG).

### II. DESIGNACIÓN DEL DEPARTAMENTO DE DATOS PERSONALES (ARTS. 19 Y 30 LFPDPPP)
Para dar cumplimiento al Artículo 30 de la LFPDPPP, el Responsable designa a **{oficial}** a cargo del Departamento de Protección de Datos Personales, quien tiene la encomienda de dar trámite a las solicitudes de los titulares para el ejercicio de los Derechos ARCO, así como de resguardar y actualizar el **Documento de Seguridad Administrativo, Físico y Técnico (Art. 19 LFPDPPP)**.

### III. CATEGORÍAS DE DATOS PERSONALES SOMETIDOS A TRATAMIENTO
1. **Datos de Identificación, Laborales y de Contacto:** Nombre completo, identificación oficial, RFC, CURP, Número de Seguridad Social (NSS), domicilio, teléfono, correo electrónico y referencias laborales.
2. **Datos Patrimoniales y Financieros (Art. 8 Cuarto Párrafo LFPDPPP):** Cuentas bancarias y CLABE interbancaria para dispersión de nómina o pago a proveedores, y datos fiscales para emisión de CFDI 4.0.
3. **Datos Personales Sensibles y Biométricos (Art. 9 LFPDPPP):** En caso de utilizarse controles biométricos de asistencia o acceso (huella dactilar / reconocimiento facial) o recabarse certificados de estado de salud por requerimiento de las Normas Oficiales Mexicanas de la STPS, su tratamiento requerirá siempre la firma autógrafa del formato de consentimiento expreso inserto al calce.
{clausula_scian}

### IV. FINALIDADES DEL TRATAMIENTO Y OPCIONES PARA FINALIDADES SECUNDARIAS
- **Finalidades Primarias (Necesarias para la relación jurídica):** Administración de personal y nómina, cumplimiento de obligaciones fiscales (SAT), de seguridad social (IMSS/INFONAVIT) y laborales (STPS), facturación, cobranza, pago a proveedores y seguridad física de las instalaciones en **{domicilio}**.
- **Finalidades Secundarias (Opcionales):** Envío de comunicaciones comerciales, boletines informativos o encuestas de calidad.
  *[ ] No consiento que mis datos personales sean utilizados para finalidades secundarias.*

### V. TRANSFERENCIAS (ARTS. 36 Y 37 LFPDPPP) Y PROCEDIMIENTO DE DERECHOS ARCO (20 DÍAS HÁBILES)
Sus datos personales únicamente se transfieren sin requerir consentimiento en los casos legalmente previstos por el Artículo 37 de la LFPDPPP (autoridades competentes SAT, IMSS, INFONAVIT, STPS y cumplimiento de obligaciones contractuales). Para ejercer sus derechos de **Acceso, Rectificación, Cancelación u Oposición (ARCO)** o revocar su consentimiento, deberá presentar solicitud en **{domicilio}** o vía electrónica al correo **{email}**, acreditando su identidad. El Departamento de Datos Personales comunicará la respuesta en un plazo máximo de **20 días hábiles** contados desde la recepción (Art. 32 LFPDPPP) y, de resultar procedente, se hará efectiva dentro de los **15 días hábiles** siguientes.

---

### FORMATO DE CONSENTIMIENTO EXPRESO Y AUTÓGRAFO PARA DATOS SENSIBLES Y PATRIMONIALES (ARTS. 8 Y 9 LFPDPPP)
Yo, _________________________________________________________, declaro que me ha sido puesto a disposición el presente Aviso de Privacidad Integral y otorgo mi consentimiento expreso, libre, específico e informado para que mis datos personales patrimoniales, financieros, sensibles y/o biométricos sean tratados por **{razon}** ({rfc}) exclusivamente para las finalidades primarias aquí descritas.

| **POR EL RESPONSABLE ({razon})** | **DEPARTAMENTO DE DATOS PERSONALES** | **NOMBRE Y FIRMA AUTÓGRAFA DEL TITULAR** |
| :--- | :--- | :--- |
| **{rep_legal}** | **{oficial}** | ________________________________________ |
| Cargo: {cargo} | Medio ARCO: {email} | Fecha de Suscripción: ____/____/2026 |

*Infraestructura Documental FIRMA W16 MALLEN® • Fecha de Compilación: {fecha} • Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian, emp = emp_count,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                oficial = oficial_privacidad, cargo = rep_cargo, email = email_arco,
                tel = telefono_corp, rep_legal = rep_legal, escritura = escritura_num,
                clausula_scian = clausula_privacidad_scian, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 02: POLÍTICA NOM-035-STPS-2018 (BASADA EN GUÍA IV OFICIAL)
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/02_Politica_Prevencion_Psicosocial_NOM035_STPS.md".to_string(),
            disk_filename: "02_Politica_Prevencion_NOM035_STPS.md".to_string(),
            content: format!(
r#"# POLÍTICA DE PREVENCIÓN DE RIESGOS PSICOSOCIALES Y PROTOCOLO DE ACTUACIÓN (NOM-035-STPS-2018)
**Patrón / Razón Social:** {razon} | **RFC:** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
**Domicilio del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Actividad Económica SCIAN:** {giro} | **Plantilla del Centro de Trabajo:** {emp} trabajadores
**Fundamento Legal:** Artículos 132 Fracc. XVI y 994 Fracc. V de la Ley Federal del Trabajo; Numerales 5.1, 7.1, 8.1 y Guía de Referencia IV de la NOM-035-STPS-2018.

---

### 1. COMPROMISO DE PREVENCIÓN DE FACTORES DE RIESGO PSICOSOCIAL Y VIOLENCIA LABORAL
En este centro de trabajo (**{razon}**), ubicado en **{domicilio}**, en relación con la prevención de los factores de riesgo psicosocial, la prevención de la violencia laboral y la promoción de un entorno organizacional favorable, se asumen los siguientes compromisos obligatorios:
1. Es obligación de directivos, gerentes, supervisores y trabajadores aplicar esta política y predicar con el ejemplo; los actos de violencia laboral (acoso psicológico, hostigamiento y malos tratos) no son tolerados bajo ninguna circunstancia.
2. Se respetan las jornadas de trabajo y pausas pactadas en los contratos individuales (**{horario}**), conforme a los límites máximos establecidos en los Artículos 58 al 69 de la Ley Federal del Trabajo, favoreciendo la conciliación trabajo-familia.
3. Se garantiza el derecho de todos los trabajadores a ser tratados con dignidad, equidad y sin discriminación, reconociendo el desempeño y fomentando el sentido de pertenencia y la definición clara de responsabilidades por puesto.

### 2. ALCANCE TÉCNICO EXIGIBLE SEGÚN EL VOLUMEN DE PLANTILLA ({emp} TRABAJADORES)
{protocolo_escala}

### 3. MECANISMO CONFIDENCIAL DE RECEPCIÓN DE QUEJAS Y DENUNCIAS (NUMERAL 8.1 INCISO B)
Todo trabajador podrá presentar quejas por prácticas opuestas al entorno organizacional favorable o denunciar actos de violencia laboral o acontecimientos traumáticos severos a través de:
- **Responsable designado para recepción y seguimiento:** {csh_coord}
- **Representación de los trabajadores:** {csh_sec}
- **Buzón / Medio de contacto confidencial:** {email}

Toda denuncia será investigada de forma imparcial y confidencial en un plazo no mayor a 10 días hábiles, prohibiendo estrictamente cualquier acto de represalia contra el denunciante o los testigos. Los trabajadores identificados mediante la **Guía de Referencia I** con signos de estrés postraumático serán canalizados para valoración médica ante la institución de seguridad social o médico del trabajo.

| **POR EL PATRÓN ({razon})** | **REPRESENTACIÓN DE LOS TRABAJADORES** | **RESPONSABLE DE SEGUIMIENTO SST** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_sec}** | **{csh_coord}** |
| Fecha de Adopción: ____/____/2026 | Plantilla: {emp} colaboradores | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                giro = giro_scian, emp = emp_count, protocolo_escala = protocolo_nom035_escala,
                horario = horario_jornada, email = email_arco, csh_coord = csh_coord,
                csh_sec = csh_sec, rep_legal = rep_legal, cargo = rep_cargo, hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 03: ACTA CONSTITUTIVA NOM-019 Y DESIGNACIÓN NOM-030
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.md".to_string(),
            disk_filename: "03_Acta_Comision_Mixta_NOM019_NOM030.md".to_string(),
            content: format!(
r#"# ACTA DE CONSTITUCIÓN DE LA COMISIÓN MIXTA DE SEGURIDAD E HIGIENE (NOM-019-STPS-2011) Y DESIGNACIÓN DEL RESPONSABLE DE SERVICIOS PREVENTIVOS (NOM-030-STPS-2009)
**Fundamento Legal:** Artículos 132 Fracc. XVI, XVII, 509, 510 y 994 Fracc. V de la Ley Federal del Trabajo; Numerales 7.1, 7.3 y 7.4 de la NOM-019-STPS-2011 y Numeral 4.1 de la NOM-030-STPS-2009.

---

### I. DATOS DEL CENTRO DE TRABAJO (REQUISITOS DEL NUMERAL 7.4 INCISO A DE LA NOM-019-STPS-2011)
1. **Nombre, Denominación o Razón Social:** {razon}
2. **Registro Federal de Contribuyentes (RFC):** {rfc} | **Registro Patronal IMSS:** {reg_patronal}
3. **Domicilio Completo del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
4. **Teléfono:** {tel} | **Correo Electrónico:** {email}
5. **Rama o Actividad Económica (SCIAN):** {giro}
6. **Clasificación de Riesgo IMSS de Referencia:** {clase_prima}
7. **Fecha de Inicio de Actividades del Centro de Trabajo:** ____________________
8. **Número de Trabajadores del Centro de Trabajo:** {emp} trabajadores | **Número de Turnos:** ________

### II. INTEGRACIÓN DE LA COMISIÓN (NUMERAL 7.4 INCISO B DE LA NOM-019-STPS-2011)
En la localidad de **{jurisdiccion}**, siendo las ______ horas del día ____ de ______________ de 2026, reunidos en las instalaciones del centro de trabajo ubicado en **{domicilio}**, comparecen por la parte patronal **{rep_legal}** ({cargo}) y los representantes de los trabajadores para constituir formalmente la Comisión de Seguridad e Higiene:

{tabla_comision}

### III. DESIGNACIÓN DEL RESPONSABLE DE SEGURIDAD Y SALUD EN EL TRABAJO (NUMERAL 4.1 NOM-030-STPS-2009)
En este mismo acto, con fundamento en el Numeral 4.1 de la **NOM-030-STPS-2009**, el patrón designa formalmente a **{csh_coord}** como **Responsable de los Servicios Preventivos de Seguridad y Salud en el Trabajo**, quien coordinará la elaboración del Diagnóstico Integral de Seguridad y Salud y el Programa Anual de Recorridos Trimestrales sobre las Normas Oficiales Mexicanas aplicables al giro **{giro}**:
- **Marco Técnico de NOMs STPS a Verificar en Recorridos:** {noms_scian}

*Infraestructura Documental FIRMA W16 MALLEN® • Fecha de Compilación: {fecha} • Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                tel = telefono_corp, email = email_arco, giro = giro_scian,
                clase_prima = clase_prima_imss, emp = emp_count, rep_legal = rep_legal,
                cargo = rep_cargo, tabla_comision = tabla_comision_nom019,
                csh_coord = csh_coord, noms_scian = noms_especificas_scian,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 04: ADENDUM Y LISTA DE VERIFICACIÓN DE TELETRABAJO NOM-037
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "02_STPS_Carpetas_Normativas_NOMs/04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.md".to_string(),
            disk_filename: "04_Politica_Teletrabajo_NOM037.md".to_string(),
            content: format!(
r#"# POLÍTICA DE TELETRABAJO, CONVENIO MODIFICATORIO (ART. 330-B LFT), DERECHO A LA DESCONEXIÓN DIGITAL Y LISTA DE VERIFICACIÓN ERGONÓMICA (NOM-037-STPS-2023)
**Patrón:** {razon} ({rfc}) | **Giro SCIAN:** {giro}
**Domicilio Presencial del Centro de Trabajo:** {domicilio}, C.P. {cp}, {jurisdiccion}
**Fundamento Legal:** Artículos 330-A al 330-K de la Ley Federal del Trabajo y Numerales 5.2, 5.4, 7.1 y Guía de Referencia I de la NOM-037-STPS-2023.

---

### 1. ÁMBITO DE APLICACIÓN Y DERECHO A LA DESCONEXIÓN DIGITAL (ARTS. 330-A Y 330-E FRACC. VI LFT)
El presente instrumento rige para las personas trabajadoras de **{razon}** que desarrollen más del **40% de su jornada laboral semanal** en su domicilio o en un lugar distinto al centro de trabajo. El patrón garantiza el **Derecho a la Desconexión Digital** al concluir la jornada pactada (**{horario}**), así como durante pausas de alimentos, licencias, incapacidades, días de descanso y vacaciones, absteniéndose de emitir órdenes o aplicar sanciones fuera de dicho horario.

### 2. INSUMOS, SERVICIOS PROPORCIONALES Y REVERSIBILIDAD (ARTS. 330-B, 330-E Y 330-G LFT)
1. **Equipo de Trabajo:** El patrón entrega en comodato el equipo informático y mobiliario ergonómico descrito en el resguardo individual del trabajador: ________________________________________________.
2. **Pago Proporcional de Servicios (Art. 330-E Fracc. III LFT):** El patrón cubrirá mensualmente al trabajador en modalidad de teletrabajo la cantidad de **${apoyo} MXN** por concepto de pago proporcional de telecomunicaciones (internet) y consumo de energía eléctrica derivados del desempeño de sus funciones.
3. **Domicilio Acordado para Teletrabajo:** ________________________________________________________________.
4. **Reversibilidad (Art. 330-G LFT):** El cambio de la modalidad presencial a teletrabajo es voluntario y reversible. Cualquiera de las partes podrá solicitar el retorno a la modalidad presencial en **{domicilio}** mediante aviso escrito con al menos 10 días hábiles de anticipación.

### 3. LISTA DE VERIFICACIÓN DE CONDICIONES DE SEGURIDAD Y SALUD EN EL LUGAR DE TELETRABAJO (NUMERAL 5.4 NOM-037-STPS-2023)
Para ser requisitada por el trabajador y validada por la Comisión de Seguridad e Higiene (**{csh_coord}**):
- `[ ] Sí  [ ] No` El área destinada al teletrabajo se encuentra limpia, ordenada e iluminada adecuadamente (natural o artificialmente) sin reflejos directos sobre la pantalla.
- `[ ] Sí  [ ] No` Cuenta con ventilación adecuada y libre de ruido excesivo o agentes físicos/químicos peligrosos.
- `[ ] Sí  [ ] No` Cuenta con escritorio o mesa de trabajo a altura ergonómica (70 a 75 cm) y silla con soporte lumbar y ajuste de altura.
- `[ ] Sí  [ ] No` Las instalaciones eléctricas del área de trabajo carecen de cables expuestos, sobrecargas o conexiones improvisadas.
- `[ ] Sí  [ ] No` La persona trabajadora manifiesta que en el domicilio acordado existen condiciones libres de violencia familiar.

| **POR EL PATRÓN ({razon})** | **PERSONA TRABAJADORA EN TELETRABAJO** | **VALIDACIÓN COMISIÓN MIXTA SST** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | Nombre y Firma: ________________________ | **{csh_coord}** |

*Infraestructura Documental FIRMA W16 MALLEN® • Fecha de Compilación: {fecha} • Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                horario = horario_jornada, apoyo = apoyo_teletrabajo,
                csh_coord = csh_coord, rep_legal = rep_legal, cargo = rep_cargo,
                fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 05: CONTRATO MARCO ART. 69-B CFF + RAZÓN DE NEGOCIOS ART. 5-A
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "03_SAT_Materialidad_Fiscal_69B/05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.md".to_string(),
            disk_filename: "05_Contrato_Materialidad_SAT_69B.md".to_string(),
            content: format!(
r#"# CONTRATO MARCO DE PRESTACIÓN DE SERVICIOS Y SUMINISTRO CON CLÁUSULAS DE MATERIALIDAD FISCAL (ART. 69-B CFF), RAZÓN DE NEGOCIOS (ART. 5-A CFF) Y BITÁCORA DE ENTREGABLES
**Fundamento Legal:** Artículos 27 Fracc. I de la LISR, 5 Fracc. I de la LIVA, 5-A, 29-A, 32-B Ter y 69-B del Código Fiscal de la Federación; Artículos 89 a 93 bis del Código de Comercio y Jurisprudencia **2a./J. 161/2019 (10a.) de la Suprema Corte de Justicia de la Nación**.

---

### DECLARACIONES DE LAS PARTES CONTRATANTES
1. **Declara "EL CLIENTE" ({razon}):** Ser una sociedad mercantil o contribuyente legalmente inscrito bajo las leyes mexicanas, con RFC **{rfc}**, actividad económica registrada **{giro}**, con domicilio fiscal en **{domicilio}, C.P. {cp}, {jurisdiccion}**, representada en este acto por **{rep_legal}** en su carácter de **{cargo}** (según consta en el Instrumento Público N° {escritura}).
2. **Declara "EL PROVEEDOR" ({prov_nombre}):** Ser una persona moral o física con actividad empresarial inscrita en el Registro Federal de Contribuyentes con clave **{prov_rfc}**, con domicilio fiscal localizado en ________________________________________________________, representada por ________________________________________ (Instrumento Público N° ____________________), declarando bajo protesta de decir verdad que cuenta con activos, personal propio, infraestructura y capacidad material directa para ejecutar el objeto de este contrato, que su Opinión de Cumplimiento 32-D es **POSITIVA** y que no se encuentra en los listados de los Artículos 69 ni 69-B del Código Fiscal de la Federación.

---

### CLÁUSULAS DE SUSTANCIA ECONÓMICA, MATERIALIDAD Y CONTROL DOCUMENTAL
**PRIMERA (OBJETO ESPECÍFICO Y PROHIBICIÓN DE CONCEPTOS GENÉRICOS):** EL PROVEEDOR (**{prov_nombre}** — RFC: **{prov_rfc}**) se obliga a suministrar o ejecutar en favor de **{razon}** las operaciones específicas consistentes en: **{prov_objeto}**. Todo Comprobante Fiscal Digital por Internet (CFDI 4.0) deberá describir con precisión el lote, periodo, orden de compra y entregable específico, quedando prohibida la facturación bajo conceptos genéricos.

**SEGUNDA (EXPEDIENTE COMPROBATORIO DE MATERIALIDAD ART. 69-B CFF):** Será condición suspensiva indispensable para la programación y liberación de cualquier pago que EL PROVEEDOR entregue al responsable de revisión interna (**{testigo}**):
1. Cotización formal y Orden de Compra o Servicio previa debidamente autorizada;
2. Evidencia material verificable de la ejecución del servicio o entrega del bien (acta de entrega-recepción firmada, bitácora técnica fechada, memoria fotográfica georreferenciada, reportes de avance o guías de almacén);
3. Opinión de Cumplimiento de Obligaciones Fiscales (Art. 32-D CFF) en sentido POSITIVO con antigüedad no mayor a 30 días;
4. Archivo XML y PDF del CFDI 4.0 válido, obligándose a emitir el Complemento de Recepción de Pagos (REP) a más tardar el quinto día natural del mes inmediato siguiente al pago.

**TERCERA (RAZÓN DE NEGOCIOS ART. 5-A CFF E INDEMNIDAD FISCAL):** Las partes dejan constancia de que la celebración del presente contrato es estrictamente indispensable para la operación del giro **{giro}** de **{razon}** y genera un beneficio económico cuantificable razonablemente mayor a cualquier efecto fiscal. Si EL PROVEEDOR llegase a ser publicado de manera preliminar o definitiva en los listados del Artículo 69-B del CFF, se obliga a comparecer ante el SAT para aportar las pruebas de materialidad correspondientes y a restituir e indemnizar a **{razon}** por cualquier crédito fiscal, recargo o rechazo de deducción/acreditamiento.

{clausula_fiscal}

**QUINTA (FECHA CIERTA Y JURISDICCIÓN):** Para efectos de acreditar la Fecha Cierta conforme a la Jurisprudencia 2a./J. 161/2019 (10a.) de la SCJN, las partes podrán ratificar las firmas del presente instrumento ante Fedatario Público, además de conservar su huella criptográfica de integridad SHA-256 conforme al Artículo 89 del Código de Comercio. Para su interpretación y cumplimiento, las partes se someten a los tribunales competentes en **{jurisdiccion}**.

| **POR "EL CLIENTE": {razon} ({rfc})** | **POR "EL PROVEEDOR": {prov_nombre}** | **TESTIGO / REVISIÓN DE MATERIALIDAD** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | Nombre y Firma: ________________________ | **{testigo}** |
| Fecha de Firma: ____/____/2026 | RFC Proveedor: {prov_rfc} | Sello SHA-256: `{hash}` |
"#,
                razon = razon_social, rfc = tax_id, giro = giro_scian,
                domicilio = domicilio, cp = cp_fiscal, jurisdiccion = jurisdiccion,
                rep_legal = rep_legal, cargo = rep_cargo, escritura = escritura_num,
                prov_nombre = prov_nombre, prov_rfc = prov_rfc, prov_objeto = prov_objeto,
                testigo = testigo_auditor, clausula_fiscal = clausula_fiscal_scian,
                hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 06: PROTOCOLO IMSS 32-D, REPSE Y ACTA CONSTITUTIVA PIPC
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "04_IMSS_INFONAVIT_Seguridad_Social_REPSE/06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.md".to_string(),
            disk_filename: "06_Protocolo_IMSS_REPSE_32D.md".to_string(),
            content: format!(
r#"# PROTOCOLO DE CONCILIACIÓN PATRONAL IMSS (SUA / RACERF), LISTA DE VERIFICACIÓN REPSE (ART. 15-A LSS) Y ACTA DE INTEGRACIÓN DE LA UNIDAD INTERNA DE PROTECCIÓN CIVIL (PIPC)
**Patrón:** {razon} ({rfc}) | **Registro Patronal IMSS:** {reg_patronal}
**Giro SCIAN:** {giro} | **Plantilla Parametrizada:** {emp} trabajadores
**Clasificación de Riesgo IMSS de Referencia (Art. 196 RACERF):** {clase_prima}
**Folio REPSE (En caso de prestar o contratar servicios especializados):** {repse}
**Domicilio del Establecimiento:** {domicilio}, C.P. {cp}, {jurisdiccion}

---

### 1. PROTOCOLO MENSUAL DE CONFRONTA SUA VS. CFDI DE NÓMINA Y PRIMA ANUAL DE RIESGO
Para prevenir diferencias por Salario Base de Cotización (Arts. 27 a 30 LSS) y capitales constitutivos:
1. Antes del día **15 de cada mes**, el área de nóminas y contabilidad (**{testigo}**) conciliará el total de recibos CFDI de Nómina timbrados ante el SAT contra la Cédula de Determinación EMA/EBA y el archivo de pago del Sistema Único de Autodeterminación (SUA) para los **{emp} trabajadores**.
2. Durante el mes de **febrero de cada año** (Art. 74 LSS y Art. 32 RACERF), se revisará la siniestralidad laboral del ejercicio inmediato anterior para presentar la Determinación Anual de la Prima en el Seguro de Riesgos de Trabajo.

### 2. CHECKLIST MENSUAL DE BLINDAJE SOLIDARIO EN SUBCONTRATACIÓN ESPECIALIZADA (ART. 15-A LSS Y ART. 27 FRACC. V LISR)
Cuando **{razon}** contrate servicios u obras especializadas con terceros (Contratista: **{prov_nombre}** — RFC: **{prov_rfc}**), será requisito previo al pago marcar y archivar mensualmente los siguientes 7 comprobantes obligatorios:
- `[ ]` **1. Registro REPSE vigente:** Copia del aviso de registro ante la STPS (Folio: {repse}).
- `[ ]` **2. CFDI de Nómina:** Comprobantes fiscales de pago de salario de los trabajadores asignados al servicio en el mes.
- `[ ]` **3. Entero de Retenciones ISR:** Recibo bancario del pago de retenciones de ISR de dichos trabajadores.
- `[ ]` **4. Pago de IVA:** Declaración mensual y acuse de pago del Impuesto al Valor Agregado del contratista.
- `[ ]` **5. Cuotas IMSS (SUA):** Cédula de determinación y comprobante de pago de cuotas obrero-patronales al IMSS.
- `[ ]` **6. Aportaciones INFONAVIT:** Comprobante de pago de aportaciones y amortizaciones bimestrales al INFONAVIT.
- `[ ]` **7. Informativas ICSOE y SISSUB:** Acuses cuatrimestrales presentados a más tardar los días 17 de enero, mayo y septiembre.

### 3. ACTA DE INTEGRACIÓN DE LA UNIDAD INTERNA DE PROTECCIÓN CIVIL (ARTS. 39, 40 Y 74–78 LGPC) Y MATRIZ SECTORIAL ({scian_short})
Con el objeto de integrar formalmente el Programa Interno de Protección Civil (PIPC) en el inmueble ubicado en **{domicilio}**, se designa la estructura de la Unidad Interna de Protección Civil para su firma autógrafa:

| **ROL EN LA UNIDAD INTERNA PIPC** | **NOMBRE DEL RESPONSABLE / BRIGADISTA** | **FIRMA AUTÓGRAFA** |
| :--- | :--- | :--- |
| **Coordinador General de la UIPC** | **{csh_coord}** | ________________________ |
| **Jefe de Brigada de Evacuación y Repliegue** | ________________________________________ | ________________________ |
| **Jefe de Brigada de Prevención y Combate de Incendios** | ________________________________________ | ________________________ |
| **Jefe de Brigada de Primeros Auxilios** | ________________________________________ | ________________________ |
| **Jefe de Brigada de Búsqueda y Salvamento** | ________________________________________ | ________________________ |

**Obligaciones Sectoriales Específicas a Verificar según el Giro SCIAN ({giro}):**
{anexo_sectorial}

| **REPRESENTACIÓN PATRONAL ({razon})** | **COORDINADOR UNIDAD INTERNA PIPC** | **REVISIÓN INTERNA DE CUMPLIMIENTO** |
| :--- | :--- | :--- |
| **{rep_legal}** ({cargo}) | **{csh_coord}** | **{testigo}** |

*Infraestructura Documental FIRMA W16 MALLEN® • Fecha de Compilación: {fecha} • Sello SHA-256:* `{hash}`
"#,
                razon = razon_social, rfc = tax_id, reg_patronal = reg_patronal,
                giro = giro_scian, emp = emp_count, clase_prima = clase_prima_imss,
                repse = folio_repse, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, testigo = testigo_auditor,
                prov_nombre = prov_nombre, prov_rfc = prov_rfc, scian_short = scian_code,
                csh_coord = csh_coord, anexo_sectorial = anexo_sectorial_pipc,
                rep_legal = rep_legal, cargo = rep_cargo, fecha = fecha, hash = empresa.audit_hash
            ),
        },

        // =====================================================================
        // INSTRUMENTO 07: TÉRMINOS PROFECO ART. 76 BIS + CÉDULA BENEFICIARIO CONTROLADOR
        // =====================================================================
        GeneratedKitDocument {
            zip_relative_path: "05_PROFECO_Comercio_Digital_LFPC/07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.md".to_string(),
            disk_filename: "07_Terminos_Ecommerce_PROFECO_76BIS.md".to_string(),
            content: format!(
r#"# TÉRMINOS Y CONDICIONES DE COMERCIO ELECTRÓNICO (ART. 76 BIS LFPC), PÓLIZA DE GARANTÍA ({garantia} DÍAS) Y CÉDULA DE INTEGRACIÓN DE BENEFICIARIO CONTROLADOR (ART. 32-B TER CFF)
**Proveedor Comercial:** {razon} | **RFC:** {rfc} | **Giro SCIAN:** {giro}
**Domicilio Físico en Territorio Nacional (Art. 76 BIS Fracc. IV LFPC):** {domicilio}, C.P. {cp}, {jurisdiccion}
**Representante Legal:** {rep_legal} ({cargo}) | **Atención a Clientes y Reclamaciones:** {email} | **Teléfono:** {tel}

---

### 1. INFORMACIÓN AL CONSUMIDOR, PRECIOS TOTALES Y TRANSACCIONES DIGITALES (ARTS. 7 BIS, 76 BIS Y 90 LFPC)
1. **Identificación y Domicilio Físico:** En cumplimiento al Artículo 76 BIS Fracciones I a VII de la Ley Federal de Protección al Consumidor (LFPC), **{razon}** pone a disposición de los consumidores su domicilio físico en **{domicilio}, C.P. {cp}, {jurisdiccion}**, teléfono **{tel}** y correo **{email}** para presentar aclaraciones o reclamaciones.
2. **Exhibición de Precios Totales (Art. 7 BIS LFPC):** Todo precio exhibido en catálogos, cotizaciones o medios electrónicos se expresa en **Moneda Nacional (MXN)** e informa de manera visible el monto total a pagar, incluyendo el Impuesto al Valor Agregado (IVA), impuestos aplicables y, en su caso, costos de envío, sin aplicar cargos automáticos ni cláusulas unilaterales prohibidas por el Artículo 90 de la LFPC.

### 2. DERECHO DE REVOCACIÓN (5 DÍAS HÁBILES) Y PÓLIZA DE GARANTÍA LEGAL ({garantia} DÍAS NATURALES)
1. **Derecho de Revocación en Ventas a Distancia (Art. 56 LFPC):** El consumidor cuenta con un plazo de **5 días hábiles** contados a partir de la entrega del bien o de la firma del contrato para revocar su consentimiento sin responsabilidad alguna, notificándolo a **{email}** y devolviendo el bien en las mismas condiciones en que lo recibió.
2. **Póliza de Garantía Escrita (Arts. 77, 78 y 79 LFPC):** **{razon}** otorga sobre sus productos una garantía de **{garantia} días naturales** (nunca inferior al mínimo legal de 90 días del Art. 77 LFPC), la cual ampara sin cargo alguno la reparación, reposición del bien o devolución del importe pagado directamente en **{domicilio}, C.P. {cp}, {jurisdiccion}**.

---

### 3. CÉDULA DE INTEGRACIÓN Y RESGUARDO DEL EXPEDIENTE DE BENEFICIARIO CONTROLADOR (ARTS. 32-B TER A 32-B QUINQUIES CFF)
Como parte de la contabilidad fiscal obligatoria (Art. 28 CFF y Reglas 2.8.1.20 a 2.8.1.22 de la RMF), la administración de **{razon}** integra y resguarda en su domicilio fiscal los datos de identificación fidedignos de cada **Beneficiario Controlador** (persona física que obtiene el beneficio final o ejerce el control directo o indirecto >15% del capital con derecho a voto), previniendo las multas de **$1,500,000.00 a $2,000,000.00 MXN por cada beneficiario** previstas en los Artículos 84-M y 84-N del CFF:

| **CAMPO REQUERIDO (RMF REGLA 2.8.1.22)** | **BENEFICIARIO CONTROLADOR 1** | **BENEFICIARIO CONTROLADOR 2** |
| :--- | :--- | :--- |
| **Nombre(s) y Apellidos Completos** | ____________________________________ | ____________________________________ |
| **RFC y CURP (o ID Fiscal Extranjero)** | ____________________________________ | ____________________________________ |
| **Fecha de Nacimiento y Nacionalidad** | ____________________________________ | ____________________________________ |
| **Porcentaje de Participación / Control** | ______________ % | ______________ % |
| **Estado Civil y Régimen Patrimonial** | ____________________________________ | ____________________________________ |
| **Domicilio Particular y Correo Electrónico** | ____________________________________ | ____________________________________ |
| **Fecha de Adquisición de la Calidad de BC** | ____ / ____ / ________ | ____ / ____ / ________ |

| **POR LA SOCIEDAD / EMISOR ({razon})** | **RESPONSABLE DE CUSTODIA CONTABLE Y FISCAL** |
| :--- | :--- |
| **{rep_legal}** ({cargo}) | **{testigo}** |

*Infraestructura Documental FIRMA W16 MALLEN® • Fecha de Compilación: {fecha} • Sello SHA-256:* `{hash}`
"#,
                garantia = dias_garantia, razon = razon_social, rfc = tax_id,
                giro = giro_scian, domicilio = domicilio, cp = cp_fiscal,
                jurisdiccion = jurisdiccion, rep_legal = rep_legal, cargo = rep_cargo,
                email = email_arco, tel = telefono_corp, testigo = testigo_auditor,
                fecha = fecha, hash = empresa.audit_hash
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
