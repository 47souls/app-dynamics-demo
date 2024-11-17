use anyhow::{Result, anyhow};
use ethers_contract::abigen;
use ethers_core::abi::Address;
use crate::constants;
use crate::prediction;
use crate::utils;

pub async fn get_latest_chainlink_round_price() -> Result<f64> {
    // can I avoid creating this client?
    let (client, _) = prediction::create_blockchain_clients(constants::NODEREAL_PROVIDER_URL).await?;
    let contract_address: Address = constants::CHAINLINK_AGGREGATOR_ADDRESS.parse().unwrap();
    
    let contract = ChainlinkPriceAggregatorBNBUSD::new(contract_address, client);
    let latest_round_data_result = contract.latest_round_data().await;
    match latest_round_data_result {
        Ok((_, price, _, _, _)) => {
            let price = utils::i256_to_f64(price);
            Ok(price)
        }, 
        Err(e) => {
            return Err(anyhow!(format!("Failed to retrieve chainlink latest price, error: {:?}", e)));
        }
    }
}

abigen!(ChainlinkPriceAggregatorBNBUSD, "./abi/chainlink-price-aggregator-bnb-usd.json");
