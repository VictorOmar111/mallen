use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::OsintResult;
use std::fs;
use std::path::Path;
use std::process::Command;

const TYPST_TEMPLATE: &str = include_str!("report_template.typ");
const CERT_DYNAMIC_TEMPLATE: &str = include_str!("cert_dynamic.typ");

pub struct CertificadoNorma {
    pub id: &'static str,
    pub titulo: &'static str,
    pub color_hex: &'static str,
    pub descripcion: &'static str,
    pub folio_prefix: &'static str,
    pub filename: &'static str,
}

pub fn get_catalogo_certificados() -> Vec<CertificadoNorma> {
    vec![
        CertificadoNorma {
            id: "nom035",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-035-STPS-2018",
            color_hex: "#2563eb",
            descripcion: "Por haber estructurado e implementado exitosamente la Política de Prevención de Riesgos Psicosociales, Evaluación del Entorno Organizacional Favorable y Protocolos de Denuncia Interna, en apego estricto a las disposiciones de la Norma Oficial Mexicana NOM-035-STPS-2018 y los artículos 540 y 1002 de la Ley Federal del Trabajo.",
            folio_prefix: "STPS-NOM035",
            filename: "Certificado_Acreditacion_NOM035.pdf",
        },
        CertificadoNorma {
            id: "nom037",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-037-STPS-2023",
            color_hex: "#2563eb",
            descripcion: "Por haber acreditado la validación de la Política de Teletrabajo, Listas de Verificación de Condiciones Ergonométricas y de Seguridad en Lugares de Trabajo Remoto, en cumplimiento con la Norma Oficial Mexicana NOM-037-STPS-2023 y el artículo 330-J de la Ley Federal del Trabajo.",
            folio_prefix: "STPS-NOM037",
            filename: "Certificado_Acreditacion_NOM037.pdf",
        },
        CertificadoNorma {
            id: "nom019",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-019-STPS-2011",
            color_hex: "#2563eb",
            descripcion: "Por la constitución, registro y funcionamiento periódico de la Comisión Mixta de Seguridad e Higiene, así como la realización de recorridos de verificación en apego a la Norma Oficial Mexicana NOM-019-STPS-2011 y el Reglamento Federal de Seguridad y Salud en el Trabajo.",
            folio_prefix: "STPS-NOM019",
            filename: "Certificado_Acreditacion_NOM019.pdf",
        },
        CertificadoNorma {
            id: "nom030",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-030-STPS-2006",
            color_hex: "#2563eb",
            descripcion: "Por la integración del Diagnóstico de Seguridad y Salud en el Trabajo y el Programa de Servicios Preventivos de Salud, garantizando la prevención de accidentes y enfermedades laborales en términos de la NOM-030-STPS-2006.",
            folio_prefix: "STPS-NOM030",
            filename: "Certificado_Acreditacion_NOM030.pdf",
        },
        CertificadoNorma {
            id: "sat_69b",
            titulo: "SECRETARÍA DE HACIENDA / SAT — CFF ART. 69-B (MATERIALIDAD FISCAL)",
            color_hex: "#dc2626",
            descripcion: "Por haber acreditado el Expediente de Soporte y Entregables de Materialidad de Operaciones B2B, desvirtuando presunciones de inexistencia contractual del Art. 69-B del Código Fiscal de la Federación y protegiendo la deducibilidad de comprobantes fiscales.",
            folio_prefix: "SAT-CFF69B",
            filename: "Certificado_Materialidad_SAT69B.pdf",
        },
        CertificadoNorma {
            id: "imss_32d",
            titulo: "INSTITUTO MEXICANO DEL SEGURO SOCIAL — OPINIÓN 32-D & REGISTRO REPSE",
            color_hex: "#16a34a",
            descripcion: "Por la verificación periódica de la Opinión de Cumplimiento Positiva en materia de Seguridad Social, Auditoría de Primas de Riesgo Laboral e integración de expedientes de Subcontratación Especializada en términos de los artículos 15 y 304 de la Ley del Seguro Social.",
            folio_prefix: "IMSS-32D",
            filename: "Certificado_Cumplimiento_IMSS32D.pdf",
        },
        CertificadoNorma {
            id: "inai_lfpdppp",
            titulo: "INSTITUTO NACIONAL DE TRANSPARENCIA (INAI) — LFPDPPP",
            color_hex: "#d97706",
            descripcion: "Por haber implementado el Aviso de Privacidad Integral Ajustado, Mecanismos de Ejercicio de Derechos ARCO y Protocolos de Encriptación y Tratamiento Legítimo de Datos Personales en Posesión de Particulares.",
            folio_prefix: "INAI-PRIVACIDAD",
            filename: "Certificado_Privacidad_INAI.pdf",
        },
        CertificadoNorma {
            id: "proteccion_civil",
            titulo: "SISTEMA NACIONAL DE PROTECCIÓN CIVIL — PROGRAMA INTERNO PIPC",
            color_hex: "#ea580c",
            descripcion: "Por la conformación de Brigadas de Emergencia, Análisis de Vulnerabilidades y la integración del Programa Interno de Protección Civil y Seguridad Operativa en cumplimiento con la Ley General de Protección Civil.",
            folio_prefix: "PROT-CIVIL",
            filename: "Certificado_Seguridad_ProtCivil.pdf",
        },
        CertificadoNorma {
            id: "profeco_lfpc",
            titulo: "PROFECO — LEY FEDERAL DE PROTECCIÓN AL CONSUMIDOR (ART. 76 BIS)",
            color_hex: "#7c3aed",
            descripcion: "Por la alineación de Términos y Condiciones comerciales, Políticas de Devolución, Garantías de Transacción y visibilidad de canales de atención al consumidor en cumplimiento con el Art. 76 BIS de la Ley Federal de Protección al Consumidor.",
            folio_prefix: "PROFECO-76BIS",
            filename: "Certificado_Comercial_PROFECO.pdf",
        },
        CertificadoNorma {
            id: "nom151_scfi",
            titulo: "SECRETARÍA DE ECONOMÍA — NOM-151-SCFI-2016 (MENSAJES DE DATOS)",
            color_hex: "#0284c7",
            descripcion: "Por la validez jurídica e integridad de contratos y expedientes digitales respaldados con Sellos de Tiempo y Conservación de Mensajes de Datos bajo la Norma Oficial Mexicana NOM-151-SCFI-2016.",
            folio_prefix: "SE-NOM151",
            filename: "Certificado_Digital_NOM151.pdf",
        },
    ]
}

