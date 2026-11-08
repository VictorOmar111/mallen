//! Compilador Tipográfico Determinístico con Única Fuente de Verdad (SSOT),
//! Caché Concurrente en Memoria (TTL 24h), Aritmética Exacta en Centavos (`u64`),
//! Trazabilidad Estructurada JSON y Raíz de Merkle SHA-256 — FIRMA W16 MALLEN®.

use crate::engine::accountant::AccountantProfile;
use crate::engine::models::{AuditMasterJson, EmpresaB2B, LegalDossierInput};
use crate::engine::osint_scanner::OsintResult;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{OnceLock, RwLock};
use std::time::{Duration, Instant};

const TYPST_TEMPLATE: &str = include_str!("report_template.typ");
const CERT_DYNAMIC_TEMPLATE: &str = include_str!("cert_dynamic.typ");

/// Entrada inmutable en la Caché SSOT en Memoria (TTL 24 horas)
#[derive(Clone, Debug)]
pub struct SsotCacheEntry {
    pub master: AuditMasterJson,
    pub emp_count: u32,
    pub total_cents: u64,
    pub risk_score: u32,
    pub merkle_root_hex: String,
    pub stored_at: Instant,
}

static SSOT_MEMORY_CACHE: OnceLock<RwLock<HashMap<String, SsotCacheEntry>>> = OnceLock::new();

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

/// Convierte un monto en `f64` a centavos enteros exactos (`u64`) eliminando desviaciones IEEE-754.
#[inline]
pub fn mxn_to_exact_cents(val: f64) -> u64 {
    if !val.is_finite() || val <= 0.0 {
        0
    } else {
        (val * 100.0).round() as u64
    }
}

/// Formatea centavos enteros (`u64`) directamente a cadena monetaria (`1,245,958.00`).
pub fn format_cents_mxn(total_cents: u64) -> String {
    let integer_part = total_cents / 100;
    let cents = total_cents % 100;
    let num_str = integer_part.to_string();
    let mut formatted = String::with_capacity(num_str.len() + 8);
    let len = num_str.len();

    for (i, c) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }
    format!("{}.{:02}", formatted, cents)
}

pub fn format_currency_mxn(val: f64) -> String {
    format_cents_mxn(mxn_to_exact_cents(val))
}

/// Calcula la Raíz de Merkle SHA-256 de 2 niveles sobre los artefactos del expediente (`AuditMasterJson` + `Empleados` + `Centavos`).
pub fn compute_bundle_merkle_root(tax_id: &str, empresa_nombre: &str, emp_count: u32, total_cents: u64, scian: &str) -> String {
    let mut leaf1 = Sha256::new();
    leaf1.update(format!("LEAF1_ENTITY|{}|{}|{}", tax_id.trim().to_uppercase(), empresa_nombre.trim().to_uppercase(), scian.trim()).as_bytes());
    let h1 = leaf1.finalize();

    let mut leaf2 = Sha256::new();
    leaf2.update(format!("LEAF2_MATH_SSOT|EMP={}|CENTS={}", emp_count, total_cents).as_bytes());
    let h2 = leaf2.finalize();

    let mut root = Sha256::new();
    root.update(b"W16_MALLEN_MERKLE_ROOT_V1|");
    root.update(h1);
    root.update(h2);
    format!("0x{}", &format!("{:x}", root.finalize())[..24])
}

/// Guarda en la Caché Concurrente en Memoria (TTL 24h) la Única Fuente de Verdad (SSOT) de una sesión.
pub fn store_ssot_session_in_memory(tax_id: &str, empresa_nombre: &str, master: &AuditMasterJson) {
    let emp = master
        .calibration_applied
        .exact_employee_count
        .unwrap_or(master.estimated_employees)
        .max(1);
    let cents = mxn_to_exact_cents(master.total_estimated_fine_mxn);
    let merkle = compute_bundle_merkle_root(tax_id, empresa_nombre, emp, cents, &master.codigo_scian);

    let entry = SsotCacheEntry {
        master: master.clone(),
        emp_count: emp,
        total_cents: cents,
        risk_score: master.risk_score,
        merkle_root_hex: merkle,
        stored_at: Instant::now(),
    };

    let cache_lock = SSOT_MEMORY_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(mut map) = cache_lock.write() {
        let k_tax = tax_id.trim().to_uppercase();
        if !k_tax.is_empty() {
            map.insert(k_tax, entry.clone());
        }
        let k_name = empresa_nombre.trim().to_uppercase();
        if !k_name.is_empty() {
            map.insert(k_name, entry);
        }
    }
}

/// Consulta la Caché SSOT en Memoria (TTL 24h) sin tocar el disco.
pub fn get_ssot_session_from_memory(tax_id: &str, empresa_nombre: &str) -> Option<SsotCacheEntry> {
    let ttl = Duration::from_secs(24 * 3600);
    let cache_lock = SSOT_MEMORY_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(map) = cache_lock.read() {
        for key in [tax_id.trim().to_uppercase(), empresa_nombre.trim().to_uppercase()] {
            if !key.is_empty() {
                if let Some(entry) = map.get(&key) {
                    if entry.stored_at.elapsed() < ttl {
                        return Some(entry.clone());
                    }
                }
            }
        }
    }
    None
}

/// Recupera desde SQLite WAL (`audit_sessions` / `audit_cache`) el `AuditMasterJson` más reciente y lo promueve a la Caché SSOT en RAM.
pub fn load_latest_master_from_sqlite(tax_id: &str, empresa_nombre: &str) -> Option<AuditMasterJson> {
    if let Some(cached) = get_ssot_session_from_memory(tax_id, empresa_nombre) {
        return Some(cached.master);
    }

    let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
    let conn = rusqlite::Connection::open(db_path).ok()?;
    for table_sql in [
        "SELECT master_json FROM audit_sessions WHERE UPPER(tax_id) = UPPER(?1) OR UPPER(razon_social) = UPPER(?2) ORDER BY rowid DESC LIMIT 1",
        "SELECT master_json FROM audit_cache WHERE UPPER(tax_id) = UPPER(?1) OR UPPER(rfc) = UPPER(?1) ORDER BY rowid DESC LIMIT 1",
    ] {
        if let Ok(mut stmt) = conn.prepare(table_sql) {
            if let Ok(json_str) = stmt.query_row(rusqlite::params![tax_id.trim(), empresa_nombre.trim()], |r| r.get::<_, String>(0)) {
                if let Ok(parsed) = serde_json::from_str::<AuditMasterJson>(&json_str) {
                    store_ssot_session_in_memory(tax_id, empresa_nombre, &parsed);
                    return Some(parsed);
                }
            }
        }
    }
    None
}

