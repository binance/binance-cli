## FutureAlgo

### [DELETE /sapi/v1/algo/futures/order](https://developers.binance.com/docs/algo/future-algo/Cancel-Algo-Order) - Cancel Algo Order

```bash
binance-cli algo cancel-algo-order-future-algo --algo-id 1 --recv-window 5000
```

### [GET /sapi/v1/algo/futures/openOrders](https://developers.binance.com/docs/algo/future-algo/Query-Current-Algo-Open-Orders) - Query Current Algo Open Orders

```bash
binance-cli algo query-current-algo-open-orders-future-algo --recv-window 5000
```

### [GET /sapi/v1/algo/futures/historicalOrders](https://developers.binance.com/docs/algo/future-algo/Query-Historical-Algo-Orders) - Query Historical Algo Orders

```bash
binance-cli algo query-historical-algo-orders-future-algo --symbol "BTCUSDT" --side "BUY" --start-time 1623319461670 --end-time 1641782889000 --page 1 --page-size 100 --recv-window 5000
```

### [GET /sapi/v1/algo/futures/subOrders](https://developers.binance.com/docs/algo/future-algo/Query-Sub-Orders) - Query Sub Orders

```bash
binance-cli algo query-sub-orders-future-algo --algo-id 1 --page 1 --page-size 100 --recv-window 5000
```

### [POST /sapi/v1/algo/futures/newOrderTwap](https://developers.binance.com/docs/algo/future-algo/Time-Weighted-Average-Price-New-Order) - Time-Weighted Average Price(Twap) New Order

```bash
binance-cli algo time-weighted-average-price-future-algo --json {}
```

### [POST /sapi/v1/algo/futures/newOrderVp](https://developers.binance.com/docs/algo/future-algo/Volume-Participation-New-Order) - Volume ParticipationNew Order

```bash
binance-cli algo volume-participation-future-algo --json {}
```

## SpotAlgo

### [DELETE /sapi/v1/algo/spot/order](https://developers.binance.com/docs/algo/spot-algo/Cancel-Algo-Order) - Cancel Algo Order

```bash
binance-cli algo cancel-algo-order-spot-algo --algo-id 1 --recv-window 5000
```

### [GET /sapi/v1/algo/spot/openOrders](https://developers.binance.com/docs/algo/spot-algo/Query-Current-Algo-Open-Orders) - Query Current Algo Open Orders

```bash
binance-cli algo query-current-algo-open-orders-spot-algo --recv-window 5000
```

### [GET /sapi/v1/algo/spot/historicalOrders](https://developers.binance.com/docs/algo/spot-algo/Query-Historical-Algo-Orders) - Query Historical Algo Orders

```bash
binance-cli algo query-historical-algo-orders-spot-algo --symbol "BTCUSDT" --side "BUY" --start-time 1623319461670 --end-time 1641782889000 --page 1 --page-size 100 --recv-window 5000
```

### [GET /sapi/v1/algo/spot/subOrders](https://developers.binance.com/docs/algo/spot-algo/Query-Sub-Orders) - Query Sub Orders

```bash
binance-cli algo query-sub-orders-spot-algo --algo-id 1 --page 1 --page-size 100 --recv-window 5000
```

### [POST /sapi/v1/algo/spot/newOrderTwap](https://developers.binance.com/docs/algo/spot-algo/Time-Weighted-Average-Price-New-Order) - Time-Weighted Average Price(Twap) New Order

```bash
binance-cli algo time-weighted-average-price-spot-algo --json {}
```
