## Account

### [GET /eapi/v1/bill](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#account-funding-flow) - Account Funding Flow (USER_DATA)

```bash
binance-cli derivatives-options account-funding-flow --currency USDT --record-id 100000 --start-time 1623319461670 --end-time 1641782889000 --limit 20 --recv-window 5000
```

### [GET /eapi/v1/marginAccount](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#option-margin-account-information) - Option Margin Account Information (USER_DATA)

```bash
binance-cli derivatives-options option-margin-account-information --recv-window 5000
```

## MarketData

### [GET /eapi/v1/time](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#check-server-time) - Check Server Time

```bash
binance-cli derivatives-options check-server-time
```

### [GET /eapi/v1/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#exchange-information) - Exchange Information

```bash
binance-cli derivatives-options exchange-information
```

### [GET /eapi/v1/exerciseHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#historical-exercise-records) - Historical Exercise Records

```bash
binance-cli derivatives-options historical-exercise-records --underlying BTCUSDT --start-time 1623319461670 --end-time 1641782889000 --limit 20
```

### [GET /eapi/v1/index](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#index-price) - Index Price

```bash
binance-cli derivatives-options index-price --underlying BTCUSDT
```

### [GET /eapi/v1/klines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#kline-candlestick-data) - Kline/Candlestick Data

```bash
binance-cli derivatives-options kline-candlestick-data --symbol BTC-200730-9000-C --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 20
```

### [GET /eapi/v1/openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#open-interest) - Open Interest

```bash
binance-cli derivatives-options open-interest --underlying-asset ETH/BTC --expiration 221225
```

### [GET /eapi/v1/mark](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#option-mark-price) - Option Mark Price

```bash
binance-cli derivatives-options option-mark-price --symbol BTC-200730-9000-C
```

### [GET /eapi/v1/depth](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#order-book) - Order Book

```bash
binance-cli derivatives-options order-book --symbol BTC-200730-9000-C --limit 20
```

### [GET /eapi/v1/blockTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-block-trades-list) - Recent Block Trades List

```bash
binance-cli derivatives-options recent-block-trades-list --symbol BTC-200730-9000-C --limit 20
```

### [GET /eapi/v1/trades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-trades-list) - Recent Trades List

```bash
binance-cli derivatives-options recent-trades-list --symbol BTC-200730-9000-C --limit 20
```

### [GET /eapi/v1/ping](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#test-connectivity) - Test Connectivity

```bash
binance-cli derivatives-options test-connectivity
```

