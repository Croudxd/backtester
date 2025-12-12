use crate::engine::indicators::ema::calc_ema;
use crate::engine::indicators::rsi::calc_avg_gain_loss;
use crate::engine::indicators::rsi::calc_init_rsi;
use crate::engine::indicators::rsi::calc_rsi;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::model::candle::Candle;
use crate::engine::model::portfolio::Portfolio;

struct avg_gain_loss {
    avg_gain: f64,
    avg_loss: f64,
}

pub struct Context {
    pub portfolio: Box<dyn Portfolio>,
    pub candle: Vec<Candle>,
    avg_history: Vec<avg_gain_loss>,
    prev_ema: Vec<f64>,
}

impl Context {
    pub fn new(portfolio: Box<dyn Portfolio>, candle: Vec<Candle>) -> Self {
        Self {
            portfolio,
            candle,
            avg_history: Vec::new(),
            prev_ema: Vec::new(),
        }
    }

    pub fn buy(&mut self) {
        let close = self.candle[self.candle.len() - 1].Close;
        self.portfolio
            .buy(close, self.candle[self.candle.len() - 1].Date);
    }
    pub fn sell(&mut self) {
        let close = self.candle[self.candle.len() - 1].Close;
        self.portfolio
            .sell(close, self.candle[self.candle.len() - 1].Date);
    }

    pub fn calc_sma(&mut self, size: usize) -> f64 {
        return calc_sma(size, &self.candle);
    }

    pub fn calc_rsi(&mut self, size: usize) -> f64 {
        if self.candle.len() < size {
            return 0.0;
        } else if self.candle.len() == size {
            let (avg_gain, avg_loss) = calc_init_rsi(&self.candle);
            let avg = avg_gain_loss {
                avg_gain: avg_gain,
                avg_loss: avg_loss,
            };
            self.avg_history.push(avg);
            return 0.0;
        } else {
            let (avg_gain, avg_loss) = calc_avg_gain_loss(
                size,
                &self.candle,
                self.avg_history[self.avg_history.len() - 1].avg_gain,
                self.avg_history[self.avg_history.len() - 1].avg_loss,
            );
            self.avg_history.push(avg_gain_loss {
                avg_gain: avg_gain,
                avg_loss: avg_loss,
            });

            return calc_rsi(avg_gain, avg_loss);
        }
    }
    pub fn calc_ema(&mut self, size: usize) -> f64 {
        if self.candle.len() < size {
            return 0.0;
        } else if self.candle.len() == size {
            let sma: f64 = calc_sma(size, &self.candle);
            self.prev_ema.push(sma);
            return 0.0;
        } else {
            let ema: f64 = calc_ema(size, &self.candle, self.prev_ema[self.prev_ema.len() - 1]);
            self.prev_ema.push(ema);
            return ema;
        }
    }

    pub fn add_candle(&mut self, candle: Candle) {
        self.candle.push(candle);
    }

    pub fn results(&mut self) {
        self.portfolio.print_results();
    }
}
