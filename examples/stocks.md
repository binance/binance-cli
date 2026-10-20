## Account

### [POST /sapi/v1/equity/account/disclaimer](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/account#sign-us-equity-disclaimer) - Sign US Equity Disclaimer (TRADE)

```bash
binance-cli stocks sign-us-equity-disclaimer --recv-window 5000
```

## MarketData

### [GET /sapi/v1/equity/market/exchangeInfo](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/market-data#exchange-info) - Exchange Info (MARKET_DATA)

```bash
binance-cli stocks exchange-info --symbol AAPL
```

### [GET /sapi/v1/equity/market/quote](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/market-data#latest-quote) - Latest Quote (MARKET_DATA)

```bash
binance-cli stocks latest-quote --symbol AAPL
```

### [GET /sapi/v1/equity/market/tokenized-assets](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/market-data#tokenized-assets) - Tokenized Assets (MARKET_DATA)

```bash
binance-cli stocks tokenized-assets
```

## Tokenized

### [GET /sapi/v1/equity/tokenized/history](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/tokenized#tokenized-convert-history) - Tokenized Convert History (USER_DATA)

```bash
binance-cli stocks tokenized-convert-history --start-time 1735800000000 --end-time 1735900000000 --last-id 10019 --size 20 --recv-window 5000
```

### [GET /sapi/v1/equity/tokenized/convert-status](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/tokenized#tokenized-convert-status) - Tokenized Convert Status (USER_DATA)

```bash
binance-cli stocks tokenized-convert-status --issuer-request-id mint-20260505-8f3b9e1a2d3c4b5a --convert-type MINT --recv-window 5000
```

### [POST /sapi/v1/equity/tokenized/mint](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/tokenized#tokenized-mint) - Tokenized Mint (TRADE)

```bash
binance-cli stocks tokenized-mint --underlying-asset AAPL --underlying-asset-amount 1 --client-order-id mint-client-id-32chars-0000000001 --recv-window 5000
```

### [POST /sapi/v1/equity/tokenized/redeem](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/tokenized#tokenized-redeem) - Tokenized Redeem (TRADE)

```bash
binance-cli stocks tokenized-redeem --tokenized-asset AAPLB --tokenized-asset-amount 1 --client-order-id redeem-client-id-32chars-000000001 --recv-window 5000
```

## Trade

### [POST /sapi/v1/equity/order/cancel-all](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#cancel-all-equity-orders) - Cancel All Equity Orders (TRADE)

```bash
binance-cli stocks cancel-all-equity-orders --recv-window 5000
```

### [POST /sapi/v1/equity/order/cancel](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#cancel-equity-order) - Cancel Equity Order (TRADE)

```bash
binance-cli stocks cancel-equity-order --order-id c3c58f49-7b0d-4b9e-a2db-1a2f9a3b8c71 --recv-window 5000
```

### [GET /sapi/v1/equity/order/open-orders](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#current-open-orders) - Current Open Orders (USER_DATA)

```bash
binance-cli stocks current-open-orders --recv-window 5000
```

### [GET /sapi/v1/equity/order/detail](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#equity-order-detail) - Equity Order Detail (USER_DATA)

```bash
binance-cli stocks equity-order-detail --order-id c3c58f49-7b0d-4b9e-a2db-1a2f9a3b8c71 --client-order-id web_2c9c92b74f1e4a7c8f3b9e1a2d3c4b5a --recv-window 5000
```

### [GET /sapi/v1/equity/order/history](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#equity-order-history) - Equity Order History (USER_DATA)

```bash
binance-cli stocks equity-order-history --start-time 1735800000000 --end-time 1735900000000 --symbol NVDA --order-type LIMIT --side BUY --order-status FILLED,CANCELED --current 1 --size 20 --recv-window 5000
```

### [GET /sapi/v1/equity/trade/history](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#equity-trade-history) - Equity Trade History (USER_DATA)

```bash
binance-cli stocks equity-trade-history --start-time 1735800000000 --end-time 1735900000000 --symbol NVDA --side BUY --order-id c3c58f49-7b0d-4b9e-a2db-1a2f9a3b8c71 --current 1 --size 20 --recv-window 5000
```

### [POST /sapi/v1/equity/order/place](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/trade#place-equity-order) - Place Equity Order (TRADE)

```bash
binance-cli stocks place-equity-order --symbol AAPL --side BUY --order-type LIMIT --quote-asset USDC --price 180.50 --quantity 1 --notional 1000.00 --time-in-force DAY --trading-session RTH --wallet-type CARD --client-order-id web_2c9c92b74f1e4a7c8f3b9e1a2d3c4b5a --tokenize true --recv-window 5000
```

## UserDataStreams

### [POST /sapi/v1/equity/listenKey](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api/user-data-streams#create-renew-listen-key) - Create / Renew Listen Key (USER_STREAM)

```bash
binance-cli stocks create-renew-listen-key --recv-window 5000
```