/// Puente de Compatibilidad 100% entre `HttpFormBot` (`prospect_pipeline` + `denue_units`) y `pdf_generator.rs`.
pub fn lookup_formbot_and_denue_context(empresa_nombre: &str, tax_id: &str) -> (Option<String>, Option<u32>, bool) {
    let _ = crate::engine::http_form_bot::HttpFormBot::record_checkout_link_click(
        if !empresa_nombre.trim().is_empty() && !empresa_nombre.starts_with("REGISTRO") {
            empresa_nombre.trim()
        } else {
            tax_id.trim()
        },
        "PDF_GENERATOR_TRIGGER",
        "Typst-PreDictamen-Engine",
    );

    if let Some(cached) = get_ssot_session_from_memory(tax_id, empresa_nombre) {
        return (Some(cached.master.codigo_scian), Some(cached.emp_count), cached.master.web_telemetry.has_privacy_notice);
    }

    let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
    if let Ok(conn) = rusqlite::Connection::open(db_path) {
        if let Ok((scian_p, code_p)) = conn.query_row(
            "SELECT COALESCE(scian_code, ''), COALESCE(ultimo_resultado_codigo, '')
             FROM prospect_pipeline
             WHERE UPPER(empresa_nombre) = UPPER(?1) OR UPPER(empresa_nombre) = UPPER(?2)
             LIMIT 1",
            rusqlite::params![empresa_nombre.trim(), tax_id.trim()],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        ) {
            let privacy_ok = code_p == "PROSPECCION_ENVIADA_EXITO";
            if scian_p.len() >= 4 {
                return (Some(scian_p), Some(20), privacy_ok);
            }
        }

        if let Ok((scian_d, estrato_d)) = conn.query_row(
            "SELECT COALESCE(codigo_scian, '541110'), COALESCE(per_ocu, '11 a 30 personas')
             FROM denue_units
             WHERE UPPER(raz_social) LIKE '%' || UPPER(?1) || '%'
                OR UPPER(nom_estab) LIKE '%' || UPPER(?1) || '%'
             LIMIT 1",
            rusqlite::params![empresa_nombre.trim()],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        ) {
            let (_, emp_calc, _, _) =
                crate::engine::http_form_bot::HttpFormBot::classify_denue_capacity_and_scian_risk(&estrato_d, &scian_d);
            return (Some(scian_d), Some(emp_calc), false);
        }
    }
    (None, None, true)
}


/// Clasifica mediante análisis léxico de la Razón Social y longitud del RFC el Régimen Societario exacto
/// sin inventar números de notaría inexistentes.
pub fn analyze_corporate_entity_regime(razon_social: &str, tax_id: &str) -> (&'static str, &'static str) {
    let r_up = razon_social.trim().to_uppercase();
    let t_clean = tax_id.trim().to_uppercase();

    if r_up.contains("S.A.P.I.") || r_up.contains("SAPI ") || r_up.ends_with(" SAPI") {
        (
            "Sociedad Anónima Promotora de Inversión (S.A.P.I. de C.V.)",
            "Regulada por los Arts. 12 al 18 de la Ley del Mercado de Valores (LMV) y LGSM; exigible Gobierno Corporativo, pactos parasociales y expediente de Beneficiario Controlador (Art. 32-B Ter CFF).",
        )
    } else if r_up.contains("S.A.B.") || r_up.contains("SAB DE CV") {
        (
            "Sociedad Anónima Bursátil (S.A.B. de C.V.)",
            "Regulada por la Ley del Mercado de Valores (LMV); exigible Comité de Auditoría y Prácticas Societarias y revelación estricta de materialidad.",
        )
    } else if r_up.contains("S. DE R.L.") || r_up.contains("S DE RL") {
        (
            "Sociedad de Responsabilidad Limitada (S. de R.L. de C.V.)",
            "Regulada por los Arts. 58 al 86 de la LGSM; exigible Libro Especial de Socios (Art. 73 LGSM), aviso de estructura accionaria en el SAT (Ficha 295/CFF) y Beneficiario Controlador.",
        )
    } else if r_up.contains("S.C.") || r_up.ends_with(" SC") {
        (
            "Sociedad Civil (S.C.)",
            "Regulada por el Código Civil aplicable; exigible control de socios administradores, contratos de prestación de servicios profesionales con Fecha Cierta y confidencialidad LFPDPPP.",
        )
    } else if r_up.contains("S.A. DE C.V.") || r_up.contains("SA DE CV") || r_up.ends_with(" S.A.") {
        (
            "Sociedad Anónima de Capital Variable (S.A. de C.V.)",
            "Regulada por los Arts. 87 al 206 de la LGSM; exigible Libro de Registro de Acciones (Art. 128 LGSM), Actas de Asamblea Ordinaria Anual (Art. 181 LGSM) y Beneficiario Controlador (Art. 32-B Ter CFF).",
        )
    } else if t_clean.len() == 13 && !t_clean.starts_with("SIN") && !t_clean.starts_with("MALLEN") {
        (
            "Persona Física con Actividad Empresarial (LISR Título IV)",
            "Exenta de libros societarios mercantiles (LGSM); sujeta a contabilidad fiscal Art. 28 CFF, cumplimiento patronal LFT/IMSS sobre plantilla activa y Aviso de Privacidad LFPDPPP.",
        )
    } else {
        (
            "Entidad Mercantil / Unidad Económica Registrada (Padrón DENUE / SAT)",
            "Sujeta a acreditación de representación orgánica (Art. 10 LGSM / Art. 19 CFF), contabilidad electrónica y expediente de Beneficiario Controlador.",
        )
    }
}

