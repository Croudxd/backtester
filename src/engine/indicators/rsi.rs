use crate::engine::model::daily_quote::DailyQuote;


pub fn calc_init_rsi(vec: Vec<DailyQuote>) -> (f64, f64){


    let up_moves = calc_upMoves(vec.clone());
    let down_moves = calc_downMoves(vec.clone());

    let upmoves_sma = calc_avg(up_moves);
    let downmoves_sma = calc_avg(down_moves);
    return (upmoves_sma, downmoves_sma);
}

pub fn calc_rsi(avg_gain: f64, avg_loss: f64) -> f64{
   if avg_loss == 0.0 && avg_gain == 0.0 {
        return 50.0; // no movement at all
    }

    if avg_loss == 0.0 {
        return 100.0; // only gains
    }

    if avg_gain == 0.0 {
        return 0.0; // only losses
    }
    let rs = avg_gain / avg_loss;
    return 100.0 - ( 100.0 / (1.0 + rs ) )
}



fn calc_upMoves(vec: Vec<DailyQuote>) -> Vec<f64> {

    let mut up_move_vec: Vec<f64> = Vec::new();
    //Calculate close t - close t-1
    // either a positive number or 0
    for idx in 0..vec.len() {
        if idx > 1{
            let up_move = vec[idx].Close - vec[idx - 1].Close;
            if up_move < 0.0 {
                    up_move_vec.push(0.0);
            }
            else {
               up_move_vec.push(up_move)
            }
        }
        else {
            continue;
        }
    }
    return up_move_vec;
}

fn calc_downMoves(vec: Vec<DailyQuote>) -> Vec<f64> {
    //Calculate close t-1 - close t
    // either a positive number or 0
    let mut down_move_vec: Vec<f64> = Vec::new();
    for idx in 0..vec.len() {
        if idx > 1{
            let down_move = vec[idx - 1].Close - vec[idx].Close;
            if down_move < 0.0
                {
                    down_move_vec.push(0.0);
                }
            else{
               down_move_vec.push(down_move)
            }
        }
        else {
            continue;
        }
    }
    return down_move_vec;
}
// Use wilders here
fn calc_wilders(prev_avg: f64, current_avg: f64, length: f64) -> f64
    {
        return (prev_avg * (length - 1.0)) + current_avg / length;
    }


pub fn calc_avg(vec: Vec<f64>) -> f64
    {
        let mut sum: f64 = 0.0;
        for idx in 0..vec.len(){
            if vec[idx] == 0.0
                {
                    continue;
                }
            else {
                sum += vec[idx];
            }
        }
        //(sum of up) / n
        return sum / vec.len() as f64;
    }
