use chrono::{NaiveDateTime, DateTime, Utc, NaiveDate, Datelike, NaiveTime};
use ethers_core::types::{U256, I256};

use crate::structs::{RoundCandlestick, BetSignal};

pub fn u256_to_bson_date_time(value: U256) -> bson::DateTime {
    return bson::DateTime::from_chrono(u256_to_date_time(value));
}

pub fn date_time_to_bson_date_time(value: DateTime<Utc>) -> bson::DateTime {
    return bson::DateTime::from_chrono(value);
}

pub fn u256_to_date_time(value: U256) -> DateTime<Utc> {
    let time_millis: i64 = value.as_usize() as i64;
    let datetime: DateTime<Utc> = i64_to_date_time(time_millis);
    datetime
}

#[allow(deprecated)]
pub fn i64_to_date_time(value: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(value, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
    datetime
}

pub fn string_to_date_time(date_time_str: &str) -> DateTime<Utc> {
    let fmt = "%Y-%m-%d %H:%M:%S";
    // let date_time_string_formated = format!("{}-01-01 00:00:00", years[0]);
    let date_time_naive = NaiveDateTime::parse_from_str(date_time_str, fmt).unwrap();
    let date_time: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date_time_naive, Utc);
    date_time
}

// this is mostly used to parse bnb amount,
// bnb has 18 decimals, thus teh exp 18
pub fn u256_to_f64(u256: U256) -> f64 {
    let string_hex = format!("{u256:#016x}");
    let without_prefix = string_hex.trim_start_matches("0x");
    let int_parsed = i128::from_str_radix(without_prefix, 16).unwrap();
    let base: i128 = 10;
    // bnb amount has 18 digits after .
    let divisor: i128 = base.pow(18);
    let f64_parsed = int_parsed as f64 / divisor as f64;
    f64_parsed
}

// this is mostly used to parse bnb price,
// bnb price 8 decimals, thus teh exp 8
pub fn i256_to_f64(i256: I256) -> f64 {
    let string_hex = format!("{i256:#032x}");
    let without_prefix = string_hex.trim_start_matches("0x");
    let int_parsed = i64::from_str_radix(without_prefix, 16).unwrap();
    let base: i64 = 10;
    // bnb price has 8 digits after .
    let divisor: i64 = base.pow(8);
    let f64_parsed = int_parsed as f64 / divisor as f64;
    f64_parsed
}

pub fn f64_to_i256(f64_parsed: f64) -> I256 {
    let base: i64 = 10;
    // bnb price has 8 digits after .
    let divisor: i64 = base.pow(8);
    let f64 = f64_parsed * divisor as f64;
    let i64 = f64 as i64;
    let i256 = I256::from_dec_str(&i64.to_string()).unwrap();
    i256
}

pub fn f64_to_u256(f64_parsed: f64, digits: u32) -> U256 {
    let base: i64 = 10;
    let divisor: i64 = base.pow(digits);
    let f64 = f64_parsed * divisor as f64;
    let i64 = f64 as i64;
    let u256 = U256::from_dec_str(&i64.to_string()).unwrap();
    u256
}

pub fn calculate_bet_multiplier(round_candle: &RoundCandlestick, bet_signal: BetSignal) -> f32 {
    let bet_multiplier: f32;
    if bet_signal == BetSignal::Up {
        // if I have 1BNB bull and 2BNB bear, then multiplier will be
        //  bull = total / bull_amount = 3 / 1 = 3
        //  bear = total / bear_amount = 3 / 2 = 2/3
        bet_multiplier = round_candle.total_amount / round_candle.bull_amount;
    } else {
        bet_multiplier = round_candle.total_amount / round_candle.bear_amount;
    }
    bet_multiplier
}

pub fn get_bet_amount(bet_amount: f64) -> U256 {
    let base: i64 = 10;
    let value_u: i64 = base.pow(18);
    let u256_as_f32_multiplication = bet_amount * value_u as f64;
    let u256_as_i32 = u256_as_f32_multiplication as i64;
    let value: U256 = u256_as_i32.try_into().unwrap();
    value
}

pub fn start_day_of_month(date_time: DateTime<Utc>) -> DateTime<Utc> { 
    let year = date_time.year();
    let month = date_time.month();
    let naive_start_of_month_date = NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap();
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let naive_start_of_month_date_time = naive_start_of_month_date.and_time(time);
    let utc_date_time = DateTime::from_naive_utc_and_offset(naive_start_of_month_date_time, Utc);
    utc_date_time
}

pub fn last_day_of_month(date_time: DateTime<Utc>) -> DateTime<Utc> {
    let year = date_time.year();
    let month = date_time.month();
    let naive_end_of_month_date = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap()
        .pred_opt()
        .unwrap();
    let time = NaiveTime::from_hms_milli_opt(23, 59, 59, 0).unwrap();
    let naive_end_of_month_date_time = naive_end_of_month_date.and_time(time);
    let utc_date_time = DateTime::from_naive_utc_and_offset(naive_end_of_month_date_time, Utc);
    utc_date_time
}

pub fn start_of_day(date_time: DateTime<Utc>) -> DateTime<Utc> { 
    let year = date_time.year();
    let month = date_time.month();
    let day = date_time.day();
    let naive_start_of_day_date = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap();
    let time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    let naive_start_of_day_date_time = naive_start_of_day_date.and_time(time);
    let utc_date_time = DateTime::from_naive_utc_and_offset(naive_start_of_day_date_time, Utc);
    utc_date_time
}

pub fn end_of_day(date_time: DateTime<Utc>) -> DateTime<Utc> { 
    let year = date_time.year();
    let month = date_time.month();
    let day = date_time.day();
    let naive_end_of_day_date = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap();
    let time = NaiveTime::from_hms_milli_opt(23, 59, 59, 0).unwrap();
    let naive_end_of_day_date_time = naive_end_of_day_date.and_time(time);
    let utc_date_time = DateTime::from_naive_utc_and_offset(naive_end_of_day_date_time, Utc);
    utc_date_time
}