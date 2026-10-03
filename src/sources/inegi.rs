use crate::engine::models::EmpresaB2B;
use crate::sources::GlobalSource;
use csv::StringRecord;
use sha2::{Digest, Sha256};

pub const INEGI_COUNTRY_CODE: &str = "MX";

/// Erzeugt einen stabilen SHA-256-Fingerabdruck (Hex) über die
/// identitätsstiftenden Felder eines Datensatzes.
fn compute_audit_hash(
    id: &str,
    nombre: &str,
    razon_social: &str,
    tax_id: Option<&str>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(id.as_bytes());
    hasher.update(b"\x1f");
    hasher.update(nombre.as_bytes());
    hasher.update(b"\x1f");
    hasher.update(razon_social.as_bytes());
    hasher.update(b"\x1f");
    hasher.update(tax_id.unwrap_or("").as_bytes());
    hex::encode(hasher.finalize())
}

pub struct InegiSource;

impl InegiSource {
    pub fn new() -> Self {
        Self
    }
}

impl GlobalSource for InegiSource {
    fn default_country_code(&self) -> &'static str {
        INEGI_COUNTRY_CODE
    }

    fn parse_row(&self, record: &StringRecord) -> Option<EmpresaB2B> {
        if record.len() < 5 {
            return None;
        }

        let id = record.get(0).unwrap_or("").to_string();
        let nombre = record.get(1).unwrap_or("").to_string();
        let razon_social = record.get(2).unwrap_or("").to_string();
        let tax_id: Option<String> = None;

        let audit_hash = compute_audit_hash(
            &id,
            &nombre,
            &razon_social,
            tax_id.as_deref(),
        );

        Some(EmpresaB2B {
            id,
            nombre,
            razon_social,
            actividad: record.get(3).unwrap_or("").to_string(),
            email: record.get(4).map(|s| s.to_string()),
            telefono: record.get(5).map(|s| s.to_string()),
            sitio_web: record.get(6).map(|s| s.to_string()),
            direccion: record.get(7).unwrap_or("").to_string(),
            estado: record.get(8).unwrap_or("").to_string(),
            e_mail_verificado: None,
            status_web: None,
            country_code: INEGI_COUNTRY_CODE.to_string(),
            tax_id,
            language: Some("es".to_string()),
            audit_hash,
        })
    }
}
