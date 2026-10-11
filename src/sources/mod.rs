use csv::StringRecord;
use crate::engine::models::EmpresaB2B;

pub trait GlobalSource {
    fn parse_row(&self, record: &StringRecord) -> Option<EmpresaB2B>;
    fn default_country_code(&self) -> &'static str;
}

pub struct InegiDenueSource;

impl GlobalSource for InegiDenueSource {
    fn default_country_code(&self) -> &'static str {
        "MX"
    }

    fn parse_row(&self, record: &StringRecord) -> Option<EmpresaB2B> {
        if record.len() < 5 {
            return None;
        }

        // Mapeo exacto del CSV:
        // 0:id | 1:nombre | 2:razon_social | 3:codigo_scian | 4:personal | 5:sitio_web | 6:correo | 7:telefono | 8:rfc
        let web = record.get(5).unwrap_or("").to_string();
        let sitio_web_opt = if web.is_empty() { None } else { Some(web) };

        Some(EmpresaB2B {
            id: record.get(0).unwrap_or("").to_string(),
            nombre: record.get(1).unwrap_or("").to_string(),
            actividad: record.get(3).unwrap_or("").to_string(), // SCIAN
            telefono: record.get(7).unwrap_or("").to_string(), // Teléfono
            email: record.get(6).unwrap_or("").to_string(),    // Correo
            direccion: record.get(4).unwrap_or("").to_string(),// Personal / Estrato
            estado: "MX".to_string(),
            sitio_web: sitio_web_opt,
            status_web: None,
            e_mail_verificado: None,
            country_code: "MX".to_string(),
            tax_id: record.get(8).map(|s| s.to_string()),     // RFC
            language: Some("es".to_string()),
            audit_hash: String::new(),
            risk_score: 0,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 0.0,
            shield_price_usd: 150.0,
            accountant_signature: None,
        })
    }
}
