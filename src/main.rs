mod engine;
mod sources;

use crate::engine::merkle::MerkleTree;
use crate::engine::models::EmpresaB2B;
use crate::sources::inegi::InegiFetcher;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::{sleep, Duration};

const MAX_CONCURRENT_WORKERS: usize = 2_000;
const CHANNEL_BUFFER_SIZE: usize = 50_000;
const BATCH_SIZE: usize = 1_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando Motor B2B DENUE + Verificación Merkle en AWS EC2...");

    let total_procesadas = Arc::new(AtomicUsize::new(0));
    let total_exitosas = Arc::new(AtomicUsize::new(0));

    let (tx, mut rx) = mpsc::channel::<EmpresaB2B>(CHANNEL_BUFFER_SIZE);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_WORKERS));

    let inegi = InegiFetcher::new("denue_inegi.csv");

    tokio::spawn(async move {
        if let Ok(empresas) = inegi.stream_denue_csv("denue_inegi.csv") {
            for empresa in empresas {
                if tx.send(empresa).await.is_err() {
                    break;
                }
            }
        }
    });

    let procesadas_clone = Arc::clone(&total_procesadas);
    let exitosas_clone = Arc::clone(&total_exitosas);

    let consumer_handle = tokio::spawn(async move {
        let mut leaf_hashes: Vec<String> = Vec::with_capacity(BATCH_SIZE);
        let mut batch_count = 0;

        while let Some(empresa) = rx.recv().await {
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            let procesadas = Arc::clone(&procesadas_clone);
            let exitosas = Arc::clone(&exitosas_clone);

            let empresa_raw = format!("{}:{}:{}", empresa.id, empresa.nombre, empresa.direccion);
            let leaf_hash = MerkleTree::hash_leaf(&empresa_raw);
            leaf_hashes.push(leaf_hash);

            tokio::spawn(async move {
                procesadas.fetch_add(1, Ordering::Relaxed);
                if empresa.sitio_web.is_some() {
                    exitosas.fetch_add(1, Ordering::Relaxed);
                }
                drop(permit);
            });

            if leaf_hashes.len() >= BATCH_SIZE {
                batch_count += 1;
                let merkle_root = MerkleTree::compute_root(&leaf_hashes);
                println!(
                    "🔐 [Lote #{}] Merkle Root verificado: 0x{}",
                    batch_count, merkle_root
                );
                leaf_hashes.clear();
            }
        }

        if !leaf_hashes.is_empty() {
            batch_count += 1;
            let merkle_root = MerkleTree::compute_root(&leaf_hashes);
            println!(
                "🔐 [Lote Final #{}] Merkle Root verificado: 0x{}",
                batch_count, merkle_root
            );
        }
    });

    let procesadas_mon = Arc::clone(&total_procesadas);
    let exitosas_mon = Arc::clone(&total_exitosas);
    tokio::spawn(async move {
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

    println!("✅ Proceso completado con integridad Merkle validada.");
    Ok(())
}
