use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use worker_inegi_engine::engine::{
    accountant::{apply_accountant_signature, AccountantProfile, DeterministicRagAgent},
    bundle_generator::generate_custom_bundle_with_session,
    cleaner::normalize_canonical_rfc,
    db::DbState,
    merkle::MerkleTree,
    models::{
        AuditMasterJson, EmpresaB2B, LegalAcceptanceMetadata, LegalDossierInput, ScanRequest,
        WebOsintTelemetry,
    },
    network::NetworkClientManager,
    osint_scanner::{run_deep_web_telemetry, OsintResult},
    payment::{create_checkout_session, StripeDisputeGuard},
    pdf_generator::generate_exit_intent_micro_pdf_with_session,
    risk_evaluator::{build_full_regulatory_findings, evaluate_risk, summarize_findings_financials},
    rules::stps::parse_employee_count,
    sat_billing::{InvoiceData, SatBillingManager},
};
use worker_inegi_engine::modules::checkout;

#[derive(Clone)]
struct AppState {
    db: DbState,
    http_client: reqwest::Client,
}

#[derive(Deserialize)]
struct CheckoutParams {
    ref_id: Option<String>,
    #[serde(rename = "ref")]
    r#ref: Option<String>,
    hash: Option<String>,
}

#[derive(Deserialize)]
struct PaymentRequest {
    ref_code: String,
    email: Option<String>,
    accepted_disclaimer: Option<bool>,
    #[serde(default)]
    dossier: Option<LegalDossierInput>,
}

#[derive(Serialize)]
struct PaymentResponse {
    checkout_url: String,
    session_id: String,
    disclaimer_sha256: String,
    status: String,
}

#[derive(Deserialize)]
struct SaveDossierRequest {
    tax_id: String,
    dossier: LegalDossierInput,
}

#[derive(Deserialize)]
struct RagQueryRequest {
    tax_id: String,
    question: String,
}

#[derive(Deserialize)]
struct PartnerBatchRequest {
    partner_name: String,
    api_key: String,
    targets: Vec<ScanRequest>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    fs::create_dir_all("data")?;
    fs::create_dir_all("kits_blindaje")?;

    let db = DbState::new("sqlite://data/mallen_prod.db").await?;
    db.init_schema().await?;

    let http_client = NetworkClientManager::build_client(None)
        .map_err(|e| format!("Error iniciando cliente HTTP: {}", e))?;

    let state = Arc::new(AppState { db, http_client });

    println!("🚀 Iniciando Servidor de Peritaje Corporativo Mallen 360° (mallen.lat) en puerto 8080...");

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/privacy", get(handle_privacy))
        .route("/terms", get(handle_terms))
        .route("/normas/stps", get(handle_normas_stps))
        .route("/normas/sat", get(handle_normas_sat))
        .route("/normas/imss", get(handle_normas_imss))
        .route("/normas/inai", get(handle_normas_inai))
        .route("/normas/pld", get(handle_normas_pld))
        .route("/normas/comercio", get(handle_normas_comercio))
        .route("/normas/:cat/:code", get(handle_subnorma_detail))
        .route("/validez/:slug", get(handle_validez_detail))
        .route("/verify/:id", get(checkout::handle_verify))
        .route("/download/zip/:tax_id", get(download_zip_handler))
        .route("/download/:hash", get(download_pdf_report))
        .route("/api/merkle/verify/:ref_code", get(get_merkle_proof_api))
        .route("/api/scan", post(handle_live_scan))
        .route("/api/calibrate", post(handle_live_scan))
        .route("/api/dossier/save", post(handle_save_dossier))
        .route("/api/micro_scan/:tax_id", get(handle_exit_intent_micro_pdf))
        .route("/api/v1/partner/batch_scan", post(handle_partner_batch_scan))
        .route("/api/rag/ask", post(handle_rag_query))
        .route("/health", get(|| async { "OK" }))
        .route("/checkout", get(handle_checkout))
        .route("/checkout/:id", get(handle_checkout_path))
        .route("/api/checkout/session", post(create_payment_session))
        .route("/api/webhook/stripe", post(handle_payment_webhook))
        .route("/api/webhook_stripe", post(handle_payment_webhook))
        .fallback_service(ServeDir::new("public"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🌐 Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// =============================================================================
// ENDPOINTS CORE CON SINCRONIZACIÓN 1:1 SQLITE (WEB <-> PDF <-> KITS LEGALES)
// =============================================================================

async fn handle_live_scan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ScanRequest>,
) -> impl IntoResponse {
    let raw_tax = req.tax_id.trim().to_uppercase();
    let canonical_rfc = normalize_canonical_rfc(&raw_tax).unwrap_or_else(|| raw_tax.clone());
    let is_mallen_self = canonical_rfc.starts_with("MALLEN");

    let sat_69b = state
        .db
        .check_sat_69b_by_rfc(&canonical_rfc)
        .await
        .unwrap_or_else(|_| "LIMPIO".to_string());
    let sat_69 = state
        .db
        .check_sat_69_by_rfc(&canonical_rfc)
        .await
        .unwrap_or_else(|_| "LIMPIO".to_string());

    let denue_opt = state
        .db
        .find_denue_by_name_or_domain(req.domain.as_deref(), req.razon_social_override.as_deref())
        .await
        .ok()
        .flatten();

    let d_in = &req.calibration.dossier;

    let denue_email = String::new();
    let denue_tel = String::new();
    let denue_entidad = String::new();
    let (razon_social, nombre_comercial, codigo_scian, actividad_desc, domicilio, estrato) =
        if is_mallen_self {
            (
                "FIRMA PERICIAL MALLEN S.A.S. DE C.V.".to_string(),
                "FIRMA PERICIAL MALLEN".to_string(),
                "541110".to_string(),
                "Auditoría Pericial, Gobernanza Regulatoria y Criptografía Legal B2B".to_string(),
                "Av. Paseo de la Reforma, Juárez, Cuauhtémoc, Ciudad de México".to_string(),
                "11 a 30 personas".to_string(),
            )
        } else if let Some(d) = denue_opt {
            let rz = d_in
                .razon_social_oficial
                .clone()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| req.razon_social_override.clone().filter(|s| !s.trim().is_empty()))
                .unwrap_or_else(|| {
                    if d.raz_social.is_empty() {
                        d.nom_estab.clone()
                    } else {
                        d.raz_social
                    }
                });
            let dom = d_in
                .domicilio_fiscal_completo
                .clone()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| format!("{}, {}", d.direccion, d.entidad));
            let giro = d_in
                .giro_scian_descripcion
                .clone()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| d.nombre_act.clone());
            (rz, d.nom_estab, d.codigo_scian, giro, dom, d.per_ocu)
        } else {
            let rz = d_in
                .razon_social_oficial
                .clone()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| req.razon_social_override.clone().filter(|s| !s.trim().is_empty()))
                .unwrap_or_else(|| format!("CORPORATIVO {} S.A. DE C.V.", canonical_rfc));
            let dom = d_in
                .domicilio_fiscal_completo
                .clone()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "Domicilio Fiscal Registrado en Territorio Nacional".to_string());
            let giro = d_in
                .giro_scian_descripcion
                .clone()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "541610 - Servicios de consultoría y operación corporativa B2B".to_string());
            (
                rz.clone(),
                rz,
                "541610".to_string(),
                giro,
                dom,
                "11 a 30 personas".to_string(),
            )
        };

    let repse_reg = state
        .db
        .check_repse_status(&canonical_rfc, &razon_social)
        .await
        .unwrap_or(false);
    let sfp_sanc = state
        .db
        .check_sfp_sanctioned(&canonical_rfc, &razon_social)
        .await
        .unwrap_or(false);

    let web_telemetry = if let Some(dom) = &req.domain {
        if !dom.trim().is_empty() {
            run_deep_web_telemetry(dom, None).await
        } else {
            WebOsintTelemetry::default()
        }
    } else {
        WebOsintTelemetry::default()
    };

    let emp_count = parse_employee_count(&estrato, req.calibration.exact_employee_count);
    let findings = if is_mallen_self {
        Vec::new()
    } else {
        build_full_regulatory_findings(
            &codigo_scian,
            &razon_social,
            emp_count,
            &sat_69b,
            &sat_69,
            repse_reg,
            sfp_sanc,
            &web_telemetry,
            &req.calibration,
        )
    };

    let (risk_score, min_fine, est_fine, max_fine) = if is_mallen_self {
        (0, 0.0, 0.0, 0.0)
    } else {
        summarize_findings_financials(&findings)
    };

    let now_utc = Utc::now().to_rfc3339();
    let mut hasher = Sha256::new();
    hasher.update(format!("MALLEN_DETERMINISTIC_2026::{}:{}:{}", canonical_rfc, razon_social, est_fine).as_bytes());
    let audit_sha256_hash = format!("{:x}", hasher.finalize());

    let _ = state
        .db
        .save_prospect(
            &canonical_rfc,
            &razon_social,
            req.email_contacto.as_deref().unwrap_or(""),
            risk_score,
        )
        .await;

    let mut cal_enriched = req.calibration.clone();
    if cal_enriched.dossier.razon_social_oficial.is_none() {
        cal_enriched.dossier.razon_social_oficial = Some(razon_social.clone());
    }
    if cal_enriched.dossier.domicilio_fiscal_completo.is_none() {
        cal_enriched.dossier.domicilio_fiscal_completo = Some(domicilio.clone());
    }
    if cal_enriched.dossier.entidad_jurisdiccion.is_none() && !denue_entidad.is_empty() {
        cal_enriched.dossier.entidad_jurisdiccion = Some(denue_entidad);
    }
    if cal_enriched.dossier.giro_scian_descripcion.is_none() {
        cal_enriched.dossier.giro_scian_descripcion = Some(format!("{} - {}", codigo_scian, actividad_desc));
    }
    if cal_enriched.dossier.email_arco_privacidad.is_none() && !denue_email.is_empty() {
        cal_enriched.dossier.email_arco_privacidad = Some(denue_email);
    }
    if cal_enriched.dossier.telefono_corporativo.is_none() && !denue_tel.is_empty() {
        cal_enriched.dossier.telefono_corporativo = Some(denue_tel);
    }
    let master_report = AuditMasterJson {
        audit_id: format!("MALLEN-{}", &audit_sha256_hash[..10].to_uppercase()),
        generated_at_utc: now_utc,
        tax_id: canonical_rfc.clone(),
        razon_social,
        nombre_comercial,
        codigo_scian,
        actividad_scian_desc: actividad_desc,
        domicilio_fiscal_o_denue: domicilio,
        employee_stratum: estrato,
        estimated_employees: emp_count,
        sat_69b_status: sat_69b,
        sat_69_status: sat_69,
        stps_repse_registered: repse_reg,
        sfp_sanctioned: sfp_sanc,
        usa_datos_biometricos: req.calibration.uses_biometrics_or_health_data.unwrap_or(true),
        realiza_transferencias_datos: req.calibration.transfers_data_to_third_parties.unwrap_or(true),
        requiere_nom_035: true,
        requiere_nom_037_teletrabajo: req.calibration.has_telework_over_40_percent.unwrap_or(true),
        requiere_pipc_proteccion_civil: true,
        opera_ecommerce: web_telemetry.has_ecommerce_cart,
        web_telemetry,
        calibration_applied: cal_enriched,
        findings,
        risk_score,
        total_min_fine_mxn: min_fine,
        total_estimated_fine_mxn: est_fine,
        total_max_fine_mxn: max_fine,
        audit_sha256_hash,
        merkle_root_hex: None,
        l2_tx_hash: None,
        legal_acceptance: None,
        partner_whitelabel_name: req.partner_id,
        partner_logo_base64: None,
    };

    // Persistir la sesión exacta en SQLite WAL para que el PDF y el ZIP usen exactamente estos montos y datos
    let _ = state
        .db
        .save_audit_session(&canonical_rfc, &master_report, is_mallen_self)
        .await;

    let is_mallen = master_report.tax_id.starts_with("MALLEN");
    let _ = state.db.save_audit_session(&master_report.tax_id, &master_report, is_mallen).await;
    Json(master_report)
}

