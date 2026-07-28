## Account

### [GET /api/v3/account/commission](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#account-commission) - Query Commission Rates (USER_DATA)

```bash
binance-cli spot account-commission --symbol BTCUSDT
```

### [GET /api/v3/allOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#all-order-list) - Query all Order lists (USER_DATA)

```bash
binance-cli spot all-order-list --from-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 1 --recv-window 5000
```

### [GET /api/v3/allOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#all-orders) - All orders (USER_DATA)

```bash
binance-cli spot all-orders --symbol LTCBTC --order-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 1 --recv-window 5000
```

### [GET /api/v3/account](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-account) - Account information (USER_DATA)

```bash
binance-cli spot get-account --omit-zero-balances false --recv-window 5000
```

### [GET /api/v3/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-open-orders) - Current open orders (USER_DATA)

```bash
binance-cli spot get-open-orders --symbol LTCBTC --recv-window 5000
```

### [GET /api/v3/order](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-order) - Query order (USER_DATA)

```bash
binance-cli spot get-order --symbol LTCBTC --order-id 1 --orig-client-order-id myOrder1 --recv-window 5000
```

### [GET /api/v3/orderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-order-list) - Query Order list (USER_DATA)

```bash
binance-cli spot get-order-list --order-list-id 27 --orig-client-order-id 1 --recv-window 5000
```

### [GET /api/v3/myAllocations](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-allocations) - Query Allocations (USER_DATA)

```bash
binance-cli spot my-allocations --symbol BTCUSDT --start-time 1735693200000 --end-time 1735693200000 --from-allocation-id 0 --limit 1 --order-id 1 --recv-window 5000
```

### [GET /api/v3/myFilters](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-filters) - Query relevant filters (USER_DATA)

```bash
binance-cli spot my-filters --symbol BNBUSDT --recv-window 5000
```

### [GET /api/v3/myPreventedMatches](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-prevented-matches) - Query Prevented Matches (USER_DATA)

```bash
binance-cli spot my-prevented-matches --symbol BTCUSDT --prevented-match-id 1 --order-id 1 --from-prevented-match-id 1 --limit 1 --recv-window 5000
```

### [GET /api/v3/myTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-trades) - Account trade list (USER_DATA)

```bash
binance-cli spot my-trades --symbol BNBBTC --order-id 100234 --start-time 1735693200000 --end-time 1735693200000 --from-id 1 --limit 1 --recv-window 5000
```

### [GET /api/v3/openOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#open-order-list) - Query Open Order lists (USER_DATA)

```bash
binance-cli spot open-order-list --recv-window 5000
```

### [GET /api/v3/order/amendments](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#order-amendments) - Query Order Amendments (USER_DATA)

```bash
binance-cli spot order-amendments --symbol BTCUSDT --order-id 9 --from-execution-id 22 --limit 1 --recv-window 5000
```

### [GET /api/v3/rateLimit/order](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#rate-limit-order) - Query Unfilled Order Count (USER_DATA)

```bash
binance-cli spot rate-limit-order --recv-window 5000
```

## General

### [GET /api/v3/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general#exchange-info) - Exchange information

```bash
binance-cli spot exchange-info --symbol ETHBTC --symbols BTCUSDT  --permissions [&quot;SPOT&quot;] --show-permission-sets false --symbol-status TRADING
```

### [GET /api/v3/executionRules](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general#execution-rules) - Query Execution Rules

```bash
binance-cli spot execution-rules --symbol BAZUSD --symbols BAZUSD  --symbol-status TRADING
```

### [GET /api/v3/ping](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general#ping) - Test connectivity

```bash
binance-cli spot ping
```

### [GET /api/v3/time](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general#time) - Check server time

```bash
binance-cli spot time
```

## Market

### [GET /api/v3/aggTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#agg-trades) - Compressed/Aggregate trades list

```bash
binance-cli spot agg-trades --symbol BNBUSDT --from-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 1
```

### [GET /api/v3/avgPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#avg-price) - Current average price

```bash
binance-cli spot avg-price --symbol BNBUSDT
```

### [GET /api/v3/depth](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#depth) - Order book

```bash
binance-cli spot depth --symbol BNBUSDT --limit 1 --symbol-status TRADING
```

### [GET /api/v3/trades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#get-trades) - Recent trades list

```bash
binance-cli spot get-trades --symbol BNBUSDT --limit 1
```

### [GET /api/v3/historicalBlockTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#historical-block-trades) - Historical Block Trades (MARKET_DATA)

```bash
binance-cli spot historical-block-trades --symbol BNBBTC --from-id 582 --limit 500
```

