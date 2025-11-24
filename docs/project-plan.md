## Goals
### **1. Build a Data Acquisition Pipeline**

Create an automated Python-based system that downloads historical market data (e.g., SPY) and stores it in a structured format (CSV). Include basic validation to ensure no missing dates, corrupted rows, or invalid price entries.

### **2. Implement a Reliable Data Loader in Rust**

Develop a Rust module that reads the cleaned market data, parses it into typed structures, and prepares it for simulation. This includes error handling, input-validation, and performance-aware parsing.

### **3. Design a Modular Backtesting Engine**

Build the core simulation engine that executes trades day-by-day. This system should be:

- deterministic
- easily extensible
- able to handle multiple trading strategies

The engine should output performance logs and summary metrics.

### **4. Implement Foundational Trading Indicators**

Add support for core technical indicators such as:

- SMA (Simple Moving Average)
- RSI
- EMA
- VWAP

Each should be modular and togglable within the strategy configuration.

### **5. Add Strategy Configuration Support**

Implement a JSON or TOML config file that controls:

- which indicators are used
- parameter values
- starting balance
- transaction fees
- risk settings

This lets the project function as a general-purpose backtester rather than a single fixed strategy.

### **6. Provide Performance & Risk Reporting**

Generate and display key evaluation metrics such as:

- Profit/Loss
- Win/Loss Ratio
- Maximum Drawdown
- Sharpe Ratio
- Volatility

These should be displayed at the end of each backtest run.