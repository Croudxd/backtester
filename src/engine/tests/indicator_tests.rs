#[cfg(test)]
mod tests {
    use crate::engine::indicators::ema::calc_ema;
    use crate::engine::indicators::rsi::calc_avg_gain_loss;
    use crate::engine::indicators::rsi::calc_init_rsi;
    use crate::engine::indicators::rsi::calc_rsi;
    use crate::engine::indicators::sma::calc_sma;
    use crate::engine::model::candle::Candle;
    use crate::engine::model::context::Context;
    use crate::engine::model::portfolio::{BacktestPortfolio, Portfolio};
    use chrono::Local;

    #[test]
    fn sma_test() {
        let candle: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 30.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle1: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 40.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle2: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };

        let mut candles: Vec<Candle> = Vec::new();
        candles.push(candle);
        candles.push(candle1);
        candles.push(candle2);

        let slice: &[Candle] = &candles[0..2];
        let result: f64 = calc_sma(2, slice);
        assert_eq!(result, 35.0);

        let slice2: &[Candle] = &candles[0..3];
        let result2: f64 = calc_sma(3, slice2);
        assert_eq!(result2, 40.0);
    }

    #[test]
    fn rsi_test() {
        let candle: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 30.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle1: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 40.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle2: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle3: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle4: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle5: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle6: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle7: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle8: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let mut vec: Vec<Candle> = Vec::new();
        vec.push(candle);
        vec.push(candle1);
        vec.push(candle2);
        vec.push(candle3);
        vec.push(candle4);
        vec.push(candle5);
        let (r1, r2) = calc_init_rsi(&vec);
        vec.push(candle6);
        vec.push(candle7);
        let (avg_gain, avg_loss) = calc_avg_gain_loss(3, &vec.clone(), r1, r2);
        let result = calc_rsi(avg_gain, avg_loss);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn ema_test() {
        let candle: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 30.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle1: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 40.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle2: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let candle3: Candle = Candle {
            Date: Local::now().date_naive(),
            Close: 50.0,
            High: 40.0,
            Low: 50.0,
            Open: 30.0,
            Volume: 35.0,
        };
        let mut vec: Vec<Candle> = Vec::new();
        vec.push(candle);
        vec.push(candle1);
        let sma: f64 = calc_sma(2, &vec);
        vec.push(candle2);
        vec.push(candle3);
        let r = calc_ema(4, &vec, sma);
        assert_eq!(r, 41.0);
    }
}
