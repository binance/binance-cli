## MarketData

### [GET /sapi/v1/convert/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#list-all-convert-pairs) - List All Convert Pairs

```bash
binance-cli convert list-all-convert-pairs --from-asset BTC --to-asset USDT
```

### [GET /sapi/v1/convert/assetInfo](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#query-order-quantity-precision-per-asset) - Query order quantity precision per asset (USER_DATA)

```bash
binance-cli convert query-order-quantity-precision-per-asset --recv-window 5000
```

## Trade

### [POST /sapi/v1/convert/acceptQuote](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote) - Accept Quote (TRADE)

```bash
binance-cli convert accept-quote --quote-id 1 --recv-window 5000
```

### [POST /sapi/v1/convert/limit/cancelOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#cancel-limit-order) - Cancel limit order (TRADE)

```bash
binance-cli convert cancel-limit-order --order-id 1603680255057330400 --recv-window 5000
```

### [GET /sapi/v1/convert/tradeFlow](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#get-convert-trade-history) - Get Convert Trade History (USER_DATA)

```bash
binance-cli convert get-convert-trade-history --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /sapi/v1/convert/orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#order-status) - Order status (USER_DATA)

```bash
binance-cli convert order-status --order-id 1 --quote-id 1
```

### [POST /sapi/v1/convert/limit/placeOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#place-limit-order) - Place limit order (TRADE)

```bash
binance-cli convert place-limit-order --base-asset BTC --quote-asset USDT --limit-price 1 --side BUY --expired-type EXPIRED_TYPE_1_D --base-amount 1 --quote-amount 1 --wallet-type SPOT --recv-window 5000
```

### [GET /sapi/v1/convert/limit/queryOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#query-limit-open-orders) - Query limit open orders (USER_DATA)

```bash
binance-cli convert query-limit-open-orders --recv-window 5000
```

### [POST /sapi/v1/convert/getQuote](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#send-quote-request) - Send Quote Request (TRADE)

```bash
binance-cli convert send-quote-request --from-asset BTC --to-asset USDT --from-amount 1 --to-amount 1 --wallet-type SPOT --valid-time VALID_TIME_10s --recv-window 5000
```
