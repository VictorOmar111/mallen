use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

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
            nombre: "C.P.C. Víctor Omar Malpica Guillen".to_string(),
            cedula_profesional: "CED-PENDIENTE-2026".to_string(),
            colegio_contador: "Colegio de Contadores Públicos de México".to_string(),
            registro_auditor: "REG-STPS-AUDIT-001".to_string(),
            hash_firma_digital: "0x8f9a2b4c6e1d3f5a7b9c8d0e2f4a6b8c".to_string(),
        }
    }
}

pub fn apply_accountant_signature(profile: &AccountantProfile, tax_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", profile.hash_firma_digital, tax_id).as_bytes());
    let stamp = format!("{:x}", hasher.finalize());

    format!(
        "DICTAMINADO_POR: {} | CÉDULA: {} | REGISTRO: {} | SHA256_STAMP: {}",
        profile.nombre,
        profile.cedula_profesional,
        profile.registro_auditor,
        &stamp[..16]
    )
}
