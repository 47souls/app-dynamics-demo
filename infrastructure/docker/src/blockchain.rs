use std::sync::Arc;
use ethers_core::k256::ecdsa::SigningKey;
use anyhow::Result;
use ethers::{
    providers::Provider,
    signers::{Wallet, MnemonicBuilder, coins_bip39::English, Signer}, middleware::SignerMiddleware,
};
use ethers_providers::Http;

pub fn create_wallet() -> Result<Wallet<SigningKey<>>> {
    // TODO: this key phrase should not be here
    let phrase: &str = "pattern soldier uncle salad evolve pencil general coconut jewel pig carpet local";
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .build()?;
    // have to manually specify the chain id
    // as by default it's 1, which fails transactions
    let chain_id: u64 = 56;
    let wallet = wallet.with_chain_id(chain_id);
    Ok(wallet)
}

pub async fn get_client(provider_url: &str) -> Arc<Provider<Http>> {
    let provider = Provider::<Http>::try_from(provider_url).unwrap();
    let client = Arc::new(provider);
    client
}

pub async fn get_signer_client(provider_url: &str, wallet: Wallet<SigningKey<>>) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey<>>> {
    let provider = Provider::<Http>::try_from(provider_url).unwrap();
    let signer_provider = SignerMiddleware::new(provider.clone(), wallet.clone());
    signer_provider
}