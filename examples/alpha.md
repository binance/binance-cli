## MarketData

### [GET /bapi/defi/v1/public/alpha-trade/agg-trades](https://developers.binance.com/docs/alpha/market-data/rest-api/Aggregated-Trades) - Aggregated Trades

```bash
binance-cli alpha aggregated-trades --symbol "symbol_example" --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500
```

### [GET /bapi/defi/v1/public/alpha-trade/get-exchange-info](https://developers.binance.com/docs/alpha/market-data/rest-api/Get-Exchange-Info) - Get Exchange Info

```bash
binance-cli alpha get-exchange-info
```

### [GET /bapi/defi/v1/public/alpha-trade/klines](https://developers.binance.com/docs/alpha/market-data/rest-api/Klines) - Klines (Candlestick Data)

```bash
binance-cli alpha klines --symbol "symbol_example" --interval "interval_example" --limit 500 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /bapi/defi/v1/public/alpha-trade/ticker](https://developers.binance.com/docs/alpha/market-data/rest-api/24hr-ticker-price-change) - Ticker (24hr Price Statistics)

```bash
binance-cli alpha ticker --symbol "symbol_example"
```

### [GET /bapi/defi/v1/public/wallet-direct/buw/wallet/cex/alpha/all/token/list](https://developers.binance.com/docs/alpha/market-data/rest-api/Token-List) - Token List

```bash
binance-cli alpha token-list
```
