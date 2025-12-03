use crate::engine::core::backtest_engine::backtest_engine;
mod engine;
use crate::engine::model::context::Context;

fn strategy(context: &mut Context) {
    if context.calc_ema(10) < context.calc_ema(30) {
        context.buy();
    }
    if context.calc_rsi(20) > context.calc_rsi(30) {
        context.sell();
    }
}

fn main() {
    let path = "data/QQQ.csv".to_string();
    backtest_engine(strategy, 100000, path); //Starting cash, path for csv, anything else needed.
    //
}
