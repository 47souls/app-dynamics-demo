use crate::structs::Candlestick;
use crate::utils;
use reqwest::Client;
use reqwest::StatusCode;
use anyhow::{Result, anyhow};

pub async fn fetch_binance_candlesticks(
    client: &Client, 
    interval: &str,
    start_date_millis: i64, end_date_millis: i64) -> Result<Vec<Candlestick>> {
    let url = format!("https://api.binance.com/api/v3/klines?symbol=BNBUSDT&interval={}&startTime={}&endTime={}", 
        interval, start_date_millis, end_date_millis);

    let response = client
        .get(url)
        .send()
        .await?;
    let too_many_requests_err = Err(anyhow!("Too many requests"));
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        return too_many_requests_err;
    } else {
        let bytes = response.bytes().await?.to_vec();

        let mut string_message = String::from_utf8(bytes).unwrap();
        string_message.remove(0);
        string_message.remove(string_message.len() - 1);
        let chunks = string_message.split("],[");
        let mut result_candlesticks: Vec<Candlestick> = vec![];

        for chunk in chunks {
            let mut string_chunk = String::from(chunk);
            string_chunk = string_chunk.replace(&['[', ']', '\"'][..], "");
            let inner_chunks: Vec<&str> = string_chunk.split(",").collect();
            // try to parse, if problem occurs, it means response is malformed/too many requests
            let open_time = inner_chunks[0].parse::<i64>();
            match open_time {
                Ok(_) => {
                    let candlestick = Candlestick {
                        // don't care about this here
                        epoch: 0 as usize,
                        open_time: utils::i64_to_date_time(inner_chunks[0].parse::<i64>().unwrap() / 1000),
                        close_time: utils::i64_to_date_time(inner_chunks[6].parse::<i64>().unwrap() / 1000),
                        open_price: inner_chunks[1].parse::<f32>().unwrap(),
                        close_price: inner_chunks[4].parse::<f32>().unwrap(),
                        high_price: inner_chunks[2].parse::<f32>().unwrap(),
                        low_price: inner_chunks[3].parse::<f32>().unwrap(),
                        volume: inner_chunks[5].parse::<f32>().unwrap(),
                    };
                    result_candlesticks.push(candlestick);
                },
                Err(_) => {
                    return too_many_requests_err;
                }
            }
        }
        Ok(result_candlesticks)
    }
} 