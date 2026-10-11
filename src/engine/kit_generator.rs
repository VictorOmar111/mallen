#[allow(dead_code)]
use crate::engine::models::EmpresaB2B;
use std::fs::{self, File};
use std::io::Write;

pub fn generate_compliance_kit(empresa: &EmpresaB2B) -> Result<String, Box<dyn std::error::Error>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);
    let kit_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&kit_dir)?;

    // 1. Generar Aviso de Privacidad Integral (INAI)
    let aviso_privacy_path = format!("{}/01_Aviso_de_Privacidad_Integral.md", kit_dir);
    let mut f_aviso = File::create(&aviso_privacy_path)?;
    let aviso_content = format!(
        "# AVISO DE PRIVACIDAD INTEGRAL\n\n**Responsable del Tratamiento de Datos Personales:** {}\n**RFC:** {}\n**Domicilio:** {}\n**Correo de Atención a Derechos ARCO:** {}\n\n--- \n\n### 1. Datos Personales Recabados\n{} recaba datos de identificación, contacto y navegación con la finalidad de proveer los servicios solicitados, gestionar facturación y cumplir con normativas aplicables.\n\n### 2. Ejercicio de Derechos ARCO (Acceso, Rectificación, Cancelación y Oposición)\nEl titular podrá ejercer en cualquier momento sus derechos ARCO enviando una solicitud formal al correo **{}** indicando:\n- Nombre completo y copia de identificación oficial.\n- Descripción clara de los datos sobre los que busca ejercer sus derechos.\n\n### 3. Medidas de Seguridad\nImplementamos medidas de seguridad administrativas, técnicas y físicas conforme a la LFPDPPP.\n\n*Fecha de emisión:* 2026-10-04 | *Sello Merkle:* {}\n",
        empresa.nombre,
        tax_id,
        empresa.direccion,
        empresa.email,
        empresa.nombre,
        empresa.email,
        empresa.audit_hash
    );
    f_aviso.write_all(aviso_content.as_bytes())?;

    // 2. Generar Carpeta NOM-035-STPS-2018 (STPS)
    let nom035_path = format!("{}/02_Politica_Prevencion_NOM035_STPS.md", kit_dir);
    let mut f_nom = File::create(&nom035_path)?;
    let nom_content = format!(
        "# POLÍTICA DE PREVENCIÓN DE RIESGOS PSICOSOCIALES (NOM-035-STPS-2018)\n\n**Empresa:** {}\n**Centro de Trabajo:** Plantilla de {}\n**Vigencia:** 2026-2027\n\n---\n\n### Declaración de Principios\nEn **{}** nos comprometemos a promover un entorno organizacional favorable, prevenir factores de riesgo psicosocial y erradicar la violencia laboral.\n\n### Protocolo de Actuación para Centros de Trabajo (11 a 30 Trabajadores)\n1. **Aplicación de Cuestionarios:** Evaluación mediante la Guía de Referencia II de la NOM-035.\n2. **Canal de Denuncias:** Mecanismo confidencial de atención para trabajadores vía {}.\n3. **Acciones de Prevención:** Difusión periódica de horarios de trabajo equitativos y pausas activas.\n\n*Dictaminado bajo el Expediente de Auditoría #*: {}\n",
        empresa.nombre,
        empresa.direccion,
        empresa.nombre,
        empresa.email,
        empresa.audit_hash
    );
    f_nom.write_all(nom_content.as_bytes())?;

    Ok(kit_dir)
}