### [GET /eapi/v1/ticker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#ticker24hr-price-change-statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli derivatives-options ticker24hr-price-change-statistics --symbol BTC-200730-9000-C
```

## MarketMakerBlockTrade

### [POST /eapi/v1/block/order/execute](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#accept-block-trade-order) - Accept Block Trade Order (TRADE)

```bash
binance-cli derivatives-options accept-block-trade-order --block-order-matching-key 7d046e6e-a429-4335-ab9d-6a681febcde5 --recv-window 5000
```

### [GET /eapi/v1/block/user-trades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#account-block-trade-list) - Account Block Trade List (USER_DATA)

```bash
binance-cli derivatives-options account-block-trade-list --end-time 1641782889000 --start-time 1623319461670 --underlying BTCUSDT --recv-window 5000
```

### [DELETE /eapi/v1/block/order/create](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#cancel-block-trade-order) - Cancel Block Trade Order (TRADE)

```bash
binance-cli derivatives-options cancel-block-trade-order --block-order-matching-key 7d046e6e-a429-4335-ab9d-6a681febcde5 --recv-window 5000
```

### [PUT /eapi/v1/block/order/create](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#extend-block-trade-order) - Extend Block Trade Order (TRADE)

```bash
binance-cli derivatives-options extend-block-trade-order --block-order-matching-key 3668822b8-1baa-6a2f-adb8-d3de6289b361 --recv-window 5000
```

### [POST /eapi/v1/block/order/create](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#new-block-trade-order) - New Block Trade Order (TRADE)

```bash
binance-cli derivatives-options new-block-trade-order --liquidity TAKER --legs [] --recv-window 5000
```

### [GET /eapi/v1/block/order/execute](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-details) - Query Block Trade Details (USER_DATA)

```bash
binance-cli derivatives-options query-block-trade-details --block-order-matching-key 12b96c28-ba05-8906-c89t-703215cfb2e6 --recv-window 5000
```

### [GET /eapi/v1/block/order/orders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-order) - Query Block Trade Order (TRADE)

```bash
binance-cli derivatives-options query-block-trade-order --block-order-matching-key 7d046e6e-a429-4335-ab9d-6a681febcde5 --end-time 1641782889000 --start-time 1623319461670 --underlying BTCUSDT --recv-window 5000
```

## MarketMakerEndpoints

### [POST /eapi/v1/countdownCancelAllHeartBeat](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#auto-cancel-all-open-orders) - Auto-Cancel All Open Orders (Kill-Switch) Heartbeat (TRADE)

```bash
binance-cli derivatives-options auto-cancel-all-open-orders --underlyings BTCUSDT,ETHUSDT --recv-window 5000
```

### [GET /eapi/v1/countdownCancelAll](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-auto-cancel-all-open-orders) - Get Auto-Cancel All Open Orders (Kill-Switch) Config (TRADE)

```bash
binance-cli derivatives-options get-auto-cancel-all-open-orders --underlying BTCUSDT --recv-window 5000
```

### [GET /eapi/v1/mmp](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-market-maker-protection-config) - Get Market Maker Protection Config (TRADE)

```bash
binance-cli derivatives-options get-market-maker-protection-config --underlying BTCUSDT --recv-window 5000
```

### [POST /eapi/v1/mmpReset](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#reset-market-maker-protection-config) - Reset Market Maker Protection Config (TRADE)

```bash
binance-cli derivatives-options reset-market-maker-protection-config --underlying BTCUSDT --recv-window 5000
```

### [POST /eapi/v1/countdownCancelAll](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-auto-cancel-all-open-orders) - Set Auto-Cancel All Open Orders (Kill-Switch) Config (TRADE)

```bash
binance-cli derivatives-options set-auto-cancel-all-open-orders --underlying BTCUSDT --countdown-time 5000 --recv-window 5000
```

### [POST /eapi/v1/mmpSet](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-market-maker-protection-config) - Set Market Maker Protection Config (TRADE)

```bash
binance-cli derivatives-options set-market-maker-protection-config --underlying BTCUSDT --window-time-in-milliseconds 1000 --frozen-time-in-milliseconds 1000 --qty-limit 1.0 --delta-limit 1.0 --recv-window 5000
```

## Trade

### [GET /eapi/v1/userTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#account-trade-list) - Account Trade List (USER_DATA)

```bash
binance-cli derivatives-options account-trade-list --symbol BTC-200730-9000-C --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 20 --recv-window 5000
```

### [DELETE /eapi/v1/allOpenOrdersByUnderlying](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-by-underlying) - Cancel All Option Orders By Underlying (TRADE)

```bash
binance-cli derivatives-options cancel-all-option-orders-by-underlying --underlying BTCUSDT --recv-window 5000
```

### [DELETE /eapi/v1/allOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-on-specific-symbol) - Cancel all Option orders on specific symbol (TRADE)

```bash
binance-cli derivatives-options cancel-all-option-orders-on-specific-symbol --symbol BTC-200730-9000-C --recv-window 5000
```

### [DELETE /eapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-multiple-option-orders) - Cancel Multiple Option Orders (TRADE)

```bash
binance-cli derivatives-options cancel-multiple-option-orders --symbol BTC-200730-9000-C --order-ids 4611875134427365000  --client-order-ids my_id_1  --recv-window 5000
```

### [DELETE /eapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-option-order) - Cancel Option Order (TRADE)

```bash
binance-cli derivatives-options cancel-option-order --symbol BTC-200730-9000-C --order-id 4611875134427365000 --client-order-id 10000 --recv-window 5000
```

### [POST /eapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#new-order) - New Order (TRADE)

```bash
binance-cli derivatives-options new-order --symbol BTC-200730-9000-C --side BUY --rtype LIMIT --quantity 1.0 --price 1.0 --time-in-force GTC --reduce-only false --post-only false --new-order-resp-type ACK --client-order-id 1 --is-mmp true --self-trade-prevention-mode NONE --recv-window 5000
```

### [GET /eapi/v1/position](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#option-position-information) - Option Position Information (USER_DATA)

```bash
binance-cli derivatives-options option-position-information --symbol BTC-200730-9000-C --recv-window 5000
```

### [POST /eapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#place-multiple-orders) - Place Multiple Orders (TRADE)

```bash
binance-cli derivatives-options place-multiple-orders --orders [] --recv-window 5000
```

### [GET /eapi/v1/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-current-open-option-orders) - Query Current Open Option Orders (USER_DATA)

```bash
binance-cli derivatives-options query-current-open-option-orders --symbol BTC-200730-9000-C --order-id 4611875134427365000 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /eapi/v1/historyOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-option-order-history) - Query Option Order History (TRADE)

```bash
binance-cli derivatives-options query-option-order-history --symbol BTC-200730-9000-C --order-id 4611875134427365000 --start-time 1623319461670 --end-time 1641782889000 --limit 20 --recv-window 5000
```

### [GET /eapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-single-order) - Query Single Order (TRADE)

```bash
binance-cli derivatives-options query-single-order --symbol BTC-200730-9000-C --order-id 4611875134427365000 --client-order-id abc123 --recv-window 5000
```

### [POST /eapi/v1/stock/contract](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#tradfi-options-contract) - TradFi Options Contract (USER_DATA)

```bash
binance-cli derivatives-options tradfi-options-contract --recv-window 5000
```

### [GET /eapi/v1/commission](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-commission) - User Commission (USER_DATA)

```bash
binance-cli derivatives-options user-commission --recv-window 5000
```

### [GET /eapi/v1/exerciseRecord](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-exercise-record) - User Exercise Record (USER_DATA)

```bash
binance-cli derivatives-options user-exercise-record --symbol BTC-200730-9000-C --start-time 1623319461670 --end-time 1641782889000 --limit 20 --recv-window 5000
```

## UserDataStreams

### [DELETE /eapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#close-user-data-stream) - Close User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-options close-user-data-stream
```

### [PUT /eapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#keepalive-user-data-stream) - Keepalive User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-options keepalive-user-data-stream
```

### [POST /eapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#start-user-data-stream) - Start User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-options start-user-data-stream
```
