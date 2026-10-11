use sha2::{Digest, Sha256};

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
            return String::from("0000000000000000000000000000000000000000000000000000000000000000");
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
}
