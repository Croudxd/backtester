use crate::engine::core::backtest_engine::backtest_engine;
mod engine;
use crate::engine::model::candle::Candle;
use std::path::Path;
use crate::engine::csv::load_full_csv::load_full_csv;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::indicators::rsi;
use crate::engine::model::portfolio::Portfolio;
use crate::engine::model::portfolio::BacktestPortfolio;


fn main() {
    //define strategy using a closure
    let mut strategy = |candle: &[Candle], portfolio: &mut BacktestPortfolio|{
        
        if calc_sma(10, candle) < calc_sma(30, candle)
        {
            let close = candle[candle.len() - 1].Close;
            portfolio.buy(close); 
            
        }
        else if calc_sma(30, candle) < calc_sma(10, candle)
        {
            let close = candle[candle.len() - 1].Close;
            portfolio.sell(close); 
        }
    };


    let mut portfolio = BacktestPortfolio::new(10000.0); 
    backtest_engine(strategy, &mut portfolio);
    portfolio.print_results();
    //live_engine(closure)
}