### [GET /api/v3/historicalTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#historical-trades) - Old trade lookup

```bash
binance-cli spot historical-trades --symbol BNBUSDT --limit 1 --from-id 1
```

### [GET /api/v3/klines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#klines) - Kline/Candlestick data

```bash
binance-cli spot klines --symbol BNBUSDT --interval INTERVAL_1s --start-time 1735693200000 --end-time 1735693200000 --time-zone 0 --limit 1
```

### [GET /api/v3/referencePrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#reference-price) - Query Reference Price

```bash
binance-cli spot reference-price --symbol BNBUSDT
```

### [GET /api/v3/referencePrice/calculation](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#reference-price-calculation) - Query Reference Price Calculation

```bash
binance-cli spot reference-price-calculation --symbol BNBUSDT --symbol-status TRADING
```

### [GET /api/v3/ticker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ticker) - Rolling window price change statistics

```bash
binance-cli spot ticker --symbol BNBUSDT --symbols BTCUSDT  --window-size WINDOW_SIZE_1m --rtype FULL --symbol-status TRADING
```

### [GET /api/v3/ticker/24hr](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ticker24hr) - 24hr ticker price change statistics

```bash
binance-cli spot ticker24hr --symbol BNBUSDT --symbols BTCUSDT  --rtype FULL --symbol-status TRADING
```

### [GET /api/v3/ticker/bookTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ticker-book-ticker) - Symbol order book ticker

```bash
binance-cli spot ticker-book-ticker --symbol BNBUSDT --symbols BTCUSDT  --symbol-status TRADING
```

### [GET /api/v3/ticker/price](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ticker-price) - Symbol price ticker

```bash
binance-cli spot ticker-price --symbol BNBUSDT --symbols BTCUSDT  --symbol-status TRADING
```

### [GET /api/v3/ticker/tradingDay](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ticker-trading-day) - Trading Day Ticker

```bash
binance-cli spot ticker-trading-day --symbol BNBUSDT --symbols BTCUSDT  --time-zone 0 --rtype FULL --symbol-status TRADING
```

### [GET /api/v3/uiKlines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market#ui-klines) - UIKlines

```bash
binance-cli spot ui-klines --symbol BNBUSDT --interval INTERVAL_1s --start-time 1735693200000 --end-time 1735693200000 --time-zone 0 --limit 1
```

## Trade

### [DELETE /api/v3/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#delete-open-orders) - Cancel All Open Orders on a Symbol (TRADE)

```bash
binance-cli spot delete-open-orders --symbol BNBUSDT --recv-window 5000
```

### [DELETE /api/v3/order](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#delete-order) - Cancel order (TRADE)

```bash
binance-cli spot delete-order --symbol BNBUSDT --order-id 1 --orig-client-order-id myOrder1 --new-client-order-id cancelMyOrder1 --cancel-restrictions ONLY_NEW --recv-window 5000
```

### [DELETE /api/v3/orderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#delete-order-list) - Cancel Order list (TRADE)

```bash
binance-cli spot delete-order-list --symbol BNBUSDT --order-list-id 1 --list-client-order-id C3wyj4WVEktd7u9aVBRXcN --new-client-order-id cancelMyOrder1 --recv-window 5000
```

### [POST /api/v3/order](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#new-order) - New order (TRADE)

```bash
binance-cli spot new-order --symbol BNBUSDT --side BUY --rtype MARKET --time-in-force GTC --quantity 1 --quote-order-qty 1 --price 400 --new-client-order-id myOrder1 --strategy-id 1 --strategy-type 1 --stop-price 1 --trailing-delta 1 --iceberg-qty 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --peg-price-type PRIMARY_PEG --peg-offset-value 1 --peg-offset-type PRICE_LEVEL --recv-window 5000
```

### [PUT /api/v3/order/amend/keepPriority](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-amend-keep-priority) - Order Amend Keep Priority (TRADE)

```bash
binance-cli spot order-amend-keep-priority --symbol BNBUSDT --new-qty 1 --order-id 1 --orig-client-order-id myOrder1 --new-client-order-id myOrder2 --recv-window 5000
```

### [POST /api/v3/order/cancelReplace](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-cancel-replace) - Cancel an Existing Order and Send a New Order (TRADE)

```bash
binance-cli spot order-cancel-replace --symbol BNBUSDT --side BUY --rtype MARKET --cancel-replace-mode STOP_ON_FAILURE --time-in-force GTC --quantity 1 --quote-order-qty 1 --price 400 --cancel-new-client-order-id cancelMyOrder1 --cancel-orig-client-order-id myOrder1 --cancel-order-id 1 --new-client-order-id myOrder2 --strategy-id 1 --strategy-type 1 --stop-price 1 --trailing-delta 1 --iceberg-qty 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --cancel-restrictions ONLY_NEW --order-rate-limit-exceeded-mode DO_NOTHING --peg-price-type PRIMARY_PEG --peg-offset-value 1 --peg-offset-type PRICE_LEVEL --recv-window 5000
```

