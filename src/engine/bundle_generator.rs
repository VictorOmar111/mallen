use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::OsintResult;
use crate::engine::pdf_generator::generate_pdf_report;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn generate_custom_bundle(empresa: &EmpresaB2B, osint: &OsintResult) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("GENERIC");
    let base_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&base_dir)?;

    // 1. Generar el Dictamen PDF
    let pdf_filename = format!("{}/Dictamen_Ejecutivo_{}.pdf", base_dir, tax_id);
    generate_pdf_report(empresa, osint, &pdf_filename)?;

    // 2. Crear el archivo ZIP final
    let zip_filename = format!("{}/Paquete_Blindaje_Integral_{}.zip", base_dir, tax_id);
    let path = Path::new(&zip_filename);
    let file = File::create(path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    // Adjuntar el Dictamen Ejecutivo PDF
    zip.start_file("00_DICTAMEN_EJECUTIVO_AUDITORIA.pdf", options)?;
    let mut pdf_file = File::open(&pdf_filename)?;
    let mut pdf_bytes = Vec::new();
    pdf_file.read_to_end(&mut pdf_bytes)?;
    zip.write_all(&pdf_bytes)?;

    // Adjuntar Kit STPS
    let stps_doc = format!(
        "POLÍTICA DE CUMPLIMIENTO NORMAS STPS (NOM-035 / NOM-037)\nEMPRESA: {}\nHASH: {}",
        empresa.nombre, empresa.audit_hash
    );
    zip.start_file("01_STPS_Seguridad_e_Higiene/Politica_NOM035_NOM037.txt", options)?;
    zip.write_all(stps_doc.as_bytes())?;

    // Adjuntar Kit INAI
    let inai_doc = format!(
        "AVISO DE PRIVACIDAD INTEGRAL (LFPDPPP)\nEMPRESA: {}\nHASH: {}",
        empresa.nombre, empresa.audit_hash
    );
    zip.start_file("02_INAI_Proteccion_Datos/Aviso_de_Privacidad_Integral.txt", options)?;
    zip.write_all(inai_doc.as_bytes())?;

    // Adjuntar Kit SAT
    let sat_doc = format!(
        "DOSSIER DE SOPORTE Y MATERIALIDAD OPERATIVA (SAT CFF ART. 69-B)\nEMPRESA: {}\nHASH: {}",
        empresa.nombre, empresa.audit_hash
    );
    zip.start_file("03_SAT_Materialidad_69B/Expediente_Soporte_Operaciones.txt", options)?;
    zip.write_all(sat_doc.as_bytes())?;

    zip.finish()?;
    Ok(zip_filename)
}
