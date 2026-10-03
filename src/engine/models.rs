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
}
