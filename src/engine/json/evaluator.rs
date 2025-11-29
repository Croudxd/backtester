use crate::engine::model::daily_quote::DailyQuote;
use crate::engine::indicators::sma::calc_sma;
use crate::engine::indicators::rsi::calc_rsi;
use crate::engine::model::ast::Node;




pub fn evaluator(node: &Node, dq: Vec<DailyQuote>) -> f64{
    match(node){

        Node::Number(n) => *n,

        Node::Indicator { &name, period } => {
            eval_indicator(&name, &period, dq);
        },


    }
}


pub fn eval_indicator(name: &str, period: f64, dq: Vec<DailyQuote>) -> f64{

    let s: &str = &name;
    match s {
      "SMA" => {
          return calc_sma(dq);
      },
       "RSI" => {
           //calc_rsi();
           return 4.0;
       }
        _ => panic!("Unknown indicator {}", name),
   }
}
