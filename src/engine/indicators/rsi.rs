use crate::engine::model::candle::Candle;

pub fn calc_init_rsi(vec: &[Candle]) -> (f64, f64) {
    let up_moves = calc_up_moves(vec);
    let down_moves = calc_down_moves(vec);

    let upmoves_sma = calc_sma(up_moves);
    let downmoves_sma = calc_sma(down_moves);
    return (upmoves_sma, downmoves_sma);
}

pub fn calc_avg_gain_loss(
    size: usize,
    vec: &[Candle],
    prev_avg_gain: f64,
    prev_avg_loss: f64,
) -> (f64, f64) {
    let (current_gain, current_loss) = calc_avg(vec[vec.len() - 2].Close, vec[vec.len() - 1].Close);
    let avg_gain = calc_wilders(prev_avg_gain, current_gain, size as f64);
    let avg_loss = calc_wilders(prev_avg_loss, current_loss, size as f64);
    return (avg_gain, avg_loss);
}

pub fn calc_rsi(avg_gain: f64, avg_loss: f64) -> f64 {
    let rs = avg_gain / avg_loss;
    let rsi = 100.00 - (100.00 / (1.0 + rs));
    return rsi;
}

fn calc_up_moves(vec: &[Candle]) -> Vec<f64> {
    let mut up_move_vec: Vec<f64> = Vec::new();
    //Calculate close t - close t-1
    // either a positive number or 0
    for idx in 0..vec.len() {
        if idx > 1 {
            let up_move = vec[idx].Close - vec[idx - 1].Close;
            if up_move < 0.0 {
                up_move_vec.push(0.0);
            } else {
                up_move_vec.push(up_move)
            }
        } else {
            continue;
        }
    }
    return up_move_vec;
}

fn calc_down_moves(vec: &[Candle]) -> Vec<f64> {
    //Calculate close t-1 - close t
    // either a positive number or 0
    let mut down_move_vec: Vec<f64> = Vec::new();
    for idx in 0..vec.len() {
        if idx > 1 {
            let down_move = vec[idx - 1].Close - vec[idx].Close;
            if down_move < 0.0 {
                down_move_vec.push(0.0);
            } else {
                down_move_vec.push(down_move)
            }
        } else {
            continue;
        }
    }
    return down_move_vec;
}
// Use wilders here
fn calc_wilders(prev_avg: f64, current_avg: f64, length: f64) -> f64 {
    return ((prev_avg * (length - 1.0)) + current_avg) / length;
}

fn calc_sma(vec: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for idx in 0..vec.len() {
        if vec[idx] == 0.0 {
            continue;
        } else {
            sum += vec[idx];
        }
    }
    //(sum of up) / n
    return sum / vec.len() as f64;
}

pub fn calc_avg(close1: f64, close2: f64) -> (f64, f64) {
    let change = close1 - close2;
    let gain = change.max(0.0);
    let loss = (-change).max(0.0);
    return (gain, loss);
}
