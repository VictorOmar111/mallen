use worker_inegi_engine::engine::accountant::{apply_accountant_signature, AccountantProfile};
use worker_inegi_engine::engine::bundle_generator::generate_custom_bundle;
use worker_inegi_engine::engine::enricher::fetch_dof_regulatory_triggers;
use worker_inegi_engine::engine::http_form_bot::HttpFormBot;
use worker_inegi_engine::engine::merkle::MerkleTree;
use worker_inegi_engine::engine::models::EmpresaB2B;
use worker_inegi_engine::engine::network::NetworkClientManager;
use worker_inegi_engine::engine::osint_scanner::run_osint_scan;
use worker_inegi_engine::engine::risk_evaluator::evaluate_risk;
use worker_inegi_engine::persistence::storage::LocalDatabase;

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    println!(" Iniciando Motor Firma Pericial Mallen + SQLite + OSINT Dual-TLS + Merkle L2 (mallen.lat)...");

    let db = Arc::new(LocalDatabase::init("b2b_audits.db")?);
    let shared_client = NetworkClientManager::build_client(None)?;

    // 0. Escaneo Matutino del Diario Oficial de la Federación (Honey Pot Predictivo)
    let dof_alerts = fetch_dof_regulatory_triggers(&shared_client).await;
    if !dof_alerts.is_empty() {
        println!(
            " [DOF HONEY POT] {} alertas regulatorias detectadas hoy en el Diario Oficial.",
            dof_alerts.len()
        );
    }

    if !Path::new("denue_prueba.csv").exists() {
        let mut sample = File::create("denue_prueba.csv")?;
        writeln!(
            sample,
            "id,nombre,actividad,telefono,email,direccion,estado,sitio_web"
        )?;
        writeln!(
            sample,
            "EST850628K51,ELECTRONICA STEREN SA DE CV,434222 Comercio al por mayor electronico,5553542200,info@steren.com.mx,251 y mas personas Azcapotzalco CDMX,CDMX,steren.com.mx"
        )?;
    }

    let file = File::open("denue_prueba.csv")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    if lines.is_empty() {
        println!("️ Archivo CSV 'denue_prueba.csv' vacío.");
        return Ok(());
    }

    let _header = lines.remove(0);
    println!(
        " {} empresas cargadas para auditoría 360° y prospección controlada...",
        lines.len()
    );

    let timestamp_now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let accountant_profile = AccountantProfile::default();
    let concurrency_guard = Arc::new(Semaphore::new(16));
    let mut tasks = JoinSet::new();

    for line in lines {
        let fields: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        if fields.len() >= 7 {
            let client_clone = shared_client.clone();
            let profile_clone = accountant_profile.clone();
            let db_clone = db.clone();
            let sem_clone = concurrency_guard.clone();

            tasks.spawn(async move {
                let _permit = sem_clone.acquire_owned().await.ok();
                let id = fields[0].clone();

                let mut e = EmpresaB2B {
                    id: id.clone(),
                    nombre: fields[1].clone(),
                    actividad: fields[2].clone(),
                    telefono: fields[3].clone(),
                    email: fields[4].clone(),
                    direccion: fields[5].clone(),
                    estado: fields[6].clone(),
                    sitio_web: if fields.len() > 7 && !fields[7].trim().is_empty() {
                        Some(fields[7].trim().to_string())
                    } else {
                        None
                    },
                    status_web: None,
                    e_mail_verificado: None,
                    country_code: "MX".to_string(),
                    tax_id: Some(if id.len() >= 12 {
                        id.clone()
                    } else {
                        format!("TAX_{}", id)
                    }),
                    language: Some("es".to_string()),
                    audit_hash: String::new(),
                    risk_score: 0,
                    audit_flags: Vec::new(),
                    estimated_fine_mxn: 0.0,
                    shield_price_usd: 0.0,
                    accountant_signature: None,
                };

                // 1. Escaneo OSINT Dual-TLS en tiempo real
                let osint_res =
                    run_osint_scan(&client_clone, e.sitio_web.as_deref(), &e.nombre).await;
                if osint_res.http_status > 0 {
                    e.status_web = Some(osint_res.http_status);
                }

                // 2. Evaluación Regulatoria de Riesgo (39 NOMs STPS, SAT, SABG/INAI, IMSS, PROFECO y 9 Sectoriales)
                evaluate_risk(&mut e);

                // 3. Prospección Comercial Automática vía HttpFormBot
                let mut status_pros = "SIN_FORMULARIO";
                if osint_res.has_form_inputs {
                    if let Some(target_url) = &osint_res.contact_form_url {
                        let pros_res = HttpFormBot::submit_form_direct(
                            &client_clone,
                            target_url,
                            &e.nombre,
                            e.estimated_fine_mxn,
                        )
                        .await;
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
                hasher.update(
                    format!("{}:{}:{}", e.id, e.estimated_fine_mxn, timestamp_now).as_bytes(),
                );
                e.audit_hash = format!("0x{}", &format!("{:x}", hasher.finalize())[..16]);

                // 6. Generación del Paquete .ZIP (Dictamen + 10 Certificados + 7 Kits Legales + Manifiesto)
                let zip_result =
                    generate_custom_bundle(&e, &osint_res).map_err(|err| err.to_string());

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
                " [DB PERSISTED | {}] {} -> Contingencia: ${:.2} MXN | Zip: {:?}",
                status_pros,
                e.nombre,
                e.estimated_fine_mxn,
                zip_res.unwrap_or_default()
            );
            empresas_auditadas.push(e);
        }
    }

    let hashes: Vec<String> = empresas_auditadas
        .iter()
        .map(|e| MerkleTree::hash_leaf(&e.audit_hash))
        .collect();
    let global_merkle_root = format!("0x{}", MerkleTree::compute_root(&hashes));
    println!(" Merkle Root Global 256-bit del Lote: {}", global_merkle_root);
    println!("️ Proceso completado en: {:.2?}", start_time.elapsed());

    Ok(())
}
