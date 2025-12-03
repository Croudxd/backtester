use crate::engine::model::trade::Trade;

pub fn max_drawdown(trades: &Vec<Trade>) -> f64 {
    //Loop through the array and find the Trade when total is the highest.
    //Loop through ... when total is smallest.
    //then the formula is (biggest/smallest) /biggest * 100
    let mut biggest: f64 = 0.0;
    let mut smallest: f64 = 10000000.00;
    for x in 0..trades.len() {
        if trades[x].total > biggest {
            biggest = trades[x].total;
        }

        if trades[x].total < smallest {
            smallest = trades[x].total;
        }
    }

    return (biggest / smallest) / biggest * 100.0;
}
