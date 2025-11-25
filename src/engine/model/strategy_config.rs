
use crate::engine::model::indicator_config::Indicators;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct StrategyConfig {

    pub name: String,
    pub indicators: Vec<Indicators>,
    pub entry: String,
    pub exit: String,
    pub starting_money: f64,
}


/*
 strategy {
	name: "sma_strat",
	indicators: [ { type: "sma", windows : [10, 20, 30, 40] } ],
	entry: "sma < 40, sma > 10",
	exit: "sma > 40, sma < 10",
}

This is what a simple sma config will look like.
Need to transform the above into code.

*/
