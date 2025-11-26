use crate::engine::model::daily_quote::DailyQuote;

pub fn calc_sma(window: Vec<DailyQuote>) -> f64
    {
        //iterate over current window, calculate average
        let mut sum: f64 = 0.0;
        for x in &window{
            sum += x.Close;
        }
        return sum / window.len() as f64;
    }
