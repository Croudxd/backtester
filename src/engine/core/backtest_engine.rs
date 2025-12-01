use crate::engine::csv::load_full_csv::load_full_csv;
use crate::engine::model::candle::Candle;
use crate::engine::model::portfolio::BacktestPortfolio;
use crate::engine::model::portfolio::Portfolio;

//Take a closure, use the closure in the loop.

pub fn backtest_engine<F>(mut strategy: F, portfolio: &mut BacktestPortfolio)
where
    F: FnMut(&[Candle], &mut BacktestPortfolio),
{
        let csv = load_full_csv();

        for idx in 0..csv.len(){

            strategy(&csv[0..idx], portfolio)

        }

        let close = csv[csv.len() - 1].Close;
        portfolio.end(close);

}



