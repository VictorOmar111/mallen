use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpresaB2B {
    pub id: String,
    pub nombre: String,
    pub actividad: String,
    pub telefono: String,
    pub email: String,
    pub direccion: String,
    pub estado: String,
    pub sitio_web: Option<String>,
    pub status_web: Option<u16>,
    pub e_mail_verificado: Option<bool>,
    pub country_code: String,
    pub tax_id: Option<String>,
    pub language: Option<String>,
    pub audit_hash: String,
    pub risk_score: u32,
    pub audit_flags: Vec<String>,
    pub estimated_fine_mxn: f64,
    pub shield_price_usd: f64,
    pub accountant_signature: Option<String>,
}
