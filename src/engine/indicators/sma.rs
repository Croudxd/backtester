




use crate::engine::model::candle::Candle;
pub fn calc_sma(size: usize, window: &[Candle]) -> f64{
    if window.len() < size{
            return 0.0; 
    }
    let start = window.len() - size;
    let sum: f64 = window[start..].iter().map(|c| c.Close).sum();

    sum / size as f64
}
