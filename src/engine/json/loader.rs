use crate::engine::model::condition::Condition;
use crate::engine::model::condition::CompareTo;
use crate::engine::model::condition::Strategy;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;



pub fn loader(path: &Path) -> Strategy {

    let file = File::open(path).expect("error opening file.");
    let config: Strategy = serde_json::from_reader(file)
        .expect("error while reading or passing json");

    return config

}
