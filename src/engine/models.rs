use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpresaB2B {
    pub id: String,
    pub nombre: String,
    pub actividad: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub estado: Option<String>,
    pub sitio_web: Option<String>,
    pub status_web: Option<String>,
    pub e_mail_verificado: Option<String>,
    pub country_code: String,
    pub tax_id: Option<String>,
    pub language: Option<String>,
    pub audit_hash: String,
    /// Puntuación de riesgo de pre-auditoría (0 = sin riesgo, 100 = riesgo máximo).
    pub risk_score: u8,
    /// Etiquetas de las fallas detectadas durante la pre-auditoría.
    pub audit_flags: Vec<String>,
    /// Multa estimada en pesos mexicanos (MXN).
    pub estimated_fine_mxn: f64,
    /// Precio dinámico del servicio Shield en USD.
    pub shield_price_usd: f64,
    /// Firma del contador responsable (opcional).
    pub accountant_signature: Option<String>,
}
