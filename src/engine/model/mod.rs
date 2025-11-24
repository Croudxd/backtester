
use chrono::NaiveDate; // date without timezone
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct DailyQuote {

    date: NaiveDate,
    price: f64,
    close: f64,
    high: f64,
    low: f64,
    open: f64,
    volume: f64,
    //Price,Close,High,Low,Open,Volume
}
