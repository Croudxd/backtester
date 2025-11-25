use chrono::NaiveDate;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct DailyQuote {

    pub date: NaiveDate,
    pub price: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: f64,
}