### [POST /api/v3/orderList/oco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oco) - New Order list - OCO (TRADE)

```bash
binance-cli spot order-list-oco --symbol BNBUSDT --side BUY --quantity 1 --above-type STOP_LOSS_LIMIT --below-type STOP_LOSS --list-client-order-id lH1YDkuQKWiXVXHPSKYEIp --above-client-order-id aboveOrder1 --above-iceberg-qty 1 --above-price 1 --above-stop-price 1 --above-trailing-delta 1 --above-time-in-force GTC --above-strategy-id 1 --above-strategy-type 1 --above-peg-price-type PRIMARY_PEG --above-peg-offset-type PRICE_LEVEL --above-peg-offset-value 1 --below-client-order-id belowOrder1 --below-iceberg-qty 1 --below-price 1 --below-stop-price 1 --below-trailing-delta 1 --below-time-in-force GTC --below-strategy-id 1 --below-strategy-type 1 --below-peg-price-type PRIMARY_PEG --below-peg-offset-type PRICE_LEVEL --below-peg-offset-value 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --recv-window 5000
```

### [POST /api/v3/orderList/opo](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opo) - New Order List - OPO (TRADE)

```bash
binance-cli spot order-list-opo --symbol BNBUSDT --working-type LIMIT --working-side BUY --working-price 1 --working-quantity 1 --pending-type LIMIT --pending-side BUY --list-client-order-id H94qCqO27P74OEiO4X8HOG --new-order-resp-type ACK --self-trade-prevention-mode NONE --working-client-order-id workingOrder1 --working-iceberg-qty 1 --working-time-in-force GTC --working-strategy-id 1 --working-strategy-type 1 --working-peg-price-type PRIMARY_PEG --working-peg-offset-type PRICE_LEVEL --working-peg-offset-value 1 --pending-client-order-id pendingOrder1 --pending-price 1 --pending-stop-price 1 --pending-trailing-delta 1 --pending-iceberg-qty 1 --pending-time-in-force GTC --pending-strategy-id 1 --pending-strategy-type 1 --pending-peg-price-type PRIMARY_PEG --pending-peg-offset-type PRICE_LEVEL --pending-peg-offset-value 1 --recv-window 5000
```

### [POST /api/v3/orderList/opoco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opoco) - New Order List - OPOCO (TRADE)

```bash
binance-cli spot order-list-opoco --symbol BNBUSDT --working-type LIMIT --working-side BUY --working-price 1 --working-quantity 1 --pending-side BUY --pending-above-type STOP_LOSS_LIMIT --list-client-order-id bcedxMpQG6nFrZUPQyshoL --new-order-resp-type ACK --self-trade-prevention-mode NONE --working-client-order-id workingOrder1 --working-iceberg-qty 1 --working-time-in-force GTC --working-strategy-id 1 --working-strategy-type 1 --working-peg-price-type PRIMARY_PEG --working-peg-offset-type PRICE_LEVEL --working-peg-offset-value 1 --pending-above-client-order-id pendingAboveOrder1 --pending-above-price 1 --pending-above-stop-price 1 --pending-above-trailing-delta 1 --pending-above-iceberg-qty 1 --pending-above-time-in-force GTC --pending-above-strategy-id 1 --pending-above-strategy-type 1 --pending-above-peg-price-type PRIMARY_PEG --pending-above-peg-offset-type PRICE_LEVEL --pending-above-peg-offset-value 1 --pending-below-type STOP_LOSS --pending-below-client-order-id pendingBelowOrder1 --pending-below-price 1 --pending-below-stop-price 1 --pending-below-trailing-delta 1 --pending-below-iceberg-qty 1 --pending-below-time-in-force GTC --pending-below-strategy-id 1 --pending-below-strategy-type 1 --pending-below-peg-price-type PRIMARY_PEG --pending-below-peg-offset-type PRICE_LEVEL --pending-below-peg-offset-value 1 --recv-window 5000
```

### [POST /api/v3/orderList/oto](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oto) - New Order list - OTO (TRADE)

