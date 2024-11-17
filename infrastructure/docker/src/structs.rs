use core::str;
use std::{fmt::Display, marker::PhantomData, future::Future};

use std::fmt;
use amqprs::channel::Channel;
use async_trait::async_trait;
use bson::doc;
use chrono::{Utc, DateTime};
use ethers_core::types::H160;
use ethers::types::U256;
use anyhow::Result;
use serde::{
    Serialize, Deserialize, Serializer, de::{Visitor, self, MapAccess}, Deserializer,
    ser::SerializeStruct
};
use serde_json::{Value, from_str};
use ta::Next;

use amqprs::{consumer::AsyncConsumer, BasicProperties, Deliver};

use crate::prediction::Round;
use crate::utils;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialOrd)]
pub enum BetResult {
    Success,
    Fail,
    InProgress,
    TxFailed
}

impl Display for BetResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bet_result_string = match self {
            Self::Success => "Success",
            Self::Fail => "Fail",
            Self::InProgress => "InProgress",
            Self::TxFailed => "TxFailed",
        };
        write!(f, "{}", bet_result_string)
    }
}


pub struct UserBet {
    pub sender: H160,
    pub epoch: U256,
    pub amount: U256,
    pub direction: BetDirection
}

impl Serialize for UserBet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut user_bet = serializer.serialize_struct("UserBet", 4)?;
        user_bet.serialize_field("sender", &self.sender)?;
        user_bet.serialize_field("epoch", &self.epoch.as_usize())?;
        user_bet.serialize_field("amount", &utils::u256_to_f64(self.amount))?;
        user_bet.serialize_field("direction", &self.direction)?;
        user_bet.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum BetDirection {
    BULL,
    BEAR,
}

impl Display for BetDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bet_direction_string = match self {
            Self::BULL => "BULL",
            Self::BEAR => "BEAR"
        };
        write!(f, "{}", bet_direction_string)
    }
}

