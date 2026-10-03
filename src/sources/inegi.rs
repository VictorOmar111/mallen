use crate::engine::cleaner::{clean_email, clean_phone, clean_url};
use crate::engine::models::EmpresaB2B;
use crate::sources::GlobalSource;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Standard-Ländercode für die mexikanische DENUE-Quelle.
pub const INEGI_COUNTRY_CODE: &str = "MX";
/// Standard-Sprachcode für die mexikanische DENUE-Quelle.
pub const INEGI_LANGUAGE: &str = "es";

pub struct InegiSource;

pub type InegiFetcher = InegiSource;

impl InegiSource {
    pub fn new() -> Self {
        Self
    }

    pub fn stream_denue_csv(
        &self,
        file_path: &str,
    ) -> Result<Vec<EmpresaB2B>, Box<dyn Error + Send + Sync>> {
        let file = File::open(file_path).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        let reader = BufReader::new(file);
        let mut empresas = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            if index == 0 {
                continue;
            }

            let record = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            if let Some(empresa) = self.parse_row(&record) {
                empresas.push(empresa);
            }
        }

        Ok(empresas)
    }
}

impl Default for InegiSource {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalSource for InegiSource {
    fn default_country_code(&self) -> &'static str {
        INEGI_COUNTRY_CODE
    }

    fn parse_row(&self, row: &str) -> Option<EmpresaB2B> {
        let fields: Vec<&str> = row.split(',').map(|s| s.trim_matches('"')).collect();

        if fields.len() < 8 {
            return None;
        }

        let email_val = clean_email(fields[4]);
        let tel_val = clean_phone(fields[5]);
        let sitio_web_val = if fields.len() > 8 {
            clean_url(fields[8])
        } else {
            None
        };

        Some(EmpresaB2B {
            id: fields[0].to_string(),
            nombre: fields[1].to_string(),
            razon_social: fields[2].to_string(),
            actividad: fields[3].to_string(),
            email: email_val,
            telefono: tel_val,
            sitio_web: sitio_web_val,
            direccion: fields[6].to_string(),
            estado: fields[7].to_string(),
            e_mail_verificado: None,
            status_web: None,
            country_code: INEGI_COUNTRY_CODE.to_string(),
            tax_id: None,
            language: Some(INEGI_LANGUAGE.to_string()),
        })
    }
}
