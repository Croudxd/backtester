use crate::engine::core::backtest_engine::backtest_engine;
mod engine;
use crate::engine::model::candle::Candle;
use std::path::Path;
use crate::engine::csv::load_full_csv::load_full_csv;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::indicators::rsi::calc_init_rsi;
use crate::engine::model::portfolio::Portfolio;
use crate::engine::model::portfolio::BacktestPortfolio;
use crate::engine::model::context::Context;

fn strategy(context: &mut Context)
{
  
    if context.calc_rsi(10) < context.calc_sma(30) {
        context.buy();
    }

    if context.calc_sma(10) > context.calc_sma(30) {
        context.sell();
    }

}

fn main() {
    let path = "data/QQQ.csv".to_string();
    backtest_engine(strategy, 100000, path); //Starting cash, path for csv, anything else needed.

}



