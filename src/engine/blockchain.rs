//! Cliente RPC ligero basado en `alloy` para anclar el Merkle Root diario en L2 (Arbitrum One / Base),
//! dotando a los dictámenes de Fecha Cierta inmutable verificable.

use alloy::{
    network::EthereumWallet,
    primitives::{Address, FixedBytes},
    providers::ProviderBuilder,
    rpc::types::TransactionReceipt,
    signers::{local::PrivateKeySigner, Signer},
    sol,
};
use std::str::FromStr;
use url::Url;

sol! {
    #[sol(rpc)]
    contract DataProofRegistry {
        function registerBatch(bytes32 _merkleRoot) external;
    }
}

pub struct BlockchainRelayer {
    rpc_url: Url,
    wallet: EthereumWallet,
    contract_address: Address,
}

impl BlockchainRelayer {
    pub fn new(
        rpc_url: &str,
        private_key: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let parsed_url = Url::parse(rpc_url)?;
        // Arbitrum One Chain ID = 42161 (Base Mainnet = 8453)
        let signer = PrivateKeySigner::from_str(private_key.trim_start_matches("0x"))?
            .with_chain_id(Some(42161));
        let wallet = EthereumWallet::from(signer);
        let address = Address::from_str(contract_address)?;

        Ok(Self {
            rpc_url: parsed_url,
            wallet,
            contract_address: address,
        })
    }

    pub fn parse_merkle_root(
        merkle_root: &str,
    ) -> Result<[u8; 32], Box<dyn std::error::Error + Send + Sync>> {
        let clean = merkle_root.trim_start_matches("0x");
        let bytes = hex::decode(clean)?;
        let mut root_bytes = [0u8; 32];
        if bytes.len() == 32 {
            root_bytes.copy_from_slice(&bytes);
            Ok(root_bytes)
        } else {
            Err("El Merkle Root debe tener exactamente 32 bytes (64 caracteres hex)".into())
        }
    }

    pub async fn send_merkle_root(
        &self,
        merkle_root: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let root_bytes = Self::parse_merkle_root(merkle_root)?;
        let fixed_root = FixedBytes::<32>::from(root_bytes);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(self.wallet.clone())
            .on_http(self.rpc_url.clone());

        let contract = DataProofRegistry::new(self.contract_address, provider);
        let call = contract.registerBatch(fixed_root);
        let pending_tx = call.send().await?;
        let receipt: TransactionReceipt = pending_tx.get_receipt().await?;

        Ok(format!("0x{:x}", receipt.transaction_hash))
    }
}