impl Serialize for Round {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut round = serializer.serialize_struct("Round", 14)?;
        round.serialize_field("epoch", &self.epoch.as_usize())?;
        round.serialize_field("start_timestamp", &utils::u256_to_bson_date_time(self.start_timestamp))?;
        round.serialize_field("lock_timestamp", &utils::u256_to_bson_date_time(self.lock_timestamp))?;
        round.serialize_field("close_timestamp", &utils::u256_to_bson_date_time(self.close_timestamp))?;
        round.serialize_field("lock_price", &utils::i256_to_f64(self.lock_price))?;
        round.serialize_field("close_price", &utils::i256_to_f64(self.close_price))?;
        round.serialize_field("lock_oracle_id", &self.lock_oracle_id)?;
        round.serialize_field("close_oracle_id", &self.close_oracle_id)?;
        round.serialize_field("total_amount", &utils::u256_to_f64(self.total_amount))?;
        round.serialize_field("bull_amount", &utils::u256_to_f64(self.bull_amount))?;
        round.serialize_field("bear_amount", &utils::u256_to_f64(self.bear_amount))?;
        round.serialize_field("reward_base_cal_amount", &utils::u256_to_f64(self.reward_base_cal_amount))?;
        round.serialize_field("reward_amount", &utils::u256_to_f64(self.reward_amount))?;
        round.serialize_field("oracle_called", &self.oracle_called)?;
        round.end()
    }
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, Epoch, StartTimestamp, LockTimestamp,
            CloseTimestamp, LockPrice, ClosePrice, LockOracleId,
            CloseOracleId, TotalAmount, BullAmount, BearAmount,
            RewardBaseCalAmount, RewardAmount, OracleCalled 
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting correct `fields`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "_id" => Ok(Field::Id),
                            "epoch" => Ok(Field::Epoch),
                            "start_timestamp" => Ok(Field::StartTimestamp),
                            "lock_timestamp" => Ok(Field::LockTimestamp),
                            "close_timestamp" => Ok(Field::CloseTimestamp),
                            "lock_price" => Ok(Field::LockPrice),
                            "close_price" => Ok(Field::ClosePrice),
                            "lock_oracle_id" => Ok(Field::LockOracleId),
                            "close_oracle_id" => Ok(Field::CloseOracleId),
                            "total_amount" => Ok(Field::TotalAmount),
                            "bull_amount" => Ok(Field::BullAmount),
                            "bear_amount" => Ok(Field::BearAmount),
                            "reward_base_cal_amount" => Ok(Field::RewardBaseCalAmount),
                            "reward_amount" => Ok(Field::RewardAmount),
                            "oracle_called" => Ok(Field::OracleCalled),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RoundVisitor;

        impl<'de> Visitor<'de> for RoundVisitor {
            type Value = Round;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Round")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Round, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut epoch = None;
                let mut start_timestamp = None;
                let mut lock_timestamp = None;
                let mut close_timestamp = None;
                let mut lock_price = None;
                let mut close_price = None;
                let mut lock_oracle_id = None;
                let mut close_oracle_id = None;
                let mut total_amount = None;
                let mut bull_amount = None;
                let mut bear_amount = None;
                let mut reward_base_cal_amount = None;
                let mut reward_amount = None;
                let mut oracle_called = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            // ignore mongo id
                        }
                        Field::Epoch => {
                            if epoch.is_some() {
                                return Err(de::Error::duplicate_field("epoch"));
                            }
                            let epoch_parsed = map.next_value::<usize>()?.to_string();
                            epoch = Some(U256::from_dec_str(&epoch_parsed).unwrap());
                        }
                        Field::StartTimestamp => {
                            if start_timestamp.is_some() {
                                return Err(de::Error::duplicate_field("start_timestamp"));
                            }

                            let date_time = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time.into();
                            let millis_string = (back_to_chrono.timestamp_millis() / 1000).to_string();
                            start_timestamp = Some(U256::from_dec_str(&millis_string).unwrap());
                        }
                        Field::LockTimestamp => {
                            if lock_timestamp.is_some() {
                                return Err(de::Error::duplicate_field("lock_timestamp"));
                            }
                            let date_time = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time.into();
                            let millis_string = (back_to_chrono.timestamp_millis() / 1000).to_string();
                            lock_timestamp = Some(U256::from_dec_str(&millis_string).unwrap());
                        }
                        Field::CloseTimestamp => {
                            if close_timestamp.is_some() {
                                return Err(de::Error::duplicate_field("close_timestamp"));
                            }
                            let date_time = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time.into();
                            let millis_string = (back_to_chrono.timestamp_millis() / 1000).to_string();
                            close_timestamp = Some(U256::from_dec_str(&millis_string).unwrap());
                        }
                        Field::LockPrice => {
                            if lock_price.is_some() {
                                return Err(de::Error::duplicate_field("lock_price"));
                            }
                            let lock_price_parsed = map.next_value::<f64>()?;
                            lock_price = Some(utils::f64_to_i256(lock_price_parsed));
                        }
                        Field::ClosePrice => {
                            if close_price.is_some() {
                                return Err(de::Error::duplicate_field("close_price"));
                            }
                            let close_price_parsed = map.next_value::<f64>()?;
                            close_price = Some(utils::f64_to_i256(close_price_parsed));
                        }
                        Field::LockOracleId => {
                            if lock_oracle_id.is_some() {
                                return Err(de::Error::duplicate_field("lock_oracle_id"));
                            }
                            lock_oracle_id = Some(map.next_value()?);
                        }
                        Field::CloseOracleId => {
                            if close_oracle_id.is_some() {
                                return Err(de::Error::duplicate_field("close_oracle_id"));
                            }
                            close_oracle_id = Some(map.next_value()?);
                        }
                        Field::TotalAmount => {
                            if total_amount.is_some() {
                                return Err(de::Error::duplicate_field("total_amount"));
                            }
                            let total_amount_parsed = map.next_value::<f64>()?;
                            total_amount = Some(utils::f64_to_u256(total_amount_parsed, 18));
                        }
                        Field::BullAmount => {
                            if bull_amount.is_some() {
                                return Err(de::Error::duplicate_field("bull_amount"));
                            }
                            let bull_amount_parsed = map.next_value::<f64>()?;
                            bull_amount = Some(utils::f64_to_u256(bull_amount_parsed, 18));
                        }
                        Field::BearAmount => {
                            if bear_amount.is_some() {
                                return Err(de::Error::duplicate_field("bear_amount"));
                            }
                            let bear_amount_parsed = map.next_value::<f64>()?;
                            bear_amount = Some(utils::f64_to_u256(bear_amount_parsed, 18));
                        }
                        Field::RewardBaseCalAmount => {
                            if reward_base_cal_amount.is_some() {
                                return Err(de::Error::duplicate_field("reward_base_cal_amount"));
                            }
                            let reward_base_cal_amount_parsed = map.next_value::<f64>()?;
                            reward_base_cal_amount = Some(utils::f64_to_u256(reward_base_cal_amount_parsed, 18));
                        }
                        Field::RewardAmount => {
                            if reward_amount.is_some() {
                                return Err(de::Error::duplicate_field("reward_amount"));
                            }
                            let reward_amount_parsed = map.next_value::<f64>()?;
                            reward_amount = Some(utils::f64_to_u256(reward_amount_parsed, 18));
                        }
                        Field::OracleCalled => {
                            if oracle_called.is_some() {
                                return Err(de::Error::duplicate_field("oracle_called"));
                            }
                            oracle_called = Some(map.next_value()?);
                        }
                    }
                }
                let epoch = epoch.ok_or_else(|| de::Error::missing_field("epoch"))?;
                let start_timestamp = start_timestamp.ok_or_else(|| de::Error::missing_field("start_timestamp"))?;
                let lock_timestamp = lock_timestamp.ok_or_else(|| de::Error::missing_field("lock_timestamp"))?;
                let close_timestamp = close_timestamp.ok_or_else(|| de::Error::missing_field("close_timestamp"))?;
                let lock_price = lock_price.ok_or_else(|| de::Error::missing_field("lock_price"))?;
                let close_price = close_price.ok_or_else(|| de::Error::missing_field("close_price"))?;
                let lock_oracle_id = lock_oracle_id.ok_or_else(|| de::Error::missing_field("lock_oracle_id"))?;
                let close_oracle_id = close_oracle_id.ok_or_else(|| de::Error::missing_field("close_oracle_id"))?;
                let total_amount = total_amount.ok_or_else(|| de::Error::missing_field("total_amount"))?;
                let bull_amount = bull_amount.ok_or_else(|| de::Error::missing_field("bull_amount"))?;
                let bear_amount = bear_amount.ok_or_else(|| de::Error::missing_field("bear_amount"))?;
                let reward_base_cal_amount = reward_base_cal_amount.ok_or_else(|| de::Error::missing_field("reward_base_cal_amount"))?;
                let reward_amount = reward_amount.ok_or_else(|| de::Error::missing_field("reward_amount"))?;
                let oracle_called = oracle_called.ok_or_else(|| de::Error::missing_field("oracle_called"))?;
                let round = Round {
                    epoch,
                    start_timestamp,
                    lock_timestamp,
                    close_timestamp,
                    lock_price,
                    close_price,
                    lock_oracle_id,
                    close_oracle_id,
                    total_amount,
                    bull_amount,
                    bear_amount,
                    reward_base_cal_amount,
                    reward_amount,
                    oracle_called,
                };

                Ok(round)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "epoch",
            "start_timestamp",
            "lock_timestamp",
            "close_timestamp",
            "lock_price",
            "close_price",
            "lock_oracle_id",
            "close_oracle_id",
            "total_amount",
            "bull_amount",
            "bear_amount",
            "reward_base_cal_amount",
            "reward_amount",
            "oracle_called"];
        deserializer.deserialize_struct("Round", FIELDS, RoundVisitor)
    }
}

