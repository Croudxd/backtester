use crate::engine::metrics::max_drawdown;
use crate::engine::metrics::max_drawdown::max_drawdown;
use crate::engine::metrics::profit_loss::profit_loss;
use crate::engine::model::trade::Trade;
use crate::engine::model::trade::TradeType;
use chrono::NaiveDate;

pub trait Portfolio {
    //Functions here such as buy sell get...
    fn buy(&mut self, price: f64, date: NaiveDate);
    fn sell(&mut self, price: f64, date: NaiveDate);
    fn print_results(&mut self);
    fn end(&mut self, price: f64, date: NaiveDate);
    fn get_trade_history(&mut self) -> &Vec<Trade>;
}

pub struct BacktestPortfolio {
    pub starting_cash: f64,
    pub current_cash: f64,
    pub stocks: f64,
    pub trade_history: Vec<Trade>,
}

impl BacktestPortfolio {
    pub fn new(starting_cash: f64) -> Self {
        Self {
            starting_cash,
            current_cash: starting_cash,
            stocks: 0.0,
            trade_history: Vec::new(),
        }
    }
}

impl Portfolio for BacktestPortfolio {
    fn buy(&mut self, price: f64, date: NaiveDate) {
        let shares_to_buy = (self.current_cash / price).trunc();
        if shares_to_buy == 0.0 {
        } else {
            self.stocks += shares_to_buy;
            self.current_cash -= shares_to_buy * price;
            self.trade_history.push(Trade {
                date: date,
                trade_type: TradeType::Buy,
                price: price,
                shares: self.stocks,
                total: self.current_cash,
            });
        }
    }

    fn sell(&mut self, price: f64, date: NaiveDate) {
        if self.stocks >= 1.0 {
            self.current_cash += self.stocks * price;
            //println!(" selling at {}, this many shares: {}", price, self.stocks);
            self.stocks = 0.0;
            self.trade_history.push(Trade {
                date: date,
                trade_type: TradeType::Buy,
                price: price,
                shares: self.stocks,
                total: self.current_cash,
            });
        }
    }
    fn print_results(&mut self) {
        println!("Current cash: {} ", self.current_cash);
        println!("Max drawdown: {}", max_drawdown(&self.trade_history));
        println!("Win Loss: ");
    }

    fn end(&mut self, price: f64, date: NaiveDate) {
        if self.stocks != 0.0 {
            self.sell(price, date);
        }
    }

    fn get_trade_history(&mut self) -> &Vec<Trade> {
        return &self.trade_history;
    }
}
