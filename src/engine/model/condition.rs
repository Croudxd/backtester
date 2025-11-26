use serde::Deserialize;



#[derive(Deserialize)]
pub struct Condition {
    pub indicator: String,       // e.g. "SMA", "RSI"
    pub window: usize,           // 10, 14, 20, etc
    pub operator: String,        // "<", ">", ">=", "<="
    pub compare_to: CompareTo,   // either another indicator OR a raw value
    pub logical: Option<String>, // "and" | "or" | None
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CompareTo {
    Indicator { indicator: String, window: usize },
    Value { value: f64 },
}

#[derive(Deserialize)]
pub struct Strategy {
    pub entry: Vec<Condition>,
    pub exit:  Vec<Condition>,
}
