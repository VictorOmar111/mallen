use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    DataProofRegistry,
    r#"[
        function registerBatch(bytes32 _merkleRoot) external
    ]"#
);

pub struct BlockchainRelayer {
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
}

impl BlockchainRelayer {
    pub fn new(rpc_url: &str, private_key: &str, contract_address: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(42161u64); // Arbitrum One
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let address: Address = contract_address.parse()?;

        Ok(Self {
            client,
            contract_address: address,
        })
    }

    pub fn parse_merkle_root(merkle_root: &str) -> Result<[u8; 32], Box<dyn std::error::Error + Send + Sync>> {
        let clean = merkle_root.trim_start_matches("0x");
        let bytes = hex::decode(clean)?;
        let mut root_bytes = [0u8; 32];
        if bytes.len() == 32 {
            root_bytes.copy_from_slice(&bytes);
            Ok(root_bytes)
        } else {
            Err("Merkle Root muss exakt 32 Bytes lang sein".into())
        }
    }

    pub async fn send_merkle_root(&self, merkle_root: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let root_bytes = Self::parse_merkle_root(merkle_root)?;
        let contract = DataProofRegistry::new(self.contract_address, self.client.clone());

        let tx = contract.register_batch(root_bytes);
        let pending_tx = tx.send().await?;
        let receipt: Option<TransactionReceipt> = pending_tx.await?;

        match receipt {
            Some(r) => Ok(format!("0x{:x}", r.transaction_hash)),
            None => Err("Transaktion fehlgeschlagen: Kein Receipt empfangen".into()),
        }
    }
}
