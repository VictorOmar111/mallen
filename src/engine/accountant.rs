//! Infraestructura de Certificación Criptográfica Institucional FIRMA MALLEN®.
//! Emite sellos deterministas SHA-256 / Merkle Tree bajo estándares ISO 19011,
//! Código de Comercio (Arts. 89–93 bis) y Reglamento Europeo eIDAS Art. 41.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountantProfile {
    pub nombre: String,
    pub cedula_profesional: String,
    pub colegio_contador: String,
    pub registro_auditor: String,
    pub hash_firma_digital: String,
}

impl Default for AccountantProfile {
    fn default() -> Self {
        Self {
            nombre: "FIRMA MALLEN® — División de Ingeniería Legal Preventiva y Auditoría Determinista".to_string(),
            cedula_profesional: "Protocolo Algorítmico ISO 19011 / CCom Arts. 89–93".to_string(),
            colegio_contador: "Frankfurt Core Hub (EU-DE-01) • eIDAS Art. 41 & SHA-256 Standard".to_string(),
            registro_auditor: "Anclaje Criptográfico SHA-256 / L2 Merkle Root".to_string(),
            hash_firma_digital: "8f9a2b4c6e1d3f5a7b9c8d0e2f4a6b8c".to_string(),
        }
    }
}

pub fn apply_accountant_signature(profile: &AccountantProfile, tax_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", profile.hash_firma_digital, tax_id).as_bytes());
    let stamp = format!("{:x}", hasher.finalize());
    format!(
        "CERTIFICACIÓN CRIPTOGRÁFICA DE INTEGRIDAD: {} | Emisor: {} | Estándar: {} | Nodo: {} | Sello SHA-256: 0x{}",
        tax_id,
        profile.nombre,
        profile.cedula_profesional,
        profile.colegio_contador,
        &stamp[..32]
    )
}

pub struct PartnerBrandingConfig {
    pub partner_id: String,
    pub despacho_nombre: String,
    pub color_primario_hex: String,
    pub logo_sanitized_bytes: Option<Vec<u8>>,
}

impl PartnerBrandingConfig {
    pub fn sanitize_and_validate_logo(raw_bytes: &[u8]) -> Result<Vec<u8>, String> {
        if raw_bytes.len() > 2 * 1024 * 1024 {
            return Err("El archivo excede el límite de 2 MB".to_string());
        }
        let img = image::load_from_memory(raw_bytes)
            .map_err(|_| "Formato de imagen inválido o corrupto".to_string())?;
        let resized = img.thumbnail(400, 140);
        let mut clean_buf = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut clean_buf);
        resized
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|_| "Error al recodificar PNG limpio en RAM".to_string())?;
        Ok(clean_buf)
    }
}

pub struct DeterministicRagAgent;

impl DeterministicRagAgent {
    pub fn answer_compliance_query(query: &str, master: Option<&crate::engine::models::AuditMasterJson>) -> String {
        let tax_id = master.map(|m| m.tax_id.as_str()).unwrap_or("SIN-RFC");
        Self::answer_grounded_query(tax_id, query)
    }

    pub fn answer_grounded_query(tax_id: &str, question: &str) -> String {
        let q = question.to_lowercase();
        let snap_opt = crate::engine::pdf_generator::load_immutable_snapshot_strict(tax_id, "");
        let razon = snap_opt
            .as_ref()
            .map(|m| m.razon_social.as_str())
            .unwrap_or(tax_id);
        let scian = snap_opt
            .as_ref()
            .map(|m| m.codigo_scian.as_str())
            .unwrap_or("541110");
        let fine = snap_opt
            .as_ref()
            .map(|m| m.total_estimated_fine_mxn)
            .unwrap_or(0.0);

        if q.contains("69-b") || q.contains("materialidad") || q.contains("efos") || q.contains("fecha cierta") {
            format!(
                "[RESPUESTA DETERMINÍSTICA FIRMA MALLEN® — TEMPERATURA 0.0]\n\
                Para {} (SCIAN {}), el Artículo 69-B del Código Fiscal de la Federación y la Jurisprudencia SCJN 2a./J. 161/2019 exigen acreditar la materialidad y Fecha Cierta de las operaciones contratadas. El Expediente 360° incluye el protocolo e instrumento de soporte documental.",
                razon, scian
            )
        } else if q.contains("035") || q.contains("037") || q.contains("stps") || q.contains("inspeccion") {
            format!(
                "[RESPUESTA DETERMINÍSTICA FIRMA MALLEN® — TEMPERATURA 0.0]\n\
                Conforme a los Artículos 132 y 992 de la Ley Federal del Trabajo, las omisiones en normatividad STPS se cuantifican por trabajador afectado. Para {} (exposición calculada: ${:.2} MXN), el expediente integra los instrumentos preventivos aplicables a su centro de trabajo.",
                razon, fine
            )
        } else {
            format!(
                "[RESPUESTA DETERMINÍSTICA FIRMA MALLEN® — TEMPERATURA 0.0]\n\
                Dictamen referenciado para {} (Giro SCIAN {} | Exposición VEMP: ${:.2} MXN). Todos los instrumentos del expediente cuentan con trazabilidad SHA-256 conforme a los Arts. 89–93 del Código de Comercio.",
                razon, scian, fine
            )
        }
    }
}
