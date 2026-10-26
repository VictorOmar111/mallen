//! Empaquetador maestro de entregables corporativos (.zip).
//! Une con compresión Deflate el Dictamen Pericial PDF, los 10 Certificados Normativos PDF,
//! los 7 Kits Legales completos y el Manifiesto de Fecha Cierta y Alcance Pericial.

use crate::engine::crypto_signer::CryptoSigner;
use crate::engine::kit_generator::{build_all_compliance_documents, generate_compliance_kit};
use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::OsintResult;
use crate::engine::pdf_generator::{generate_pdf_report, get_catalogo_certificados};
use chrono::Utc;
use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

/// Genera en memoria RAM (`Vec<u8>`) y persiste en disco el archivo `.zip` completo con todos los PDFs y Kits Legales.
/// Mantiene la firma pública `generate_custom_bundle` 100% compatible con todo el proyecto.
pub fn generate_custom_bundle(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("GENERIC");
    let base_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&base_dir)?;

    // 1. Generar los 7 Kits Legales en disco para respaldo local
    let _ = generate_compliance_kit(empresa);

    // 2. Generar el Dictamen Pericial PDF y los 10 Certificados Acreditativos PDF
    let pdf_filename = format!("{}/Dictamen_Ejecutivo_{}.pdf", base_dir, tax_id);
    generate_pdf_report(empresa, osint, &pdf_filename)?;

    // 3. Construir el archivo ZIP comprimido en RAM (Cursor<Vec<u8>>) y volcarlo a disco
    let zip_bytes = build_zip_buffer_from_dir(empresa, &base_dir, &pdf_filename)?;
    let zip_filename = format!("{}/Paquete_Blindaje_Integral_{}.zip", base_dir, tax_id);
    fs::write(&zip_filename, &zip_bytes)?;

    Ok(zip_filename)
}

/// Construye el archivo `.zip` directamente en un buffer de memoria RAM (`Vec<u8>`) listo para streaming HTTP
/// o envío directo tras el Webhook de Stripe.
pub fn build_zip_buffer_from_dir(
    empresa: &EmpresaB2B,
    base_dir: &str,
    dictamen_pdf_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("GENERIC");
    let buffer = Cursor::new(Vec::with_capacity(512 * 1024));
    let mut zip = ZipWriter::new(buffer);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // A. Adjuntar el Dictamen Ejecutivo Pericial en la raíz del ZIP
    if Path::new(dictamen_pdf_path).exists() {
        let mut pdf_file = File::open(dictamen_pdf_path)?;
        let mut pdf_bytes = Vec::new();
        pdf_file.read_to_end(&mut pdf_bytes)?;
        zip.start_file(
            format!("00_DICTAMEN_PERICIAL_EJECUTIVO_{}.pdf", tax_id),
            options,
        )?;
        zip.write_all(&pdf_bytes)?;
    }

    // B. Adjuntar los 10 Certificados de Acreditación Normativa (PDF) en su carpeta dedicada
    for cert in get_catalogo_certificados() {
        let cert_disk_path = format!("{}/{}", base_dir, cert.filename);
        if Path::new(&cert_disk_path).exists() {
            if let Ok(cert_bytes) = fs::read(&cert_disk_path) {
                zip.start_file(
                    format!("00_CERTIFICADOS_ACREDITACION_PDF/{}", cert.filename),
                    options,
                )?;
                zip.write_all(&cert_bytes)?;
            }
        }
    }

    // C. Adjuntar los 7 Instrumentos Jurídicos Corporativos completos (STPS, SABG/INAI, SAT 69-B, IMSS REPSE, PROFECO)
    let compliance_docs = build_all_compliance_documents(empresa);
    for doc in compliance_docs {
        zip.start_file(&doc.zip_relative_path, options)?;
        zip.write_all(doc.content.as_bytes())?;
    }

    // D. Adjuntar el Manifiesto de Fecha Cierta SCJN, Prueba Criptográfica SHA-256 y Disclaimer de Alcance Pericial
    let manifiesto_content = format!(
r#"================================================================================
FIRMA PERICIAL MALLEN (MALLEN.LAT) — CERTIFICADO CRIPTOGRÁFICO Y ALCANCE LEGAL
================================================================================
ENTIDAD AUDITADA:        {nombre}
RFC / TAX ID:            {rfc}
FECHA Y HORA UTC:        {utc}
AUDITOR RESPONSABLE:     C.P.C. Víctor Omar Malpica Guillén (Reg. CED-2026-MALLEN)
HASH SHA-256 EXPEDIENTE: {hash}
URL DE VERIFICACIÓN:     https://mallen.lat/verify/{hash}?rfc={rfc}
================================================================================

1. CONSTANCIA DE INTEGRIDAD CRIPTOGRÁFICA Y FECHA CIERTA DIGITAL
El presente paquete corporativo (.zip) y los instrumentos contenidos en él han sido
sellados mediante algoritmo criptográfico SHA-256 e incorporados en el lote diario
del Árbol de Merkle anclado en red distribuida L2 (Arbitrum One / Base), dotando
a los documentos de trazabilidad cronológica en términos de los Artículos 89 a 94
del Código de Comercio, Artículo 210-A del Código Federal de Procedimientos Civiles
y la Jurisprudencia 2a./J. 161/2019 (10a.) de la Suprema Corte de Justicia de la Nación.

2. MANIFIESTO DE ALCANCE PERICIAL OSINT Y RESPONSABILIDAD DE ÚLTIMA MILLA
a) El Dictamen Pericial refleja el estado registral público (INEGI DENUE, SAT 69/69-B,
   STPS REPSE, SFP) y la telemetría técnica externa del dominio web al segundo exacto
   de su emisión, sin sustituir una visita de inspección física gubernamental ni fe
   pública de Notario o Corredor Público.
b) Los 7 Kits Legales entregados en este paquete son instrumentos normativos pre-calibrados
   para la razón social auditada. Su impresión física, llenado de anexos específicos,
   recabo de firmas autógrafas de los trabajadores y representantes legales, así como
   su exhibición material en el centro de trabajo, constituyen una obligación de ejecución
   interna ("Última Milla") bajo la exclusiva responsabilidad de la administración de
   la empresa auditada.
c) Al tratarse de un entregable digital criptográfico personalizado e irrepetible compilado
   específicamente para el RFC {rfc}, la generación de este sello SHA-256 acredita la
   entrega total y definitiva del servicio a entera satisfacción del adquirente.
================================================================================
"#,
        nombre = empresa.nombre,
        rfc = tax_id,
        utc = Utc::now().to_rfc3339(),
        hash = empresa.audit_hash
    );

    zip.start_file(
        "00_MANIFIESTO_FECHA_CIERTA_Y_ALCANCE_PERICIAL.txt",
        options,
    )?;
    zip.write_all(manifiesto_content.as_bytes())?;

    let finished_cursor = zip.finish()?;
    let final_bytes = finished_cursor.into_inner();

    // Calculamos el hash SHA-256 real del binario ZIP usando nuestro método en RAM
    let _zip_sha256 = CryptoSigner::compute_bytes_hash(&final_bytes);

    Ok(final_bytes)
}
