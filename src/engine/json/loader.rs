
use std::path::Path;
use crate::engine::model::strategy_config::StrategyConfig;
use std::fs::File;


pub fn loader(path: &Path) -> StrategyConfig {

    let file = File::open(path).expect("error opening file.");
    let config: StrategyConfig = serde_json::from_reader(file)
        .expect("error while reading or passing json");

    return config

}
