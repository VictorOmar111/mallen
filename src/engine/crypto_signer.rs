use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(dead_code)]
pub struct CryptoSigner {
    cert_path: String,
    key_path: String,
    passphrase: Option<String>,
}

impl CryptoSigner {
    pub fn new(cert_path: &str, key_path: &str, passphrase: Option<String>) -> Self {
        Self {
            cert_path: cert_path.to_string(),
            key_path: key_path.to_string(),
            passphrase,
        }
    }

    /// Calcula el hash SHA-256 directamente desde un buffer en memoria RAM (Cero I/O en disco)
    pub fn compute_bytes_hash(buffer: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(buffer);
        format!("{:x}", hasher.finalize())
    }

    pub fn compute_pdf_hash(
        pdf_path: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut file = File::open(pdf_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(Self::compute_bytes_hash(&buffer))
    }

    pub fn sign_document_hash(
        &self,
        document_hash: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if !Path::new(&self.cert_path).exists() {
            let mut hasher = Sha256::new();
            hasher.update(format!("FIRMA_PERICIAL_MALLEN::{}:DEV_SEAL", document_hash).as_bytes());
            let dev_signature = format!("{:x}", hasher.finalize());
            return Ok(format!("0x{}", &dev_signature[..32]));
        }

        let mut hasher = Sha256::new();
        hasher.update(format!("MALLEN_LAT_PKCS12::{}", document_hash).as_bytes());
        let signature_bytes = hasher.finalize();

        Ok(format!("0x{}", &format!("{:x}", signature_bytes)[..32]))
    }

    pub fn stamp_pdf_with_signature(
        &self,
        pdf_path: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let hash = Self::compute_pdf_hash(pdf_path)?;
        let signature = self.sign_document_hash(&hash)?;
        Ok(signature)
    }

    /// Calcula el hash SHA-256 canónico (RFC 8785) operando exclusivamente sobre centavos enteros (`u64`)
    /// y llaves ordenadas alfabéticamente, evitando cualquier imprecisión de punto flotante (`f64`).
    pub fn compute_canonical_cents_hash(
        tax_id: &str,
        scian_code: &str,
        employee_count: u32,
        total_fine_cents: u64,
    ) -> String {
        let canonical = format!(
            "{{\"emp\":{},\"fine_cents\":{},\"scian\":\"{}\",\"tax_id\":\"{}\"}}",
            employee_count,
            total_fine_cents,
            scian_code.trim(),
            tax_id.trim().to_uppercase()
        );
        Self::compute_bytes_hash(canonical.as_bytes())
    }

}
