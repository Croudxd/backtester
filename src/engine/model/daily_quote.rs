use chrono::NaiveDate;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct DailyQuote {

    pub Date: NaiveDate,
    pub Close: f64,
    pub High: f64,
    pub Low: f64,
    pub Open: f64,
    pub Volume: f64,
}