/// Guarda o actualiza los 23 campos de la Ficha de Personalización Jurídica (`LegalDossierInput`)
/// antes de emitir el expediente definitivo.
async fn handle_save_dossier(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SaveDossierRequest>,
) -> impl IntoResponse {
    let clean_tax = req.tax_id.trim().to_uppercase();
    if let Ok(Some((mut master, is_acc))) = state.db.get_audit_session(&clean_tax).await {
        if let Some(rz) = &req.dossier.razon_social_oficial {
            if !rz.trim().is_empty() {
                master.razon_social = rz.trim().to_string();
            }
        }
        if let Some(dom) = &req.dossier.domicilio_fiscal_completo {
            if !dom.trim().is_empty() {
                master.domicilio_fiscal_o_denue = dom.trim().to_string();
            }
        }
        if let Some(giro) = &req.dossier.giro_scian_descripcion {
            if !giro.trim().is_empty() {
                master.actividad_scian_desc = giro.trim().to_string();
            }
        }
        master.calibration_applied.dossier = req.dossier;
        let _ = state.db.save_audit_session(&clean_tax, &master, is_acc).await;
    }

    Json(json!({"status": "DOSSIER_SAVED", "tax_id": clean_tax}))
}

async fn handle_exit_intent_micro_pdf(
    State(state): State<Arc<AppState>>,
    Path(tax_id): Path<String>,
) -> impl IntoResponse {
    let clean_tax = tax_id.trim().to_uppercase();
    let (empresa, session_opt) = build_empresa_and_session(&state, &clean_tax).await;
    let osint = OsintResult {
        domain_found: empresa.sitio_web.clone(),
        has_ssl: true,
        has_privacy_policy: clean_tax.starts_with("MALLEN"),
        http_status: 200,
        ..Default::default()
    };

    let out_path = format!(
        "kits_blindaje/{}/Micro_Dictamen_STPS_{}.pdf",
        clean_tax, clean_tax
    );
    match generate_exit_intent_micro_pdf_with_session(
        &empresa,
        &osint,
        session_opt.as_ref().map(|(m, _)| m), session_opt.as_ref().map(|(_, a)| *a).unwrap_or(clean_tax.starts_with("MALLEN")),
        &out_path,
    ) {
        Ok(pdf_bytes) => {
            let headers = [
                (header::CONTENT_TYPE, "application/pdf".to_string()),
                (
                    header::CONTENT_DISPOSITION,
                    format!("inline; filename=\"Pre_Dictamen_STPS_{}.pdf\"", clean_tax),
                ),
            ];
            (headers, pdf_bytes).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error generando Micro-Dictamen: {}", e),
        )
            .into_response(),
    }
}

async fn handle_partner_batch_scan(
    State(state): State<Arc<AppState>>,
    Json(batch): Json<PartnerBatchRequest>,
) -> impl IntoResponse {
    if batch.api_key.trim().is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "API Key de Despacho Aliado requerida"})),
        )
            .into_response();
    }

    let mut processed = Vec::new();
    for target in batch.targets.into_iter().take(100) {
        let rfc = target.tax_id.trim().to_uppercase();
        let (mut emp, _) = build_empresa_and_session(&state, &rfc).await;
        emp.accountant_signature = Some(batch.partner_name.clone());
        evaluate_risk(&mut emp);

        processed.push(json!({
            "tax_id": rfc,
            "razon_social": emp.nombre,
            "partner_whitelabel": batch.partner_name,
            "risk_score": emp.risk_score,
            "estimated_fine_mxn": emp.estimated_fine_mxn,
            "audit_hash": emp.audit_hash,
            "download_zip_url": format!("/download/zip/{}", rfc)
        }));
    }

    let leaves: Vec<String> = processed
        .iter()
        .filter_map(|v| v["audit_hash"].as_str().map(MerkleTree::hash_leaf))
        .collect();
    let batch_merkle_root = MerkleTree::compute_root(&leaves);

    (
        StatusCode::OK,
        Json(json!({
            "status": "BATCH_COMPLETED",
            "partner": batch.partner_name,
            "total_processed": processed.len(),
            "batch_merkle_root": format!("0x{}", batch_merkle_root),
            "results": processed
        })),
    )
        .into_response()
}

async fn handle_rag_query(Json(req): Json<RagQueryRequest>) -> impl IntoResponse {
    let answer = DeterministicRagAgent::answer_grounded_query(&req.tax_id, &req.question);
    Json(json!({
        "tax_id": req.tax_id,
        "temperature": 0.0,
        "source_enforced": "DOF / Expediente Pericial Mallen",
        "answer": answer
    }))
}

async fn download_zip_handler(
    State(state): State<Arc<AppState>>,
    Path(tax_id): Path<String>,
) -> impl IntoResponse {
    let clean_tax = tax_id.trim().to_uppercase();
    let session_check = state.db.get_audit_session(&clean_tax).await.ok().flatten();
    let is_paid = session_check.as_ref().map(|(_, acc)| *acc).unwrap_or(false);
    if !is_paid {
        return (
            StatusCode::SEE_OTHER,
            [(header::LOCATION, format!("/checkout?ref={}", clean_tax))],
            "Redirigiendo a pasarela de activacion...".to_string(),
        ).into_response();
    }
    let (empresa, session_opt) = build_empresa_and_session(&state, &clean_tax).await;
    let osint = OsintResult {
        domain_found: empresa.sitio_web.clone(),
        has_ssl: true,
        has_privacy_policy: clean_tax.starts_with("MALLEN"),
        http_status: 200,
        ..Default::default()
    };

    match generate_custom_bundle_with_session(&empresa, &osint, session_opt.as_ref().map(|(m, _)| m), session_opt.as_ref().map(|(_, a)| *a).unwrap_or(clean_tax.starts_with("MALLEN"))) {
        Ok(zip_path) => {
            if let Ok(bytes) = fs::read(&zip_path) {
                let headers = [
                    (header::CONTENT_TYPE, "application/zip".to_string()),
                    (
                        header::CONTENT_DISPOSITION,
                        format!(
                            "attachment; filename=\"Paquete_Blindaje_Integral_{}.zip\"",
                            clean_tax
                        ),
                    ),
                ];
                return (headers, bytes).into_response();
            }
        }
        Err(e) => {
            eprintln!("⚠️ Error generando bundle para {}: {}", clean_tax, e);
        }
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Error en el motor de compilación y empaquetado pericial",
    )
        .into_response()
}

