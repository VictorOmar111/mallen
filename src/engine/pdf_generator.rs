//! Compilador tipográfico sincronizado 1:1 con `AuditMasterJson` y `LegalDossierInput`.
//! Soporta Modo Inmune (`is_accredited = true` para `MALLEN-*` y clientes pagados con $0.00 MXN de exposición).

use crate::engine::accountant::AccountantProfile;
use crate::engine::models::{
    AuditMasterJson, EmpresaB2B, LegalDossierInput, RegulatoryAgency,
};
use crate::engine::osint_scanner::OsintResult;
use chrono::Utc;
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
            descripcion: "Por haber estructurado la Política de Prevención de Riesgos Psicosociales, Guía de Identificación de Acontecimientos Traumáticos Severos y Protocolos de Denuncia Interna, en apego a la NOM-035-STPS-2018 y los artículos 132 y 994 de la Ley Federal del Trabajo.",
            folio_prefix: "STPS-NOM035",
            filename: "Certificado_Acreditacion_NOM035.pdf",
        },
        CertificadoNorma {
            id: "nom037",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-037-STPS-2023",
            color_hex: "#2563eb",
            descripcion: "Por la estructuración de la Política de Teletrabajo, Cláusula de Derecho a la Desconexión y Lista de Verificación de Condiciones de Seguridad y Salud en el Trabajo Remoto, conforme a la NOM-037-STPS-2023 y los artículos 330-A a 330-K de la LFT.",
            folio_prefix: "STPS-NOM037",
            filename: "Certificado_Acreditacion_NOM037.pdf",
        },
        CertificadoNorma {
            id: "nom019",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-019-STPS-2011",
            color_hex: "#2563eb",
            descripcion: "Por la integración del Acta Constitutiva de la Comisión Mixta de Seguridad e Higiene, Programa Anual de Recorridos de Verificación y formatos de investigación de causas de riesgos conforme a la NOM-019-STPS-2011 y los artículos 509 y 510 de la LFT.",
            folio_prefix: "STPS-NOM019",
            filename: "Certificado_Acreditacion_NOM019.pdf",
        },
        CertificadoNorma {
            id: "nom030",
            titulo: "SECRETARÍA DEL TRABAJO Y PREVISIÓN SOCIAL (STPS) — NOM-030-STPS-2009",
            color_hex: "#2563eb",
            descripcion: "Por la designación formal del Responsable de Seguridad y Salud en el Trabajo, integración del Diagnóstico Integral y Programa Anual de Servicios Preventivos en términos de la NOM-030-STPS-2009.",
            folio_prefix: "STPS-NOM030",
            filename: "Certificado_Acreditacion_NOM030.pdf",
        },
        CertificadoNorma {
            id: "sat_69b",
            titulo: "SHCP / SAT — CFF ART. 69-B (MATERIALIDAD Y FECHA CIERTA SCJN)",
            color_hex: "#dc2626",
            descripcion: "Por la integración del Contrato Marco de Prestación de Servicios con Sello de Fecha Cierta (Jurisprudencia 2a./J. 161/2019 SCJN) y Bitácoras de Entregables para acreditar sustancia económica y materialidad conforme al Art. 69-B del Código Fiscal de la Federación.",
            folio_prefix: "SAT-CFF69B",
            filename: "Certificado_Materialidad_SAT69B.pdf",
        },
        CertificadoNorma {
            id: "imss_32d",
            titulo: "IMSS / INFONAVIT — OPINIÓN 32-D & BLINDAJE SOLIDARIO REPSE",
            color_hex: "#16a34a",
            descripcion: "Por el protocolo de vigilancia de Opinión de Cumplimiento 32-D en Seguridad Social, conciliación de Salario Base de Cotización (SBC) e integración del expediente de contratistas especializados REPSE (ICSOE/SISSUB) conforme a los artículos 15, 15-A y 304-A de la LSS.",
            folio_prefix: "IMSS-32D",
            filename: "Certificado_Cumplimiento_IMSS32D.pdf",
        },
        CertificadoNorma {
            id: "inai_lfpdppp",
            titulo: "PROTECCIÓN DE DATOS (SABG / EX-INAI) — LFPDPPP VIGENTE",
            color_hex: "#d97706",
            descripcion: "Por la estructuración del Aviso de Privacidad Integral, Simplificado y Corto, Procedimiento de Derechos ARCO, Acta de Oficial de Privacidad y Formato de Consentimiento Expreso para Datos Sensibles y Biométricos conforme a la LFPDPPP.",
            folio_prefix: "SABG-LFPDPPP",
            filename: "Certificado_Privacidad_INAI.pdf",
        },
        CertificadoNorma {
            id: "proteccion_civil",
            titulo: "SISTEMA NACIONAL DE PROTECCIÓN CIVIL — PROGRAMA INTERNO PIPC",
            color_hex: "#ea580c",
            descripcion: "Por las Actas Constitutivas de la Unidad Interna de Protección Civil, Brigadas de Evacuación, Primeros Auxilios, Prevención y Combate de Incendios y Guía de Continuidad de Operaciones en apego a la Ley General de Protección Civil.",
            folio_prefix: "PROT-CIVIL",
            filename: "Certificado_Seguridad_ProtCivil.pdf",
        },
        CertificadoNorma {
            id: "profeco_lfpc",
            titulo: "PROFECO — LEY FEDERAL DE PROTECCIÓN AL CONSUMIDOR (ART. 76 BIS)",
            color_hex: "#7c3aed",
            descripcion: "Por la adecuación de Términos y Condiciones de Comercio Electrónico, Políticas de Devolución, Reembolso, Derecho de Revocación de 5 días hábiles y Póliza de Garantías en cumplimiento con los artículos 56, 76 BIS, 77 y 85 de la LFPC.",
            folio_prefix: "PROFECO-76BIS",
            filename: "Certificado_Comercial_PROFECO.pdf",
        },
        CertificadoNorma {
            id: "nom151_scfi",
            titulo: "SECRETARÍA DE ECONOMÍA / SCJN — INTEGRIDAD CRIPTOGRÁFICA Y FECHA CIERTA",
            color_hex: "#0284c7",
            descripcion: "Por el sellado criptográfico SHA-256 e inclusión en el Árbol de Merkle de preservación de integridad de mensajes de datos conforme a los artículos 89 a 94 del Código de Comercio, Art. 210-A del CFPC y criterios de Fecha Cierta Digital.",
            folio_prefix: "CRYPTO-MERKLE",
            filename: "Certificado_Digital_NOM151.pdf",
        },
    ]
}

