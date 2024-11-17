use std::{env, vec};
use bson::{Document, to_document, doc};
use chrono::{Utc, DateTime};
use ethers::types::U256;
use log::{info, warn};
use mongodb::results::{DeleteResult, InsertManyResult};
use mongodb::{options::{ClientOptions, ResolverConfig, FindOptions, FindOneOptions}, Client, Collection};
use anyhow::Result;
use futures::TryStreamExt;
use ta::indicators::BollingerBandsOutput;

use crate::structs::{BetAmount, RoundCandlestick, RestrictBetting, BotBet, VWAPIndicator, BetDirection, UserBet};
use crate::{prediction::Round, structs::{Candlestick, BetResult, VolumeWeightedAveragePriceRoundCandlestickOutput}};

pub async fn get_mongo_client() -> Result<Client> {
    let mongo_hostname = env::var("MONGO_HOSTNAME")
        .unwrap_or_else(|_| "localhost".to_string());
    let client_uri = format!("mongodb://admin:admin@{}", mongo_hostname);
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    Ok(client)
}

pub async fn save_bet_amount(bet_amount: f64) -> Result<()> { 
    let bet_amount_collection = get_collection_by_name("prediction", "bet_amount").await;
    
    let bet_amount_struct = BetAmount {
        bet_amount
    };

    bet_amount_collection.insert_one(to_document(&bet_amount_struct).unwrap(), None).await.unwrap();
    Ok(())
}

pub async fn find_bet_amount() -> Result<Option<BetAmount>> { 
    let bet_amount_collection = get_collection_by_name("prediction", "bet_amount").await;

    let bet_amount_option = bet_amount_collection
        .find_one(None, None)
        .await
        .unwrap();

    let mut bet_amount = None;

    match bet_amount_option {
        Some(bet_amount_option) => {
            bet_amount = Some(bson::from_bson(bson::Bson::Document(bet_amount_option)).unwrap());

        },
        None => {}
    }

    Ok(bet_amount)
}

pub async fn clear_bet_amount() -> Result<()> { 
    let bet_amount_collection = get_collection_by_name("prediction", "bet_amount").await;

    bet_amount_collection
        .delete_many(doc! {}, None)
        .await
        .unwrap();
    Ok(())
}

pub async fn save_restrict_betting(until_epoch: U256) -> Result<()> { 
    let restrict_betting_collection = get_collection_by_name("prediction", "restrict_betting").await;
    
    let restrict_betting = RestrictBetting {
        until_epoch: until_epoch.to_string()
    };

    restrict_betting_collection.insert_one(to_document(&restrict_betting).unwrap(), None).await.unwrap();
    Ok(())
}

pub async fn fetch_latest_restrict_betting() -> Result<Option<RestrictBetting>> { 
    let restrict_betting_collection = get_collection_by_name("prediction", "restrict_betting").await;

    let options = FindOneOptions::builder()
        .sort(doc! { "until_epoch": -1})
        .build();

    let restrict_betting_option = restrict_betting_collection
        .find_one(None, options)
        .await
        .unwrap();

    let mut restrict_betting = None;

    match restrict_betting_option {
        Some(restrict_betting_option) => {
            restrict_betting = Some(bson::from_bson(bson::Bson::Document(restrict_betting_option)).unwrap());

        },
        None => {}
    }

    Ok(restrict_betting)
}

pub async fn save_bot_bet(
    date_time: DateTime<Utc>,
    epoch: U256,
    amount: f64,
    tx_hash: String,
    direction: BetDirection,
    result: BetResult,
    latest_chainlink_price: f64,
    close_price: f64,
    amount_of_win_bnb: f64) -> Result<()> { 
    let bot_bet_collection = get_collection_by_name("prediction", "bot_bet").await;
    
    let bot_bet = BotBet {
        date_time,
        epoch, 
        amount,
        tx_hash,
        direction, 
        result,
        latest_chainlink_price,
        close_price,
        amount_of_win_bnb
    };

    bot_bet_collection.insert_one(to_document(&bot_bet).unwrap(), None).await.unwrap();

    Ok(())
}