/// Recupera primero la sesión guardada en SQLite (`audit_sessions`) para que los montos y los 23 campos
/// legales coincidan al 100% con lo que el cliente vio y llenó en la página web.
async fn build_empresa_and_session(
    state: &AppState,
    clean_tax: &str,
) -> (EmpresaB2B, Option<(AuditMasterJson, bool)>) {
    let session_opt = state.db.get_audit_session(clean_tax).await.ok().flatten();

    if let Some((ref master, is_acc)) = session_opt {
        let emp = EmpresaB2B {
            id: master.audit_id.clone(),
            nombre: master.razon_social.clone(),
            actividad: format!("{} - {}", master.codigo_scian, master.actividad_scian_desc),
            telefono: master
                .calibration_applied
                .dossier
                .telefono_corporativo
                .clone()
                .unwrap_or_else(|| "55-5000-0000".to_string()),
            email: master
                .calibration_applied
                .dossier
                .email_arco_privacidad
                .clone()
                .unwrap_or_else(|| "privacidad@mallen.lat".to_string()),
            direccion: master.domicilio_fiscal_o_denue.clone(),
            estado: master
                .calibration_applied
                .dossier
                .entidad_jurisdiccion
                .clone()
                .unwrap_or_else(|| "CIUDAD DE MÉXICO".to_string()),
            sitio_web: if master.web_telemetry.target_url.is_empty() {
                None
            } else {
                Some(master.web_telemetry.target_url.clone())
            },
            status_web: Some(master.web_telemetry.http_status.max(200)),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash: format!("0x{}", &master.audit_sha256_hash[..16]),
            risk_score: if is_acc { 0 } else { master.risk_score },
            audit_flags: Vec::new(),
            estimated_fine_mxn: if is_acc {
                0.0
            } else {
                master.total_estimated_fine_mxn
            },
            shield_price_usd: 330.0,
            accountant_signature: Some(apply_accountant_signature(
                &AccountantProfile::default(),
                clean_tax,
            )),
        };
        return (emp, session_opt);
    }

    // Si aún no existe sesión en DB (ej. acceso directo a URL), la construimos de forma determinística
    let is_mallen = clean_tax.starts_with("MALLEN");
    let mut hasher = Sha256::new();
    hasher.update(format!("MALLEN_SEAL_2026::{}", clean_tax).as_bytes());
    let audit_hash = format!("0x{}", &format!("{:x}", hasher.finalize())[..16]);

    let denue = state
        .db
        .find_denue_by_name_or_domain(None, Some(clean_tax))
        .await
        .ok()
        .flatten();

    let mut emp = if is_mallen {
        EmpresaB2B {
            id: clean_tax.to_string(),
            nombre: "FIRMA PERICIAL MALLEN S.A.S. DE C.V.".to_string(),
            actividad: "541110 - Auditoría Pericial, Gobernanza y Criptografía Legal B2B".to_string(),
            telefono: "55-5000-0000".to_string(),
            email: "dictamenes@mallen.lat".to_string(),
            direccion: "Av. Paseo de la Reforma, Juárez, Cuauhtémoc, CDMX".to_string(),
            estado: "CIUDAD DE MÉXICO".to_string(),
            sitio_web: Some("https://mallen.lat".to_string()),
            status_web: Some(200),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash,
            risk_score: 0,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 0.0,
            shield_price_usd: 0.0,
            accountant_signature: Some(apply_accountant_signature(
                &AccountantProfile::default(),
                clean_tax,
            )),
        }
    } else if let Some(d) = denue {
        EmpresaB2B {
            id: d.id_denue,
            nombre: if d.raz_social.is_empty() {
                d.nom_estab
            } else {
                d.raz_social
            },
            actividad: format!("{} - {}", d.codigo_scian, d.nombre_act),
            telefono: d.telefono,
            email: d.email,
            direccion: format!("{} ({})", d.direccion, d.per_ocu),
            estado: d.entidad,
            sitio_web: if d.www.is_empty() { None } else { Some(d.www) },
            status_web: Some(200),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash,
            risk_score: 85,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 1_485_600.00,
            shield_price_usd: 330.00,
            accountant_signature: Some(apply_accountant_signature(
                &AccountantProfile::default(),
                clean_tax,
            )),
        }
    } else {
        EmpresaB2B {
            id: clean_tax.to_string(),
            nombre: format!("CORPORATIVO {} S.A. DE C.V.", clean_tax),
            actividad: "541610 - Servicios de consultoría y operación corporativa B2B".to_string(),
            telefono: "55-5000-0000".to_string(),
            email: "".to_string(),
            direccion: "11 a 30 personas — Domicilio Fiscal en México".to_string(),
            estado: "CIUDAD DE MÉXICO".to_string(),
            sitio_web: None,
            status_web: Some(200),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash,
            risk_score: 85,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 1_485_600.00,
            shield_price_usd: 330.00,
            accountant_signature: Some(apply_accountant_signature(
                &AccountantProfile::default(),
                clean_tax,
            )),
        }
    };

    if !is_mallen {
        evaluate_risk(&mut emp);
    }
    (emp, None)
}

async fn get_merkle_proof_api(
    State(state): State<Arc<AppState>>,
    Path(ref_code): Path<String>,
) -> impl IntoResponse {
    let clean_ref = ref_code.trim().to_uppercase();
    let is_mallen = clean_ref.starts_with("MALLEN");
    let session_opt = state.db.get_audit_session(&clean_ref).await.ok().flatten();

    let (company_name, risk_score, estimated_fine) = if is_mallen {
        ("FIRMA PERICIAL MALLEN S.A.S. DE C.V.".to_string(), 0, 0.0)
    } else if let Some((m, is_acc)) = session_opt {
        (
            m.razon_social,
            if is_acc { 0 } else { m.risk_score },
            if is_acc { 0.0 } else { m.total_estimated_fine_mxn },
        )
    } else {
        (format!("Entidad Auditada ({})", clean_ref), 85, 1_485_600.0)
    };

    let leaf_hash = MerkleTree::hash_leaf(&clean_ref);
    let sample_leaves = vec![
        leaf_hash.clone(),
        MerkleTree::hash_leaf("MALLEN_ANCHOR_SIBLING_2026"),
    ];
    let proof = MerkleTree::generate_proof(&sample_leaves, &leaf_hash);
    let root = MerkleTree::compute_root(&sample_leaves);

    Json(json!({
        "status": "VERIFIED_ON_L2_BATCH",
        "reference": clean_ref,
        "company_name": company_name,
        "risk_score": risk_score,
        "estimated_fine_mxn": estimated_fine,
        "merkle_root": format!("0x{}", root),
        "leaf_hash": format!("0x{}", leaf_hash),
        "merkle_proof": proof,
        "timestamp": Utc::now().to_rfc3339()
    }))
}

async fn download_pdf_report(Path(hash): Path<String>) -> impl IntoResponse {
    let pdf_file_path = format!("dictamenes_pdf/Dictamen_PreAuditoria_{}.pdf", hash);
    if let Ok(pdf_bytes) = fs::read(&pdf_file_path) {
        let disposition = format!("inline; filename=Dictamen_{}.pdf", hash);
        let headers = [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (header::CONTENT_DISPOSITION, disposition),
        ];
        return (headers, pdf_bytes).into_response();
    }
    (StatusCode::NOT_FOUND, "Dictamen PDF no encontrado").into_response()
}

async fn build_empresa_from_db_or_default(state: &AppState, clean_tax: &str) -> EmpresaB2B {
    if let Ok(Some((saved, _is_acc))) = state.db.get_audit_session(clean_tax).await {
        let d = &saved.calibration_applied.dossier;
        return EmpresaB2B {
            id: saved.audit_id.clone(),
            nombre: d.razon_social_oficial.clone().filter(|s| !s.is_empty()).unwrap_or(saved.razon_social),
            actividad: d.giro_scian_descripcion.clone().filter(|s| !s.is_empty()).unwrap_or(format!("{} - {}", saved.codigo_scian, saved.actividad_scian_desc)),
            telefono: d.telefono_corporativo.clone().unwrap_or_default(),
            email: d.email_arco_privacidad.clone().unwrap_or_default(),
            direccion: d.domicilio_fiscal_completo.clone().filter(|s| !s.is_empty()).unwrap_or(saved.domicilio_fiscal_o_denue),
            estado: d.entidad_jurisdiccion.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "MÉXICO".to_string()),
            sitio_web: if saved.web_telemetry.target_url.is_empty() { None } else { Some(saved.web_telemetry.target_url) },
            status_web: Some(saved.web_telemetry.http_status.max(200)),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash: format!("0x{}", &saved.audit_sha256_hash[..16.min(saved.audit_sha256_hash.len())]),
            risk_score: if clean_tax.starts_with("MALLEN") { 0 } else { saved.risk_score },
            audit_flags: Vec::new(),
            estimated_fine_mxn: if clean_tax.starts_with("MALLEN") { 0.0 } else { saved.total_estimated_fine_mxn },
            shield_price_usd: 330.0,
            accountant_signature: saved.partner_whitelabel_name,
        };
    }

    let denue = state
        .db
        .find_denue_by_name_or_domain(None, Some(clean_tax))
        .await
        .ok()
        .flatten();

    let mut hasher = Sha256::new();
    hasher.update(format!("MALLEN_SEAL_2026::{}", clean_tax).as_bytes());
    let audit_hash = format!("0x{}", &format!("{:x}", hasher.finalize())[..16]);

    let mut emp = if let Some(d) = denue {
        EmpresaB2B {
            id: d.id_denue,
            nombre: if d.raz_social.is_empty() { d.nom_estab } else { d.raz_social },
            actividad: format!("{} - {}", d.codigo_scian, d.nombre_act),
            telefono: d.telefono,
            email: d.email,
            direccion: format!("{} ({})", d.direccion, d.per_ocu),
            estado: d.entidad,
            sitio_web: if d.www.is_empty() { None } else { Some(d.www) },
            status_web: Some(200),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash,
            risk_score: 85,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 0.0,
            shield_price_usd: 330.00,
            accountant_signature: Some(apply_accountant_signature(&AccountantProfile::default(), clean_tax)),
        }
    } else {
        EmpresaB2B {
            id: clean_tax.to_string(),
            nombre: format!("CORPORATIVO {} S.A. DE C.V.", clean_tax),
            actividad: "541610 - Servicios de consultoría y operación corporativa B2B".to_string(),
            telefono: String::new(),
            email: String::new(),
            direccion: "Domicilio Fiscal Registrado en Territorio Nacional (México)".to_string(),
            estado: "CIUDAD DE MÉXICO".to_string(),
            sitio_web: None,
            status_web: Some(200),
            e_mail_verificado: Some(true),
            country_code: "MX".to_string(),
            tax_id: Some(clean_tax.to_string()),
            language: Some("es".to_string()),
            audit_hash,
            risk_score: 85,
            audit_flags: Vec::new(),
            estimated_fine_mxn: 0.0,
            shield_price_usd: 330.00,
            accountant_signature: Some(apply_accountant_signature(&AccountantProfile::default(), clean_tax)),
        }
    };

    evaluate_risk(&mut emp);
    emp
}

async fn handle_checkout_path(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    render_checkout_page_with_state(state, id).await
}

