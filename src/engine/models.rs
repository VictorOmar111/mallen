use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpresaB2B {
    pub id: String,
    pub nombre: String,
    pub razon_social: String,
    pub actividad: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub sitio_web: Option<String>,
    pub direccion: String,
    pub estado: String,
    pub e_mail_verificado: Option<String>,
    pub status_web: Option<u16>,
    /// ISO-3166-1 alpha-2 Ländercode, z. B. "MX", "DE", "US".
    pub country_code: String,
    /// Generische Steuernummer / Register-ID (z. B. RFC, USt-IdNr., EIN).
    pub tax_id: Option<String>,
    /// ISO-639-1 Sprachcode der Quelldaten, z. B. "es", "de", "en".
    pub language: Option<String>,
}
