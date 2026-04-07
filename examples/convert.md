# Convert

## MarketData

### [GET /sapi/v1/convert/exchangeInfo](https://developers.binance.com/docs/convert/market-data/) - List All Convert Pairs

```bash
binance-cli convert list-all-convert-pairs --from-asset "fromAsset_example" --to-asset "toAsset_example"
```

### [GET /sapi/v1/convert/assetInfo](https://developers.binance.com/docs/convert/market-data/Query-order-quantity-precision-per-asset) - Query order quantity precision per asset

```bash
binance-cli convert query-order-quantity-precision-per-asset --recv-window 5000
```

## Trade

### [POST /sapi/v1/convert/acceptQuote](https://developers.binance.com/docs/convert/trade/Accept-Quote) - Accept Quote

```bash
binance-cli convert accept-quote --json {}
```

### [POST /sapi/v1/convert/limit/cancelOrder](https://developers.binance.com/docs/convert/trade/Cancel-Order) - Cancel limit order

```bash
binance-cli convert cancel-limit-order --json {}
```

### [GET /sapi/v1/convert/tradeFlow](https://developers.binance.com/docs/convert/trade/Get-Convert-Trade-History) - Get Convert Trade History

```bash
binance-cli convert get-convert-trade-history --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /sapi/v1/convert/orderStatus](https://developers.binance.com/docs/convert/trade/Order-Status) - Order status

```bash
binance-cli convert order-status --order-id "1" --quote-id "1"
```

### [POST /sapi/v1/convert/limit/placeOrder](https://developers.binance.com/docs/convert/trade/Place-Order) - Place limit order

```bash
binance-cli convert place-limit-order --json {}
```

### [GET /sapi/v1/convert/limit/queryOpenOrders](https://developers.binance.com/docs/convert/trade/Query-Order) - Query limit open orders

```bash
binance-cli convert query-limit-open-orders --recv-window 5000
```

### [POST /sapi/v1/convert/getQuote](https://developers.binance.com/docs/convert/trade/Send-quote-request) - Send Quote Request

```bash
binance-cli convert send-quote-request --json {}
```
