## Market

### [index_price_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#index-price-streams) - Index Price Streams

```bash
binance-cli derivatives-options index-price-streams --id 532601580
```

### [kline_candlestick_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#kline-candlestick-streams) - Kline/Candlestick Streams

```bash
binance-cli derivatives-options kline-candlestick-streams --symbol btcusdt --interval INTERVAL_1m --id 532601580
```

### [new_symbol_info](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#new-symbol-info) - New Symbol Info

```bash
binance-cli derivatives-options new-symbol-info --id 532601580
```

### [open_interest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#open-interest) - Open Interest

```bash
binance-cli derivatives-options open-interest --underlying btcusdt --expiration-date 220930 --id 532601580
```

### [option_mark_price](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#option-mark-price) - Option Mark Price

```bash
binance-cli derivatives-options option-mark-price --underlying btcusdt --id 532601580
```

## Public

### [diff_book_depth_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#diff-book-depth-streams) - Diff Book Depth Streams

```bash
binance-cli derivatives-options diff-book-depth-streams --symbol btcusdt --update-speed UPDATE_SPEED_100ms --id 532601580
```

### [hour24_ticker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#hour24-ticker) - 24-hour TICKER

```bash
binance-cli derivatives-options hour24-ticker --symbol btcusdt --id 532601580 --expiration-date 251230
```

### [individual_symbol_book_ticker_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#individual-symbol-book-ticker-streams) - Individual Symbol Book Ticker Streams

```bash
binance-cli derivatives-options individual-symbol-book-ticker-streams --symbol btcusdt --id 532601580
```

### [partial_book_depth_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#partial-book-depth-streams) - Partial Book Depth Streams

```bash
binance-cli derivatives-options partial-book-depth-streams --symbol btcusdt --level LEVEL_5 --update-speed UPDATE_SPEED_100ms --id 532601580
```

### [trade_streams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#trade-streams) - Trade Streams

```bash
binance-cli derivatives-options trade-streams --symbol btcusdt --id 532601580
```
