use crate::structs::{BetSignal, TrendDirection, RoundCandlestick, VolumeWeightedAveragePriceRoundCandlestick, VolumeWeightedAveragePriceRoundCandlestickOutput};
use ta::{indicators::{BollingerBandsOutput, BollingerBands, RelativeStrengthIndex}, Next};
use log::{info, warn};

// I can call this strat as channel strategy, as here the idea is
// to identify deviation from channel, when candles cross channel
// we expect that price will go back. This strat might be doing great 
// when we identify the channel in some time frame and apply it until
// channel break out. But it's a good starting point for my own interest
pub fn compute_vwap_strategy_indicators_for_round_candlesticks(market_data_round_candles: &mut Vec<RoundCandlestick>) -> (
    VolumeWeightedAveragePriceRoundCandlestick,
    BollingerBands,
    RelativeStrengthIndex,
    Vec<VolumeWeightedAveragePriceRoundCandlestickOutput>,
    Vec<BollingerBandsOutput>,
    Vec<f64>) {
    
    // 1. VWAP indicator
    let mut vwap = VolumeWeightedAveragePriceRoundCandlestick::new().unwrap();
    // 2. BB
    //  step or period is 14, standart deviation is standart - 2.0
    let mut bb = BollingerBands::new(14, 2f64).unwrap();
    // 3. RSI
    //  step or period is 16
    let mut rsi = RelativeStrengthIndex::new(16).unwrap();

    let mut vwap_outs: Vec<VolumeWeightedAveragePriceRoundCandlestickOutput> = vec![];
    let mut bb_outs: Vec<BollingerBandsOutput> = vec![];
    let mut rsi_outs: Vec<f64> = vec![];
    // initial vwap data
    let mut volume_weighted_average_price_output = VolumeWeightedAveragePriceRoundCandlestickOutput {
        previous_total_volume_price: 0.0_f32,
        previous_total_volume: 0.0_f32,
        vwap: 0.0_f32,
        round_candlestick: market_data_round_candles[0]
    };

    for round_candle in market_data_round_candles {
        // set before each iteration
        volume_weighted_average_price_output.set_candle(*round_candle);

        let vwap_output = vwap.next(volume_weighted_average_price_output);
        vwap_outs.push(vwap_output);
        bb_outs.push(bb.next(round_candle.close_price as f64));
        rsi_outs.push(rsi.next(round_candle.close_price as f64));

        volume_weighted_average_price_output = vwap_output;
    }
    (vwap, bb, rsi, vwap_outs, bb_outs, rsi_outs)
}

pub fn get_vwap_strategy_signal_for_next_round_candle(
    previous_candles: &Vec<RoundCandlestick>,
    vwaps: &Vec<f32>, 
    bb_upper_bound: f64,
    bb_lower_bound: f64,
    rsi_out: f64,
    info: bool,
    volume: f32) -> BetSignal {

    let mut result_signal = BetSignal::NoSignal;
    let trend_direction: TrendDirection;
    let required_sequential_candles_count = 15;
    let mut sequential_candles_count = 0;
    let rsi_lower_bound = 25.0_f64;
    let rsi_upper_bound = 75.0_f64;

    // 81% winrate, 300-800
    let volume_lower_bound_1 = 300_f32;
    let volume_upper_bound_1 = 800_f32;
    // 80% winrate, 1800-2500
    let volume_lower_bound_2 = 1800_f32;
    let volume_upper_bound_2 = 2500_f32;

    let last_candle_to_compare = previous_candles[previous_candles.len() - 1];
    if info {
        info!("Last candle epoch is: {:?}", last_candle_to_compare.epoch);
        info!("Volume is: {:?}", volume);
    }
    
    for (index, candle) in previous_candles.iter().enumerate() {
        let corresponding_vwap_value = vwaps[index];
        if candle.close_price > corresponding_vwap_value {
            sequential_candles_count += 1;
        }
        if candle.close_price < corresponding_vwap_value {
            sequential_candles_count -= 1;
        }
    }

    if sequential_candles_count >= required_sequential_candles_count {
        if info { 
            info!("Uptrend determined");
        }
        trend_direction = TrendDirection::Uptrend;
    } else if sequential_candles_count <= -required_sequential_candles_count {
        if info {
            info!("Downtrend determined");
        }
        trend_direction = TrendDirection::Downtrend;
    } else {
        if info {
            info!("Could not determine a trend");
        }
        return BetSignal::NoSignal;
    }

    if trend_direction == TrendDirection::Uptrend {
        if info {
            info!("Inside Uptrend bb_out.lower {:?}, candle.close_price {:?}, rsi_out {:?}", 
                bb_lower_bound, last_candle_to_compare.close_price as f64, rsi_out);
        }
        if bb_lower_bound > last_candle_to_compare.close_price.into() {
            if info {
                info!("bb_lower_bound > last_candle_to_compare.close_price = true");
            }
        } else {
            if info {
                info!("bb_lower_bound > last_candle_to_compare.close_price = false");
            }
        }

        // lower
        if bb_lower_bound > last_candle_to_compare.close_price.into()
            && ((volume > volume_lower_bound_1 && volume < volume_upper_bound_1))
            || (volume > volume_lower_bound_2 && volume < volume_upper_bound_2)
            && rsi_out < rsi_lower_bound
        {
            result_signal = BetSignal::Up;
            if info {
                warn!("Received a bet signal Up!");
            }
        }
    } else if trend_direction == TrendDirection::Downtrend {
        if info {
            info!("Inside Downtrend bb_out.upper {:?}, candle.close_price {:?}, rsi_out {:?}", 
                bb_upper_bound, last_candle_to_compare.close_price as f64, rsi_out);
        }
        if bb_upper_bound < last_candle_to_compare.close_price.into() {
            if info {
                info!("bb_upper_bound < last_candle_to_compare.close_price = true");
            }
        } else {
            if info {
                info!("bb_upper_bound < last_candle_to_compare.close_price = false");
            }
        }

        // upper
        if bb_upper_bound < last_candle_to_compare.close_price.into() 
            && ((volume > volume_lower_bound_1 && volume < volume_upper_bound_1)
            || (volume > volume_lower_bound_2 && volume < volume_upper_bound_2))
            && rsi_out > rsi_upper_bound 
        {
            result_signal = BetSignal::Down;
            if info {
                warn!("Received a bet signal Down!");
            }
        }
    }
    result_signal
}
