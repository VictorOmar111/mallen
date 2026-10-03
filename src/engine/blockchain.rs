use ethers::prelude::*;
use ethers::types::Bytes;
use std::sync::Arc;

abigen!(
    MerkleRegistry,
    r#"[
        function registerBatch(bytes32 merkleRoot) external
    ]"#,
);

/// Sendet Merkle-Roots verarbeiteter Lote an einen Smart Contract auf einer EVM-Kette.
pub struct BlockchainRelayer {
    contract: MerkleRegistry<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl BlockchainRelayer {
    /// Erstellt einen neuen Relayer aus RPC-URL, privatem Schlüssel und Contract-Adresse.
    pub fn new(
        rpc_url: &str,
        private_key: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let address: Address = contract_address.parse()?;
        let contract = MerkleRegistry::new(address, client);

        Ok(Self { contract })
    }

    /// Wandelt einen Hex-String (ohne "0x") in ein bytes32 um und ruft `registerBatch` auf.
    pub async fn send_merkle_root(
        &self,
        merkle_root: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let clean = merkle_root.trim_start_matches("0x");
        let raw = hex::decode(clean)?;

        if raw.len() != 32 {
            return Err(format!(
                "Merkle Root muss 32 Bytes (bytes32) lang sein, war {} Bytes",
                raw.len()
            )
            .into());
        }

        let mut root = [0u8; 32];
        root.copy_from_slice(&raw);
        let root_bytes32: [u8; 32] = root;

        let pending = self.contract.register_batch(root_bytes32).send().await?;
        let receipt = pending.await?;

        let tx_hash = receipt
            .map(|r| format!("0x{:x}", r.transaction_hash))
            .unwrap_or_else(|| "0x0".to_string());

        Ok(tx_hash)
    }
}
