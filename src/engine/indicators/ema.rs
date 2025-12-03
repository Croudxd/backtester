// Formula: \(EMA=(Price_{t}\times Multiplier)+(EMA_{previous}\times (1-Multiplier))\)
// Multiplier = 2/timeperiod -1
// first period is sma of n days.
use crate::engine::model::candle::Candle;

pub fn calc_ema(size: usize, candle: &[Candle], prev_ema: f64) -> f64 {
    let multipler: f64 = 2.0 / (size as f64 + 1.0);
    let ema: f64 = multipler * candle[candle.len() - 1].Close + (1.0 - multipler) * prev_ema;

    return ema;
}
