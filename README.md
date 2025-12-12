# **Strategy-backtest-engine**
- - -
_A modular, extensible backtesting engine written in Rust with automated data acquisition._
- - -

Strategy-backtest-engine is a lightweight but extensible backtesting framework built in Rust with automated Python-based data ingestion. It is designed to simulate trading strategies on historical market data while emphasizing clarity, performance, and modularity. The project serves as both a practical tool for experimentation and a strong portfolio example of systems design, data processing, and quantitative analysis.

Fully built in rust.

- - - 
## Running main and tests
To run:
1. Clone the repo.
2. Inside backtester run `cargo build`
3. run `cargo run`
This will run main which holds a pre configured strategy.

To test:
1. `cargo test`

- - - 
## How to use:
To gather the data:
Create a venv and install yfinance
`pip install yfinance`
`python/pull_data.py`

Below is a basic usage of the program. 

Just create a function we can then pass this function name to the backtest_engine function with the starting cash and path of the csv.

Once main is run it will produce the results to console.
```rust
use crate::engine::core::backtest_engine::backtest_engine;
use crate::engine::model::context::Context;

//basic strategy
fn strategy(context: &mut Context) {
    if context.calc_ema(10) < context.calc_ema(30) {
        context.buy();
    }
    if context.calc_rsi(20) > context.calc_rsi(30) {
        context.sell();
    }
}

fn main() {
    let path = "data/QQQ.csv".to_string();
    backtest_engine(strategy, 100000, path); //pass in the strategy, starting cash and the path to csv.
}
```

### Conclusion
- - -
This project is quite messy. It was my first time building something like this and so I dont expect it to be perfect, also learning indicators for the first time so only a minimal amount.

Somethings that can be improved:
 - Cleaning up the code
 - Adding more indicators
 - Adding more metrics
 - Adding a better output
 - Adding more tests
 - Adding benchmarks for speed
 - Probably a few ways to speed up the engine, however as its not life its not too threatening.

Overall this was a fun and good project built to learn about backtesters, and put my foot into the world of quantative development it also helped teach me some more rust skills, now i know about function pointers and impl/traits.

This probably wont be continued unless I find the motivation down the road.
