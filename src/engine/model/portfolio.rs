pub trait Portfolio {
    //Functions here such as buy sell get...
    fn buy(&mut self, price: f64);
    fn sell(&mut self, price: f64);
    fn print_results(&mut self);
    fn end(&mut self, price: f64);
}

pub struct BacktestPortfolio {
    pub starting_cash: f64,
    pub current_cash: f64,
    pub stocks: f64,
}

impl BacktestPortfolio {
    pub fn new(starting_cash: f64) -> Self {
        Self {
            starting_cash,
            current_cash: starting_cash,
            stocks: 0.0,
        }
    }
}

impl Portfolio for BacktestPortfolio {
    fn buy(&mut self, price: f64) {
        let shares_to_buy = (self.current_cash / price).trunc();
        if shares_to_buy == 0.0 {
        } else {
            self.stocks += shares_to_buy;
            self.current_cash -= shares_to_buy * price;
            //println!(" buying at {}, this many shares: {}", price, shares_to_buy);
        }
    }

    fn sell(&mut self, price: f64) {
        if self.stocks >= 1.0 {
            self.current_cash += self.stocks * price;
            //println!(" selling at {}, this many shares: {}", price, self.stocks);
            self.stocks = 0.0;
        }
    }
    fn print_results(&mut self) {
        print!("Current cash: {} ", self.current_cash);
    }

    fn end(&mut self, price: f64) {
        if self.stocks != 0.0 {
            self.sell(price);
        }
    }
}
