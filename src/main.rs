use crate::engine::core::engine::backtest_engine;

mod engine;

use std::path::Path;


fn main() {
    let path = Path::new("../sample.json");
    backtest_engine(path);


}