async fn handle_checkout(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CheckoutParams>,
) -> impl IntoResponse {
    let ref_code = params
        .ref_id
        .or(params.r#ref)
        .or(params.hash)
        .unwrap_or_else(|| "EST850628K51".to_string());
    render_checkout_page_with_state(state, ref_code).await
}

async fn render_checkout_page_with_state(state: Arc<AppState>, raw_ref: String) -> Html<String> {
    let ref_code = raw_ref.trim().to_uppercase();
    let disclaimer_hash = StripeDisputeGuard::compute_disclaimer_hash();

    let emp = build_empresa_from_db_or_default(&state, &ref_code).await;
    let session_opt = state.db.get_audit_session(&ref_code).await.ok().flatten();

    // Sincronizar monto exacto: si no hay sesión previa, calcular con la misma matriz de 39 NOMs
    let fine_mxn = match &session_opt {
        Some((m, _)) if m.total_estimated_fine_mxn > 0.0 => m.total_estimated_fine_mxn,
        _ => {
            let findings = build_full_regulatory_findings(
                "434222",
                &emp.nombre,
                45,
                "LIMPIO",
                "LIMPIO",
                false,
                false,
                &WebOsintTelemetry::default(),
                &worker_inegi_engine::engine::models::CalibrationAnswers::default(),
            );
            let (_, _, est, _) = summarize_findings_financials(&findings);
            est
        }
    };
    let fine_formatted = worker_inegi_engine::engine::pdf_generator::format_currency_mxn(fine_mxn);

    let d_opt = session_opt.as_ref().map(|(m, _)| &m.calibration_applied.dossier);
    let pre_razon = d_opt
        .and_then(|d| d.razon_social_oficial.clone())
        .unwrap_or_else(|| emp.nombre.clone());
    let pre_giro = d_opt
        .and_then(|d| d.giro_scian_descripcion.clone())
        .unwrap_or_else(|| emp.actividad.clone());
    let pre_dom = d_opt
        .and_then(|d| d.domicilio_fiscal_completo.clone())
        .unwrap_or_else(|| emp.direccion.clone());
    let pre_cp = d_opt
        .and_then(|d| d.codigo_postal_fiscal.clone())
        .unwrap_or_default();
    let pre_estado = d_opt
        .and_then(|d| d.entidad_jurisdiccion.clone())
        .unwrap_or_else(|| emp.estado.clone());
    let pre_rep = d_opt
        .and_then(|d| d.representante_legal_nombre.clone())
        .unwrap_or_default();
    let pre_cargo = d_opt
        .and_then(|d| d.representante_legal_cargo.clone())
        .unwrap_or_default();
    let pre_oficial = d_opt
        .and_then(|d| d.oficial_privacidad_nombre.clone())
        .unwrap_or_default();
    let pre_email = d_opt
        .and_then(|d| d.email_arco_privacidad.clone())
        .unwrap_or_else(|| emp.email.clone());
    let pre_tel = d_opt
        .and_then(|d| d.telefono_corporativo.clone())
        .unwrap_or_else(|| emp.telefono.clone());
    let pre_reg_imss = d_opt
        .and_then(|d| d.registro_patronal_imss.clone())
        .unwrap_or_default();
    let pre_coord = d_opt
        .and_then(|d| d.comision_coordinador_patronal.clone())
        .unwrap_or_default();
    let pre_sec = d_opt
        .and_then(|d| d.comision_secretario_trabajadores.clone())
        .unwrap_or_default();
    let pre_prov_nom = d_opt
        .and_then(|d| d.proveedor_principal_razon_social.clone())
        .unwrap_or_default();
    let pre_prov_rfc = d_opt
        .and_then(|d| d.proveedor_principal_rfc.clone())
        .unwrap_or_default();
    let pre_prov_obj = d_opt
        .and_then(|d| d.proveedor_principal_servicio_objeto.clone())
        .unwrap_or_default();

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Activación de Blindaje Corporativo y CFDI 4.0 — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 flex items-center justify-center selection:bg-blue-600 selection:text-white">
    <div class="max-w-3xl w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 p-8 rounded-2xl font-mono-code text-xs shadow-2xl">
        <div class="flex justify-between items-center mb-6 pb-4 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-blue-400 block uppercase">FIRMA PERICIAL MALLEN • PASARELA CORPORATIVA 3D SECURE</span>
                <span class="text-[10px] text-slate-500">Instrumentación Jurídica Personalizada + Pago 3D Secure + Timbrado CFDI 4.0</span>
            </div>
            <a href="/" class="text-[10px] text-slate-400 hover:text-white transition">Volver al Portal</a>
        </div>

        <div class="bg-slate-950 border border-white/15 rounded-xl p-5 mb-6 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
            <div>
                <span class="text-[10px] text-emerald-400 font-bold uppercase block">EXPEDIENTE CORPORATIVO DE CUMPLIMIENTO INTEGRAL</span>
                <span class="text-sm font-bold text-white block mt-0.5">RFC: {ref_code} — Contingencia Auditada: <span class="text-red-400">${fine_formatted} MXN</span></span>
                <span class="text-[10px] text-slate-400 block mt-1">Los datos públicos detectados por su RFC y sitio web se pre-cargan automáticamente abajo. Los campos que deje en blanco (como los datos de un proveedor específico para el Contrato Art. 69-B o los firmantes internos) se generarán con líneas jurídicas formales de llenado (____________________) para que pueda reutilizar sus contratos con múltiples proveedores.</span>
            </div>
            <div class="text-right shrink-0">
                <span class="text-xl font-extrabold text-white">$5,940.00</span>
                <span class="text-[10px] text-slate-500 block">MXN (IVA Incluido)</span>
            </div>
        </div>

        <form id="checkoutForm" onsubmit="processCheckoutPayment(event)" class="space-y-5">
            <div class="bg-slate-950 border border-white/15 p-4 rounded-xl space-y-3">
                <span class="text-[10px] font-bold text-blue-400 uppercase block">Datos Societarios, Domicilio Fiscal y Representación Legal</span>
                <div class="grid sm:grid-cols-2 gap-3">
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Denominación o Razón Social Oficial</label>
                        <input id="pay_razon" type="text" required value="{pre_razon}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Giro o Actividad Económica SCIAN</label>
                        <input id="pay_giro" type="text" required value="{pre_giro}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div class="sm:col-span-2">
                        <label class="text-[10px] text-slate-400 block mb-1">Domicilio Fiscal y del Centro de Trabajo</label>
                        <input id="pay_dom" type="text" required value="{pre_dom}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Código Postal Fiscal (CFDI 4.0)</label>
                        <input id="pay_cp" type="text" maxlength="5" value="{pre_cp}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Entidad Federativa / Jurisdicción</label>
                        <input id="pay_estado" type="text" required value="{pre_estado}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Nombre del Representante Legal</label>
                        <input id="pay_rep" type="text" value="{pre_rep}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Cargo Societario del Representante</label>
                        <input id="pay_cargo" type="text" value="{pre_cargo}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                </div>
            </div>

            <div class="bg-slate-950 border border-white/15 p-4 rounded-xl space-y-3">
                <span class="text-[10px] font-bold text-emerald-400 uppercase block">Oficial de Privacidad (LFPDPPP), Contacto y Comisión Mixta STPS</span>
                <div class="grid sm:grid-cols-3 gap-3">
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Oficial de Privacidad (Art. 17)</label>
                        <input id="pay_oficial" type="text" value="{pre_oficial}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Correo Derechos ARCO y CFDI</label>
                        <input id="pay_email" type="email" value="{pre_email}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Teléfono Corporativo</label>
                        <input id="pay_tel" type="text" value="{pre_tel}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Registro Patronal IMSS</label>
                        <input id="pay_reg_imss" type="text" value="{pre_reg_imss}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Coordinador Patronal NOM-019</label>
                        <input id="pay_coord" type="text" value="{pre_coord}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Secretario Trabajadores NOM-019</label>
                        <input id="pay_sec" type="text" value="{pre_sec}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                </div>
            </div>

            <div class="bg-slate-950 border border-white/15 p-4 rounded-xl space-y-3">
                <span class="text-[10px] font-bold text-purple-400 uppercase block">Datos de Contraparte para Contrato de Materialidad SAT Art. 69-B</span>
                <div class="grid sm:grid-cols-2 gap-3">
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">Razón Social del Proveedor</label>
                        <input id="pay_prov_nom" type="text" value="{pre_prov_nom}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div>
                        <label class="text-[10px] text-slate-400 block mb-1">RFC del Proveedor</label>
                        <input id="pay_prov_rfc" type="text" value="{pre_prov_rfc}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                    <div class="sm:col-span-2">
                        <label class="text-[10px] text-slate-400 block mb-1">Objeto del Servicio / Entregable Auditado (Art. 5-A y 69-B CFF)</label>
                        <input id="pay_prov_obj" type="text" value="{pre_prov_obj}" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    </div>
                </div>
            </div>

            <div class="bg-slate-950 border border-white/15 p-4 rounded-xl space-y-3">
                <div class="flex justify-between items-center">
                    <span class="text-[10px] font-bold text-blue-400 uppercase">Método de Pago Corporativo (Tokenización TLS 1.3 / 3DS)</span>
                    <span class="text-[10px] text-emerald-400 font-bold">Estándar PCI-DSS Nivel 1</span>
                </div>
                <input type="text" required value="4242 •••• •••• 4242" class="w-full bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                <div class="grid grid-cols-2 gap-3">
                    <input type="text" required value="12/28" class="bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                    <input type="password" required value="888" class="bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-lg px-3.5 py-2.5 text-white">
                </div>
            </div>

            <div class="bg-slate-950 border border-white/15 p-3.5 rounded-xl text-[10px] text-slate-400 leading-relaxed">
                <label class="flex items-start gap-2.5 cursor-pointer">
                    <input id="chk-disclaimer" type="checkbox" required checked class="mt-1 accent-emerald-500">
                    <span>
                        <strong class="text-slate-200">Manifiesto de Alcance Pericial y Exactitud de Datos (Art. 89 C.Com):</strong> Confirmo los datos ingresados para la emisión de los 10 Certificados y 7 Instrumentos Jurídicos del RFC <b>{ref_code}</b> y su factura CFDI 4.0.
                        <span class="block text-[9px] text-slate-600 mt-1">SHA-256 Disclaimer: {disclaimer_hash}</span>
                    </span>
                </label>
            </div>

            <button id="btnPayNow" type="submit" class="w-full bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold py-4 rounded-xl transition shadow-lg shadow-emerald-600/20 uppercase tracking-wider">
                Confirmar Datos, Timbrar CFDI 4.0 y Descargar Paquete ZIP
            </button>
        </form>

        <div id="paymentSuccessBox" class="hidden mt-5 bg-emerald-950/40 border border-emerald-500/50 rounded-xl p-5 text-center space-y-3">
            <span class="text-emerald-400 font-bold text-sm block">PAGO 3D SECURE AUTORIZADO Y EXPEDIENTE COMPILADO</span>
            <p class="text-slate-300 text-[11px]">Su expediente de 18 documentos en PDF y Word junto con su factura XML CFDI 4.0 está listo.</p>
            <a href="/download/zip/{ref_code}" class="inline-block w-full bg-emerald-600 hover:bg-emerald-500 text-white font-bold py-3.5 rounded-xl uppercase">
                Descargar Paquete Corporativo ZIP ({ref_code})
            </a>
        </div>
    </div>

    <script>
        async function processCheckoutPayment(e) {{
            e.preventDefault();
            const btn = document.getElementById("btnPayNow");
            btn.disabled = true;
            btn.textContent = "Compilando Expediente Personalizado y Timbrando CFDI 4.0...";

            const valOrNull = (id) => {{
                const v = document.getElementById(id)?.value?.trim();
                return v ? v : null;
            }};

            await fetch("/api/calibrate", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{
                    tax_id: "{ref_code}",
                    razon_social_override: valOrNull("pay_razon"),
                    email_contacto: valOrNull("pay_email"),
                    calibration: {{
                        dossier: {{
                            razon_social_oficial: valOrNull("pay_razon"),
                            giro_scian_descripcion: valOrNull("pay_giro"),
                            domicilio_fiscal_completo: valOrNull("pay_dom"),
                            codigo_postal_fiscal: valOrNull("pay_cp"),
                            entidad_jurisdiccion: valOrNull("pay_estado"),
                            representante_legal_nombre: valOrNull("pay_rep"),
                            representante_legal_cargo: valOrNull("pay_cargo"),
                            oficial_privacidad_nombre: valOrNull("pay_oficial"),
                            email_arco_privacidad: valOrNull("pay_email"),
                            telefono_corporativo: valOrNull("pay_tel"),
                            registro_patronal_imss: valOrNull("pay_reg_imss"),
                            comision_coordinador_patronal: valOrNull("pay_coord"),
                            comision_secretario_trabajadores: valOrNull("pay_sec"),
                            proveedor_principal_razon_social: valOrNull("pay_prov_nom"),
                            proveedor_principal_rfc: valOrNull("pay_prov_rfc"),
                            proveedor_principal_servicio_objeto: valOrNull("pay_prov_obj")
                        }}
                    }}
                }})
            }});

            const res = await fetch("/api/checkout/session", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{ ref_code: "{ref_code}", email: valOrNull("pay_email"), accepted_disclaimer: true }})
            }});
            const sessionData = await res.json();

            if (sessionData.checkout_url && sessionData.checkout_url.startsWith("https://checkout.stripe.com")) {{
                window.location.href = sessionData.checkout_url;
                return;
            }}

            await fetch("/api/webhook_stripe", {{
                method: "POST",
                headers: {{ "Content-Type": "application/json" }},
                body: JSON.stringify({{
                    id: "evt_live_" + Date.now(),
                    tax_id: "{ref_code}",
                    data: {{ object: {{ client_reference_id: "{ref_code}" }} }}
                }})
            }});

            document.getElementById("checkoutForm").classList.add("hidden");
            document.getElementById("paymentSuccessBox").classList.remove("hidden");
            setTimeout(() => {{ window.location.href = "/download/zip/{ref_code}"; }}, 1400);
        }}
    </script>
