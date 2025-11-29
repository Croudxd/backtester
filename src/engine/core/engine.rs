use crate::engine::json::loader::loader;
use crate::engine::csv::load_full_csv::load_full_csv;
use std::path::Path;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::metrics::profit_loss::profit_loss;
use crate::engine::model::trade::Trade;
use crate::engine::metrics::win_loss::win_loss;
use crate::engine::model::daily_quote::DailyQuote;
use crate::engine::json::tokenizer::tokenizer;
pub fn backtest_engine(path: &Path)
{
        let csv = load_full_csv();
        let mut trades: Vec<Trade> = Vec::new();

        trades = main_loop(csv);
}


pub fn main_loop(csv: Vec<DailyQuote>) -> Vec<Trade>{
        let sma_one: usize = 10;
        let sma_two: usize = 20;
        let starting_cash: f64 = 1000.0;
        let mut shares: f64 = 0.0;
        let mut current_cash: f64 = starting_cash;
        let mut buy_count: usize = 0;
        let mut sell_count: usize = 0;
        let mut trades: Vec<Trade> = Vec::new();




        for idx in 0..csv.len() {

                if idx <= sma_two {
                        continue;
                } else {
                        let slice_one = csv[idx - sma_one..idx].to_vec();
                        let slice_two = csv[idx - sma_two..idx].to_vec();
                        let sma_calc_1 = calc_sma(slice_one);
                        let sma_calc_2 = calc_sma(slice_two);
                        if sma_calc_1 < sma_calc_2  && current_cash > 0.0
                                {
                                        //This should wait one day before selling/buying
                                        let buy_price = csv[idx].Close;
                                        let mut trade = Trade { id: "buy".to_string(), price: current_cash, shares: 0.0 };
                                        shares =current_cash / buy_price;
                                        trade.shares = shares;
                                        trades.push(trade);

                                        current_cash = 0.0;
                                        println!("{}, {}", shares, current_cash);
                                        buy_count+=1;
                                }
                        if sma_calc_1 > sma_calc_2 && shares > 0.0
                                {
                                        //This should wait one day before selling/buying
                                        let sell_price = csv[idx].Close;
                                        let mut trade = Trade {id: "sell".to_string(), price: 0.0, shares: shares };
                                        current_cash = sell_price * shares;
                                        trade.price = current_cash;
                                        trades.push(trade);
                                        shares = 0.0;
                                        sell_count+=1;
                                        println!("{}, {}", shares, current_cash);
                                }
                        else {
                        }
                }
        }

        let wl = win_loss(trades.clone());
        current_cash += csv[csv.len() - 1].Close * shares;
        let prof_loss = profit_loss(current_cash, starting_cash);
        println!("Current Cash: {}, Buy Count: {}, Sell Count: {}, profit/loss: {}, win/loss: {}", current_cash, buy_count, sell_count, prof_loss, wl);
        return trades;
}
