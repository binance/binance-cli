## MarketData

### [GET /bapi/defi/v1/public/alpha-trade/agg-trades](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#aggregated-trades) - Aggregated Trades

```bash
binance-cli alpha aggregated-trades --symbol ALPHA_118USDC --from-id 58470 --start-time 1752568680000 --end-time 1752572280000 --limit 500
```

### [GET /bapi/defi/v1/public/alpha-trade/fullDepth](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#full-depth) - Full Depth

```bash
binance-cli alpha full-depth --symbol ALPHA_175USDT --limit 789
```

### [GET /bapi/defi/v1/public/alpha-trade/get-exchange-info](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#get-exchange-info) - Get Exchange Info

```bash
binance-cli alpha get-exchange-info
```

### [GET /bapi/defi/v1/public/alpha-trade/klines](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#klines) - Klines

```bash
binance-cli alpha klines --symbol ALPHA_175USDT --interval INTERVAL_1s --limit 500 --start-time 1752642000000 --end-time 1752645599999
```

### [GET /bapi/defi/v1/public/alpha-trade/ticker](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#ticker) - Ticker

```bash
binance-cli alpha ticker --symbol ALPHA_175USDT
```

### [GET /bapi/defi/v1/public/wallet-direct/buw/wallet/cex/alpha/all/token/list](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api/market-data#token-list) - Token List

```bash
binance-cli alpha token-list
```
