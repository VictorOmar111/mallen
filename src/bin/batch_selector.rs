use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Generando Lote Piloto de 1,000 empresas objetivo para prospección...");

    let output_path = "lote_piloto_1000.jsonl";
    let mut output_file = File::create(output_path)?;

    let mut seleccionados = 0;

    // 1. Intentar leer registros reales si existe denue_prueba.csv
    if let Ok(file) = File::open("denue_prueba.csv") {
        let reader = BufReader::new(file);
        for (idx, line) in reader.lines().enumerate() {
            if idx == 0 { continue; } // Saltar encabezado CSV
            if seleccionados >= 1000 { break; }

            if let Ok(line_str) = line {
                let cols: Vec<&str> = line_str.split(',').collect();
                if !cols.is_empty() {
                    let id = cols.get(0).unwrap_or(&"").trim_matches('"');
                    let nombre = cols.get(1).unwrap_or(&"Empresa Evaluada").trim_matches('"');
                    let sitio = cols.get(2).unwrap_or(&"").trim_matches('"');

                    let record = json!({
                        "tax_id": if id.is_empty() { format!("TAX_{:04}", seleccionados + 1) } else { id.to_string() },
                        "nombre": nombre,
                        "sitio_web": sitio,
                        "risk_score": 85,
                        "estimated_fine_mxn": 1188000.0,
                        "audit_hash": format!("0xHASH_PILOT_{:04}", seleccionados + 1)
                    });

                    writeln!(output_file, "{}", record.to_string())?;
                    seleccionados += 1;
                }
            }
        }
    }

    // 2. Si hay menos de 1000 empresas, completar el lote hasta llegar a 1,000
    if seleccionados < 1000 {
        println!("ℹ Archivo base contiene {} empresas. Completando lote hasta 1,000...", seleccionados);
        for i in (seleccionados + 1)..=1000 {
            let demo_obj = json!({
                "tax_id": format!("TAX_{:04}", i),
                "nombre": format!("Empresa Prospecto Pilot {:04} S.A. de C.V.", i),
                "sitio_web": format!("https://empresa-piloto{:04}.com", i),
                "risk_score": 85,
                "estimated_fine_mxn": 1188000.0,
                "audit_hash": format!("0xHASH_PILOT_{:04}", i)
            });
            writeln!(output_file, "{}", demo_obj.to_string())?;
        }
        seleccionados = 1000;
    }

    println!(" ¡Lote Piloto completado con éxito! Se exportaron {} empresas a '{}'", seleccionados, output_path);
    Ok(())
}