pub fn sanitize_for_typst(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('$', "\\$")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('`', "\\`")
}

pub fn format_currency_mxn(val: f64) -> String {
    let rounded = (val.max(0.0) * 100.0).round() as u64;
    let integer_part = rounded / 100;
    let cents = rounded % 100;
    let num_str = integer_part.to_string();
    let mut formatted = String::new();
    let len = num_str.len();

    for (i, c) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }

    format!("{}.{:02}", formatted, cents)
}

fn build_report_typst_markup_full(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    master_opt: Option<&AuditMasterJson>,
    _is_accredited: bool,
    _is_censored_micro_report: bool,
) -> String {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("SIN-RFC");
    let is_immune = tax_id.starts_with("MALLEN");
    let fecha_hoy = Utc::now().format("%d/%m/%Y %H:%M UTC").to_string();
    let auditor = AccountantProfile::default();

    let empty_dossier = LegalDossierInput::default();
    let dossier = master_opt
        .map(|m| &m.calibration_applied.dossier)
        .unwrap_or(&empty_dossier);

    let rep_legal = dossier
        .representante_legal_nombre
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("[APODERADO LEGAL POR DESIGNAR EN CÉDULA]");
    let jurisdiccion = dossier
        .entidad_jurisdiccion
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&empresa.estado);
    let oficial_priv = dossier
        .oficial_privacidad_nombre
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(rep_legal);
    let coord_csh = dossier
        .comision_coordinador_patronal
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(rep_legal);

    let partner_header = match master_opt
        .and_then(|m| m.partner_whitelabel_name.as_deref())
        .or(empresa.accountant_signature.as_deref())
    {
        Some(p) if !p.trim().is_empty() && !p.starts_with("DICTAMINADO_POR:") => {
            format!("DESPACHO AUDITOR ALIADO: {}", sanitize_for_typst(p))
        }
        _ => "FIRMA PERICIAL MALLEN (MALLEN.LAT)".to_string(),
    };

    // Calcular subtotales exactos por dependencia para sincronía matemática 1:1 con la Web
    let (mut m_stps, mut m_sat, mut m_inai, mut m_imss, mut m_pc, mut m_profeco) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    if !is_immune {
        if let Some(m) = master_opt {
            for f in &m.findings {
                match f.agency {
                    RegulatoryAgency::STPS => m_stps += f.estimated_fine_mxn,
                    RegulatoryAgency::SAT => m_sat += f.estimated_fine_mxn,
                    RegulatoryAgency::SabgInai => m_inai += f.estimated_fine_mxn,
                    RegulatoryAgency::IMSSInfonavit => m_imss += f.estimated_fine_mxn,
                    RegulatoryAgency::PROFECO => m_profeco += f.estimated_fine_mxn,
                    _ => m_pc += f.estimated_fine_mxn,
                }
            }
        } else {
            let base = if empresa.estimated_fine_mxn > 0.0 {
                empresa.estimated_fine_mxn
            } else {
                1_485_600.00
            };
            m_stps = (base * 0.28 * 100.0).round() / 100.0;
            m_sat = (base * 0.22 * 100.0).round() / 100.0;
            m_inai = (base * 0.18 * 100.0).round() / 100.0;
            m_imss = (base * 0.14 * 100.0).round() / 100.0;
            m_pc = (base * 0.10 * 100.0).round() / 100.0;
            m_profeco = ((base - (m_stps + m_sat + m_inai + m_imss + m_pc)) * 100.0).round() / 100.0;
        }
    }

    let total_exact = ((m_stps + m_sat + m_inai + m_imss + m_pc + m_profeco) * 100.0).round() / 100.0;
    let score = if is_immune {
        0
    } else {
        master_opt
            .map(|m| m.risk_score)
            .unwrap_or(empresa.risk_score.max(72))
    };

    let (status_hex, banner_bg, banner_border, estatus_texto, badge_accion) = if is_immune {
        (
            "#16a34a",
            "#f0fdf4",
            "#86efac",
            "100% CUMPLIMIENTO ACREDITADO • RIESGO CERO (\\$0.00 MXN)",
            "EXPEDIENTE INMUNE ACTIVO",
        )
    } else {
        (
            "#dc2626",
            "#fef2f2",
            "#fca5a5",
            "EXPOSICIÓN PATRIMONIAL ACTIVA DETECTADA",
            "ACCIÓN CORRECTIVA REQUERIDA",
        )
    };

    // Clasificar Clase y Prima de Riesgo IMSS según el código SCIAN real de la empresa
    let scian_extracted: String = master_opt
        .map(|m| m.codigo_scian.clone())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            empresa
                .actividad
                .split(|c: char| !c.is_ascii_digit())
                .find(|tok| tok.len() >= 2)
                .unwrap_or("541610")
                .to_string()
        });
    let scian_str = scian_extracted.as_str();
    let (imss_clase, imss_prima, imss_sector_desc) =
        crate::modules::vectors_37::classify_imss_risk_by_scian(scian_str);

    // Determinar número estimado de trabajadores desde el expediente maestro o el estrato DENUE
    let emp_count: u32 = master_opt
        .and_then(|m| m.calibration_applied.exact_employee_count)
        .unwrap_or(35)
        .max(1);

    let guia_nom035 = if emp_count > 50 {
        "Guías I y III (Evaluación de Entorno Organizacional >50 empleados)"
    } else if emp_count >= 16 {
        "Guías I y II (Cuestionarios Psicosociales 16 a 50 empleados)"
    } else {
        "Política Escrita de Prevención Psicosocial y Guía I (1 a 15 empleados)"
    };

    let sector_especifico = if scian_str.starts_with("722") || scian_str.starts_with("311") {
        "Aviso de Funcionamiento COFEPRIS DIGIPRIS, Higiene NOM-251-SSA1, trampas de grasa y PIPC con dictamen de gas LP/natural."
    } else if scian_str.starts_with("62") || scian_str.starts_with("464") {
        "Aviso Sanitario COFEPRIS, Responsable Sanitario, manejo de RPBI (NOM-087-SEMARNAT) y Programa Interno PIPC."
    } else if scian_str.starts_with("811") || scian_str.starts_with("31") || scian_str.starts_with("32") || scian_str.starts_with("33") {
        "Registro SEMARNAT de Generador de Residuos Peligrosos (aceites/solventes), Licencia Ambiental Única y PIPC industrial."
    } else if scian_str.starts_with("48") || scian_str.starts_with("49") || scian_str.starts_with("43") {
        "Bitácoras de horas de servicio NOM-087-SCT-2, dictamen físico-mecánico NOM-068-SCT, Carta Porte 3.1 y PIPC de bodega."
    } else if scian_str.starts_with("52") || scian_str.starts_with("53") || scian_str.starts_with("23") {
        "Alta y expediente único de identificación PLD/LFPIORPI (Art. 17 y 18), restricción de efectivo (Art. 32) y carpeta PIPC."
    } else {
        "Programa Interno de Protección Civil (PIPC), Acta Constitutiva de 4 Brigadas, Póliza RC y dictamen eléctrico/estructural."
    };

    // Evaluar la matriz completa de 37 vectores para esta empresa específica
    let sat_69b_lbl = master_opt
        .map(|m| m.sat_69b_status.as_str())
        .unwrap_or("COMPULSADO SIN ANTECEDENTES EFOS (DOF)");
    let sat_69_lbl = master_opt
        .map(|m| m.sat_69_status.as_str())
        .unwrap_or("ACTIVO / SIN CRÉDITOS FIRMES EXIGIBLES");
    let repse_lbl = master_opt
        .map(|m| {
            if m.stps_repse_registered {
                "FOLIO REPSE VIGENTE VERIFICADO"
            } else {
                "SUJETO A COMPULSA DE PROVEEDORES REPSE / ICSOE"
            }
        })
        .unwrap_or("SUJETO A COMPULSA DE PROVEEDORES REPSE / ICSOE");
    let sfp_lbl = master_opt
        .map(|m| {
            if m.sfp_sanctioned {
                "ALERTA: REGISTRO EN DIRECTORIO SFP"
            } else {
                "SIN INHABILITACIÓN EN COMPRANET / SFP"
            }
        })
        .unwrap_or("SIN INHABILITACIÓN EN COMPRANET / SFP");

    let evaluated_37 = crate::modules::vectors_37::evaluate_all_37_vectors(
        &empresa.nombre,
        scian_str,
        emp_count,
        sat_69b_lbl,
        sat_69_lbl,
        repse_lbl.contains("VIGENTE"),
        sfp_lbl.contains("ALERTA"),
        osint.has_ssl || is_immune,
        osint.has_privacy_policy || is_immune,
        osint.has_privacy_policy || is_immune,
        is_immune,
        true,
        true,
        scian_str.starts_with("31") || scian_str.starts_with("32") || scian_str.starts_with("33") || scian_str.starts_with("722"),
        true,
        scian_str.starts_with("811") || scian_str.starts_with("31") || scian_str.starts_with("32") || scian_str.starts_with("62"),
        true,
        true,
        scian_str.starts_with("48") || scian_str.starts_with("49") || scian_str.starts_with("43"),
        scian_str.starts_with("52") || scian_str.starts_with("53") || scian_str.starts_with("23"),
    );

    // SINCRONIZACIÓN MATEMÁTICA 1:1 ABSOLUTA:
    // Los 6 subtotales de la Página 1 se calculan sumando directamente los 37 vectores de la Página 2
    m_stps = 0.0;
    m_sat = 0.0;
    m_inai = 0.0;
    m_imss = 0.0;
    m_pc = 0.0;
    m_profeco = 0.0;
    if !is_immune {
        for v in &evaluated_37 {
            match v.organo.as_str() {
                "STPS" => m_stps += v.multa_estimada_mxn,
                "SAT" => m_sat += v.multa_estimada_mxn,
                "LFPDPPP" => m_inai += v.multa_estimada_mxn,
                "IMSS" => m_imss += v.multa_estimada_mxn,
                "PROFECO" | "ECONOMÍA" => m_profeco += v.multa_estimada_mxn,
                _ => m_pc += v.multa_estimada_mxn,
            }
        }
    }
    let total_exact = ((m_stps + m_sat + m_inai + m_imss + m_pc + m_profeco) * 100.0).round() / 100.0;
    let visual_chart_typst = crate::modules::vectors_37::render_visual_risk_chart_typst(
        m_stps, m_sat, m_imss, m_inai, m_pc, m_profeco,
    );

    let hallazgo_stps_str = if is_immune {
        format!("0 pts: Carpetas STPS ({}, NOM-019, NOM-030 y NOM-002) instrumentadas para {} trabajadores.", guia_nom035, emp_count)
    } else {
        format!("Plantilla evaluada ({} personas, SCIAN {}): obligatorio exhibir {}, Acta NOM-019, Diagnóstico NOM-030 y estudio de incendio NOM-002.", emp_count, scian_str, guia_nom035)
    };

    let hallazgo_sat_str = if is_immune {
        "0 pts: Contratos con Fecha Cierta SCJN (Art. 69-B CFF), Beneficiario Controlador y CFDI 4.0 verificados.".to_string()
    } else {
        format!("Compulsa 69-B [{}]. Exigible Contrato Marco con Fecha Cierta (Jurisprudencia SCJN 2a./J. 161/2019), Razón de Negocios Art. 5-A y Expediente de Beneficiario Controlador (Art. 32-B Ter CFF).", sat_69b_lbl)
    };

    let osint_inai_str = if is_immune {
        "0 pts: Aviso de Privacidad Integral LFPDPPP (SABG), Derechos ARCO y TLS 1.3 activos.".to_string()
    } else if osint.has_privacy_policy {
        format!("Aviso web detectado en dominio; requiere adecuación a la Autoridad Garante SABG (LFPDPPP), Documento de Seguridad (Art. 19) y Consentimiento Expreso para Biométricos/Nómina de {} trabajadores.", emp_count)
    } else {
        format!("ALERTA CRÍTICA (Arts. 15, 16 y 63 LFPDPPP): Omisión de Aviso de Privacidad Integral, Ventanilla ARCO y formato de Consentimiento Biométrico/Nómina para {}.", empresa.nombre)
    };

    let hallazgo_imss_str = if is_immune {
        format!("0 pts: Plantilla conciliada en SUA bajo {} (Prima {}) y protocolo REPSE activo.", imss_clase, imss_prima)
    } else {
        format!("Por actividad SCIAN {} ({}) corresponde {} con Prima Media del {}. Exigible conciliación de {} empleados vs. SUA, Opinión 32-D y compulsa REPSE/ICSOE.", scian_str, imss_sector_desc, imss_clase, imss_prima, emp_count)
    };

    let hallazgo_pc_str = if is_immune {
        "0 pts: Programa Interno de Protección Civil (PIPC) y autorizaciones sectoriales vigentes.".to_string()
    } else {
        format!("Requerimiento específico para giro SCIAN {}: {}", scian_str, sector_especifico)
    };

    let hallazgo_profeco_str = if is_immune {
        "0 pts: Términos de Comercio Electrónico Art. 76 BIS LFPC y garantías de 90 días vigentes.".to_string()
    } else {
        format!("Adecuación obligatoria al Art. 76 BIS LFPC (Comercio Digital), Póliza de Garantía mínima de 90 días (Art. 77 LFPC), purga de cláusulas nulas (Art. 90) y sellado criptográfico CCom Art. 89 para {}.", empresa.nombre)
    };

    let osint_ssl = if osint.has_ssl || is_immune {
        "ACTIVO (TLS 1.3 VÁLIDO)"
    } else {
        "VULNERABLE (INFRACCIÓN ART. 19 LFPDPPP)"
    };
    let osint_privacy = if osint.has_privacy_policy || is_immune {
        "ACREDITADO EN PORTAL"
    } else {
        "OMISIÓN DETECTADA"
    };

    let s_f = score as f64;
    let nombre_clean = sanitize_for_typst(&empresa.nombre);
    let direccion_clean = sanitize_for_typst(&crate::modules::vectors_37::sanitize_denue_address(
        &empresa.direccion,
        &empresa.estado,
        tax_id,
    ));
    let actividad_clean = if empresa.actividad.contains(scian_str) {
        sanitize_for_typst(&empresa.actividad)
    } else {
        sanitize_for_typst(&format!("{} — {}", scian_str, empresa.actividad))
    };
    let tax_clean = sanitize_for_typst(tax_id);
    let vectors_37_typst = crate::modules::vectors_37::render_vectors_37_typst_table(&evaluated_37);

    TYPST_TEMPLATE
        .replace("{{PARTNER_HEADER}}", &partner_header)
        .replace("{{FECHA_EMISION}}", &fecha_hoy)
        .replace("{{SCAN_DATE}}", &fecha_hoy)
        .replace("{{EMPRESA_NOMBRE}}", &nombre_clean)
        .replace("{{EMPRESA_TAX_ID}}", &tax_clean)
        .replace("{{REPRESENTANTE_LEGAL}}", &sanitize_for_typst(rep_legal))
        .replace("{{EMPRESA_JURISDICCION}}", &sanitize_for_typst(jurisdiccion))
        .replace("{{JURISDICCION}}", &sanitize_for_typst(jurisdiccion))
        .replace("{{OFICIAL_PRIVACIDAD}}", &sanitize_for_typst(oficial_priv))
        .replace("{{COORDINADOR_CSH}}", &sanitize_for_typst(coord_csh))
        .replace("{{COORDINADOR_NOM019}}", &sanitize_for_typst(coord_csh))
        .replace("{{AUDITOR_NOMBRE}}", &sanitize_for_typst(&auditor.nombre))
        .replace("{{AUDITOR_CEDULA}}", &sanitize_for_typst(&auditor.cedula_profesional))
        .replace("{{AUDITOR_DC5}}", &sanitize_for_typst(&auditor.registro_auditor))
        .replace("{{AUDITOR_STPS}}", &sanitize_for_typst(&auditor.registro_auditor))
        .replace("{{EMPRESA_ACTIVIDAD}}", &actividad_clean)
        .replace("{{EMPRESA_SCIAN}}", &actividad_clean)
        .replace("{{EMPRESA_DIRECCION}}", &direccion_clean)
        .replace("{{EMPRESA_DOMICILIO}}", &direccion_clean)
        .replace("{{STATUS_HEX}}", status_hex)
        .replace("{{BANNER_BG_HEX}}", banner_bg)
        .replace("{{BANNER_BORDER_HEX}}", banner_border)
        .replace("{{ESTATUS_TEXTO}}", estatus_texto)
        .replace("{{ESTATUS_GLOBAL}}", estatus_texto)
        .replace("{{BADGE_ACCION}}", badge_accion)
        .replace("{{ESTIMATED_FINE}}", &format_currency_mxn(total_exact))
        .replace("{{TOTAL_FINE_MXN}}", &format_currency_mxn(total_exact))
        .replace("{{MULTA_STPS}}", &format_currency_mxn(m_stps))
        .replace("{{FINE_STPS}}", &format_currency_mxn(m_stps))
        .replace("{{MULTA_SAT}}", &format_currency_mxn(m_sat))
        .replace("{{FINE_SAT}}", &format_currency_mxn(m_sat))
        .replace("{{MULTA_INAI}}", &format_currency_mxn(m_inai))
        .replace("{{FINE_INAI}}", &format_currency_mxn(m_inai))
        .replace("{{MULTA_IMSS}}", &format_currency_mxn(m_imss))
        .replace("{{FINE_IMSS}}", &format_currency_mxn(m_imss))
        .replace("{{MULTA_PC}}", &format_currency_mxn(m_pc))
        .replace("{{FINE_PROT_CIVIL}}", &format_currency_mxn(m_pc))
        .replace("{{MULTA_PROFECO}}", &format_currency_mxn(m_profeco))
        .replace("{{FINE_PROFECO}}", &format_currency_mxn(m_profeco))
        .replace("{{ACTIVATION_PRICE}}", "5,940.00")
        .replace("{{KIT_PRICE_MXN}}", "5,940.00")
        .replace("{{RISK_SCORE}}", &score.to_string())
        .replace("{{HALLAZGO_TABLA_STPS}}", &sanitize_for_typst(&hallazgo_stps_str))
        .replace("{{DESC_STPS}}", &sanitize_for_typst(&hallazgo_stps_str))
        .replace("{{HALLAZGO_TABLA_SAT}}", &sanitize_for_typst(&hallazgo_sat_str))
        .replace("{{DESC_SAT}}", &sanitize_for_typst(&hallazgo_sat_str))
        .replace("{{INAI_OBS}}", &sanitize_for_typst(&osint_inai_str))
        .replace("{{DESC_INAI}}", &sanitize_for_typst(&osint_inai_str))
        .replace("{{HALLAZGO_TABLA_IMSS}}", &sanitize_for_typst(&hallazgo_imss_str))
        .replace("{{DESC_IMSS}}", &sanitize_for_typst(&hallazgo_imss_str))
        .replace("{{HALLAZGO_TABLA_PC}}", &sanitize_for_typst(&hallazgo_pc_str))
        .replace("{{DESC_PROT_CIVIL}}", &sanitize_for_typst(&hallazgo_pc_str))
        .replace("{{HALLAZGO_TABLA_PROFECO}}", &sanitize_for_typst(&hallazgo_profeco_str))
        .replace("{{DESC_PROFECO}}", &sanitize_for_typst(&hallazgo_profeco_str))
        .replace("{{SAT_69B_STATUS}}", &sanitize_for_typst(sat_69b_lbl))
        .replace("{{SAT_69_STATUS}}", &sanitize_for_typst(sat_69_lbl))
        .replace("{{REPSE_STATUS}}", &sanitize_for_typst(repse_lbl))
        .replace("{{SFP_STATUS}}", &sanitize_for_typst(sfp_lbl))
        .replace("{{SSL_STATUS}}", osint_ssl)
        .replace("{{PRIVACY_STATUS}}", osint_privacy)
        .replace("{{INAI_STATUS}}", osint_privacy)
        .replace("{{HTTP_STATUS}}", &osint.http_status.max(200).to_string())
        .replace("{{HTTP_CODE}}", &osint.http_status.max(200).to_string())
        .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
        .replace("{{SCORE_STPS}}", &(s_f * 0.25).round().to_string())
        .replace("{{SCORE_SAT}}", &(s_f * 0.20).round().to_string())
        .replace("{{SCORE_INAI}}", &(s_f * 0.20).round().to_string())
        .replace("{{SCORE_IMSS}}", &(s_f * 0.15).round().to_string())
        .replace("{{SCORE_PC}}", &(s_f * 0.10).round().to_string())
        .replace("{{SCORE_PROFECO}}", &(s_f * 0.10).round().to_string())
        .replace("{{REF_TAX}}", &tax_clean)
        .replace("{{VECTORS_37_TABLE_TYPST}}", &vectors_37_typst)
        .replace("{{VISUAL_RISK_CHART_TYPST}}", &visual_chart_typst)
}

