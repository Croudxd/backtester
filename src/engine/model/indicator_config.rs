use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Indicators {
    Sma { windows: Vec<usize> },
}
