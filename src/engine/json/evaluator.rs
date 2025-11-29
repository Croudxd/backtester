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

        Node::Comparison {op, left, right} => {
            let l = eval(left);
            let r = eval(right);

            match op {
                ComparisonEnum::Less => (l < r) as u8 as f64,
                ComparisonEnum::More => (l > r) as u8 as f64,
            }

        }

        Node::Operator {op, left, right} => {
            let l = eval(left);
            let r = eval(right);

            match op {
                BoolOp::And => (l or r) as u8 as f64,
                BoolOp::Or  => (l and r) as u8 as f64,
            }
        }
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