```bash
binance-cli spot order-list-oto --symbol BNBUSDT --working-type LIMIT --working-side BUY --working-price 1 --working-quantity 1 --pending-type LIMIT --pending-side BUY --pending-quantity 1 --list-client-order-id yl2ERtcar1o25zcWtqVBTC --new-order-resp-type ACK --self-trade-prevention-mode NONE --working-client-order-id workingOrder1 --working-iceberg-qty 1 --working-time-in-force GTC --working-strategy-id 1 --working-strategy-type 1 --working-peg-price-type PRIMARY_PEG --working-peg-offset-type PRICE_LEVEL --working-peg-offset-value 1 --pending-client-order-id pendingOrder1 --pending-price 1 --pending-stop-price 1 --pending-trailing-delta 1 --pending-iceberg-qty 1 --pending-time-in-force GTC --pending-strategy-id 1 --pending-strategy-type 1 --pending-peg-price-type PRIMARY_PEG --pending-peg-offset-type PRICE_LEVEL --pending-peg-offset-value 1 --recv-window 5000
```

### [POST /api/v3/orderList/otoco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-otoco) - New Order list - OTOCO (TRADE)

```bash
binance-cli spot order-list-otoco --symbol BNBUSDT --working-type LIMIT --working-side BUY --working-price 1 --working-quantity 1 --pending-side BUY --pending-quantity 1 --pending-above-type STOP_LOSS_LIMIT --list-client-order-id RumwQpBaDctlUu5jyG5rs0 --new-order-resp-type ACK --self-trade-prevention-mode NONE --working-client-order-id workingOrder1 --working-iceberg-qty 1 --working-time-in-force GTC --working-strategy-id 1 --working-strategy-type 1 --working-peg-price-type PRIMARY_PEG --working-peg-offset-type PRICE_LEVEL --working-peg-offset-value 1 --pending-above-client-order-id pendingAboveOrder1 --pending-above-price 1 --pending-above-stop-price 1 --pending-above-trailing-delta 1 --pending-above-iceberg-qty 1 --pending-above-time-in-force GTC --pending-above-strategy-id 1 --pending-above-strategy-type 1 --pending-above-peg-price-type PRIMARY_PEG --pending-above-peg-offset-type PRICE_LEVEL --pending-above-peg-offset-value 1 --pending-below-type STOP_LOSS --pending-below-client-order-id pendingBelowOrder1 --pending-below-price 1 --pending-below-stop-price 1 --pending-below-trailing-delta 1 --pending-below-iceberg-qty 1 --pending-below-time-in-force GTC --pending-below-strategy-id 1 --pending-below-strategy-type 1 --pending-below-peg-price-type PRIMARY_PEG --pending-below-peg-offset-type PRICE_LEVEL --pending-below-peg-offset-value 1 --recv-window 5000
```

### [POST /api/v3/order/oco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-oco) - New OCO - Deprecated (TRADE)

```bash
binance-cli spot order-oco --symbol BNBUSDT --side BUY --quantity 1 --price 1 --stop-price 1 --list-client-order-id JYVpp3F0f5CAG15DhtrqLp --limit-client-order-id limitOrder1 --limit-strategy-id 1 --limit-strategy-type 1 --limit-iceberg-qty 1 --trailing-delta 1 --stop-client-order-id stopOrder1 --stop-strategy-id 1 --stop-strategy-type 1 --stop-limit-price 1 --stop-iceberg-qty 1 --stop-limit-time-in-force GTC --new-order-resp-type ACK --self-trade-prevention-mode NONE --recv-window 5000
```

### [POST /api/v3/order/test](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-test) - Test new order (TRADE)

```bash
binance-cli spot order-test --symbol BNBUSDT --side BUY --rtype MARKET --compute-commission-rates false --time-in-force GTC --quantity 1 --quote-order-qty 1 --price 400 --new-client-order-id myOrder1 --strategy-id 1 --strategy-type 1 --stop-price 1 --trailing-delta 1 --iceberg-qty 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --peg-price-type PRIMARY_PEG --peg-offset-value 1 --peg-offset-type PRICE_LEVEL --recv-window 5000
```

### [POST /api/v3/sor/order](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order) - New order using SOR (TRADE)

```bash
binance-cli spot sor-order --symbol BNBUSDT --side BUY --rtype MARKET --quantity 1 --time-in-force GTC --price 400 --new-client-order-id myOrder1 --strategy-id 1 --strategy-type 1 --iceberg-qty 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --recv-window 5000
```

### [POST /api/v3/sor/order/test](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order-test) - Test new order using SOR (TRADE)

```bash
binance-cli spot sor-order-test --symbol BNBUSDT --side BUY --rtype MARKET --quantity 1 --compute-commission-rates false --time-in-force GTC --price 400 --new-client-order-id myOrder1 --strategy-id 1 --strategy-type 1 --iceberg-qty 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --recv-window 5000
```
