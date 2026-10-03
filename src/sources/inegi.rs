use crate::engine::models::EmpresaB2B;
use crate::sources::GlobalSource;
use csv::StringRecord;

pub const INEGI_COUNTRY_CODE: &str = "MX";

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

        Some(EmpresaB2B {
            id: record.get(0).unwrap_or("").to_string(),
            nombre: record.get(1).unwrap_or("").to_string(),
            razon_social: record.get(2).unwrap_or("").to_string(),
            actividad: record.get(3).unwrap_or("").to_string(),
            email: record.get(4).map(|s| s.to_string()),
            telefono: record.get(5).map(|s| s.to_string()),
            sitio_web: record.get(6).map(|s| s.to_string()),
            direccion: record.get(7).unwrap_or("").to_string(),
            estado: record.get(8).unwrap_or("").to_string(),
            e_mail_verificado: None,
            status_web: None,
            country_code: INEGI_COUNTRY_CODE.to_string(),
            tax_id: None,
            language: Some("es".to_string()),
        })
    }
}