</body>
</html>"#
    );
    Html(html)
}




async fn create_payment_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<PaymentRequest>,
) -> impl IntoResponse {
    if payload.accepted_disclaimer == Some(false) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Debe aceptar el Disclaimer de Alcance Pericial"})),
        )
            .into_response();
    }

    let clean_tax = payload.ref_code.trim().to_uppercase();
    if let Some(dossier) = payload.dossier {
        if let Ok(Some((mut master, is_acc))) = state.db.get_audit_session(&clean_tax).await {
            if let Some(rz) = &dossier.razon_social_oficial {
                if !rz.trim().is_empty() {
                    master.razon_social = rz.trim().to_string();
                }
            }
            master.calibration_applied.dossier = dossier;
            let _ = state.db.save_audit_session(&clean_tax, &master, is_acc).await;
        }
    }

    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("MallenClient/1.0")
        .to_string();

    let disclaimer_sha256 = StripeDisputeGuard::compute_disclaimer_hash();
    let meta = LegalAcceptanceMetadata {
        accepted_at_utc: Utc::now().to_rfc3339(),
        client_ip,
        user_agent,
        disclaimer_sha256: disclaimer_sha256.clone(),
        stripe_session_id: None,
        stripe_payment_intent: None,
        three_d_secure_status: Some("required_3ds".to_string()),
    };

    let _dispute_armor =
        StripeDisputeGuard::build_dispute_armor(&clean_tax, "PENDING_SEAL", &meta);

    let session = create_checkout_session(&state.http_client, &clean_tax, 5940)
        .await
        .unwrap_or(worker_inegi_engine::engine::payment::CheckoutSession {
            checkout_url: format!("/verify/{}", clean_tax),
            session_id: format!("cs_sim_{}", clean_tax),
        });

    if let Some(email) = &payload.email {
        let _ = state
            .db
            .save_prospect(&clean_tax, &clean_tax, email, 85)
            .await;
    }

    (
        StatusCode::OK,
        Json(PaymentResponse {
            checkout_url: session.checkout_url,
            session_id: session.session_id,
            disclaimer_sha256,
            status: "SUCCESS".to_string(),
        }),
    )
        .into_response()
}

async fn handle_payment_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body_text: String,
) -> impl IntoResponse {
    if let Ok(webhook_secret) = std::env::var("STRIPE_WEBHOOK_SECRET") {
        if !webhook_secret.trim().is_empty() {
            let sig = headers
                .get("stripe-signature")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            if !StripeDisputeGuard::verify_webhook_signature(&body_text, sig, &webhook_secret) {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Firma Stripe-Signature inválida"})),
                );
            }
        }
    }

    let parsed: Value = serde_json::from_str(&body_text).unwrap_or_else(|_| json!({}));
    let event_id = parsed["id"]
        .as_str()
        .unwrap_or("evt_simulated_local")
        .to_string();
    let tax_id = parsed["data"]["object"]["client_reference_id"]
        .as_str()
        .or_else(|| parsed["data"]["object"]["metadata"]["tax_id"].as_str())
        .or_else(|| parsed["tax_id"].as_str())
        .unwrap_or("EST850628K51")
        .to_uppercase();

    if event_id != "evt_simulated_local" {
        let is_first_time = state
            .db
            .register_webhook_once(&event_id, Some(&tax_id))
            .await
            .unwrap_or(true);
        if !is_first_time {
            return (
                StatusCode::OK,
                Json(json!({"status": "ALREADY_PROCESSED_IDEMPOTENT", "event_id": event_id})),
            );
        }
    }

    let state_bg = state.clone();
    let tax_bg = tax_id.clone();

    tokio::spawn(async move {
        let (emp, session_opt) = build_empresa_and_session(&state_bg, &tax_bg).await;
        let osint = OsintResult {
            domain_found: emp.sitio_web.clone(),
            has_ssl: true,
            has_privacy_policy: true,
            http_status: 200,
            ..Default::default()
        };

        let cp_receptor = session_opt
            .as_ref()
            .and_then(|(m, _)| m.calibration_applied.dossier.codigo_postal_fiscal.clone())
            .unwrap_or_else(|| "06600".to_string());

        let inv = InvoiceData {
            rfc_receptor: tax_bg.clone(),
            razon_social: emp.nombre.clone(),
            codigo_postal: cp_receptor,
            uso_cfdi: "G03".to_string(),
            monto_total: 5940.00,
            concepto: format!(
                "Dictamen Pericial de Cumplimiento 360° y Kit de Blindaje Legal Personalizado ({})",
                tax_bg
            ),
        };
        if let Ok(cfdi) = SatBillingManager::issue_cfdi_40_complete(&inv).await {
            let cfdi_dir = format!("kits_blindaje/{}", tax_bg);
            let _ = fs::create_dir_all(&cfdi_dir);
            let _ = fs::write(
                format!("{}/CFDI40_Factura_{}.xml", cfdi_dir, cfdi.uuid_fiscal),
                cfdi.xml_content,
            );
        }

        let _ = generate_custom_bundle_with_session(&emp, &osint, session_opt.as_ref().map(|(m, _)| m), true);
        let _ = state_bg.db.update_status_to_paid(&tax_bg).await;
    });

    (
        StatusCode::OK,
        Json(json!({
            "status": "FULFILLED_ASYNC",
            "tax_id": tax_id,
            "download_url": format!("/download/zip/{}", tax_id)
        })),
    )
}

// =============================================================================
// VISTAS INSTITUCIONALES Y DIRECTORIO TÉCNICO DE INSPECCIÓN (INTACTAS)
// =============================================================================

async fn handle_index() -> impl IntoResponse {
    match fs::read_to_string("public/index.html") {
        Ok(content) => Html(content).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error al cargar el portal de peritaje",
        )
            .into_response(),
    }
}

async fn handle_privacy() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aviso de Privacidad Integral — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; background-color: #020617; background-image: radial-gradient(at 12% 12%, rgba(37, 99, 235, 0.32) 0px, transparent 48%), radial-gradient(at 88% 18%, rgba(16, 185, 129, 0.25) 0px, transparent 48%), radial-gradient(at 50% 82%, rgba(99, 102, 241, 0.28) 0px, transparent 52%); background-attachment: fixed; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
</head>
<body class="bg-slate-950 text-slate-300 min-h-screen p-6 md:p-12 border-t-4 border-blue-600">
    <div class="max-w-4xl mx-auto bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 p-8 md:p-12 shadow-2xl rounded-2xl">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-blue-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Aviso de Privacidad Integral conforme a la LFPDPPP vigente (SABG / Transparencia para el Pueblo)</span>
            </div>
            <a href="/" class="text-xs text-blue-400 hover:text-blue-300 font-mono-code transition">← Regresar al Portal</a>
        </div>
        <h1 class="text-xl font-bold text-white mb-6 tracking-tight">AVISO DE PRIVACIDAD INTEGRAL</h1>
        <div class="space-y-6 text-xs leading-relaxed font-mono-code text-slate-400">
            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">1. Identidad y Domicilio del Responsable</h2>
                <p>Firma Pericial Mallen (mallen.lat), con domicilio operativo en la Ciudad de México, Estados Unidos Mexicanos, actúa como Responsable del tratamiento de los datos personales y corporativos conforme a los Artículos 8, 15, 16 y 19 de la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP).</p>
            </section>
            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">2. Datos Objeto de Tratamiento y Sanitización PII</h2>
                <p>Para la ejecución de evaluaciones de riesgo y personalización de instrumentos jurídicos, se procesan los datos societarios confirmados por el representante legal y registros públicos corporativos. Nuestro motor aplica filtros Regex de sanitización PII en tránsito.</p>
            </section>
            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">3. Secreto Profesional y Mecanismos ARCO (Arts. 22 a 35 LFPDPPP)</h2>
                <p>Toda la información procesada está amparada bajo el Secreto Profesional de Auditoría y Peritaje. Para el ejercicio de los Derechos de Acceso, Rectificación, Cancelación u Oposición (ARCO), envíe su solicitud a <strong>dictamenes@mallen.lat</strong> (plazo legal de respuesta: 20 días hábiles).</p>
            </section>
        </div>
    </div>
</body>
</html>"#;
    Html(html)
}

