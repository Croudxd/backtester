import os
import yfinance as yf

data_dir = os.path.join("..", "data")
os.makedirs(data_dir, exist_ok=True)

tickers = ["SPY", "QQQ"]
for ticker in tickers:
    data = yf.download(ticker, period="1y", interval="1d")
    filepath = os.path.join(data_dir, f"{ticker}.csv")
    data.to_csv(filepath)
    print(f"Saved {ticker} data to {filepath}")
