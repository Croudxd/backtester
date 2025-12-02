use crate::engine::model::candle::Candle;
use crate::engine::model::portfolio::Portfolio;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::indicators::rsi::calc_rsi;
use crate::engine::indicators::rsi::calc_init_rsi;
struct Avg_gain_loss{
   avg_gain: f64,
   avg_loss: f64,
}


pub struct Context{

    pub portfolio: Box<dyn Portfolio>,
    pub candle: Vec<Candle>,
    avg_history: Vec<Avg_gain_loss>
}


impl Context{
    pub fn new(portfolio: Box<dyn Portfolio>, candle: Vec<Candle>) -> Self{
        Self{
            portfolio,
            candle,
            avg_history: Vec::new(),
        }
    }

    pub fn buy(&mut self){
        
        let close = self.candle[self.candle.len() -1].Close;
        self.portfolio.buy(close);
    }

    pub fn sell(&mut self){
        let close = self.candle[self.candle.len() -1].Close;
        self.portfolio.sell(close);
    }

    pub fn calc_sma(&mut self, size: usize)-> f64{
       return calc_sma(size, &self.candle);
    }

    pub fn calc_rsi(&mut self, size: usize)->f64{

        if self.candle.len() < size{
            return 0.0; 
        }
        else if self.candle.len() == size{
            let (avg_gain, avg_loss) = calc_init_rsi(size, &self.candle);
            let avg = Avg_gain_loss{avg_gain: avg_gain, avg_loss: avg_loss};
            self.avg_history.push(avg);
            return 0.0;
        }
        else{
            return calc_rsi(size, &self.candle, self.avg_history[self.avg_history.len()-1].avg_gain,  self.avg_history[self.avg_history.len()-1].avg_loss);
        }
    


    }

    pub fn add_candle(&mut self, candle: Candle){
        self.candle.push(candle);
    }

}