impl<'de> Deserialize<'de> for Candlestick {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, Epoch, OpenTime, CloseTime, OpenPrice,
            ClosePrice, HighPrice, LowPrice, Volume }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting correct `fields`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "_id" => Ok(Field::Id),
                            "epoch" => Ok(Field::Epoch),
                            "open_time" => Ok(Field::OpenTime),
                            "close_time" => Ok(Field::CloseTime),
                            "close_price" => Ok(Field::ClosePrice),
                            "open_price" => Ok(Field::OpenPrice),
                            "high_price" => Ok(Field::HighPrice),
                            "low_price" => Ok(Field::LowPrice),
                            "volume" => Ok(Field::Volume),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CandlestickVisitor;

        impl<'de> Visitor<'de> for CandlestickVisitor {
            type Value = Candlestick;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Round")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Candlestick, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut epoch = None;
                let mut open_time = None;
                let mut close_time = None;
                let mut open_price = None;
                let mut close_price = None;
                let mut high_price = None;
                let mut low_price = None;
                let mut volume = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            // ignore mongo id
                        }
                        Field::Epoch => {
                            if epoch.is_some() {
                                return Err(de::Error::duplicate_field("epoch"));
                            }
                            epoch = Some(map.next_value()?);
                        }
                        Field::OpenTime => {
                            if open_time.is_some() {
                                return Err(de::Error::duplicate_field("open_time"));
                            }
                            let date_time = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time.into();
                            open_time = Some(back_to_chrono);
                        }
                        Field::CloseTime => {
                            if close_time.is_some() {
                                return Err(de::Error::duplicate_field("close_time"));
                            }

                            let date_time = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time.into();
                            close_time = Some(back_to_chrono);
                        }
                        Field::OpenPrice => {
                            if open_price.is_some() {
                                return Err(de::Error::duplicate_field("open_price"));
                            }
                            open_price = Some(map.next_value()?);
                        }
                        Field::ClosePrice => {
                            if close_price.is_some() {
                                return Err(de::Error::duplicate_field("close_price"));
                            }
                            close_price = Some(map.next_value()?);
                        }
                        Field::HighPrice => {
                            if high_price.is_some() {
                                return Err(de::Error::duplicate_field("high_price"));
                            }
                            high_price = Some(map.next_value()?);
                        }
                        Field::LowPrice => {
                            if low_price.is_some() {
                                return Err(de::Error::duplicate_field("low_price"));
                            }
                            low_price = Some(map.next_value()?);
                        }
                        Field::Volume => {
                            if volume.is_some() {
                                return Err(de::Error::duplicate_field("volume"));
                            }
                            volume = Some(map.next_value()?);
                        }
                    }
                }
                let epoch = epoch.ok_or_else(|| de::Error::missing_field("epoch"))?;
                let open_time = open_time.ok_or_else(|| de::Error::missing_field("open_time"))?;
                let close_time = close_time.ok_or_else(|| de::Error::missing_field("close_time"))?;
                let open_price = open_price.ok_or_else(|| de::Error::missing_field("open_price"))?;
                let close_price = close_price.ok_or_else(|| de::Error::missing_field("close_price"))?;
                let high_price = high_price.ok_or_else(|| de::Error::missing_field("high_price"))?;
                let low_price = low_price.ok_or_else(|| de::Error::missing_field("low_price"))?;
                let volume = volume.ok_or_else(|| de::Error::missing_field("volume"))?;

                let candlestick = Candlestick {
                    epoch,
                    open_time,
                    close_time,
                    open_price,
                    close_price,
                    high_price,
                    low_price,
                    volume
                };

                Ok(candlestick)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "open_time",
            "close_time",
            "open_price",
            "close_price",
            "high_price",
            "low_price",
            "volume"];
        deserializer.deserialize_struct("Candlestick", FIELDS, CandlestickVisitor)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VWAPIndicator {
    pub epoch: usize,
    pub date_time: DateTime<Utc>,
    pub bb_upper_bound: f64,
    pub bb_lower_bound: f64,
    pub vwap: f32,
    pub rsi: f64
}

