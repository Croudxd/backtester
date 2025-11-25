use crate::engine::json::loader::loader;
use crate::engine::csv::load_full_csv::load_full_csv;
use std::path::Path;
use crate::engine::indicators::sma::calc_sma;
use std::collections::HashMap;
use crate::engine::model::indicator_config::Indicators;
use crate::engine::model::strategy_config::StrategyConfig;

pub fn backtest_engine(path: Path)
{
        //Define all our variables we are going to need.
        let config = loader(path);
        let csv = load_full_csv();
        let starting_money: f64 = config.starting_money;
        let mut sma_hashmap: HashMap<usize, f64> = HashMap::new();


        let mut indicatorPeriods = Vec::new();
        let max_window = indicatorPeriods.iter().max().expect("vector is empty");
        for indicator in &config.indicators {
                match indicator {
                        Indicators::Sma { windows } => {
                                for w in windows {
                                        indicatorPeriods.push(*w);
                                }
                        }
                }
        }



        //Main loop to calculate sma.
        for day_index in (*max_window - 1)..csv.len() {
                for &window in window_sizes.iter() {
                        let start_index = day_index + 1 - window; //day_index (max_window - 1) + 1 - current window
                        let sma = calc_sma(csv[start_index..=day_index].to_vec()); //calculate sma between start_index and day_index
                        sma_hashmap.insert(window, sma);
                }

        }

}
