## FutureAlgo

### [DELETE /sapi/v1/algo/futures/order](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#cancel-algo-order-future-algo) - Cancel Futures Algo Order (TRADE)

```bash
binance-cli algo cancel-algo-order-future-algo --algo-id 1 --recv-window 5000
```

### [GET /sapi/v1/algo/futures/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#query-current-algo-open-orders-future-algo) - Query Current Futures Algo Open Orders (USER_DATA)

```bash
binance-cli algo query-current-algo-open-orders-future-algo --recv-window 5000
```

### [GET /sapi/v1/algo/futures/historicalOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#query-historical-algo-orders-future-algo) - Query Historical Futures Algo Orders (USER_DATA)

```bash
binance-cli algo query-historical-algo-orders-future-algo --symbol BTCUSDT --side BUY --start-time 1623319461670 --end-time 1641782889000 --page 1 --page-size 100 --recv-window 5000
```

### [GET /sapi/v1/algo/futures/subOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#query-sub-orders-future-algo) - Query Futures Sub Orders (USER_DATA)

```bash
binance-cli algo query-sub-orders-future-algo --algo-id 1 --page 1 --page-size 100 --recv-window 5000
```

### [POST /sapi/v1/algo/futures/newOrderTwap](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#time-weighted-average-price-future-algo) - Time-Weighted Futures Average Price (Twap) New Order (TRADE)

```bash
binance-cli algo time-weighted-average-price-future-algo --symbol BTCUSDT --side BUY --quantity 1 --duration 5000 --position-side BOTH --client-algo-id 1 --reduce-only false --limit-price 1 --recv-window 5000
```

### [POST /sapi/v1/algo/futures/newOrderVp](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/future-algo#volume-participation-future-algo) - Volume Participation (VP) New Order (TRADE)

```bash
binance-cli algo volume-participation-future-algo --symbol BTCUSDT --side BUY --quantity 1 --urgency LOW --position-side BOTH --client-algo-id 1 --reduce-only false --limit-price 1 --recv-window 5000
```

## SpotAlgo

### [DELETE /sapi/v1/algo/spot/order](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/spot-algo#cancel-algo-order-spot-algo) - Cancel Spot Algo Order (TRADE)

```bash
binance-cli algo cancel-algo-order-spot-algo --algo-id 14511 --recv-window 5000
```

### [GET /sapi/v1/algo/spot/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/spot-algo#query-current-algo-open-orders-spot-algo) - Query Current Spot Algo Open Orders (USER_DATA)

```bash
binance-cli algo query-current-algo-open-orders-spot-algo --recv-window 5000
```

### [GET /sapi/v1/algo/spot/historicalOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/spot-algo#query-historical-algo-orders-spot-algo) - Query Historical Spot Algo Orders (USER_DATA)

```bash
binance-cli algo query-historical-algo-orders-spot-algo --symbol BTCUSDT --side BUY --start-time 1623319461670 --end-time 1641782889000 --page 1 --page-size 10 --recv-window 5000
```

### [GET /sapi/v1/algo/spot/subOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/spot-algo#query-sub-orders-spot-algo) - Query Spot Sub Orders (USER_DATA)

```bash
binance-cli algo query-sub-orders-spot-algo --algo-id 1 --page 1 --page-size 10 --recv-window 5000
```

### [POST /sapi/v1/algo/spot/newOrderTwap](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api/spot-algo#time-weighted-average-price-spot-algo) - Time-Weighted Spot Average Price(Twap) New Order (TRADE)

```bash
binance-cli algo time-weighted-average-price-spot-algo --symbol BTCUSDT --side BUY --quantity 1 --duration 5000 --client-algo-id 1 --limit-price 1
```
