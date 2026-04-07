# Spot

## Account

### [GET /api/v3/account/commission](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-commission-rates-user_data) - Query Commission Rates

```bash
binance-cli spot account-commission --symbol "BNBUSDT"
```

### [GET /api/v3/allOrderList](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-all-order-lists-user_data) - Query all Order lists

```bash
binance-cli spot all-order-list --from-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 500 --recv-window 5000.0
```

### [GET /api/v3/allOrders](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#all-orders-user_data) - All orders

```bash
binance-cli spot all-orders --symbol "BNBUSDT" --order-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 500 --recv-window 5000.0
```

### [GET /api/v3/account](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#account-information-user_data) - Account information

```bash
binance-cli spot get-account --omit-zero-balances false --recv-window 5000.0
```

### [GET /api/v3/openOrders](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#current-open-orders-user_data) - Current open orders

```bash
binance-cli spot get-open-orders --symbol "BNBUSDT" --recv-window 5000.0
```

### [GET /api/v3/order](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-order-user_data) - Query order

```bash
binance-cli spot get-order --symbol "BNBUSDT" --order-id 1 --orig-client-order-id "origClientOrderId_example" --recv-window 5000.0
```

### [GET /api/v3/orderList](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-order-list-user_data) - Query Order list

```bash
binance-cli spot get-order-list --order-list-id 1 --orig-client-order-id "origClientOrderId_example" --recv-window 5000.0
```

### [GET /api/v3/myAllocations](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-allocations-user_data) - Query Allocations

```bash
binance-cli spot my-allocations --symbol "BNBUSDT" --start-time 1735693200000 --end-time 1735693200000 --from-allocation-id 1 --limit 500 --order-id 1 --recv-window 5000.0
```

### [GET /api/v3/myFilters](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-relevant-filters-user_data) - Query relevant filters

```bash
binance-cli spot my-filters --symbol "BNBUSDT" --recv-window 5000.0
```

### [GET /api/v3/myPreventedMatches](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-prevented-matches-user_data) - Query Prevented Matches

```bash
binance-cli spot my-prevented-matches --symbol "BNBUSDT" --prevented-match-id 1 --order-id 1 --from-prevented-match-id 1 --limit 500 --recv-window 5000.0
```

### [GET /api/v3/myTrades](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#account-trade-list-user_data) - Account trade list

```bash
binance-cli spot my-trades --symbol "BNBUSDT" --order-id 1 --start-time 1735693200000 --end-time 1735693200000 --from-id 1 --limit 500 --recv-window 5000.0
```

### [GET /api/v3/openOrderList](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-open-order-lists-user_data) - Query Open Order lists

```bash
binance-cli spot open-order-list --recv-window 5000.0
```

### [GET /api/v3/order/amendments](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-order-amendments-user_data) - Query Order Amendments

```bash
binance-cli spot order-amendments --symbol "BNBUSDT" --order-id 1 --from-execution-id 1 --limit 500 --recv-window 5000.0
```

### [GET /api/v3/rateLimit/order](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-unfilled-order-count-user_data) - Query Unfilled Order Count

```bash
binance-cli spot rate-limit-order --recv-window 5000.0
```

## General

### [GET /api/v3/exchangeInfo](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-endpoints#exchange-information) - Exchange information

```bash
binance-cli spot exchange-info --symbol "BNBUSDT" --symbols "string_example"  --permissions "string_example"  --show-permission-sets true --symbol-status TRADING
```

### [GET /api/v3/executionRules](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-endpoints#query-execution-rules) - Query Execution Rules

```bash
binance-cli spot execution-rules --symbol "BNBUSDT" --symbols "string_example"  --symbol-status TRADING
```

### [GET /api/v3/ping](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-endpoints#test-connectivity) - Test connectivity

```bash
binance-cli spot ping
```

### [GET /api/v3/time](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-endpoints#check-server-time) - Check server time

```bash
binance-cli spot time
```

## Market

### [GET /api/v3/aggTrades](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#compressedaggregate-trades-list) - Compressed/Aggregate trades list

```bash
binance-cli spot agg-trades --symbol "BNBUSDT" --from-id 1 --start-time 1735693200000 --end-time 1735693200000 --limit 500
```

### [GET /api/v3/avgPrice](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#current-average-price) - Current average price

```bash
binance-cli spot avg-price --symbol "BNBUSDT"
```

### [GET /api/v3/depth](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#order-book) - Order book

```bash
binance-cli spot depth --symbol "BNBUSDT" --limit 500 --symbol-status TRADING
```

### [GET /api/v3/trades](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#recent-trades-list) - Recent trades list

```bash
binance-cli spot get-trades --symbol "BNBUSDT" --limit 500
```

### [GET /api/v3/historicalTrades](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#old-trade-lookup) - Old trade lookup

```bash
binance-cli spot historical-trades --symbol "BNBUSDT" --limit 500 --from-id 1
```

### [GET /api/v3/klines](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#klinecandlestick-data) - Kline/Candlestick data

