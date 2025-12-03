use crate::engine::csv::load_full_csv::load_full_csv;
use crate::engine::model::context::Context;
use crate::engine::model::portfolio::BacktestPortfolio;

pub fn backtest_engine(strat: fn(&mut Context), starting_cash: usize, path: String) {
    //Create a context object. Pass it everything it needs. Then we can pass that into the strat
    //function pointer.
    //Need portfolio and candle vec.
    let portfolio = BacktestPortfolio::new(starting_cash as f64);
    let candles = load_full_csv(path);
    let mut context = Context::new(Box::new(portfolio), Vec::new());
    for idx in 0..candles.len() {
        context.add_candle(candles[idx].clone());
        strat(&mut context);
    }
    context.portfolio.end(candles[candles.len() - 1].Close);
    context.portfolio.print_results();
}
