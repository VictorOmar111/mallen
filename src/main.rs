mod engine;
mod persistence;

use crate::engine::accountant::{apply_accountant_signature, AccountantProfile};
use crate::engine::bundle_generator::generate_custom_bundle;
use crate::engine::http_form_bot::HttpFormBot;
use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::run_osint_scan;
use crate::engine::risk_evaluator::evaluate_risk;
use crate::persistence::storage::LocalDatabase;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::task::JoinSet;

fn compute_merkle_root(hashes: &[String]) -> String {
    if hashes.is_empty() {
        return "0x00000000000000000000000000000000".to_string();
    }
    let mut current = hashes.to_vec();
    while current.len() > 1 {
        let mut next = Vec::new();
        for chunk in current.chunks(2) {
            let mut hasher = Sha256::new();
            if chunk.len() == 2 {
                hasher.update(format!("{}{}", chunk[0], chunk[1]).as_bytes());
            } else {
                hasher.update(format!("{}{}", chunk[0], chunk[0]).as_bytes());
            }
            next.push(format!("{:x}", hasher.finalize()));
        }
        current = next;
    }
    format!("0x{}", &current[0][..16])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    println!("🚀 Iniciando Motor Firma Pericial Mallen + SQLite + OSINT + Merkle (mallen.lat)...");

    let db = Arc::new(LocalDatabase::init("b2b_audits.db")?);

    let shared_client = Client::builder()
        .timeout(Duration::from_millis(3000))
        .pool_max_idle_per_host(100)
        .danger_accept_invalid_certs(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) FirmaPericialMallen/1.0 (https://mallen.lat)")
        .build()?;

    let file = File::open("denue_prueba.csv")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    if lines.is_empty() {
        println!("⚠️ Archivo CSV 'denue_prueba.csv' vacío.");
        return Ok(());
    }

    let _header = lines.remove(0);
    println!("📥 {} empresas cargadas para proceso de auditoría y prospección...", lines.len());

    let timestamp_now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let accountant_profile = AccountantProfile::default();
    let mut tasks = JoinSet::new();

    for line in lines {
        let fields: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        if fields.len() >= 7 {
            let client_clone = shared_client.clone();
            let profile_clone = accountant_profile.clone();
            let db_clone = db.clone();

            tasks.spawn(async move {
                let id = fields[0].clone();

                let mut e = EmpresaB2B {
                    id: id.clone(),
                    nombre: fields[1].clone(),
                    actividad: fields[2].clone(),
                    telefono: fields[3].clone(),
                    email: fields[4].clone(),
                    direccion: fields[5].clone(),
                    estado: fields[6].clone(),
                    sitio_web: if fields.len() > 7 && !fields[7].is_empty() { Some(fields[7].clone()) } else { None },
                    status_web: None,
                    e_mail_verificado: None,
                    country_code: "MX".to_string(),
                    tax_id: Some(format!("TAX_{}", id)),
                    language: Some("es".to_string()),
                    audit_hash: String::new(),
                    risk_score: 0,
                    audit_flags: Vec::new(),
                    estimated_fine_mxn: 0.0,
                    shield_price_usd: 0.0,
                    accountant_signature: None,
                };

                // 1. Escaneo OSINT en tiempo real
                let osint_res = run_osint_scan(&client_clone, e.sitio_web.as_deref(), &e.nombre).await;
                if osint_res.http_status > 0 {
                    e.status_web = Some(osint_res.http_status);
                }

                // 2. Evaluación Regulatoria de Riesgo (6 Sectores México)
                evaluate_risk(&mut e);

                // 3. Prospección Comercial Automática vía HttpFormBot (Inyección Formulario Web)
                let mut status_pros = "SIN_FORMULARIO";
                if osint_res.has_form_inputs {
                    if let Some(target_url) = &osint_res.contact_form_url {
                        let pros_res = HttpFormBot::submit_form_direct(&client_clone, target_url, &e.nombre, e.estimated_fine_mxn).await;
                        status_pros = match pros_res.as_str() {
                            "PROSPECCION_ENVIADA_EXITO" => "ENVIADO_EXITO",
                            _ => "FALLIDO_ENVIO",
                        };
                    }
                }

                // 4. Firma Pericial Contable C.P.C.
                let tax_str = e.tax_id.clone().unwrap_or_default();
                e.accountant_signature = Some(apply_accountant_signature(&profile_clone, &tax_str));

                // 5. Hash Único de Auditoría SHA-256
                let mut hasher = Sha256::new();
                hasher.update(format!("{}{}{}", e.id, e.estimated_fine_mxn, timestamp_now).as_bytes());
                e.audit_hash = format!("0x{}", &format!("{:x}", hasher.finalize())[..12]);

                // 6. Generación del Paquete .ZIP (11 PDFs: Dictamen 2 Páginas + 10 Certificados)
                let zip_result = generate_custom_bundle(&e, &osint_res).map_err(|err| err.to_string());

                // 7. Persistir en SQLite (b2b_audits.db)
                let _ = db_clone.save_empresa(&e, status_pros, timestamp_now);

                (e, zip_result, status_pros.to_string())
            });
        }
    }

    let mut empresas_auditadas: Vec<EmpresaB2B> = Vec::new();
    while let Some(res) = tasks.join_next().await {
        if let Ok((e, zip_res, status_pros)) = res {
            println!(
                "💾 [DB PERSISTED | {}] {} -> Multa: ${:.2} MXN | Zip: {:?}",
                status_pros, e.nombre, e.estimated_fine_mxn, zip_res.unwrap_or_default()
            );
            empresas_auditadas.push(e);
        }
    }

    let hashes: Vec<String> = empresas_auditadas.iter().map(|e| e.audit_hash.clone()).collect();
    let global_merkle_root = compute_merkle_root(&hashes);
    println!("🌳 Merkle Root Global del Lote: {}", global_merkle_root);

    let duration = start_time.elapsed();
    println!("⏱️ Proceso completado en: {:.2?}", duration);
    println!("🗄️ Todos los registros guardados en 'b2b_audits.db'.");

    Ok(())
}
