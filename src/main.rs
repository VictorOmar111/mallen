mod engine;
mod sources;

use crate::engine::blockchain::BlockchainRelayer;
use crate::engine::enricher::WebEnricher;
use crate::engine::merkle::MerkleTree;
use crate::engine::models::EmpresaB2B;
use crate::engine::storage::StorageManager;
use crate::sources::inegi::InegiSource;
use crate::sources::GlobalSource;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::{sleep, Duration};

const MAX_CONCURRENT_WORKERS: usize = 2_000;
const CHANNEL_BUFFER_SIZE: usize = 50_000;
const BATCH_SIZE: usize = 1_000;
const OUTPUT_FILE: &str = "resultado_denue_enriquecido.jsonl";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando Motor B2B DENUE + Verificación Merkle en AWS EC2...");

    let total_procesadas = Arc::new(AtomicUsize::new(0));
    let total_exitosas = Arc::new(AtomicUsize::new(0));

    let (tx, mut rx) = mpsc::channel::<EmpresaB2B>(CHANNEL_BUFFER_SIZE);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_WORKERS));
    let enricher = Arc::new(WebEnricher::new());

    let relayer: Option<Arc<BlockchainRelayer>> = match (
        std::env::var("RPC_URL"),
        std::env::var("PRIVATE_KEY"),
        std::env::var("CONTRACT_ADDRESS"),
    ) {
        (Ok(rpc_url), Ok(private_key), Ok(contract_address)) => {
            match BlockchainRelayer::new(&rpc_url, &private_key, &contract_address) {
                Ok(relayer) => Some(Arc::new(relayer)),
                Err(err) => {
                    println!(
                        "⚠️️ Blockchain-Relayer deaktiviert (Initialisierung fehlgeschlagen): {}",
                        err
                    );
                    None
                }
            }
        }
        _ => {
            println!(
                "⚠️️ Blockchain-Relayer deaktiviert (RPC_URL, PRIVATE_KEY oder CONTRACT_ADDRESS nicht gesetzt)."
            );
            None
        }
    };

    let inegi = InegiSource::new();

    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || {
            let mut empresas: Vec<EmpresaB2B> = Vec::new();
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_path("denue_inegi.csv")?;

            for record in reader.records() {
                let record = record?;
                if let Some(empresa) = inegi.parse_row(&record) {
                    empresas.push(empresa);
                }
            }

            Ok::<Vec<EmpresaB2B>, Box<dyn std::error::Error + Send + Sync>>(empresas)
        })
        .await;

        if let Ok(Ok(empresas)) = result {
            for empresa in empresas {
                if tx.send(empresa).await.is_err() {
                    break;
                }
            }
        }
    });

    let procesadas_clone = Arc::clone(&total_procesadas);
    let exitosas_clone = Arc::clone(&total_exitosas);
    let semaphore_clone = Arc::clone(&semaphore);
    let enricher_clone = Arc::clone(&enricher);
    let relayer_clone = relayer.clone();

    let consumer_handle = tokio::spawn(async move {
        let mut batch: Vec<EmpresaB2B> = Vec::with_capacity(BATCH_SIZE);
        let mut batch_count = 0;

        while let Some(mut empresa) = rx.recv().await {
            let permit = Arc::clone(&semaphore_clone).acquire_owned().await.unwrap();
            let procesadas = Arc::clone(&procesadas_clone);
            let exitosas = Arc::clone(&exitosas_clone);
            let enricher = Arc::clone(&enricher_clone);

            let handle = tokio::spawn(async move {
                procesadas.fetch_add(1, Ordering::Relaxed);

                if let Some(url) = empresa.sitio_web.clone() {
                    let (status, email, body) = enricher.inspect_site(&url).await;
                    empresa.status_web = status.map(|s| s.to_string());
                    empresa.e_mail_verificado = email;

                    if status == Some(200) {
                        exitosas.fetch_add(1, Ordering::Relaxed);
                    }

                    // Pre-Auditoría: Risikopunktzahl und Etiketten berechnen.
                    let (risk_score, audit_flags) = enricher
                        .calculate_pre_audit_risk(&url, body.as_deref().unwrap_or(""));
                    empresa.risk_score = risk_score;
                    empresa.audit_flags = audit_flags;
                }

                drop(permit);
                empresa
            });

            // Warten, damit die angereicherten Felder persistiert werden können.
            if let Ok(enriched) = handle.await {
                batch.push(enriched);
            }

            if batch.len() >= BATCH_SIZE {
                batch_count += 1;
                let leaf_hashes: Vec<String> = batch
                    .iter()
                    .map(|e| {
                        MerkleTree::hash_leaf(&format!(
                            "{}:{}:{}",
                            e.id,
                            e.nombre,
                            e.direccion.as_deref().unwrap_or("")
                        ))
                    })
                    .collect();
                let merkle_root = MerkleTree::compute_root(&leaf_hashes);
                println!(
                    "🔐 [Lote #{}] Merkle Root verificado: 0x{}",
                    batch_count, merkle_root
                );

                if let Some(relayer) = relayer_clone.as_ref() {
                    match relayer.send_merkle_root(&merkle_root).await {
                        Ok(tx_hash) => println!(
                            "⛓️  [Lote #{}] Merkle Root on-chain gesendet: {}",
                            batch_count, tx_hash
                        ),
                        Err(err) => eprintln!(
                            "⚠️  Fehler beim Senden des Merkle Roots für Lote #{}: {}",
                            batch_count, err
                        ),
                    }
                }

                if let Err(err) =
                    StorageManager::save_batch(OUTPUT_FILE, &batch, &merkle_root).await
                {
                    eprintln!("⚠️  Fehler beim Speichern des Lotes #{}: {}", batch_count, err);
                }

                batch.clear();
            }
        }

        if !batch.is_empty() {
            batch_count += 1;
            let leaf_hashes: Vec<String> = batch
                .iter()
                .map(|e| {
                    MerkleTree::hash_leaf(&format!(
                        "{}:{}:{}",
                        e.id,
                        e.nombre,
                        e.direccion.as_deref().unwrap_or("")
                    ))
                })
                .collect();
            let merkle_root = MerkleTree::compute_root(&leaf_hashes);
            println!(
                "🔐 [Lote Final #{}] Merkle Root verificado: 0x{}",
                batch_count, merkle_root
            );

            if let Some(relayer) = relayer_clone.as_ref() {
                match relayer.send_merkle_root(&merkle_root).await {
                    Ok(tx_hash) => println!(
                        "⛓️  [Lote Final #{}] Merkle Root on-chain gesendet: {}",
                        batch_count, tx_hash
                    ),
                    Err(err) => eprintln!(
                        "⚠️  Fehler beim Senden des finalen Merkle Roots: {}",
                        err
                    ),
                }
            }

            if let Err(err) = StorageManager::save_batch(OUTPUT_FILE, &batch, &merkle_root).await {
                eprintln!("⚠️  Fehler beim Speichern des finalen Lotes: {}", err);
            }
        }
    });

    let procesadas_mon = Arc::clone(&total_procesadas);
    let exitosas_mon = Arc::clone(&total_exitosas);
    let monitor_handle = tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            println!(
                "📊 [Estado] Registros: {} | Validados: {}",
                procesadas_mon.load(Ordering::Relaxed),
                exitosas_mon.load(Ordering::Relaxed)
            );
        }
    });

    let _ = consumer_handle.await;
    monitor_handle.abort();

    println!("✅ Proceso completado con integridad Merkle validada.");
    Ok(())
}