impl Serialize for VWAPIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut round = serializer.serialize_struct("VWAPIndicator", 6)?;
        round.serialize_field("epoch", &self.epoch)?;
        round.serialize_field("date_time", &utils::date_time_to_bson_date_time(self.date_time))?;
        round.serialize_field("bb_upper_bound", &self.bb_upper_bound)?;
        round.serialize_field("bb_lower_bound", &self.bb_lower_bound)?;
        round.serialize_field("vwap", &self.vwap)?;
        round.serialize_field("rsi", &self.rsi)?;
        round.end()
    }
}

impl<'de> Deserialize<'de> for VWAPIndicator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, Epoch, DateTime, BbUpperBound, BbLowerBound, Vwap, Rsi }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting correct `fields`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "_id" => Ok(Field::Id),
                            "epoch" => Ok(Field::Epoch),
                            "date_time" => Ok(Field::DateTime),
                            "bb_upper_bound" => Ok(Field::BbUpperBound),
                            "bb_lower_bound" => Ok(Field::BbLowerBound),
                            "vwap" => Ok(Field::Vwap),
                            "rsi" => Ok(Field::Rsi),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct VWAPVisitor;

        impl<'de> Visitor<'de> for VWAPVisitor {
            type Value = VWAPIndicator;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct VWAPIndicator")
            }

            fn visit_map<V>(self, mut map: V) -> Result<VWAPIndicator, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut epoch = None;
                let mut date_time = None;
                let mut bb_upper_bound = None;
                let mut bb_lower_bound = None;
                let mut vwap = None;
                let mut rsi = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            // ignore mongo id
                        }
                        Field::Epoch => {
                            if epoch.is_some() {
                                return Err(de::Error::duplicate_field("epoch"));
                            }
                            // let epoch_parsed = map.next_value::<usize>()?.to_string();
                            epoch = Some(map.next_value()?);
                        }
                        Field::DateTime => {
                            if date_time.is_some() {
                                return Err(de::Error::duplicate_field("date_time"));
                            }
                            let date_time_bson = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time_bson.into();
                            date_time = Some(back_to_chrono);
                        }
                        Field::BbUpperBound => {
                            if bb_upper_bound.is_some() {
                                return Err(de::Error::duplicate_field("bb_upper_bound"));
                            }

                            bb_upper_bound = Some(map.next_value()?);
                        }
                        Field::BbLowerBound => {
                            if bb_lower_bound.is_some() {
                                return Err(de::Error::duplicate_field("bb_lower_bound"));
                            }

                            bb_lower_bound = Some(map.next_value()?);
                        }
                        Field::Vwap => {
                            if vwap.is_some() {
                                return Err(de::Error::duplicate_field("vwap"));
                            }

                            vwap = Some(map.next_value()?);
                        }
                        Field::Rsi => {
                            if rsi.is_some() {
                                return Err(de::Error::duplicate_field("rsi"));
                            }

                            rsi = Some(map.next_value()?);
                        }
                    }
                }
                let epoch = epoch.ok_or_else(|| de::Error::missing_field("epoch"))?;
                let date_time = date_time.ok_or_else(|| de::Error::missing_field("date_time"))?;
                let bb_upper_bound = bb_upper_bound.ok_or_else(|| de::Error::missing_field("bb_upper_bound"))?;
                let bb_lower_bound = bb_lower_bound.ok_or_else(|| de::Error::missing_field("bb_lower_bound"))?;
                let vwap = vwap.ok_or_else(|| de::Error::missing_field("vwap"))?;
                let rsi = rsi.ok_or_else(|| de::Error::missing_field("rsi"))?;

                let vwap_indicator = VWAPIndicator {
                    epoch,
                    date_time,
                    bb_upper_bound,
                    bb_lower_bound,
                    vwap,
                    rsi
                };

                Ok(vwap_indicator)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "epoch",
            "date_time",
            "bb_upper_bound",
            "bb_lower_bound",
            "vwap",
            "rsi"];
        deserializer.deserialize_struct("VWAPIndicator", FIELDS, VWAPVisitor)
    }
}

