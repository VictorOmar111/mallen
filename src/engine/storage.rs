use crate::engine::models::EmpresaB2B;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

/// Persistiert verarbeitete Firmen-Batches als JSON-Lines-Datei.
pub struct StorageManager;

#[derive(Serialize)]
struct StoredRecord<'a> {
    merkle_root: &'a str,
    #[serde(flatten)]
    empresa: &'a EmpresaB2B,
}

impl StorageManager {
    /// Hängt einen Batch von Firmen als JSON-Zeilen an die Zieldatei an.
    /// Jede Zeile enthält zusätzlich den Merkle-Root des zugehörigen Lotes.
    pub async fn save_batch(
        file_path: &str,
        empresas: &[EmpresaB2B],
        merkle_root: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let mut writer = BufWriter::new(file);

        for empresa in empresas {
            let record = StoredRecord {
                merkle_root,
                empresa,
            };
            let line = serde_json::to_string(&record)?;
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;
        Ok(())
    }
}
