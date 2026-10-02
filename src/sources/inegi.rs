use crate::engine::models::EmpresaB2B;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct InegiSource {
    pub api_token_or_path: String,
}

pub type InegiFetcher = InegiSource;

impl InegiSource {
    pub fn new(api_token_or_path: &str) -> Self {
        Self {
            api_token_or_path: api_token_or_path.to_string(),
        }
    }

    pub fn stream_denue_csv(&self, file_path: &str) -> Result<Vec<EmpresaB2B>, Box<dyn Error>> {
        let file = File::open(file_path)?;
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

            let fields: Vec<&str> = record.split(',').map(|s| s.trim_matches('"')).collect();

            if fields.len() >= 8 {
                let email_val = if !fields[4].is_empty() { Some(fields[4].to_string()) } else { None };
                let tel_val = if !fields[5].is_empty() { Some(fields[5].to_string()) } else { None };
                let sitio_web_val = if fields.len() > 8 && !fields[8].is_empty() { Some(fields[8].to_string()) } else { None };

                empresas.push(EmpresaB2B {
                    id: fields[0].to_string(),
                    nombre: fields[1].to_string(),
                    razon_social: fields[2].to_string(),
                    actividad: fields[3].to_string(),
                    email: email_val,
                    telefono: tel_val,
                    sitio_web: sitio_web_val,
                    direccion: fields[6].to_string(),
                    estado: fields[7].to_string(),
                });
            }
        }

        Ok(empresas)
    }
}