fn sanitize_for_typst(input: &str) -> String {
    input
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

fn format_currency_mxn(val: f64) -> String {
    let integer_part = val.trunc() as u64;
    let num_str = integer_part.to_string();
    let mut formatted = String::new();
    let len = num_str.len();

    for (i, c) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }

    format!("{}.00", formatted)
}

pub fn generate_pdf_report(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("N/A");
    let typst_source_path = format!("{}.typ", output_path);

    let parent_dir = match Path::new(output_path).parent() {
        Some(p) => {
            fs::create_dir_all(p)?;
            p.to_path_buf()
        }
        None => Path::new(".").to_path_buf(),
    };

    let osint_inai = if osint.has_privacy_policy {
        "Aviso de Privacidad Detectado en portal web"
    } else {
        "ALERTA: Ausencia de Aviso de Privacidad. Alto riesgo por el INAI"
    };

    let osint_ssl = if osint.has_ssl {
        "SEGURO (HTTPS)"
    } else {
        "INSEGURO (HTTP)"
    };

    let osint_privacy = if osint.has_privacy_policy {
        "DETECTADO"
    } else {
        "AUSENTE / NO VISIBLE"
    };

    let score = empresa.risk_score as f64;

    let score_stps = (score * 0.25).round();
    let score_sat = (score * 0.20).round();
    let score_inai = (score * 0.20).round();
    let score_imss = (score * 0.15).round();
    let score_pc = (score * 0.10).round();
    let score_profeco = (score * 0.10).round();

    let avg_risk_mxn = 1188000.00;
    let activation_fee_mxn = avg_risk_mxn * 0.005;

    let fine_formatted = format_currency_mxn(empresa.estimated_fine_mxn);
    let avg_risk_formatted = format_currency_mxn(avg_risk_mxn);
    let activation_fee_formatted = format_currency_mxn(activation_fee_mxn);

    let nombre_clean = sanitize_for_typst(&empresa.nombre);
    let direccion_clean = sanitize_for_typst(&empresa.direccion);
    let actividad_clean = sanitize_for_typst(&empresa.actividad);

    let typst_content = TYPST_TEMPLATE
        .replace("{{EMPRESA_NOMBRE}}", &nombre_clean)
        .replace("{{EMPRESA_TAX_ID}}", tax_id)
        .replace("{{EMPRESA_ACTIVIDAD}}", &actividad_clean)
        .replace("{{EMPRESA_DIRECCION}}", &direccion_clean)
        .replace("{{ESTIMATED_FINE}}", &fine_formatted)
        .replace("{{AVG_RISK}}", &avg_risk_formatted)
        .replace("{{ACTIVATION_PRICE}}", &activation_fee_formatted)
        .replace("{{RISK_SCORE}}", &empresa.risk_score.to_string())
        .replace("{{INAI_OBS}}", osint_inai)
        .replace("{{SSL_STATUS}}", osint_ssl)
        .replace("{{PRIVACY_STATUS}}", osint_privacy)
        .replace("{{HTTP_STATUS}}", &osint.http_status.to_string())
        .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
        .replace("{{SCORE_STPS}}", &score_stps.to_string())
        .replace("{{SCORE_SAT}}", &score_sat.to_string())
        .replace("{{SCORE_INAI}}", &score_inai.to_string())
        .replace("{{SCORE_IMSS}}", &score_imss.to_string())
        .replace("{{SCORE_PC}}", &score_pc.to_string())
        .replace("{{SCORE_PROFECO}}", &score_profeco.to_string())
        .replace("{{REF_TAX}}", tax_id);

    fs::write(&typst_source_path, typst_content)?;

    let status = Command::new("typst")
        .arg("compile")
        .arg(&typst_source_path)
        .arg(output_path)
        .status()?;

    let _ = fs::remove_file(typst_source_path);

    let mut generated_pdfs = vec![output_path.to_string()];
    let catalogo = get_catalogo_certificados();

    for cert in catalogo {
        let cert_pdf_path = parent_dir.join(cert.filename);
        let cert_typ_path = parent_dir.join(format!("{}.typ", cert.id));

        let cert_content = CERT_DYNAMIC_TEMPLATE
            .replace("{{EMPRESA_NOMBRE}}", &nombre_clean)
            .replace("{{EMPRESA_TAX_ID}}", tax_id)
            .replace("{{NORMA_TITULO}}", cert.titulo)
            .replace("{{NORMA_COLOR}}", cert.color_hex)
            .replace("{{NORMA_DESCRIPCION}}", cert.descripcion)
            .replace("{{NORMA_FOLIO_PREFIX}}", cert.folio_prefix)
            .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
            .replace("{{REF_TAX}}", tax_id);

        fs::write(&cert_typ_path, cert_content)?;

        let cert_status = Command::new("typst")
            .arg("compile")
            .arg(&cert_typ_path)
            .arg(&cert_pdf_path)
            .status();

        if let Err(e) = cert_status {
            eprintln!("⚠️ Error al compilar el certificado {}: {}", cert.filename, e);
        }

        let _ = fs::remove_file(cert_typ_path);
        generated_pdfs.push(cert_pdf_path.to_str().unwrap().to_string());
    }

    let zip_filename = format!("Paquete_Blindaje_Integral_{}.zip", tax_id);
    let zip_path = parent_dir.join(&zip_filename);

    let mut zip_cmd = Command::new("zip");
    zip_cmd.arg("-j").arg(&zip_path);

    for pdf in &generated_pdfs {
        zip_cmd.arg(pdf);
    }

    let _ = zip_cmd.status();

    if status.success() {
        Ok(())
    } else {
        Err("Fallo al compilar el documento PDF pericial con Typst".into())
    }
}