pub async fn update_bot_bet(epoch: U256, result: BetResult, close_price: f64, amount_of_win_bnb: f64) -> Result<()> {
    let bot_bet_collection = get_collection_by_name("prediction", "bot_bet").await;
    let query = doc!{"epoch": epoch.as_usize().to_string()};
    let update = doc!{"$set": {
        "result": result.to_string(), "close_price": close_price, "amount_of_win_bnb": amount_of_win_bnb}};
    bot_bet_collection
        .update_one(query, update, None)
        .await
        .unwrap();

    Ok(())
}

pub async fn retrieve_recent_bot_bets(amount_of_bets: i64) -> Result<Vec<BotBet>> {
    let bot_bet_collection = get_collection_by_name("prediction", "bot_bet").await;
    
    let find_options = FindOptions::builder()
        .sort(doc! { "epoch": -1 })
        .limit(Some(amount_of_bets))
        .build();

    let mut cursor = bot_bet_collection
        .find(None, find_options)
        .await
        .unwrap();

    let mut result = vec![];

    while let Some(document) = cursor.try_next().await? {
        let bot_bet = bson::from_bson(bson::Bson::Document(document)).unwrap();
        result.push(bot_bet);
    }

    Ok(result)
}

pub async fn retrieve_bot_bets(from_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<BotBet>> {
    let bot_bet_collection = get_collection_by_name("prediction", "bot_bet").await;
    let mut result = vec![];

    let filter = doc! {
        "date_time": doc! { "$gte": from_date, "$lte": end_date }
    };

    let options = FindOptions::builder()
        .sort(doc! { "date_time": 1})
        .build();

    let mut cursor = bot_bet_collection
        .find(filter, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let bot_bet = bson::from_bson(bson::Bson::Document(document)).unwrap();
        result.push(bot_bet);
    }
    Ok(result)
}


pub async fn retrieve_bot_bet(epoch: U256) -> Result<Option<BotBet>> {
    let bot_bet_collection = get_collection_by_name("prediction", "bot_bet").await;
    let filter = doc! {
        "epoch": epoch.as_usize().to_string()
    };
    let bot_bet_option = bot_bet_collection
        .find_one(filter, None)
        .await
        .unwrap();
    let mut result = None;
    match bot_bet_option {
        Some(bot_bet) => {
            result = Some(bson::from_bson(bson::Bson::Document(bot_bet)).unwrap());
        },
        None => {}
    } 
    Ok(result)
}

pub async fn save_wvap_indicators(
    candles: &Vec<RoundCandlestick>,
    vwap_outputs: &Vec<VolumeWeightedAveragePriceRoundCandlestickOutput>,
    bb_outputs: &Vec<BollingerBandsOutput>,
    rsi_outputs: Vec<f64>) -> Result<()> {
    let vwap_indicator_collection = get_collection_by_name("prediction", "vwap_indicator").await;

    let vwap_indicators: Vec<VWAPIndicator> = map_vwap_indicators_to_objects(&candles, &vwap_outputs, &bb_outputs, &rsi_outputs);

    let vwap_indicator_docs = vwap_indicators
        .iter()
        .map(|vwap_indicator| to_document(vwap_indicator).unwrap())
        .collect::<Vec<Document>>();

    vwap_indicator_collection
        .insert_many(vwap_indicator_docs, None)
        .await
        .unwrap();

    info!("{:?} vwap indicators were saved into db", vwap_indicators.len());

    Ok(())
}

pub fn map_vwap_indicators_to_objects(
    candles: &Vec<RoundCandlestick>,
    vwap_outputs: &Vec<VolumeWeightedAveragePriceRoundCandlestickOutput>,
    bb_outputs: &Vec<BollingerBandsOutput>,
    rsi_outputs: &Vec<f64>) -> Vec<VWAPIndicator> {
    let mut vwap_indicators: Vec<VWAPIndicator> = vec![];

    for (index, candle) in candles.iter().enumerate() {
        let vwap_indicator = VWAPIndicator {
            epoch: candle.epoch,
            date_time: candle.close_time,
            bb_upper_bound: bb_outputs[index].upper,
            bb_lower_bound: bb_outputs[index].lower,
            vwap: vwap_outputs[index].vwap,
            rsi: rsi_outputs[index]
        };
        vwap_indicators.push(vwap_indicator);
    }
    vwap_indicators
}

pub async fn read_vwap_indicators_in_range(start_date_time: DateTime<Utc>, end_date_time: DateTime<Utc>) -> Result<Vec<VWAPIndicator>> { 
    let mut vwap_indicators: Vec<VWAPIndicator> = vec![];
    let vwap_indicators_collection = get_collection_by_name("prediction", "vwap_indicator").await;

    let filter = doc! {
        "date_time": doc! { "$gte": start_date_time, "$lte": end_date_time }
    };
    let options = FindOptions::builder()
        .sort(doc! { "date_time": 1})
        .build();
    let mut cursor = vwap_indicators_collection
        .find(filter, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let vwap_indicator = bson::from_bson(bson::Bson::Document(document)).unwrap();
        vwap_indicators.push(vwap_indicator);
    }

    Ok(vwap_indicators)
}

pub async fn check_if_vwap_indicators_present() -> Result<bool> { 
    let vwap_indicators_collection = get_collection_by_name("prediction", "vwap_indicator").await;
    let doc_count = vwap_indicators_collection
        .count_documents(None, None)
        .await
        .unwrap();
    let result = doc_count != 0;
    Ok(result)
}

// mongo writes will be sequential, since they are pretty quick, no concurrency
pub async fn save_rounds(mut rounds: Vec<Round>) -> Result<()> {
    let round_collection = get_collection_by_name("prediction", "round").await;

    rounds
        .sort_by(|round1, round2| round1.epoch.cmp(&round2.epoch));

    let round_docs = rounds
        .iter()
        .map(|round| to_document(round).unwrap())
        .collect::<Vec<Document>>();

    round_collection
        .insert_many(round_docs, None)
        .await
        .unwrap();
    
    Ok(())
}

pub async fn read_rounds(start_epoch: i32, end_epoch: i32) -> Result<Vec<Round>> {
    let mut rounds: Vec<Round> = vec![];
    let round_collection = get_collection_by_name("prediction", "round").await;
    let filter = doc! {
        "epoch": doc! { "$gte": start_epoch, "$lte": end_epoch }
    };
    let options = FindOptions::builder()
        .sort(doc! { "epoch": 1})
        .build();
    let mut cursor = round_collection
        .find(filter, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let round = bson::from_bson(bson::Bson::Document(document)).unwrap();
        rounds.push(round);
    }

    Ok(rounds)
}

pub async fn save_user_bet(user_bet: UserBet) -> Result<()> {
    let user_bet_collection = get_collection_by_name("prediction", "user_bet").await;

    user_bet_collection
        .insert_one(to_document(&user_bet).unwrap(), None)
        .await
        .unwrap();
    
    Ok(())
}

pub async fn fetch_last_db_epoch() -> Result<usize> {
    let round_collection = get_collection_by_name("prediction", "round").await;

    let find_options = FindOneOptions::builder()
        .sort(doc! { "epoch": -1 })
        .build();

    let last_db_epoch_option = round_collection
        .find_one(None, find_options)
        .await
        .unwrap();

    let last_db_epoch: usize;
    match last_db_epoch_option {
        Some(epoch) => {
            last_db_epoch = epoch.get_i64("epoch").unwrap().try_into().unwrap();
            info!("Last round epoch in db is {:?}", last_db_epoch);
        },
        None => {
            last_db_epoch = 0;
            info!("No epoch data present in db");
        }
    } 
    Ok(last_db_epoch)
}

pub async fn save_candlesticks_into_db(candlesticks: &mut Vec<Candlestick>, collection_name: &str) -> Result<InsertManyResult> {
    let candlestick_collection = get_collection_by_name("binance", collection_name).await;

    // candlesticks.sort();
    candlesticks
        .sort_by(|candlestick1, candlestick2| candlestick1.open_time.cmp(&candlestick2.open_time));

    let candlestick_docs = candlesticks
        .iter()
        .map(|round| to_document(round).unwrap())
        .collect::<Vec<Document>>();

    let saved_result = candlestick_collection
        .insert_many(candlestick_docs, None)
        .await
        .unwrap();
    Ok(saved_result)
}

pub async fn read_candlesticks_in_range(start_date_time: DateTime<Utc>, end_date_time: DateTime<Utc>, collection_name: &str) -> Result<Vec<Candlestick>> {
    let mut candlesticks: Vec<Candlestick> = vec![];
    let candlestick_collection = get_collection_by_name("binance", collection_name).await;

    let filter = doc! {
        "open_time": doc! { "$gte": start_date_time, "$lte": end_date_time }
    };
    let options = FindOptions::builder()
        .sort(doc! { "open_time": 1})
        .build();
    let mut cursor = candlestick_collection
        .find(filter, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let candlestick = bson::from_bson(bson::Bson::Document(document)).unwrap();
        candlesticks.push(candlestick);
    }

    Ok(candlesticks)
}

pub async fn delete_previous_candlesticks(end_date_time: DateTime<Utc>, collection_name: &str) -> Result<DeleteResult> {
    let candlestick_collection = get_collection_by_name("binance", collection_name).await;
    let query = doc! {
        "open_time": doc! { "$lte": end_date_time }
    };

    Ok(candlestick_collection.delete_many(query, None).await.unwrap())
}

pub async fn update_candlestick(epoch: U256, close_price: f64, collection_name: &str) -> Result<()> {
    let bot_bet_collection = get_collection_by_name("binance", collection_name).await;
    let query = doc!{"epoch": epoch.as_usize().to_string()};
    let update = doc!{"$set": {"close_price": close_price}};
    bot_bet_collection
        .update_one(query, update, None)
        .await
        .unwrap();

    Ok(())
}

pub async fn read_first_candlestick(collection_name: &str) -> Result<Candlestick> {
    let candlestick_collection = get_collection_by_name("binance", collection_name).await;

    let options = FindOneOptions::builder()
        .sort(doc! { "open_time": 1})
        .build();

    let document = candlestick_collection
        .find_one(None, options)
        .await
        .unwrap()
        .unwrap();
    let candlestick = bson::from_bson(bson::Bson::Document(document)).unwrap();

    Ok(candlestick)
}

pub async fn candlesticks_len(collection_name: &str) -> Result<i32> {
    let candlestick_collection = get_collection_by_name("binance", collection_name).await;
    let amount = candlestick_collection
        .count_documents(None, None)
        .await.unwrap() as i32;
    Ok(amount)
}

// TOOD: move it to better place
pub fn combine_1s_candles_to_5_minute_candles(epoch: usize, chainlink_latest_price: f64, candles_1s: &Vec<Candlestick>) -> Vec<Candlestick> {
    let mut candles_5m_aggregated: Vec<Candlestick> = vec![];
    // split the based on 5 min intervals
    // 300 is size of 1s candles in 5 min interval
    for candles_chunk in candles_1s.chunks(300) {
        let chunk_len = candles_chunk.len() - 1;
        let open_time = candles_chunk[0].open_time;
        let close_time = candles_chunk[chunk_len].close_time;
        let open_price = candles_chunk[0].open_price;
        let close_price = candles_chunk[chunk_len].close_price;
        // to be changed
        let mut high_price = candles_chunk[0].high_price;
        let mut low_price = candles_chunk[0].low_price;
        let mut volume = 0_f32;
        for candle in candles_chunk {
            if candle.high_price > high_price {
                high_price = candle.high_price;
            }
            if candle.low_price < low_price {
                low_price = candle.low_price;
            }
            volume += candle.volume;
        }
        let candlestick_aggregated = Candlestick {
            epoch,
            open_time, 
            close_time,
            open_price, 
            close_price,
            high_price,
            low_price,
            volume
        };
        candles_5m_aggregated.push(candlestick_aggregated);
    }
    // updated taking chainlink price into consideration
    update_last_candle_with_chainlink_price(&candles_5m_aggregated, chainlink_latest_price);

    candles_5m_aggregated
}

// TOOD: move it to better place
fn update_last_candle_with_chainlink_price(candles_5m_aggregated: &Vec<Candlestick>, chainlink_latest_price: f64) {
    let len = candles_5m_aggregated.len();
    if len == 0 {
        warn!("Unable to update candlesticks with chainlink price, as candles_5m_aggregated it's empty");
    } else {
        let mut candle_to_modify = candles_5m_aggregated[len - 1];
        if candle_to_modify.high_price < chainlink_latest_price as f32 {
            candle_to_modify.high_price = chainlink_latest_price as f32;
        }
        if candle_to_modify.low_price > chainlink_latest_price as f32 {
            candle_to_modify.low_price = chainlink_latest_price as f32;
        }
        candle_to_modify.close_price = chainlink_latest_price as f32;
    }
}

pub async fn save_round_candlesticks(mut round_candlesticks: Vec<RoundCandlestick>) {
    let round_candlestick_collection = get_collection_by_name("binance", "round_candlestick").await;

    round_candlesticks
        .sort_by(|candlestick1, candlestick2| candlestick1.open_time.cmp(&candlestick2.open_time));

    let round_candlestick_docs = round_candlesticks
        .iter()
        .map(|round| to_document(round).unwrap())
        .collect::<Vec<Document>>();

    round_candlestick_collection
        .insert_many(round_candlestick_docs, None)
        .await
        .unwrap();
}

// TODO: should this be based on open_time or closed_time?
pub async fn read_round_candlesticks() -> Result<Vec<RoundCandlestick>> {
    let mut round_candlesticks: Vec<RoundCandlestick> = vec![];
    let round_candlestick_collection = get_collection_by_name("binance", "round_candlestick").await;

    let options = FindOptions::builder()
        .sort(doc! { "open_time": 1})
        .build();
    let mut cursor = round_candlestick_collection
        .find(None, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let round_candlestick = bson::from_bson(bson::Bson::Document(document)).unwrap();
        round_candlesticks.push(round_candlestick);
    }

    Ok(round_candlesticks)
}

pub async fn read_round_candlesticks_by_date(
    start_date_time: DateTime<Utc>, end_date_time: DateTime<Utc>
) -> Result<Vec<RoundCandlestick>> {
    let mut round_candlesticks: Vec<RoundCandlestick> = vec![];
    let round_candlestick_collection = get_collection_by_name("binance", "round_candlestick").await;

    let filter = doc! {
        "open_time": doc! { "$gte": start_date_time, "$lte": end_date_time }
    };
    let options = FindOptions::builder()
        .sort(doc! { "open_time": 1})
        .build();
    let mut cursor = round_candlestick_collection
        .find(filter, options)
        .await.unwrap();
    while let Some(document) = cursor.try_next().await? {
        let round_candlestick = bson::from_bson(bson::Bson::Document(document)).unwrap();
        round_candlesticks.push(round_candlestick);
    }

    Ok(round_candlesticks)
}

// pub async fn read_round_candlesticks_by_epoch(
//     start_epoch: usize, end_epoch: usize
// ) -> Result<Vec<RoundCandlestick>> {
//     let mut round_candlesticks: Vec<RoundCandlestick> = vec![];
//     let round_candlestick_collection = get_collection_by_name("binance", "round_candlestick").await;

//     let start_epoch_pre_parsed = start_epoch.to_string();
//     let start_epoch_parsed = Some(U256::from_dec_str(&start_epoch_pre_parsed).unwrap()).unwrap();

//     let end_epoch_pre_parsed = end_epoch.to_string();
//     let end_epoch_parsed = Some(U256::from_dec_str(&end_epoch_pre_parsed).unwrap()).unwrap();

//     let filter = doc! {
//         "epoch": doc! { "$gte": start_epoch_parsed, "$lte": end_epoch_parsed }
//     };
//     let options = FindOptions::builder()
//         .sort(doc! { "open_time": 1})
//         .build();
//     let mut cursor = round_candlestick_collection
//         .find(filter, options)
//         .await.unwrap();
//     while let Some(document) = cursor.try_next().await? {
//         let round_candlestick = bson::from_bson(bson::Bson::Document(document)).unwrap();
//         round_candlesticks.push(round_candlestick);
//     }

//     Ok(round_candlesticks)
// }

pub async fn fetch_last_round_candlestick_db_epoch() -> Result<usize> {
    let round_collection = get_collection_by_name("binance", "round_candlestick").await;

    let find_options = FindOneOptions::builder()
        .sort(doc! { "epoch": -1 })
        .build();

    let last_db_epoch_option = round_collection
        .find_one(None, find_options)
        .await
        .unwrap();

    let last_db_epoch: usize;
    match last_db_epoch_option {
        Some(epoch) => {
            last_db_epoch = epoch.get_i64("epoch").unwrap().try_into().unwrap();
            info!("Last round candlestick epoch in db is {:?}", last_db_epoch);
        },
        None => {
            last_db_epoch = 0;
            info!("No round candlestick epoch data present in db");
        }
    } 
    Ok(last_db_epoch)
}

// helpers
async fn get_collection_by_name(database_name: &str, collection_name: &str) -> Collection<Document> {
    let client = get_mongo_client().await.unwrap();

    let round_collection = client
        .database(database_name)
        .collection::<Document>(collection_name);
    
    round_collection
}
