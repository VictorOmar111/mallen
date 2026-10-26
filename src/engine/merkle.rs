//! Motor de Árboles de Merkle y Batching Diario para Fecha Cierta Digital en L2 (Arbitrum / Base).
//! Mantiene `hash_leaf` y `compute_root` 100% compatibles y agrega generación y verificación
//! de pruebas criptográficas individuales (`MerkleProof`) para `mallen.lat/verify/:hash`.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofStep {
    pub sibling_hash: String,
    pub is_left_sibling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_hash: String,
    pub root_hash: String,
    pub path: Vec<MerkleProofStep>,
}

pub struct MerkleTree;

impl MerkleTree {
    pub fn hash_leaf(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn hash_nodes(left: &str, right: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn compute_root(leaves: &[String]) -> String {
        if leaves.is_empty() {
            return String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            );
        }

        let mut current_level: Vec<String> = leaves.to_vec();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(Self::hash_nodes(&chunk[0], &chunk[1]));
                } else {
                    next_level.push(Self::hash_nodes(&chunk[0], &chunk[0]));
                }
            }
            current_level = next_level;
        }

        current_level[0].clone()
    }

    /// Genera la prueba criptográfica individual (`MerkleProof`) para demostrar que un dictamen
    /// específico forma parte del Merkle Root diario anclado en Arbitrum / Base L2.
    pub fn generate_proof(leaves: &[String], target_leaf_hash: &str) -> Option<MerkleProof> {
        let mut idx = leaves.iter().position(|h| h == target_leaf_hash)?;
        let mut current_level: Vec<String> = leaves.to_vec();
        let mut path = Vec::new();

        while current_level.len() > 1 {
            let is_right_node = idx % 2 == 1;
            let sibling_idx = if is_right_node {
                idx - 1
            } else if idx + 1 < current_level.len() {
                idx + 1
            } else {
                idx
            };

            path.push(MerkleProofStep {
                sibling_hash: current_level[sibling_idx].clone(),
                is_left_sibling: is_right_node,
            });

            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(Self::hash_nodes(&chunk[0], &chunk[1]));
                } else {
                    next_level.push(Self::hash_nodes(&chunk[0], &chunk[0]));
                }
            }

            idx /= 2;
            current_level = next_level;
        }

        Some(MerkleProof {
            leaf_hash: target_leaf_hash.to_string(),
            root_hash: current_level[0].clone(),
            path,
        })
    }

    /// Verifica matemáticamente en O(log N) que un `MerkleProof` reconstruye exactamente el `root_hash`.
    pub fn verify_proof(proof: &MerkleProof) -> bool {
        let mut computed = proof.leaf_hash.clone();
        for step in &proof.path {
            if step.is_left_sibling {
                computed = Self::hash_nodes(&step.sibling_hash, &computed);
            } else {
                computed = Self::hash_nodes(&computed, &step.sibling_hash);
            }
        }
        computed == proof.root_hash
    }
}
