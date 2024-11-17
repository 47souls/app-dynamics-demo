use crate::constants;

use crate::blockchain;
use std::sync::Arc;

use std::result::Result::Ok;
use ethers_core::k256::ecdsa::SigningKey;

use ethers_providers::Http;
use ethers_contract::abigen;

use anyhow::Result;
use ethers::{
    providers::Provider,
    types::Address,
    signers::Wallet, middleware::SignerMiddleware,
};
use ethers_providers::Ws;

pub async fn create_event_listening_structs() -> Result<(PancakePredictionInterface<Provider<Ws>>, PredictionDataAggregator<Provider<Ws>>)> {
    let provider = Provider::<Ws>::connect(constants::NODEREAL_WSS_PROVIDER_URL).await?;
    let client = Arc::new(provider);

    let prediction_contract_address: Address = constants::PANCAKE_PREDICTION_ADDRESS.parse().unwrap();
    let prediction_contract: PancakePredictionInterface<Provider<Ws>> = PancakePredictionInterface::new(prediction_contract_address, client.clone());

    let prediction_data_aggregator_contract_address: Address = constants::PREDICTION_DATA_AGGREGATOR_ADDRESS.parse().unwrap();
    let prediction_data_aggregator_contract: PredictionDataAggregator<Provider<Ws>> = PredictionDataAggregator::new(prediction_data_aggregator_contract_address, client.clone());

    Ok((prediction_contract, prediction_data_aggregator_contract))
}

pub async fn create_blockchain_clients(provider_url: &str) -> Result<(Arc<Provider<Http>>, SignerMiddleware<Provider<Http>, Wallet<SigningKey>>)> {
    let wallet = blockchain::create_wallet().unwrap();
    let client: Arc<Provider<Http>> = blockchain::get_client(provider_url).await;
    let signer_client = blockchain::get_signer_client(provider_url, wallet).await;
    Ok((client, signer_client))
}

// generates abi
abigen!(PancakePredictionInterface, "./abi/prediction-contract-abi.json", derives(serde::Deserialize));
abigen!(PredictionDataAggregator, "./abi/prediction-data-aggregator-abi.json");