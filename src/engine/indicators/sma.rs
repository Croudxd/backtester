




use crate::engine::model::candle::Candle;
pub fn calc_sma(size: usize, window: &[Candle]) -> f64
    {
        if window.len() < size{

        }
        let mut sum: f64 = 0.0;

        let u = if window.len() > size {
            window.len() - size
        } else {
            0
        };
        for x in &window[u..]{
            sum += x.Close;
        }
        return sum / window.len() as f64;
    }