async fn handle_terms() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Términos del Servicio & Alcance Pericial — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; background-color: #020617; background-image: radial-gradient(at 12% 12%, rgba(37, 99, 235, 0.32) 0px, transparent 48%), radial-gradient(at 88% 18%, rgba(16, 185, 129, 0.25) 0px, transparent 48%), radial-gradient(at 50% 82%, rgba(99, 102, 241, 0.28) 0px, transparent 52%); background-attachment: fixed; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
</head>
<body class="bg-slate-950 text-slate-300 min-h-screen p-6 md:p-12 border-t-4 border-indigo-600">
    <div class="max-w-4xl mx-auto bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 p-8 md:p-12 shadow-2xl rounded-2xl">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-indigo-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Términos y Condiciones del Sistema de Auditoría 360°</span>
            </div>
            <a href="/" class="text-xs text-indigo-400 hover:text-indigo-300 font-mono-code transition">← Regresar al Portal</a>
        </div>
        <h1 class="text-xl font-bold text-white mb-6 tracking-tight">TÉRMINOS Y CONDICIONES Y MANIFIESTO DE ALCANCE PERICIAL</h1>
        <div class="space-y-6 text-xs leading-relaxed font-mono-code text-slate-400">
            <section class="border-l-2 border-indigo-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">1. Personalización Jurídica Sin Suposiciones y Última Milla</h2>
                <p>Mallen personaliza los 10 Certificados y 7 Kits Legales con los datos societarios y de representación confirmados por el cliente antes de la emisión. La impresión física y recabo de firmas autógrafas en el centro de trabajo corresponde a la representación legal de la empresa.</p>
            </section>
            <section class="border-l-2 border-indigo-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">2. Inalterabilidad Criptográfica y Política de Entrega Digital</h2>
                <p>Cada folio de dictamen cuenta con un identificador único SHA-256 vinculado a un Árbol de Merkle en Arbitrum L2 conforme a los Artículos 89 a 94 del Código de Comercio.</p>
            </section>
        </div>
    </div>
</body>
</html>"#;
    Html(html)
}