```bash
binance-cli spot klines --symbol "BNBUSDT" --interval 1s --start-time 1735693200000 --end-time 1735693200000 --time-zone "timeZone_example" --limit 500
```

### [GET /api/v3/referencePrice](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#query-reference-price) - Query Reference Price

```bash
binance-cli spot reference-price --symbol "BNBUSDT"
```

### [GET /api/v3/referencePrice/calculation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#query-reference-price-calculation) - Query Reference Price Calculation

```bash
binance-cli spot reference-price-calculation --symbol "BNBUSDT" --symbol-status TRADING
```

### [GET /api/v3/ticker](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#rolling-window-price-change-statistics) - Rolling window price change statistics

```bash
binance-cli spot ticker --symbol "BNBUSDT" --symbols "string_example"  --window-size 1m --type FULL --symbol-status TRADING
```

### [GET /api/v3/ticker/24hr](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#24hr-ticker-price-change-statistics) - 24hr ticker price change statistics

```bash
binance-cli spot ticker24hr --symbol "BNBUSDT" --symbols "string_example"  --type FULL --symbol-status TRADING
```

### [GET /api/v3/ticker/bookTicker](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#symbol-order-book-ticker) - Symbol order book ticker

```bash
binance-cli spot ticker-book-ticker --symbol "BNBUSDT" --symbols "string_example"  --symbol-status TRADING
```

### [GET /api/v3/ticker/price](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#symbol-price-ticker) - Symbol price ticker

```bash
binance-cli spot ticker-price --symbol "BNBUSDT" --symbols "string_example"  --symbol-status TRADING
```

### [GET /api/v3/ticker/tradingDay](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#trading-day-ticker) - Trading Day Ticker

```bash
binance-cli spot ticker-trading-day --symbol "BNBUSDT" --symbols "string_example"  --time-zone "timeZone_example" --type FULL --symbol-status TRADING
```

### [GET /api/v3/uiKlines](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints#uiklines) - UIKlines

```bash
binance-cli spot ui-klines --symbol "BNBUSDT" --interval 1s --start-time 1735693200000 --end-time 1735693200000 --time-zone "timeZone_example" --limit 500
```

## Trade

### [DELETE /api/v3/openOrders](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#cancel-all-open-orders-on-a-symbol-trade) - Cancel All Open Orders on a Symbol

```bash
binance-cli spot delete-open-orders --symbol "BNBUSDT" --recv-window 5000.0
```

### [DELETE /api/v3/order](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#cancel-order-trade) - Cancel order

```bash
binance-cli spot delete-order --symbol "BNBUSDT" --order-id 1 --orig-client-order-id "origClientOrderId_example" --new-client-order-id "newClientOrderId_example" --cancel-restrictions ONLY_NEW --recv-window 5000.0
```

### [DELETE /api/v3/orderList](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#cancel-order-list-trade) - Cancel Order list

```bash
binance-cli spot delete-order-list --symbol "BNBUSDT" --order-list-id 1 --list-client-order-id "listClientOrderId_example" --new-client-order-id "newClientOrderId_example" --recv-window 5000.0
```

### [POST /api/v3/order](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-trade) - New order

```bash
binance-cli spot new-order --json {}
```

### [PUT /api/v3/order/amend/keepPriority](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#order-amend-keep-priority-trade) - Order Amend Keep Priority

```bash
binance-cli spot order-amend-keep-priority --json {}
```

### [POST /api/v3/order/cancelReplace](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#cancel-an-existing-order-and-send-a-new-order-trade) - Cancel an Existing Order and Send a New Order

```bash
binance-cli spot order-cancel-replace --json {}
```

### [POST /api/v3/orderList/oco](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-list---oco-trade) - New Order list - OCO

```bash
binance-cli spot order-list-oco --json {}
```

### [POST /api/v3/orderList/opo](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-list---opo-trade) - New Order List - OPO

```bash
binance-cli spot order-list-opo --json {}
```

### [POST /api/v3/orderList/opoco](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-list---opoco-trade) - New Order List - OPOCO

```bash
binance-cli spot order-list-opoco --json {}
```

### [POST /api/v3/orderList/oto](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-list---oto-trade) - New Order list - OTO

```bash
binance-cli spot order-list-oto --json {}
```

### [POST /api/v3/orderList/otoco](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-list---otoco-trade) - New Order list - OTOCO

```bash
binance-cli spot order-list-otoco --json {}
```

### [POST /api/v3/order/oco](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-oco---deprecated-trade) - New OCO - Deprecated

```bash
binance-cli spot order-oco --json {}
```

### [POST /api/v3/order/test](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#test-new-order-trade) - Test new order

```bash
binance-cli spot order-test --json {}
```

### [POST /api/v3/sor/order](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#new-order-using-sor-trade) - New order using SOR

```bash
binance-cli spot sor-order --json {}
```

### [POST /api/v3/sor/order/test](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/trading-endpoints#test-new-order-using-sor-trade) - Test new order using SOR

```bash
binance-cli spot sor-order-test --json {}
```