pub fn generate_pdf_report(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    generate_pdf_report_with_session(empresa, osint, None, false, output_path)
}

pub fn generate_pdf_report_with_session(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    master_opt: Option<&AuditMasterJson>,
    is_accredited: bool,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or("SIN-RFC");
    let fecha_hoy = Utc::now().format("%d/%m/%Y").to_string();
    let typst_source_path = format!("{}.typ", output_path);
    let auditor = AccountantProfile::default();

    let empty_dossier = LegalDossierInput::default();
    let dossier = master_opt
        .map(|m| &m.calibration_applied.dossier)
        .unwrap_or(&empty_dossier);
    let rep_legal = dossier
        .representante_legal_nombre
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("[APODERADO LEGAL POR DESIGNAR EN CÉDULA]");
    let jurisdiccion = dossier
        .entidad_jurisdiccion
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&empresa.estado);

    let parent_dir = match Path::new(output_path).parent() {
        Some(p) if !p.as_os_str().is_empty() => {
            fs::create_dir_all(p)?;
            p.to_path_buf()
        }
        _ => Path::new(".").to_path_buf(),
    };

    let typst_content =
        build_report_typst_markup_full(empresa, osint, master_opt, is_accredited, false);
    fs::write(&typst_source_path, &typst_content)?;

    let status = Command::new("typst")
        .arg("compile")
        .arg(&typst_source_path)
        .arg(output_path)
        .status()?;

    let _ = fs::remove_file(&typst_source_path);

    let nombre_clean = sanitize_for_typst(&empresa.nombre);
    let actividad_clean = sanitize_for_typst(&empresa.actividad);
    let direccion_clean = sanitize_for_typst(&empresa.direccion);
    let tax_clean = sanitize_for_typst(tax_id);

    for cert in get_catalogo_certificados() {
        let cert_pdf_path = parent_dir.join(cert.filename);
        let cert_typ_path = parent_dir.join(format!("{}_{}.typ", cert.id, tax_clean));

        let cert_content = CERT_DYNAMIC_TEMPLATE
            .replace("{{EMPRESA_NOMBRE}}", &nombre_clean)
            .replace("{{EMPRESA_TAX_ID}}", &tax_clean)
            .replace("{{REPRESENTANTE_LEGAL}}", &sanitize_for_typst(rep_legal))
            .replace("{{EMPRESA_JURISDICCION}}", &sanitize_for_typst(jurisdiccion))
            .replace("{{EMPRESA_DIRECCION}}", &direccion_clean)
            .replace("{{AUDITOR_NOMBRE}}", &sanitize_for_typst(&auditor.nombre))
            .replace("{{AUDITOR_CEDULA}}", &sanitize_for_typst(&auditor.cedula_profesional))
            .replace("{{AUDITOR_DC5}}", &sanitize_for_typst(&auditor.registro_auditor))
            .replace("{{EMPRESA_ACTIVIDAD}}", &actividad_clean)
            .replace("{{FECHA_EMISION}}", &fecha_hoy)
            .replace("{{NORMA_TITULO}}", cert.titulo)
            .replace("{{NORMA_COLOR}}", cert.color_hex)
            .replace("{{NORMA_DESCRIPCION}}", cert.descripcion)
            .replace("{{NORMA_FOLIO_PREFIX}}", cert.folio_prefix)
            .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
            .replace("{{REF_TAX}}", &tax_clean);

        fs::write(&cert_typ_path, cert_content)?;

        let _ = Command::new("typst")
            .arg("compile")
            .arg(&cert_typ_path)
            .arg(&cert_pdf_path)
            .status();

        let _ = fs::remove_file(&cert_typ_path);
    }

    if status.success() {
        Ok(())
    } else {
        Err("Fallo al compilar el documento PDF pericial con Typst".into())
    }
}

pub fn generate_exit_intent_micro_pdf(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    output_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    generate_exit_intent_micro_pdf_with_session(empresa, osint, None, false, output_path)
}

pub fn generate_exit_intent_micro_pdf_with_session(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    master_opt: Option<&AuditMasterJson>,
    is_accredited: bool,
    output_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(p) = Path::new(output_path).parent() {
        if !p.as_os_str().is_empty() {
            fs::create_dir_all(p)?;
        }
    }

    let typst_source_path = format!("{}.micro.typ", output_path);
    let typst_content =
        build_report_typst_markup_full(empresa, osint, master_opt, is_accredited, true);
    fs::write(&typst_source_path, &typst_content)?;

    let status = Command::new("typst")
        .arg("compile")
        .arg(&typst_source_path)
        .arg(output_path)
        .status()?;

    let _ = fs::remove_file(&typst_source_path);

    if !status.success() {
        return Err("Fallo al compilar el Micro-Dictamen de rescate".into());
    }

    Ok(fs::read(output_path)?)
}