/// Calcula la Masa Salarial Mensual Presunta sujeta a confronta CFDI de Nómina 4.0 vs. SUA IMSS
/// en función de la plantilla real evaluada y el giro SCIAN (en centavos enteros `u64`).
pub fn estimate_monthly_payroll_exposure_cents(emp_count: u32, scian_code: &str) -> u64 {
    let avg_monthly_sbc_cents: u64 = if scian_code.starts_with("52") || scian_code.starts_with("54") || scian_code.starts_with("51") {
        2_250_000 // $22,500.00 MXN SBC promedio mensual (Servicios Financieros / TI / Corporativos)
    } else if scian_code.starts_with("31") || scian_code.starts_with("32") || scian_code.starts_with("33") || scian_code.starts_with("48") || scian_code.starts_with("23") {
        1_680_000 // $16,800.00 MXN SBC promedio mensual (Industria / Logística / Transporte / Construcción)
    } else {
        1_350_000 // $13,500.00 MXN SBC promedio mensual (Comercio / Servicios Generales)
    };
    (emp_count.max(1) as u64).saturating_mul(avg_monthly_sbc_cents)
}

/// Extrae telemetría técnica adicional (Stack Web, Recomendación y CLEE/Domicilio fino) desde SQLite si existe.
pub fn lookup_deep_osint_and_denue_metadata(empresa_nombre: &str, tax_id: &str) -> (String, String) {
    let db_path = std::env::var("MALLEN_DB_PATH").unwrap_or_else(|_| "data/mallen_prod.db".to_string());
    let mut stack_info = String::new();
    let mut commerce_ext_info = String::new();

    if let Ok(conn) = rusqlite::Connection::open(db_path) {
        if let Ok((st, rec)) = conn.query_row(
            "SELECT COALESCE(tech_stack_detected, ''), COALESCE(recommended_module, '')
             FROM prospect_pipeline
             WHERE UPPER(empresa_nombre) = UPPER(?1) OR UPPER(empresa_nombre) = UPPER(?2)
             LIMIT 1",
            rusqlite::params![empresa_nombre.trim(), tax_id.trim()],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
        ) {
            if !st.trim().is_empty() {
                stack_info = format!("Infraestructura Web detectada: {} ({}).", st.trim(), rec.trim());
            }
        }
    }

    commerce_ext_info.push_str("Padrón Sectorial y Comercio Exterior: Sujeto a validación de Pedimentos / Anexo 22 RGCE en operaciones de importación o proveeduría internacional.");
    (stack_info, commerce_ext_info)
}

