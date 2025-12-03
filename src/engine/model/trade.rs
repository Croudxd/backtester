use chrono::NaiveDate;

pub struct Trade {
    pub date: NaiveDate,
    pub trade_type: TradeType,
    pub price: f64,
    pub shares: f64,
    pub total: f64,
}

pub enum TradeType {
    Sell,
    Buy,
}