#[derive(Debug, Clone)]
pub struct BotBet {
    pub date_time: DateTime<Utc>,
    pub epoch: U256,
    pub amount: f64,
    pub tx_hash: String,
    pub direction: BetDirection,
    pub result: BetResult,
    pub latest_chainlink_price: f64,
    pub close_price: f64,
    pub amount_of_win_bnb: f64
}

impl Serialize for BotBet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut round = serializer.serialize_struct("BotBet", 6)?;
        round.serialize_field("date_time", &utils::date_time_to_bson_date_time(self.date_time))?;
        round.serialize_field("epoch", &self.epoch.as_usize().to_string())?;
        round.serialize_field("amount", &self.amount)?;
        round.serialize_field("tx_hash", &self.tx_hash)?;
        round.serialize_field("direction", &self.direction.to_string())?;
        round.serialize_field("result", &self.result.to_string())?;
        round.serialize_field("latest_chainlink_price", &self.latest_chainlink_price)?;
        round.serialize_field("close_price", &self.close_price)?;
        round.serialize_field("amount_of_win_bnb", &self.amount_of_win_bnb)?;
        round.end()
    }
}

impl<'de> Deserialize<'de> for BotBet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, DateTime, Epoch, Amount, TxHash, Direction, Result, LatestChainlinkPrice, ClosePrice, AmountOfWinBnb }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("expecting correct `fields`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "_id" => Ok(Field::Id),
                            "date_time" => Ok(Field::DateTime),
                            "epoch" => Ok(Field::Epoch),
                            "amount" => Ok(Field::Amount),
                            "tx_hash" => Ok(Field::TxHash),
                            "direction" => Ok(Field::Direction),
                            "result" => Ok(Field::Result),
                            "latest_chainlink_price" => Ok(Field::LatestChainlinkPrice),
                            "close_price" => Ok(Field::ClosePrice),
                            "amount_of_win_bnb" => Ok(Field::AmountOfWinBnb),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct BotBetVisitor;

        impl<'de> Visitor<'de> for BotBetVisitor {
            type Value = BotBet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BotBet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<BotBet, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut date_time = None;
                let mut epoch = None;
                let mut amount = None;
                let mut tx_hash = None;
                let mut direction = None;
                let mut result = None;
                let mut latest_chainlink_price = None;
                let mut close_price = None;
                let mut amount_of_win_bnb = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            // ignore mongo id
                        }
                        Field::DateTime => {
                            if date_time.is_some() {
                                return Err(de::Error::duplicate_field("date_time"));
                            }
                            let date_time_bson = map.next_value::<bson::DateTime>()?;
                            let back_to_chrono: chrono::DateTime<Utc> = date_time_bson.into();
                            date_time = Some(back_to_chrono);
                        }
                        Field::Epoch => {
                            if epoch.is_some() {
                                return Err(de::Error::duplicate_field("epoch"));
                            }
                            let value = map.next_value::<String>()?;
                            // let usize_value = value.parse::<usize>()?; 
                            // let epoch_parsed = map.next_value::<usize>()?.to_string();
                            epoch = Some(U256::from_dec_str(&value).unwrap());
                        }
                        Field::Amount => {
                            if amount.is_some() {
                                return Err(de::Error::duplicate_field("amount"));
                            }
                            amount = Some(map.next_value()?);
                        }
                        Field::TxHash => {
                            if tx_hash.is_some() {
                                return Err(de::Error::duplicate_field("tx_hash"));
                            }

                            tx_hash = Some(map.next_value()?);
                        }
                        Field::Direction => {
                            if direction.is_some() {
                                return Err(de::Error::duplicate_field("direction"));
                            }

                            direction = Some(map.next_value()?);
                        }
                        Field::Result => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field("result"));
                            }

                            result = Some(map.next_value()?);
                        }
                        Field::LatestChainlinkPrice => {
                            if latest_chainlink_price.is_some() {
                                return Err(de::Error::duplicate_field("latest_chainlink_price"));
                            }

                            latest_chainlink_price = Some(map.next_value()?);
                        }
                        Field::ClosePrice => {
                            if close_price.is_some() {
                                return Err(de::Error::duplicate_field("close_price"));
                            }

                            close_price = Some(map.next_value()?);
                        },
                        Field::AmountOfWinBnb => {
                            if amount_of_win_bnb.is_some() {
                                return Err(de::Error::duplicate_field("amount_of_win_bnb"));
                            }

                            amount_of_win_bnb = Some(map.next_value()?);
                        }
                    }
                }
                let date_time = date_time.ok_or_else(|| de::Error::missing_field("date_time"))?;
                let epoch = epoch.ok_or_else(|| de::Error::missing_field("epoch"))?;
                let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;
                let tx_hash = tx_hash.ok_or_else(|| de::Error::missing_field("tx_hash"))?;
                let direction = direction.ok_or_else(|| de::Error::missing_field("direction"))?;
                let result = result.ok_or_else(|| de::Error::missing_field("result"))?;
                let latest_chainlink_price = latest_chainlink_price.ok_or_else(|| de::Error::missing_field("latest_chainlink_price"))?;
                let close_price = close_price.ok_or_else(|| de::Error::missing_field("close_price"))?;
                let amount_of_win_bnb = amount_of_win_bnb.ok_or_else(|| de::Error::missing_field("amount_of_win_bnb"))?;

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

                Ok(bot_bet)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "date_time",
            "epoch",
            "amount",
            "tx_hash",
            "direction",
            "result",
            "latest_chainlink_price",
            "close_price",
            "amount_of_win_bnb"];
        deserializer.deserialize_struct("BotBet", FIELDS, BotBetVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct RestrictBetting {
    pub until_epoch: String
}

impl Serialize for RestrictBetting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut round = serializer.serialize_struct("RestrictBetting", 1)?;
        round.serialize_field("until_epoch", &self.until_epoch)?;
        round.end()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BetAmount {
    pub bet_amount: f64
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct RoundCandlestick {
    pub epoch: usize,

    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,

    // close_timestamp
    pub finish_time: DateTime<Utc>,

    // lock_price
    pub open_price: f32,
    // close_price
    pub close_price: f32,

    pub high_price: f32,
    pub low_price: f32,
    pub volume: f32,

    pub total_amount: f32,
    pub bull_amount: f32,
    pub bear_amount: f32,

    // reward_base_cal_amount * (1 - reward_fee) / 100
    pub reward_base_cal_amount: f32,
    pub reward_amount: f32
}


#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Candlestick {
    pub epoch: usize,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub open_price: f32,
    pub close_price: f32,
    pub high_price: f32,
    pub low_price: f32,
    pub volume: f32
}

impl Serialize for Candlestick {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
    {
        let mut candlestick = serializer.serialize_struct("Candlestick", 8)?;
        candlestick.serialize_field("epoch", &self.epoch)?;
        candlestick.serialize_field("open_time", &utils::date_time_to_bson_date_time(self.open_time))?;
        candlestick.serialize_field("close_time", &utils::date_time_to_bson_date_time(self.close_time))?;
        candlestick.serialize_field("open_price", &self.open_price)?;
        candlestick.serialize_field("close_price", &self.close_price)?;
        candlestick.serialize_field("high_price", &self.high_price)?;
        candlestick.serialize_field("low_price", &self.low_price)?;
        candlestick.serialize_field("volume", &self.volume)?;
        candlestick.end()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BetSignal {
    Up,
    Down,
    NoSignal,
    NoData
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrendDirection {
    Uptrend,
    Downtrend
}

// ta indicators section
// candlestick specific impl
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VolumeWeightedAveragePrice { }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VolumeWeightedAveragePriceOutput {
    pub previous_total_volume_price: f32,
    pub previous_total_volume: f32,
    pub vwap: f32,
    pub candlestick: Candlestick
}

impl Next<VolumeWeightedAveragePriceOutput> for VolumeWeightedAveragePrice {
    type Output = VolumeWeightedAveragePriceOutput;

    fn next(&mut self, 
        previous_volume_weighted_average_price_output: VolumeWeightedAveragePriceOutput) 
        -> Self::Output {
        // previous iteration lets
        let previous_volume = previous_volume_weighted_average_price_output.previous_total_volume;
        let previous_volume_price = previous_volume_weighted_average_price_output.previous_total_volume_price;
        let candle = previous_volume_weighted_average_price_output.candlestick;

        // current iteration lets
        let current_volume = candle.volume;
        let current_price = (candle.high_price + candle.low_price + candle.close_price) / 3.0_f32;
        let current_volume_price = current_price * current_volume;

        // computing totals
        let total_volume = previous_volume + current_volume;
        let total_volume_price = previous_volume_price + current_volume_price;
        let vwap = total_volume_price / total_volume;

        // result
        let output = VolumeWeightedAveragePriceOutput {
            // in next iteration those will be previous, thus the name
            previous_total_volume: total_volume,
            previous_total_volume_price: total_volume_price,
            vwap,
            candlestick: candle
        };
        output
    }
}

// candlestick round section
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VolumeWeightedAveragePriceRoundCandlestick { }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VolumeWeightedAveragePriceRoundCandlestickOutput {
    pub previous_total_volume_price: f32,
    pub previous_total_volume: f32,
    pub vwap: f32,
    pub round_candlestick: RoundCandlestick
}

#[allow(dead_code)]
impl VolumeWeightedAveragePriceRoundCandlestickOutput {
    pub fn set_candle(&mut self, round_candlestick: RoundCandlestick) {
        self.round_candlestick = round_candlestick;
    }
}

impl VolumeWeightedAveragePrice {
    pub fn new(
    ) -> Result<Self> {
        Ok(Self { })
    }
}
impl VolumeWeightedAveragePriceRoundCandlestick {
    pub fn new(
    ) -> Result<Self> {
        Ok(Self { })
    }
}

impl Next<VolumeWeightedAveragePriceRoundCandlestickOutput> for VolumeWeightedAveragePriceRoundCandlestick {
    type Output = VolumeWeightedAveragePriceRoundCandlestickOutput;

    fn next(&mut self, 
        previous_volume_weighted_average_price_output: VolumeWeightedAveragePriceRoundCandlestickOutput) 
        -> Self::Output {
        // previous iteration lets
        let previous_volume = previous_volume_weighted_average_price_output.previous_total_volume;
        let previous_volume_price = previous_volume_weighted_average_price_output.previous_total_volume_price;
        let round_candlestick = previous_volume_weighted_average_price_output.round_candlestick;

        // current iteration lets
        let current_volume = round_candlestick.volume;
        let current_price = (round_candlestick.high_price + round_candlestick.low_price + round_candlestick.close_price) / 3.0_f32;
        let current_volume_price = current_price * current_volume;

        // computing totals
        let total_volume = previous_volume + current_volume;
        let total_volume_price = previous_volume_price + current_volume_price;
        let vwap = total_volume_price / total_volume;

        // result
        let output = VolumeWeightedAveragePriceRoundCandlestickOutput {
            // in next iteration those will be previous, thus the name
            previous_total_volume: total_volume,
            previous_total_volume_price: total_volume_price,
            vwap,
            round_candlestick
        };
        output
    }
}

// rabbitmq structs
pub struct BaseConsumer<F, Fut>
    where
        F: Fn(Value) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
{
    callback: F,
    _phantom: PhantomData<(Value, Fut)>
}

impl<F, Fut> BaseConsumer<F, Fut>
    where
        F: Fn(Value) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
{
    pub fn new(callback: F) -> Self {
        BaseConsumer {
            callback,
            _phantom: PhantomData
        }
    }
}

#[async_trait]
impl<F, Fut> AsyncConsumer for BaseConsumer<F, Fut>
    where
        F: Fn(Value) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static
{
    async fn consume(
        &mut self,
        _channel: &Channel,
        _deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let callback = &self.callback;
        let s = str::from_utf8(&content).unwrap();
        let parsed_content: Value = from_str(&s).unwrap();
        tokio::spawn(callback(parsed_content));
    }
}
