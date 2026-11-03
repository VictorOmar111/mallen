//! Empaquetador maestro de entregables corporativos (.zip) sincronizado con `AuditMasterJson` y `LegalDossierInput`.

use crate::engine::crypto_signer::CryptoSigner;
use crate::engine::kit_generator::{
    build_all_compliance_documents_with_session, generate_compliance_kit_with_session,
};
use crate::engine::models::{AuditMasterJson, EmpresaB2B, LegalDossierInput};
use crate::engine::osint_scanner::OsintResult;
use crate::engine::pdf_generator::{
    generate_pdf_report_with_session, get_catalogo_certificados, sanitize_for_typst,
};
use chrono::Utc;
use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::process::Command;
use zip::write::FileOptions;
use zip::ZipWriter;

fn compile_kit_document_to_pdf(
    title: &str,
    markdown_body: &str,
    empresa: &EmpresaB2B,
    out_pdf_path: &str,
) -> Option<Vec<u8>> {
    let typ_path = format!("{}.typ", out_pdf_path);
    let tax_id = empresa.tax_id.as_deref().unwrap_or("SIN-RFC");

    let mut typst_lines = String::new();
    for line in markdown_body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            typst_lines.push_str(&format!(
                "\n#text(size: 10.5pt, weight: \"bold\", fill: rgb(\"#0f172a\"))[{}]\n#v(0.3em)\n",
                sanitize_for_typst(&trimmed[2..])
            ));
        } else if trimmed.starts_with("### ") {
            typst_lines.push_str(&format!(
                "\n#v(0.4em)\n#text(size: 9pt, weight: \"bold\", fill: rgb(\"#1e3a8a\"))[{}]\n#v(0.2em)\n",
                sanitize_for_typst(&trimmed[4..])
            ));
        } else if trimmed == "---" {
            typst_lines.push_str("\n#line(length: 100%, stroke: 0.5pt + rgb(\"#cbd5e1\"))\n");
        } else if trimmed.starts_with('|') {
            typst_lines.push_str(&format!(
                "#text(size: 7.5pt)[{}]\n\n",
                sanitize_for_typst(trimmed)
            ));
        } else {
            let clean = sanitize_for_typst(line).replace("\\*\\*", "*");
            typst_lines.push_str(&format!("{}\n", clean));
        }
    }

    let full_typ = format!(
        r##"#set page(
  paper: "a4",
  margin: (x: 1.8cm, y: 1.8cm),
  header: align(right)[#text(size: 7pt, fill: rgb("#64748b"), weight: "bold")[FIRMA PERICIAL MALLEN — INSTRUMENTO JURÍDICO PERSONALIZADO ({rfc})]],
  footer: [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(2pt)
    #text(size: 6.8pt, fill: rgb("#64748b"))[Entidad: {nombre} ({rfc}) | Sello Criptográfico SHA-256: {hash}]
  ]
)
#set text(size: 8.6pt, fill: rgb("#1e293b"))
#rect(width: 100%, fill: rgb("#0f172a"), inset: 8pt, radius: 3pt)[
  #text(fill: white, weight: "bold", size: 9.5pt)[{titulo}]
]
#v(0.5em)
{body}
"##,
        rfc = sanitize_for_typst(tax_id),
        nombre = sanitize_for_typst(&empresa.nombre),
        hash = sanitize_for_typst(&empresa.audit_hash),
        titulo = sanitize_for_typst(title),
        body = typst_lines
    );

    fs::write(&typ_path, full_typ).ok()?;
    let status = Command::new("typst")
        .arg("compile")
        .arg(&typ_path)
        .arg(out_pdf_path)
        .status()
        .ok()?;
    let _ = fs::remove_file(&typ_path);

    if status.success() {
        fs::read(out_pdf_path).ok()
    } else {
        None
    }
}