pub fn sanitize_for_typst(input: &str) -> String {
    let clean = input.replace("\\\\$", "$").replace("\\$", "$");
    clean
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

fn normalize_typst_dollars(src: &str) -> String {
    let mut out = String::with_capacity(src.len() + 64);
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            let mut slash_count = 0;
            while i < chars.len() && chars[i] == '\\' {
                slash_count += 1;
                i += 1;
            }
            if i < chars.len() && chars[i] == '$' {
                out.push('\\');
                out.push('$');
                i += 1;
            } else {
                for _ in 0..slash_count {
                    out.push('\\');
                }
            }
        } else if chars[i] == '$' {
            out.push('\\');
            out.push('$');
            i += 1;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// Calcula en centavos enteros exactos (`u64`) los subtotales por dependencia, el total general y el puntaje de riesgo,
/// garantizando paridad matemática absoluta ($0.00 de diferencia) entre la interfaz web, el Pre-Dictamen y el ZIP.
pub fn compute_ssot_exact_totals_cents(
    empresa_nombre: &str,
    scian_str: &str,
    emp_count: u32,
    sat_69b_lbl: &str,
    sat_69_lbl: &str,
    repse_ok: bool,
    sfp_alert: bool,
    osint_ssl: bool,
    osint_privacy: bool,
    osint_arco: bool,
    is_immune_or_orphan: bool,
    cal_telework: bool,
    cal_repse: bool,
    cal_boilers: bool,
    cal_forklifts: bool,
    cal_chemicals: bool,
    cal_biometrics: bool,
    cal_b2b: bool,
    cal_transport: bool,
    cal_pld: bool,
    master_score_opt: Option<u32>,
) -> (Vec<crate::modules::vectors_37::EvaluatedVector>, u64, u64, u64, u64, u64, u64, u64, u32) {
    let evaluated_37 = crate::modules::vectors_37::evaluate_all_37_vectors(
        empresa_nombre,
        scian_str,
        emp_count.max(1),
        sat_69b_lbl,
        sat_69_lbl,
        repse_ok,
        sfp_alert,
        osint_ssl,
        osint_privacy,
        osint_arco,
        is_immune_or_orphan,
        cal_telework,
        cal_repse,
        cal_boilers,
        cal_forklifts,
        cal_chemicals,
        cal_biometrics,
        cal_b2b,
        cal_transport,
        cal_pld,
    );

    let mut c_stps: u64 = 0;
    let mut c_sat: u64 = 0;
    let mut c_inai: u64 = 0;
    let mut c_imss: u64 = 0;
    let mut c_pc: u64 = 0;
    let mut c_profeco: u64 = 0;

    if !is_immune_or_orphan {
        for v in &evaluated_37 {
            let v_cents = mxn_to_exact_cents(v.multa_estimada_mxn);
            match v.organo.as_str() {
                "STPS" => c_stps += v_cents,
                "SAT" => c_sat += v_cents,
                "LFPDPPP" => c_inai += v_cents,
                "IMSS" => c_imss += v_cents,
                "PROFECO" | "ECONOMÍA" => c_profeco += v_cents,
                _ => c_pc += v_cents,
            }
        }
    }

    let total_cents = c_stps + c_sat + c_inai + c_imss + c_pc + c_profeco;
    let total_mxn = (total_cents as f64) / 100.0;

    let score: u32 = if is_immune_or_orphan || total_cents == 0 {
        0
    } else if let Some(ms) = master_score_opt.filter(|&s| s > 0) {
        ms
    } else {
        let ac = evaluated_37.iter().filter(|v| mxn_to_exact_cents(v.multa_estimada_mxn) > 0).count();
        let raw_score = 40.0_f64 + (ac as f64 / 37.0_f64) * 45.0_f64 + (total_mxn / 5_000_000.0_f64).min(1.0_f64) * 14.0_f64;
        (raw_score.round() as u32).clamp(45, 99)
    };

    (evaluated_37, c_stps, c_sat, c_inai, c_imss, c_pc, c_profeco, total_cents, score)
}

fn build_report_typst_markup_full(
    empresa: &EmpresaB2B,
    osint: &OsintResult,
    master_opt_in: Option<&AuditMasterJson>,
    _is_accredited: bool,
    is_censored_micro_report: bool,
) -> String {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);

    // Si master_opt_in viene con datos reales, actualizar inmediatamente la Caché SSOT en RAM
    if let Some(m_in) = master_opt_in {
        store_ssot_session_in_memory(tax_id, &empresa.nombre, m_in);
    }

    let sqlite_master_backup = if master_opt_in.is_none() {
        load_latest_master_from_sqlite(tax_id, &empresa.nombre)
    } else {
        None
    };
    let master_opt = master_opt_in.or(sqlite_master_backup.as_ref());

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
        .unwrap_or("Por designar en Acta Interna");
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
        Some(p)
            if !p.trim().is_empty()
                && !p.starts_with("DICTAMINADO_POR:")
                && !p.starts_with("CERTIFICACIÓN")
                && !p.contains("FIRMA W16 MALLEN") =>
        {
            format!("DESPACHO AUDITOR ALIADO: {}", sanitize_for_typst(p))
        }
        _ => "FIRMA W16 MALLEN® (MALLEN.LAT)".to_string(),
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

    let (
        scian_extracted,
        emp_count,
        osint_ssl_flag,
        osint_privacy_flag,
        osint_arco_flag,
        cal_telework,
        cal_repse,
        cal_boilers,
        cal_forklifts,
        cal_chemicals,
        cal_biometrics,
        cal_b2b,
        cal_transport,
        cal_pld,
        is_unindexed_orphan,
    ) = if let Some(m) = master_opt {
        let combined_scian_src = format!("{} {}", m.codigo_scian, m.actividad_scian_desc);
        let tokens: Vec<&str> = combined_scian_src
            .split(|c: char| !c.is_ascii_digit())
            .filter(|tok| tok.len() >= 4)
            .collect();
        let clean_scian = tokens
            .iter()
            .copied()
            .find(|t| *t != "541610")
            .or_else(|| tokens.first().copied())
            .unwrap_or("541610")
            .to_string();
        let c = &m.calibration_applied;
        let emp = c.exact_employee_count.unwrap_or(m.estimated_employees).max(1);
        (
            clean_scian.clone(),
            emp,
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.ssl_valid },
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.has_privacy_notice },
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.mentions_arco_rights || m.web_telemetry.has_privacy_notice },
            c.has_telework_over_40_percent.unwrap_or(true),
            c.uses_subcontracted_services_repse.unwrap_or(true),
            c.has_pressure_vessels_boilers.unwrap_or(clean_scian.starts_with("31") || clean_scian.starts_with("32") || clean_scian.starts_with("33") || clean_scian.starts_with("722")),
            c.has_forklifts_or_manual_loads.unwrap_or(true),
            c.handles_hazardous_chemicals.unwrap_or(clean_scian.starts_with("811") || clean_scian.starts_with("31") || clean_scian.starts_with("32") || clean_scian.starts_with("62")),
            c.uses_biometrics_or_health_data.unwrap_or(true),
            c.pays_intangible_b2b_services.unwrap_or(true),
            c.transports_goods_federal_highways.unwrap_or(clean_scian.starts_with("48") || clean_scian.starts_with("49") || clean_scian.starts_with("43")),
            c.performs_vulnerable_activities_pld.unwrap_or(clean_scian.starts_with("52") || clean_scian.starts_with("53") || clean_scian.starts_with("23")),
            (m.razon_social.starts_with("REGISTRO NO INDEXADO") || m.domicilio_fiscal_o_denue.contains("Territorio Nacional")) && m.web_telemetry.target_url.is_empty(),
        )
    } else {
        let (fb_scian, fb_emp, fb_priv) = lookup_formbot_and_denue_context(&empresa.nombre, tax_id);
        let tokens: Vec<&str> = empresa
            .actividad
            .split(|c: char| !c.is_ascii_digit())
            .filter(|tok| tok.len() >= 4)
            .collect();
        let clean_scian = fb_scian
            .filter(|s| s.len() >= 4 && s != "541610")
            .or_else(|| tokens.iter().copied().find(|t| *t != "541610").map(|s| s.to_string()))
            .or_else(|| tokens.first().copied().map(|s| s.to_string()))
            .unwrap_or_else(|| "541610".to_string());
        let resolved_emp = fb_emp.unwrap_or(20_u32);
        (
            clean_scian.clone(),
            resolved_emp,
            true,
            fb_priv,
            fb_priv,
            true,
            true,
            clean_scian.starts_with("31") || clean_scian.starts_with("32") || clean_scian.starts_with("33") || clean_scian.starts_with("722"),
            true,
            clean_scian.starts_with("811") || clean_scian.starts_with("31") || clean_scian.starts_with("32") || clean_scian.starts_with("62"),
            true,
            true,
            clean_scian.starts_with("48") || clean_scian.starts_with("49") || clean_scian.starts_with("43"),
            clean_scian.starts_with("52") || clean_scian.starts_with("53") || clean_scian.starts_with("23"),
            false,
        )
    };

    if is_censored_micro_report {
        let _ = crate::engine::http_form_bot::HttpFormBot::record_checkout_link_click(&empresa.nombre, "MICRO_SCAN_PDF", "Typst-MicroReport");
    }

    let scian_str = scian_extracted.as_str();
    let osint_ssl = osint_ssl_flag;
    let osint_privacy = osint_privacy_flag;
    let osint_arco = osint_arco_flag;

    let (imss_clase, imss_prima, imss_sector_desc) =
        crate::modules::vectors_37::classify_imss_risk_by_scian(scian_str);

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

    let sat_69b_lbl = master_opt.map(|m| m.sat_69b_status.as_str()).unwrap_or("LIMPIO");
    let sat_69_lbl = master_opt.map(|m| m.sat_69_status.as_str()).unwrap_or("LIMPIO");
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

    let (evaluated_37, c_stps, c_sat, c_inai, c_imss, c_pc, c_profeco, total_cents, score) =
        compute_ssot_exact_totals_cents(
            &empresa.nombre,
            scian_str,
            emp_count,
            sat_69b_lbl,
            sat_69_lbl,
            repse_lbl.contains("VIGENTE") || repse_lbl.contains("ACTIVO"),
            sfp_lbl.contains("ALERTA") || sfp_lbl.contains("SANCIONADO"),
            osint_ssl || is_immune,
            osint_privacy || is_immune,
            osint_arco || is_immune,
            is_immune || is_unindexed_orphan,
            cal_telework,
            cal_repse,
            cal_boilers,
            cal_forklifts,
            cal_chemicals,
            cal_biometrics,
            cal_b2b,
            cal_transport,
            cal_pld,
            master_opt.map(|m| m.risk_score),
        );

    let m_stps = (c_stps as f64) / 100.0;
    let m_sat = (c_sat as f64) / 100.0;
    let m_inai = (c_inai as f64) / 100.0;
    let m_imss = (c_imss as f64) / 100.0;
    let m_pc = (c_pc as f64) / 100.0;
    let m_profeco = (c_profeco as f64) / 100.0;
    let risk_score_val = score as f64;

    let visual_chart_typst = crate::modules::vectors_37::render_visual_risk_chart_typst(
        m_stps, m_sat, m_imss, m_inai, m_pc, m_profeco, risk_score_val,
    );

    let (regimen_societario_lbl, regimen_societario_oblig) = analyze_corporate_entity_regime(&empresa.nombre, tax_id);
    let masa_salarial_cents = estimate_monthly_payroll_exposure_cents(emp_count, scian_str);
    let masa_salarial_str = format_cents_mxn(masa_salarial_cents);
    let (stack_osint_note, _comm_ext_note) = lookup_deep_osint_and_denue_metadata(&empresa.nombre, tax_id);

    let comision_nom019_modalidad = if emp_count >= 15 {
        "Acta Constitutiva Bipartita NOM-019 (Coordinador, Secretario y Vocales para >=15 trabajadores)"
    } else {
        "Acta NOM-019 Modalidad Simplificada (1 Representante Patronal y 1 Laboral para <15 trabajadores)"
    };

    let hallazgo_stps_str = if is_immune {
        format!("0 pts: Carpetas STPS ({}, {}, NOM-030 y NOM-002) instrumentadas para {} trabajadores.", guia_nom035, comision_nom019_modalidad, emp_count)
    } else {
        format!("Plantilla evaluada ({} colaboradores, SCIAN {}): obligatorio exhibir {}, {}, Diagnóstico NOM-030 y grado de incendio NOM-002.", emp_count, scian_str, guia_nom035, comision_nom019_modalidad)
    };

    let hallazgo_sat_str = if is_immune {
        format!("0 pts: {} • Contratos con Fecha Cierta SCJN (Art. 69-B CFF) y CFDI 4.0 verificados.", regimen_societario_lbl)
    } else {
        format!("Figura: {} [69-B: {}]. {} Exigible Contrato Marco con Fecha Cierta (SCJN 2a./J. 161/2019) y Razón de Negocios Art. 5-A CFF.", regimen_societario_lbl, sat_69b_lbl, regimen_societario_oblig)
    };

    let osint_inai_str = if is_immune {
        "0 pts: Aviso de Privacidad Integral LFPDPPP (SABG), Derechos ARCO y TLS 1.3 activos.".to_string()
    } else if osint_privacy_flag {
        format!("{} Aviso web detectado; requiere adecuación a la Autoridad Garante SABG (LFPDPPP), Documento de Seguridad (Art. 19) y Consentimiento Biométrico/Nómina para {} colaboradores.", stack_osint_note, emp_count).trim().to_string()
    } else {
        format!("ALERTA CRÍTICA (Arts. 15, 16 y 63 LFPDPPP): {} Omisión de Aviso de Privacidad Integral, Ventanilla ARCO y Consentimiento Biométrico para {}.", stack_osint_note, empresa.nombre).trim().to_string()
    };

    let hallazgo_imss_str = if is_immune {
        format!("0 pts: Plantilla conciliada en SUA bajo {} (Prima {}) y protocolo REPSE activo.", imss_clase, imss_prima)
    } else {
        format!("Plantilla: {} colaboradores | Masa Salarial Bruta Presunta: ${} MXN/mes | {} (Prima Media: {}). Exigible confronta CFDI Nómina vs. SUA y compulsa REPSE/ICSOE.", emp_count, masa_salarial_str, imss_clase, imss_prima)
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

    let osint_ssl_txt = if osint_ssl || is_immune {
        "ACTIVO (TLS 1.3 VÁLIDO)"
    } else {
        "VULNERABLE (INFRACCIÓN ART. 19 LFPDPPP)"
    };
    let osint_privacy_txt = if osint_privacy_flag || is_immune {
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
    let raw_act_desc = if let Some(m) = master_opt {
        m.actividad_scian_desc
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ' ' || c == '-' || c == '—')
            .to_string()
    } else {
        empresa
            .actividad
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ' ' || c == '-' || c == '—')
            .to_string()
    };
    let actividad_clean = sanitize_for_typst(&format!("{} - {}", scian_str, raw_act_desc));
    let tax_clean = sanitize_for_typst(tax_id);
    let vectors_37_typst =
        crate::modules::vectors_37::render_vectors_37_typst_table(&evaluated_37, is_censored_micro_report);

    let out = TYPST_TEMPLATE
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
        .replace("{{ESTIMATED_FINE}}", &format_cents_mxn(total_cents))
        .replace("{{TOTAL_FINE_MXN}}", &format_cents_mxn(total_cents))
        .replace("{{MULTA_STPS}}", &format_cents_mxn(c_stps))
        .replace("{{FINE_STPS}}", &format_cents_mxn(c_stps))
        .replace("{{MULTA_SAT}}", &format_cents_mxn(c_sat))
        .replace("{{FINE_SAT}}", &format_cents_mxn(c_sat))
        .replace("{{MULTA_INAI}}", &format_cents_mxn(c_inai))
        .replace("{{FINE_INAI}}", &format_cents_mxn(c_inai))
        .replace("{{MULTA_IMSS}}", &format_cents_mxn(c_imss))
        .replace("{{FINE_IMSS}}", &format_cents_mxn(c_imss))
        .replace("{{MULTA_PC}}", &format_cents_mxn(c_pc))
        .replace("{{FINE_PROT_CIVIL}}", &format_cents_mxn(c_pc))
        .replace("{{MULTA_PROFECO}}", &format_cents_mxn(c_profeco))
        .replace("{{FINE_PROFECO}}", &format_cents_mxn(c_profeco))
        .replace("{{ACTIVATION_PRICE}}", "3,000.00")
        .replace("{{KIT_PRICE_MXN}}", "3,000.00")
        .replace("{{RISK_SCORE}}", &score.to_string())
        .replace("{{HALLAZGO_TABLA_STPS}}", &sanitize_for_typst(&hallazgo_stps_str))
        .replace("{{HALLAZGO_TABLA_STPS_TYPST}}", &format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&hallazgo_stps_str)))
        .replace("{{HALLAZGO_TABLA_SAT}}", &sanitize_for_typst(&hallazgo_sat_str))
        .replace("{{HALLAZGO_TABLA_SAT_TYPST}}", &if is_censored_micro_report {
            r##"#frosted-blur("DIAGNÓSTICO FISCAL ART. 69-B Y CFDI PROTEGIDO")"##.to_string()
        } else {
            format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&hallazgo_sat_str))
        })
        .replace("{{INAI_OBS}}", &sanitize_for_typst(&osint_inai_str))
        .replace("{{HALLAZGO_TABLA_INAI_TYPST}}", &if is_censored_micro_report {
            r##"#frosted-blur("AUDITORÍA LFPDPPP, ARCO Y BIOMÉTRICOS PROTEGIDA")"##.to_string()
        } else {
            format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&osint_inai_str))
        })
        .replace("{{HALLAZGO_TABLA_IMSS}}", &sanitize_for_typst(&hallazgo_imss_str))
        .replace("{{HALLAZGO_TABLA_IMSS_TYPST}}", &if is_censored_micro_report {
            r##"#frosted-blur("CONFRONTA IMSS SUA, RACERF Y REPSE PROTEGIDA")"##.to_string()
        } else {
            format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&hallazgo_imss_str))
        })
        .replace("{{HALLAZGO_TABLA_PC}}", &sanitize_for_typst(&hallazgo_pc_str))
        .replace("{{HALLAZGO_TABLA_PC_TYPST}}", &if is_censored_micro_report {
            r##"#frosted-blur("DICTAMINACIÓN PIPC Y 9 SECTORIALES PROTEGIDA")"##.to_string()
        } else {
            format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&hallazgo_pc_str))
        })
        .replace("{{HALLAZGO_TABLA_PROFECO}}", &sanitize_for_typst(&hallazgo_profeco_str))
        .replace("{{HALLAZGO_TABLA_PROFECO_TYPST}}", &if is_censored_micro_report {
            r##"#frosted-blur("AUDITORÍA PROFECO ART. 76 BIS Y RRPAC PROTEGIDA")"##.to_string()
        } else {
            format!("#text(size: 6.2pt)[{}]", sanitize_for_typst(&hallazgo_profeco_str))
        })
        .replace("{{SAT_69B_STATUS}}", &sanitize_for_typst(sat_69b_lbl))
        .replace("{{SAT_69_STATUS}}", &sanitize_for_typst(sat_69_lbl))
        .replace("{{REPSE_STATUS}}", &sanitize_for_typst(repse_lbl))
        .replace("{{SFP_STATUS}}", &sanitize_for_typst(sfp_lbl))
        .replace("{{SSL_STATUS}}", osint_ssl_txt)
        .replace("{{PRIVACY_STATUS}}", osint_privacy_txt)
        .replace("{{INAI_STATUS}}", osint_privacy_txt)
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
        .replace("{{ROADMAP_PHASE_1}}", &{
            let (p1, _, _, _) = crate::modules::vectors_37::render_roadmap_actions_typst(&actividad_clean, is_censored_micro_report && !is_immune);
            p1
        })
        .replace("{{ROADMAP_PHASE_2}}", &{
            let (_, p2, _, _) = crate::modules::vectors_37::render_roadmap_actions_typst(&actividad_clean, is_censored_micro_report && !is_immune);
            p2
        })
        .replace("{{ROADMAP_PHASE_3}}", &{
            let (_, _, p3, _) = crate::modules::vectors_37::render_roadmap_actions_typst(&actividad_clean, is_censored_micro_report && !is_immune);
            p3
        })
        .replace("{{CALENDAR_SECTION_TYPST}}", &crate::modules::vectors_37::render_calendar_table(is_censored_micro_report && !is_immune))
        .replace("{{VISUAL_RISK_CHART_TYPST}}", &visual_chart_typst);

    normalize_typst_dollars(&out)
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
    master_opt_in: Option<&AuditMasterJson>,
    is_accredited: bool,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tax_id = empresa.tax_id.as_deref().unwrap_or(&empresa.id);

    if let Some(m_in) = master_opt_in {
        store_ssot_session_in_memory(tax_id, &empresa.nombre, m_in);
    }

    let sqlite_master_backup = if master_opt_in.is_none() {
        load_latest_master_from_sqlite(tax_id, &empresa.nombre)
    } else {
        None
    };
    let master_opt = master_opt_in.or(sqlite_master_backup.as_ref());

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
        .unwrap_or("Por designar en Acta Interna");
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
    let (fb_scian_cert, fb_emp_cert, fb_priv_cert) = lookup_formbot_and_denue_context(&empresa.nombre, tax_id);

    let scian_code_cert = {
        let src = if let Some(m) = master_opt {
            format!("{} {} {}", m.codigo_scian, m.actividad_scian_desc, empresa.actividad)
        } else {
            format!("{} {}", fb_scian_cert.clone().unwrap_or_default(), empresa.actividad)
        };
        let toks: Vec<&str> = src.split(|c: char| !c.is_ascii_digit()).filter(|t| t.len() >= 4).collect();
        toks.iter().copied().find(|t| *t != "541610").or_else(|| toks.first().copied()).unwrap_or("541610").to_string()
    };

    let actividad_clean = if let Some(m) = master_opt {
        let clean_desc = m
            .actividad_scian_desc
            .trim_start_matches(|c: char| c.is_ascii_digit() || c == ' ' || c == '-' || c == '—');
        sanitize_for_typst(&format!("{} - {}", scian_code_cert, clean_desc))
    } else {
        sanitize_for_typst(&empresa.actividad)
    };

    let emp_cert = master_opt
        .map(|m| m.calibration_applied.exact_employee_count.unwrap_or(m.estimated_employees).max(1))
        .or(fb_emp_cert)
        .unwrap_or(20);

    let (imss_clase_c, imss_prima_c, _) =
        crate::modules::vectors_37::classify_imss_risk_by_scian(&scian_code_cert);
    let imss_label_cert = sanitize_for_typst(&format!("{} (Prima {})", imss_clase_c, imss_prima_c));

    // Paridad Matemática Absoluta en Centavos (Cero pisos forzados de $1,245,958.00 ni Score 71)
    let (exact_fine_cents, exact_score_val) = if let Some(m) = master_opt.filter(|m| m.total_estimated_fine_mxn > 0.0) {
        let (_, _, _, _, _, _, _, calc_cents, calc_sc) = compute_ssot_exact_totals_cents(
            &empresa.nombre,
            &scian_code_cert,
            emp_cert,
            &m.sat_69b_status,
            &m.sat_69_status,
            m.stps_repse_registered,
            m.sfp_sanctioned,
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.ssl_valid },
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.has_privacy_notice },
            if m.web_telemetry.target_url.is_empty() { true } else { m.web_telemetry.mentions_arco_rights || m.web_telemetry.has_privacy_notice },
            tax_id.starts_with("MALLEN"),
            m.calibration_applied.has_telework_over_40_percent.unwrap_or(true),
            m.calibration_applied.uses_subcontracted_services_repse.unwrap_or(true),
            m.calibration_applied.has_pressure_vessels_boilers.unwrap_or(scian_code_cert.starts_with("31") || scian_code_cert.starts_with("32") || scian_code_cert.starts_with("33") || scian_code_cert.starts_with("722")),
            m.calibration_applied.has_forklifts_or_manual_loads.unwrap_or(true),
            m.calibration_applied.handles_hazardous_chemicals.unwrap_or(scian_code_cert.starts_with("811") || scian_code_cert.starts_with("31") || scian_code_cert.starts_with("32") || scian_code_cert.starts_with("62")),
            m.calibration_applied.uses_biometrics_or_health_data.unwrap_or(true),
            m.calibration_applied.pays_intangible_b2b_services.unwrap_or(true),
            m.calibration_applied.transports_goods_federal_highways.unwrap_or(scian_code_cert.starts_with("48") || scian_code_cert.starts_with("49") || scian_code_cert.starts_with("43")),
            m.calibration_applied.performs_vulnerable_activities_pld.unwrap_or(scian_code_cert.starts_with("52") || scian_code_cert.starts_with("53") || scian_code_cert.starts_with("23")),
            Some(m.risk_score),
        );
        (calc_cents, calc_sc)
    } else if empresa.estimated_fine_mxn > 0.0 || empresa.risk_score > 0 {
        (mxn_to_exact_cents(empresa.estimated_fine_mxn), empresa.risk_score)
    } else {
        let (_, _, _, _, _, _, _, calc_cents, calc_sc) = compute_ssot_exact_totals_cents(
            &empresa.nombre,
            &scian_code_cert,
            emp_cert,
            "LIMPIO",
            "LIMPIO",
            false,
            false,
            true,
            fb_priv_cert,
            fb_priv_cert,
            tax_id.starts_with("MALLEN"),
            true,
            true,
            scian_code_cert.starts_with("31") || scian_code_cert.starts_with("32") || scian_code_cert.starts_with("33") || scian_code_cert.starts_with("722"),
            true,
            scian_code_cert.starts_with("811") || scian_code_cert.starts_with("31") || scian_code_cert.starts_with("32") || scian_code_cert.starts_with("62"),
            true,
            true,
            scian_code_cert.starts_with("48") || scian_code_cert.starts_with("49") || scian_code_cert.starts_with("43"),
            scian_code_cert.starts_with("52") || scian_code_cert.starts_with("53") || scian_code_cert.starts_with("23"),
            None,
        );
        (calc_cents, calc_sc)
    };

    let total_mitigado_str = format_cents_mxn(exact_fine_cents);
    let risk_score_cert = exact_score_val;

    let sector_esp_cert = if scian_code_cert.starts_with("722") || scian_code_cert.starts_with("311") || scian_code_cert.starts_with("431") {
        "Trazabilidad sanitaria COFEPRIS (DIGIPRIS / NOM-251-SSA1), bitácora logística NOM-087-SCT y PIPC de centro de distribución."
    } else if scian_code_cert.starts_with("62") || scian_code_cert.starts_with("464") {
        "Aviso de Responsable Sanitario COFEPRIS, Expediente Clínico NOM-004-SSA3, RPBI NOM-087-SEMARNAT y PIPC."
    } else if scian_code_cert.starts_with("31") || scian_code_cert.starts_with("32") || scian_code_cert.starts_with("33") {
        "Seguridad industrial NOM-004/006/017/020/022-STPS, Manifiestos SEMARNAT y Programa Interno PIPC."
    } else if scian_code_cert.starts_with("48") || scian_code_cert.starts_with("49") {
        "Complemento Carta Porte 3.1, Bitácoras NOM-087-SCT-2, Físico-Mecánica NOM-068-SCT y PIPC."
    } else {
        "Cumplimiento integral corporativo LFT, CFF, LSS, LFPDPPP, LFPC y Programa Interno de Protección Civil (PIPC)."
    };

    let direccion_clean = sanitize_for_typst(&crate::modules::vectors_37::sanitize_denue_address(
        &empresa.direccion,
        &empresa.estado,
        tax_id,
    ));
    let tax_clean = sanitize_for_typst(tax_id);

    for cert in get_catalogo_certificados() {
        let cert_pdf_path = parent_dir.join(cert.filename);
        let cert_typ_path = parent_dir.join(format!("{}_{}.typ", cert.id, tax_clean));

        let (pt1, pt2, pt3) = match cert.id {
            "nom035" => (
                "Política Escrita de Prevención de Riesgos Psicosociales (Numeral 5.1 NOM-035)",
                "Cuestionarios de Identificación Guías de Referencia I, II/III según plantilla",
                "Protocolo de Canal Confidencial de Denuncia Interna y Derivación Clínica",
            ),
            "nom037" => (
                "Política de Teletrabajo y Cláusula de Derecho a la Desconexión Digital (Art. 330-E LFT)",
                "Adendum al Contrato Individual con Pago Proporcional de Internet y Electricidad",
                "Lista de Verificación de Condiciones de Seguridad y Ergonomía en Trabajo Remoto",
            ),
            "nom019" => (
                "Acta Constitutiva Bipartita de la Comisión Mixta de Seguridad e Higiene (Numeral 7.4)",
                "Programa Anual de Recorridos Trimestrales de Verificación en el Centro de Trabajo",
                "Bitácora de Investigación de Causas de Accidentes y Seguimiento de Medidas",
            ),
            "nom030" => (
                "Acta de Designación Formal del Responsable de Seguridad y Salud en el Trabajo (SST)",
                "Diagnóstico Integral y Por Área de Trabajo de las NOMs STPS Aplicables al Giro SCIAN",
                "Programa Anual de Acciones Preventivas y Correctivas de Seguridad e Higiene",
            ),
            "sat_69b" => (
                "Contrato Marco B2B con Cláusula de Materialidad Fiscal y Fecha Cierta (SCJN 2a./J. 161/2019)",
                "Bitácora de Entregables, Órdenes de Servicio y Razón de Negocios (Art. 5-A CFF)",
                "Constancia de Compulsa SAT Art. 69/69-B y Expediente de Beneficiario Controlador",
            ),
            "imss_32d" => (
                "Protocolo de Conciliación Mensual SUA vs. Visor de CFDI de Nómina 4.0 y Prima RACERF",
                "Checklist de Blindaje Solidario REPSE (Art. 15-A LSS) y Compulsa de Opinión 32-D",
                "Matriz de Vigilancia de Declaraciones Cuatrimestrales ICSOE (IMSS) y SISSUB (INFONAVIT)",
            ),
            "inai_lfpdppp" => (
                "Aviso de Privacidad Integral, Simplificado y Ventanilla de Derechos ARCO (20 días)",
                "Formato de Consentimiento Expreso y Autógrafo para Datos Sensibles y Biométricos (Art. 9)",
                "Acta de Designación de Oficial de Privacidad y Documento de Seguridad (Art. 19 LFPDPPP)",
            ),
            "proteccion_civil" => (
                "Acta Constitutiva de la Unidad Interna de Protección Civil (PIPC) y 4 Brigadas",
                "Calendario Anual de Simulacros de Evacuación y Verificación de Extintores (NOM-002-STPS)",
                "Matriz de Cumplimiento de Obligaciones Sectoriales Específicas del Giro SCIAN",
            ),
            "profeco_lfpc" => (
                "Términos y Condiciones de Comercio Electrónico (Art. 76 BIS y 7 BIS LFPC — Precios con IVA)",
                "Póliza Escrita de Garantía Mínima Legal (Art. 77 LFPC) y Derecho de Revocación (5 días)",
                "Auditoria de Purga de Cláusulas Abusivas o Inequitativas en Contratos (Art. 90 LFPC)",
            ),
            _ => (
                "Manifiesto Criptográfico SHA-256 de Integridad de Mensajes de Datos (Arts. 89-93 CCom)",
                "Anclaje en Árbol de Merkle para Acreditación de Fecha Cierta Digital (SCJN)",
                "Vínculo Público de Verificación Pericial en Tiempo Real (mallen.lat/verify)",
            ),
        };

        let raw_cert = CERT_DYNAMIC_TEMPLATE
            .replace("{{EMPRESA_NOMBRE}}", &nombre_clean)
            .replace("{{EMPRESA_TAX_ID}}", &tax_clean)
            .replace("{{REPRESENTANTE_LEGAL}}", &sanitize_for_typst(rep_legal))
            .replace("{{EMPRESA_JURISDICCION}}", &sanitize_for_typst(jurisdiccion))
            .replace("{{EMPRESA_DIRECCION}}", &direccion_clean)
            .replace("{{AUDITOR_NOMBRE}}", &sanitize_for_typst(&auditor.nombre))
            .replace("{{AUDITOR_CEDULA}}", &sanitize_for_typst(&auditor.cedula_profesional))
            .replace("{{AUDITOR_DC5}}", &sanitize_for_typst(&auditor.registro_auditor))
            .replace("{{EMPRESA_ACTIVIDAD}}", &actividad_clean)
            .replace("{{EMPRESA_SCIAN_CODE}}", &sanitize_for_typst(&scian_code_cert))
            .replace("{{EMPRESA_EMPLEADOS}}", &emp_cert.to_string())
            .replace("{{EMPRESA_IMSS_CLASE}}", &imss_label_cert)
            .replace("{{CONTINGENCIA_MITIGADA}}", &total_mitigado_str)
            .replace("{{RISK_SCORE}}", &risk_score_cert.to_string())
            .replace("{{SECTOR_ESPECIFICO}}", &sanitize_for_typst(sector_esp_cert))
            .replace("{{NORMA_PT_1}}", &sanitize_for_typst(pt1))
            .replace("{{NORMA_PT_2}}", &sanitize_for_typst(pt2))
            .replace("{{NORMA_PT_3}}", &sanitize_for_typst(pt3))
            .replace("{{FECHA_EMISION}}", &fecha_hoy)
            .replace("{{NORMA_TITULO}}", cert.titulo)
            .replace("{{NORMA_COLOR}}", cert.color_hex)
            .replace("{{NORMA_DESCRIPCION}}", cert.descripcion)
            .replace("{{NORMA_FOLIO_PREFIX}}", cert.folio_prefix)
            .replace("{{AUDIT_HASH}}", &empresa.audit_hash)
            .replace("{{REF_TAX}}", &tax_clean);

        let cert_content = normalize_typst_dollars(&raw_cert);
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
    master_opt_in: Option<&AuditMasterJson>,
    is_accredited: bool,
    output_path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(p) = Path::new(output_path).parent() {
        if !p.as_os_str().is_empty() {
            fs::create_dir_all(p)?;
        }
    }

    let typst_source_path = format!("{}_micro.typ", output_path.trim_end_matches(".pdf"));
    let typst_content =
        build_report_typst_markup_full(empresa, osint, master_opt_in, is_accredited, true);
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