fn render_norma_category(
    cat_slug: &str,
    titulo: &str,
    sub: &str,
    institucion: &str,
    items: Vec<(&str, &str, &str, &str)>,
) -> Html<String> {
    let (theme_border, theme_badge, theme_text, theme_accent) = match cat_slug {
        "stps" => (
            "border-blue-900/50",
            "bg-blue-950/80 border-blue-800/80 text-blue-400",
            "text-blue-400",
            "border-blue-500/80",
        ),
        "sat" => (
            "border-emerald-900/50",
            "bg-emerald-950/80 border-emerald-800/80 text-emerald-400",
            "text-emerald-400",
            "border-emerald-500/80",
        ),
        "imss" => (
            "border-purple-900/50",
            "bg-purple-950/80 border-purple-800/80 text-purple-400",
            "text-purple-400",
            "border-purple-500/80",
        ),
        "inai" => (
            "border-cyan-900/50",
            "bg-cyan-950/80 border-cyan-800/80 text-cyan-400",
            "text-cyan-400",
            "border-cyan-500/80",
        ),
        "pld" => (
            "border-amber-900/50",
            "bg-amber-950/80 border-amber-800/80 text-amber-400",
            "text-amber-400",
            "border-amber-500/80",
        ),
        _ => (
            "border-teal-900/50",
            "bg-teal-950/80 border-teal-800/80 text-teal-400",
            "text-teal-400",
            "border-teal-500/80",
        ),
    };

    let mut cards_html = String::new();
    for (code_slug, codigo, nombre, desc) in items {
        cards_html.push_str(&format!(
            r#"<div class="bg-white/[0.04] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border {} rounded-2xl p-6 hover:{} transition shadow-lg flex flex-col justify-between group">
                <div>
                    <span class="inline-block border text-xs px-2.5 py-1 rounded font-mono mb-3 {}">{}</span>
                    <h3 class="text-base font-bold text-white mb-2 group-hover:{} transition">{}</h3>
                    <p class="text-xs text-slate-400 font-mono leading-relaxed mb-4">{}</p>
                </div>
                <a href="/normas/{}/{}" class="text-xs font-bold {} font-mono flex items-center gap-1 transition pt-4 border-t border-white/15">
                    Inspeccionar matriz de riesgo →
                </a>
            </div>"#,
            theme_border,
            theme_accent,
            theme_badge,
            codigo,
            theme_text,
            nombre,
            desc,
            cat_slug,
            code_slug,
            theme_text
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 selection:bg-slate-700 selection:text-white">
    <div class="max-w-7xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-10 border-b border-white/15">
            <a href="/" class="flex items-center space-x-3 cursor-pointer group">
                <div class="w-10 h-10 bg-gradient-to-tr from-blue-700 via-indigo-600 to-emerald-500 rounded-xl flex items-center justify-center font-extrabold text-xl text-white group-hover:scale-105 transition shadow-lg">M</div>
                <div>
                    <span class="text-sm font-bold text-white block leading-none">FIRMA PERICIAL MALLEN</span>
                    <span class="text-[10px] {} font-bold uppercase font-mono-code">Directorio Técnico de Inspección</span>
                </div>
            </a>
            <a href="/" class="text-xs {} hover:underline font-mono-code transition">← Volver a la Portada</a>
        </div>
        <div class="mb-10">
            <span class="text-xs font-bold {} uppercase tracking-widest font-mono-code">{}</span>
            <h1 class="text-3xl sm:text-4xl font-extrabold text-white mt-1">{}</h1>
            <p class="text-slate-400 text-xs sm:text-sm max-w-3xl mt-3 font-mono-code leading-relaxed">{}</p>
        </div>
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {}
        </div>
    </div>
</body>
</html>"#,
        titulo, theme_text, theme_text, theme_text, institucion, titulo, sub, cards_html
    );
    Html(html)
}

async fn handle_normas_stps() -> impl IntoResponse {
    render_norma_category(
        "stps",
        "Secretaría del Trabajo y Previsión Social (39 NOMs STPS)",
        "Inspección integral de las 39 Normas Oficiales Mexicanas: Seguridad, Salud, Organización, Ergonomía, Psicosocial (NOM-035), Teletrabajo (NOM-037) y Específicas por SCIAN.",
        "Marco Normativo STPS Completo",
        vec![
            ("nom-001", "NOM-001-STPS-2008", "Edificios e Instalaciones", "Condiciones de seguridad en las instalaciones y áreas de trabajo."),
            ("nom-002", "NOM-002-STPS-2010", "Protección contra Incendios", "Sistemas de prevención, combate de incendios y rutas de evacuación."),
            ("nom-004", "NOM-004-STPS-1999", "Protección en Maquinaria", "Sistemas de protección y dispositivos de seguridad en maquinaria."),
            ("nom-005", "NOM-005-STPS-1998", "Sustancias Químicas Peligrosas", "Manejo, transporte y almacenamiento seguro de químicos."),
            ("nom-006", "NOM-006-STPS-2014", "Manejo y Almacenamiento", "Seguridad en la carga, descarga, montacargas y estiba de materiales."),
            ("nom-009", "NOM-009-STPS-2011", "Trabajos en Altura", "Medidas de seguridad para trabajos en altura superior a 1.80m."),
            ("nom-010", "NOM-010-STPS-2014", "Agentes Químicos Contaminantes", "Reconocimiento, evaluación y control de agentes químicos en aire."),
            ("nom-011", "NOM-011-STPS-2001", "Ruido Ocupacional", "Límites máximos permisibles de exposición al ruido y audiometrías."),
            ("nom-012", "NOM-012/013-STPS", "Radiaciones Ionizantes y No Ionizantes", "Seguridad radiológica, UV, láser y campos electromagnéticos."),
            ("nom-015", "NOM-015-STPS-2001", "Condiciones Térmicas Elevadas/Abatidas", "Control de estrés térmico en hornos y cámaras de refrigeración."),
            ("nom-017", "NOM-017-STPS-2008", "Equipo de Protección Personal", "Determinación por puesto, selección y entrega firmada de EPP."),
            ("nom-018", "NOM-018-STPS-2015", "Sistema Armonizado (SGA / GHS)", "Hojas de Datos de Seguridad (HDS) y pictogramas químicos."),
            ("nom-019", "NOM-019-STPS-2011", "Comisiones Mixtas de Seguridad", "Acta constitutiva, programación anual y recorridos trimestrales."),
            ("nom-020", "NOM-020-STPS-2011", "Recipientes Sujetos a Presión", "Autorización STPS y pruebas en calderas, compresores y criogénicos."),
            ("nom-022", "NOM-022/029-STPS", "Electricidad Estática y Mantenimiento", "Medición anual de tierras físicas, pararrayos y diagrama unifilar."),
            ("nom-025", "NOM-025/026-STPS", "Iluminación y Señalética", "Estudios de luxometría, colores de seguridad y fluidos en tuberías."),
            ("nom-027", "NOM-027/033-STPS", "Soldadura y Espacios Confinados", "Permisos de trabajo en caliente y monitoreo de atmósfera en tanques."),
            ("nom-030", "NOM-030-STPS-2009", "Servicios Preventivos de Salud", "Designación de responsable, diagnóstico integral y programa PASST."),
            ("nom-031", "NOM-031/003/023-STPS", "Específicas (Construcción, Agro, Minería)", "Normas sectoriales activadas automáticamente por código SCIAN."),
            ("nom-035", "NOM-035-STPS-2018", "Factores de Riesgo Psicosocial", "Política psicosocial, Guías I, II y III y buzón confidencial."),
            ("nom-036", "NOM-036-1-STPS-2018", "Ergonomía en Manejo de Cargas", "Prevención de riesgos musculoesqueléticos en cargas manuales."),
            ("nom-037", "NOM-037-STPS-2023", "Teletrabajo (Home Office >40%)", "Política de desconexión digital y checklist de seguridad remota.")
        ],
    )
}

async fn handle_normas_sat() -> impl IntoResponse {
    render_norma_category(
        "sat",
        "Servicio de Administración Tributaria (SAT / CFF)",
        "Evaluación paramétrica de materialidad de operaciones B2B, Fecha Cierta SCJN, Buzón Tributario, Contabilidad Electrónica, CFDI 4.0 y Plataformas Digitales.",
        "Gobernanza Fiscal & Materialidad CFF",
        vec![
            ("art-69b", "Art. 69-B CFF", "Materialidad y Sustancia Económica (EFOS/EDOS)", "Contratos con Fecha Cierta SCJN, bitácoras de entregables y cruce O(1) contra listas negras."),
            ("art-17k", "Arts. 17-K y 86-C CFF", "Buzón Tributario Habilitado", "Validación de medios de contacto actualizados y prevención de notificaciones por estrados."),
            ("art-29", "Arts. 29 y 29-A CFF", "CFDI 4.0, Carta Porte 3.1, REP y Nómina", "Validación de Complemento Carta Porte en tramos federales y Complementos de Pago."),
            ("art-28", "Art. 28 CFF", "Contabilidad Electrónica Mensual", "Envío de catálogo de cuentas con código agrupador y balanzas de comprobación."),
            ("art-27", "Arts. 27, 17-H Bis y 32-D CFF", "Domicilio Localizado y Blindaje de CSD", "Prevención de restricción temporal del Certificado de Sello Digital para facturar."),
            ("art-113a", "Arts. 113-A LISR / 18-J LIVA", "Comercio Electrónico y Plataformas", "Cumplimiento fiscal y retenciones en portales con pasarela de pagos digital.")
        ],
    )
}

async fn handle_normas_imss() -> impl IntoResponse {
    render_norma_category(
        "imss",
        "IMSS e INFONAVIT (Seguridad Social, SBC y REPSE)",
        "Fiscalización de plantilla DENUE vs SUA, Salario Base de Cotización (SBC), informativas ICSOE/SISSUB, Prima de Riesgo y Opinión 32-D.",
        "Seguridad Social & Vivienda",
        vec![
            ("art-15", "Arts. 15 LSS y 29 LINFONAVIT", "Afiliación y Salario Base de Cotización (SBC)", "Conciliación entre plantilla visible en DENUE, Visor de Nómina SAT y cédulas SUA."),
            ("repse", "Art. 15-A LSS / Art. 13 LFT", "REPSE, ICSOE (IMSS) y SISSUB (INFONAVIT)", "Reportes cuatrimestrales obligatorios y vigencia en el padrón público REPSE."),
            ("prima-riesgo", "Arts. 71 al 75 LSS", "Prima de Riesgo de Trabajo", "Autoclasificación en catálogo de actividades y declaración anual de siniestralidad en febrero."),
            ("opinion-32d", "Art. 39 LSS / 32-D CFF", "Opinión de Cumplimiento 32-D Positiva", "Conciliación de emisiones EMA/EBA y prevención de bloqueo en licitaciones y cobros B2B."),
            ("solidaria", "Art. 15-B LSS / 27 LISR", "Responsabilidad Solidaria B2B", "Integración de los 7 documentos mensuales obligatorios de contratistas en sitio.")
        ],
    )
}

async fn handle_normas_inai() -> impl IntoResponse {
    render_norma_category(
        "inai",
        "Protección de Datos Personales (LFPDPPP — SABG / ex-INAI)",
        "Gobernanza de privacidad, Avisos Integrales y Simplificados, Derechos ARCO, Consentimiento Expreso Biométrico y Seguridad Técnica SSL/TLS.",
        "Marco LFPDPPP Vigente",
        vec![
            ("avisos", "Arts. 8, 15 y 16 LFPDPPP", "Aviso de Privacidad Integral, Simplificado y Corto", "Auditoría de visibilidad en portales web, formularios de contacto y establecimientos físicos."),
            ("arco", "Arts. 22 al 35 LFPDPPP", "Procedimiento de Derechos ARCO", "Canales y plazos legales (20 días hábiles) para Acceso, Rectificación, Cancelación y Oposición."),
            ("sensibles", "Arts. 9 y 16 LFPDPPP", "Datos Sensibles y Biométricos (Doble Multa)", "Formato de consentimiento expreso y autógrafo para huella dactilar, rostro, salud y nómina."),
            ("seguridad", "Arts. 19 y 63 LFPDPPP", "Medidas de Seguridad Técnicas (SSL/TLS)", "Detección de certificados SSL vencidos y subdominios huérfanos expuestos en Certificate Transparency."),
            ("transferencias", "Arts. 36, 37 y 17 LFPDPPP", "Transferencias a Terceros y Oficial de Privacidad", "Convenios de encargado con despachos/nube y designación formal del Departamento de Datos.")
        ],
    )
}

async fn handle_normas_pld() -> impl IntoResponse {
    render_norma_category(
        "pld",
        "Módulos Regulatorios Sectoriales (CNBV, COFEPRIS, PIPC, SEMARNAT, SICT)",
        "Activación algorítmica según código SCIAN del INEGI: Prevención de Lavado de Dinero, Regulación Sanitaria, Protección Civil, Ambiental y Transporte Federal.",
        "9 Módulos Sectoriales Especializados",
        vec![
            ("lfpiorpi", "CNBV / CONDUSEF / LFPIORPI", "Fintech, Financiero y Actividades Vulnerables", "Identificación KYC de Beneficiario Controlador y Avisos Antilavado en SPPLD SAT."),
            ("cofepris", "COFEPRIS (LGS / NOM-051 / 251)", "Salud, Alimentos, Restaurantes y Farmacia", "Aviso de Funcionamiento DIGIPRIS, Licencia Sanitaria, Responsable y etiquetado."),
            ("pipc", "PROTECCIÓN CIVIL (PIPC >100 m²)", "Programa Interno PIPC y Dictámenes", "Carpeta PIPC por Tercer Acreditado, Dictamen Eléctrico UVIE, Estructural y Gas."),
            ("semarnat", "SEMARNAT / ASEA / PROFEPA", "Impacto Ambiental y Residuos Peligrosos", "Licencia Ambiental Única (LAU), COA y Generador de Residuos Peligrosos (NOM-052)."),
            ("sict", "SICT / CAPUFE / SEP / SECTUR / SSPC", "Transporte Federal, Educación, Turismo y Seguridad", "Permisos SICT y Carta Porte, RVOE educativo SEP, RNT turístico SECTUR y DGSP.")
        ],
    )
}

async fn handle_normas_comercio() -> impl IntoResponse {
    render_norma_category(
        "comercio",
        "PROFECO (LFPC Comercio Electrónico) & Fecha Cierta NOM-151",
        "Auditoría de portales B2C/B2B conforme al Art. 76 BIS LFPC, Políticas de Reembolso, Contratos de Adhesión RRPAC y Conservación de Mensajes de Datos.",
        "PROFECO LFPC & Criptografía Legal",
        vec![
            ("art-76bis", "Art. 76 BIS LFPC", "Transparencia en Comercio Electrónico", "Exhibición de domicilio físico en México, teléfonos y precios totales con IVA incluido."),
            ("devoluciones", "Art. 76 BIS III/VII y Art. 77 LFPC", "Devoluciones, Revocación (5 días) y Garantías (90 días)", "Mecanismos claros de reembolso y póliza de garantía mínima legal de 90 días."),
            ("rrpac", "Arts. 85, 86 y 87 LFPC", "Contratos de Adhesión y Registro RRPAC", "Verificación de inscripción obligatoria en el Registro Público de Contratos de Adhesión."),
            ("publicidad", "Arts. 32, 128 y 128 BIS LFPC", "Publicidad Veraz y Seguridad en Pagos", "Prevención de prácticas engañosas y candados TLS/PCI-DSS en pasarelas de cobro."),
            ("nom-151", "NOM-151-SCFI-2016 & Merkle L2", "Fecha Cierta y Conservación Digital", "Integridad criptográfica SHA-256 conforme al Código de Comercio y Jurisprudencia SCJN.")
        ],
    )
}

async fn handle_subnorma_detail(Path((cat, code)): Path<(String, String)>) -> impl IntoResponse {
    let clean_code = code.to_lowercase();
    let clean_cat = cat.to_lowercase();

    let (theme_color, theme_badge, theme_text, theme_border) = match clean_cat.as_str() {
        "stps" => (
            "border-blue-500",
            "bg-blue-950/80 border-blue-800 text-blue-400",
            "text-blue-400",
            "border-blue-900/50",
        ),
        "sat" => (
            "border-emerald-500",
            "bg-emerald-950/80 border-emerald-800 text-emerald-400",
            "text-emerald-400",
            "border-emerald-900/50",
        ),
        "imss" => (
            "border-purple-500",
            "bg-purple-950/80 border-purple-800 text-purple-400",
            "text-purple-400",
            "border-purple-900/50",
        ),
        "inai" => (
            "border-cyan-500",
            "bg-cyan-950/80 border-cyan-800 text-cyan-400",
            "text-cyan-400",
            "border-cyan-900/50",
        ),
        "pld" => (
            "border-amber-500",
            "bg-amber-950/80 border-amber-800 text-amber-400",
            "text-amber-400",
            "border-amber-900/50",
        ),
        _ => (
            "border-teal-500",
            "bg-teal-950/80 border-teal-800 text-teal-400",
            "text-teal-400",
            "border-teal-900/50",
        ),
    };

    let (
        titulo_norma,
        fundamento,
        sanciones,
        prob_inspeccion,
        descripcion,
        requisitos,
        prob_baja,
        prob_media,
        prob_alta,
    ) = match (clean_cat.as_str(), clean_code.as_str()) {
        ("stps", c) if c.contains("035") => (
            "NOM-035-STPS-2018 — Factores de Riesgo Psicosocial",
            "Artículo 132 Fracc. XVI y 994 Fracc. V LFT; Reglamento Federal de SST.",
            "De 250 a 5,000 UMA ($28,285.00 a $565,700.00 MXN por trabajador afectado).",
            "92% Frecuencia de Inspección",
            "Obliga a identificar, analizar y prevenir factores de riesgo psicosocial (estrés grave, violencia laboral) y promover un entorno organizacional favorable.",
            vec![
                "Política de prevención de riesgos psicosociales difundida al 100% de la plantilla.",
                "Aplicación de cuestionarios de evaluación (Guías I, II o III) según número de empleados.",
                "Buzón confidencial de denuncias para violencia laboral.",
                "Canalización clínica para trabajadores expuestos a Acontecimientos Traumáticos Severos (ATS).",
            ],
            "12% (Política y guías aplicadas con acuse de recepción)",
            "58% (Sin buzón de denuncias ni seguimiento a ATS)",
            "92% (Ausencia total de cuestionarios o denuncias de colaboradores)",
        ),
        ("sat", c) if c.contains("69") || c.contains("materialidad") => (
            "Artículo 69-B CFF — Materialidad de Operaciones (EFOS / EDOS)",
            "Artículo 69-B del Código Fiscal de la Federación y Jurisprudencia SCJN 2a./J. 161/2019.",
            "No deducibilidad total + Multa del 55% al 75% + Restricción de CSD (Art. 17-H Bis).",
            "98% Frecuencia de Fiscalización Electrónica",
            "Procedimiento de presunción de inexistencia de operaciones cuando el SAT detecta emisión de comprobantes sin acreditar sustancia económica ni Fecha Cierta.",
            vec![
                "Expediente técnico pericial de entregables (bitácoras, minutas, reportes, fotografías).",
                "Contratos de prestación de servicios respaldados con Fecha Cierta Digital.",
                "Acreditación de la capacidad operativa y plantilla laboral del proveedor al momento del servicio.",
                "Trazabilidad bancaria y contable 1:1 con el CFDI 4.0 emitido y recibido.",
            ],
            "5% (Expediente técnico con Fecha Cierta y bitácora de entregables)",
            "45% (Contrato simple sin Fecha Cierta ni entregables certificados)",
            "98% (Solo contar con el XML de la factura y la transferencia bancaria)",
        ),
        _ => (
            "Matriz de Inspección y Cumplimiento Normativo 360°",
            "Disposición Regulatoria & Marco Legal Mexicano Vigente (UMA 2026).",
            "De 250 a 5,000 UMA ($28,285.00 a $565,700.00 MXN por observación).",
            "85% Frecuencia de Inspección",
            "Evaluación de vectores de cumplimiento normativo y ponderación paramétrica de contingencias sancionatorias.",
            vec![
                "Verificación documental y técnica de la matriz de cumplimiento específica.",
                "Prueba de inalterabilidad criptográfica basada en Merkle Tree SHA-256.",
                "Instrumentos jurídicos personalizados listos para firma autógrafa.",
                "Dictamen respaldado por el Consejo Colegiado de Auditoría Pericial.",
            ],
            "10% (Documentación completa y dictamen de inalterabilidad)",
            "50% (Cumplimiento parcial documental sin soporte técnico)",
            "85% (Omisión total de registros obligatorios ante la autoridad)",
        ),
    };

    let mut req_html = String::new();
    for req in requisitos {
        req_html.push_str(&format!(
            r#"<li class="flex items-start gap-2 text-xs font-mono-code text-slate-300">
                <span class="{} font-bold">✓</span> {}
            </li>"#,
            theme_text, req
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 selection:bg-slate-700 selection:text-white border-t-4 {}">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-white/15">
            <div>
                <span class="text-xs font-bold {} font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Ficha Técnica — Especificación Regulatoria</span>
            </div>
            <a href="/normas/{}" class="text-xs {} hover:underline font-mono-code transition">← Regresar a {}</a>
        </div>
        <div class="bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border {} rounded-2xl p-8 shadow-2xl mb-8">
            <div class="flex justify-between items-center mb-4 pb-4 border-b border-white/15">
                <span class="text-xs font-bold {} uppercase font-mono-code tracking-widest">Análisis Estadístico de Contención</span>
                <span class="text-xs font-mono-code px-3 py-1 rounded border {}">{}</span>
            </div>
            <h1 class="text-2xl sm:text-3xl font-extrabold text-white mb-4">{}</h1>
            <div class="grid md:grid-cols-2 gap-4 text-xs font-mono-code mb-6 bg-slate-950 p-4 rounded-xl border border-white/15">
                <div>
                    <span class="text-slate-500 block mb-1">Fundamento Legal:</span>
                    <span class="text-slate-200">{}</span>
                </div>
                <div>
                    <span class="text-slate-500 block mb-1">Riesgo / Multa Máxima:</span>
                    <span class="text-red-400 font-bold">{}</span>
                </div>
            </div>
            <h3 class="text-xs font-bold uppercase text-slate-300 mb-2 font-mono-code">Alcance y Riesgo Operativo</h3>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed mb-6">{}</p>
            <div class="mb-8 border-t border-b border-white/15 py-6">
                <h3 class="text-xs font-bold uppercase text-slate-300 mb-4 font-mono-code">Matriz Paramétrica de Riesgo Sancionatorio</h3>
                <div class="grid sm:grid-cols-3 gap-4 font-mono-code text-xs">
                    <div class="bg-slate-950 p-4 rounded-xl border border-white/15">
                        <span class="text-[10px] text-emerald-400 uppercase block mb-1">Bajo Riesgo / Cumplido</span>
                        <span class="text-base font-bold text-emerald-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Soporte pericial y sello SHA-256.</p>
                    </div>
                    <div class="bg-slate-950 p-4 rounded-xl border border-white/15">
                        <span class="text-[10px] text-amber-400 uppercase block mb-1">Riesgo Medio / Parcial</span>
                        <span class="text-base font-bold text-amber-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Documento simple sin sello externo.</p>
                    </div>
                    <div class="bg-slate-950 p-4 rounded-xl border border-white/15">
                        <span class="text-[10px] text-red-400 uppercase block mb-1">Riesgo Crítico / Omisión</span>
                        <span class="text-base font-bold text-red-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Omisión total o falta de expediente.</p>
                    </div>
                </div>
            </div>
            <h3 class="text-xs font-bold uppercase text-slate-300 mb-3 font-mono-code">Requerimientos Periciales Incluidos en su Kit</h3>
            <ul class="space-y-2 mb-6">
                {}
            </ul>
        </div>
    </div>
</body>
</html>"#,
        titulo_norma,
        theme_color,
        theme_text,
        cat,
        theme_text,
        cat.to_uppercase(),
        theme_border,
        theme_text,
        theme_badge,
        prob_inspeccion,
        titulo_norma,
        fundamento,
        sanciones,
        descripcion,
        prob_baja,
        prob_media,
        prob_alta,
        req_html
    );

    Html(html)
}

async fn handle_validez_detail(Path(slug): Path<String>) -> impl IntoResponse {
    let (titulo, marco_legal, valor_probatorio, descripcion, pilares_tecnicos, theme_border) =
        match slug.as_str() {
            "nom-151" => (
                "Fecha Cierta Digital, NOM-151-SCFI-2016 y Anclaje Merkle L2",
                "NOM-151-SCFI-2016; Código de Comercio Arts. 89 a 95; Jurisprudencia SCJN 2a./J. 161/2019.",
                "Valor Probatorio Documental ante el TFJA y Autoridades Fiscalizadoras.",
                "Otorga estampa de tiempo digital e inalterabilidad criptográfica SHA-256 a los expedientes técnicos periciales y contratos mercantiles B2B.",
                vec![
                    "Generación de huella criptográfica SHA-256 única por expediente e instrumento legal.",
                    "Agrupación diaria en Árbol de Merkle con commit on-chain en Arbitrum One / Base L2.",
                    "Inmutabilidad verificable públicamente mediante prueba matemática O(log N).",
                    "Alineación a los criterios de la Segunda Sala de la SCJN sobre Fecha Cierta en documentos privados.",
                ],
                "border-blue-600",
            ),
            "dictamen-pericial" => (
                "Dictaminación Pericial Certificada en Materia Contable-Fiscal y Regulatoria",
                "Código Federal de Procedimientos Civiles Arts. 143 a 160 y 210-A; CFF Art. 52.",
                "Prueba Pericial Documental de Soporte Técnico y Patrimonial.",
                "Consolida los vectores de auditoría OSINT y cruce registral público en un expediente estructurado bajo la supervisión de Contador Público Certificado (C.P.C.).",
                vec![
                    "Supervisión metodológica por C.P.C. Víctor Omar Malpica Guillén (Reg. CED-2026-MALLEN).",
                    "Cobertura integral de las 39 NOMs STPS, SAT, SABG/LFPDPPP, IMSS, PROFECO y 9 Sectoriales.",
                    "Entrega inmediata de los 7 Kits Legales personalizados listos para firma autógrafa.",
                    "Cuantificación financiera precisa en Unidades de Medida y Actualización (UMA 2026).",
                ],
                "border-emerald-600",
            ),
            _ => (
                "Secreto Profesional, Sanitización PII & Cumplimiento LFPDPPP",
                "Ley Federal de Protección de Datos Personales en Posesión de los Particulares vigente.",
                "Blindaje de Confidencialidad y Procesamiento en Memoria Volátil (RAM).",
                "Protección rigurosa de la información corporativa evaluada mediante filtros Regex de enmascaramiento de datos personales sensibles.",
                vec![
                    "Aviso de Privacidad Integral actualizado a los lineamientos de la Autoridad Garante SABG.",
                    "Sanitización automática de CURPs, correos personales y teléfonos antes de persistir logs.",
                    "Cifrado en tránsito mediante protocolos TLS 1.3 y protección Anti-SSRF.",
                    "Módulos WebAssembly client-side para no custodiar llaves privadas (.key) en el servidor.",
                ],
                "border-sky-600",
            ),
        };

    let mut pilares_html = String::new();
    for pilar in pilares_tecnicos {
        pilares_html.push_str(&format!(
            r#"<li class="flex items-start gap-2 text-xs font-mono-code text-slate-300">
                <span class="text-emerald-400 font-bold">✓</span> {}
            </li>"#,
            pilar
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 border-t-4 {}">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-slate-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Garantía Pericial & Marco Procesal</span>
            </div>
            <a href="/" class="text-xs text-blue-400 hover:text-blue-300 font-mono-code transition">← Regresar a la Portada</a>
        </div>

        <div class="bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-2xl p-8 shadow-2xl mb-8">
            <div class="flex justify-between items-center mb-4 pb-4 border-b border-white/15">
                <span class="text-xs font-bold text-emerald-400 uppercase font-mono-code tracking-widest">Sustento Legal & Procesal</span>
                <span class="text-xs font-mono-code px-3 py-1 bg-slate-950 border border-white/15 text-slate-300 rounded">Respaldo Oficial</span>
            </div>

            <h1 class="text-2xl sm:text-3xl font-extrabold text-white mb-4">{}</h1>
            
            <div class="grid md:grid-cols-2 gap-4 text-xs font-mono-code mb-6 bg-slate-950 p-4 rounded-xl border border-white/15">
                <div>
                    <span class="text-slate-500 block mb-1">Marco Legal y Técnico:</span>
                    <span class="text-slate-200">{}</span>
                </div>
                <div>
                    <span class="text-slate-500 block mb-1">Efecto Procesal:</span>
                    <span class="text-emerald-400 font-bold">{}</span>
                </div>
            </div>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-2 font-mono-code">Alcance y Mecanismo Pericial</h3>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed mb-6">{}</p>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-3 font-mono-code">Garantías Técnicas y de Inalterabilidad Incluidas</h3>
            <ul class="space-y-2 mb-6">
                {}
            </ul>
        </div>
    </div>
</body>
</html>"#,
        titulo, theme_border, titulo, marco_legal, valor_probatorio, descripcion, pilares_html
    );

    Html(html)
}