fn render_kit_as_printable_html(title: &str, markdown_body: &str, empresa: &EmpresaB2B) -> String {
    let mut body_html = String::new();
    for line in markdown_body.lines() {
        let t = line.trim();
        if t.starts_with("# ") {
            body_html.push_str(&format!("<h1>{}</h1>", &t[2..]));
        } else if t.starts_with("### ") {
            body_html.push_str(&format!("<h3>{}</h3>", &t[4..]));
        } else if t == "---" {
            body_html.push_str("<hr>");
        } else if t.is_empty() {
            body_html.push_str("<br>");
        } else {
            let formatted = line.replace("**", "<b>");
            body_html.push_str(&format!("<p>{}</p>", formatted));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
<meta charset="UTF-8">
<title>{}</title>
<style>
  body {{ font-family: 'Segoe UI', Arial, sans-serif; color: #1e293b; max-width: 820px; margin: 40px auto; line-height: 1.6; padding: 20px; }}
  h1 {{ color: #0f172a; font-size: 18px; border-bottom: 2px solid #0f172a; padding-bottom: 8px; }}
  h3 {{ color: #1e3a8a; font-size: 14px; margin-top: 18px; }}
  p {{ font-size: 12.5px; margin: 4px 0; }}
  hr {{ border: 0; border-top: 1px solid #cbd5e1; margin: 16px 0; }}
</style>
</head>
<body>
  <div style="background:#f8fafc;border:1px solid #cbd5e1;padding:10px 14px;font-size:11px;margin-bottom:20px;">
    <strong>FIRMA PERICIAL MALLEN (MALLEN.LAT)</strong> — Instrumento Jurídico Personalizado para <strong>{}</strong> | Sello: <code>{}</code>
  </div>
  {}
</body>
</html>"#,
        title, empresa.nombre, empresa.audit_hash, body_html
    )
}

pub fn generate_custom_bundle(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    generate_custom_bundle_with_session(empresa, osint, None, false)
}

pub fn generate_custom_bundle_with_session(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    master_opt: Option<&AuditMasterJson>,
    is_accredited: bool,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("GENERIC");
    let base_dir = format!("kits_blindaje/{}", tax_id);
    fs::create_dir_all(&base_dir)?;

    let empty_dossier = LegalDossierInput::default();
    let dossier = master_opt
        .map(|m| &m.calibration_applied.dossier)
        .unwrap_or(&empty_dossier);

    let _ = generate_compliance_kit_with_session(empresa, dossier, master_opt);

    let pdf_filename = format!("{}/Dictamen_Ejecutivo_{}.pdf", base_dir, tax_id);
    // Aseguramos la inyección limpia del master_opt para preservar el SCIAN y la parametrización exacta
    generate_pdf_report_with_session(empresa, osint, master_opt, is_accredited, &pdf_filename)?;

    let zip_bytes = build_zip_buffer_from_dir_with_session(empresa, dossier, master_opt, &base_dir, &pdf_filename)?;
    let zip_filename = format!("{}/Paquete_Blindaje_Integral_{}.zip", base_dir, tax_id);
    fs::write(&zip_filename, &zip_bytes)?;

    Ok(zip_filename)
}

pub fn build_zip_buffer_from_dir(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
    base_dir: &str,
    dictamen_pdf_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    build_zip_buffer_from_dir_with_session(empresa, dossier, None, base_dir, dictamen_pdf_path)
}

pub fn build_zip_buffer_from_dir_with_session(
    empresa: &EmpresaB2B,
    dossier: &LegalDossierInput,
    master_opt: Option<&AuditMasterJson>,
    base_dir: &str,
    dictamen_pdf_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("GENERIC");
    let buffer = std::io::Cursor::new(Vec::with_capacity(1024 * 1024));
    let mut zip = zip::ZipWriter::new(buffer);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    if std::path::Path::new(dictamen_pdf_path).exists() {
        let mut pdf_file = std::fs::File::open(dictamen_pdf_path)?;
        let mut pdf_bytes = Vec::new();
        std::io::Read::read_to_end(&mut pdf_file, &mut pdf_bytes)?;
        zip.start_file(
            format!("00_DICTAMEN_PERICIAL_EJECUTIVO_{}.pdf", tax_id),
            options,
        )?;
        std::io::Write::write_all(&mut zip, &pdf_bytes)?;
    }

    for cert in crate::engine::pdf_generator::get_catalogo_certificados() {
        let cert_disk_path = format!("{}/{}", base_dir, cert.filename);
        if std::path::Path::new(&cert_disk_path).exists() {
            if let Ok(cert_bytes) = std::fs::read(&cert_disk_path) {
                zip.start_file(
                    format!("00_CERTIFICADOS_ACREDITACION_PDF/{}", cert.filename),
                    options,
                )?;
                std::io::Write::write_all(&mut zip, &cert_bytes)?;
            }
        }
    }

    let compliance_docs = crate::engine::kit_generator::build_all_compliance_documents_with_session(empresa, dossier, master_opt);
    for doc in compliance_docs {
        let pdf_rel_path = doc.zip_relative_path.replace(".md", ".pdf");
        let html_rel_path = doc.zip_relative_path.replace(".md", "_Editable_Word.html");
        let temp_pdf_disk = format!("{}/{}", base_dir, doc.disk_filename.replace(".md", ".pdf"));

        if let Some(kit_pdf_bytes) =
            compile_kit_document_to_pdf(&doc.disk_filename, &doc.content, empresa, &temp_pdf_disk)
        {
            zip.start_file(&pdf_rel_path, options)?;
            std::io::Write::write_all(&mut zip, &kit_pdf_bytes)?;
        }

        let html_str = render_kit_as_printable_html(&doc.disk_filename, &doc.content, empresa);
        zip.start_file(&html_rel_path, options)?;
        std::io::Write::write_all(&mut zip, html_str.as_bytes())?;
    }

    if let Ok(entries) = std::fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_string();
            if fname.starts_with("CFDI40_Factura_") && fname.ends_with(".xml") {
                if let Ok(xml_bytes) = std::fs::read(entry.path()) {
                    zip.start_file(format!("06_FACTURA_FISCAL_CFDI40/{}", fname), options)?;
                    std::io::Write::write_all(&mut zip, &xml_bytes)?;
                }
            }
        }
    }

    let manifiesto_content = format!(
        "FIRMA PERICIAL MALLEN (MALLEN.LAT) — CERTIFICADO CRIPTOGRAFICO Y ALCANCE LEGAL\r\nENTIDAD AUDITADA: {}\r\nRFC / TAX ID: {}\r\nFECHA Y HORA UTC: {}\r\nHASH SHA-256 EXPEDIENTE: {}\r\nURL DE VERIFICACION: https://mallen.lat/verify/{}\r\n",
        empresa.nombre,
        tax_id,
        chrono::Utc::now().to_rfc3339(),
        empresa.audit_hash,
        tax_id
    );

    zip.start_file(
        "00_MANIFIESTO_FECHA_CIERTA_Y_ALCANCE_PERICIAL.txt",
        options,
    )?;
    std::io::Write::write_all(&mut zip, manifiesto_content.as_bytes())?;

    let finished_cursor = zip.finish()?;
    let final_bytes = finished_cursor.into_inner();
    let _zip_sha256 = crate::engine::crypto_signer::CryptoSigner::compute_bytes_hash(&final_bytes);

    Ok(final_bytes)
}
