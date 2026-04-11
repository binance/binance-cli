## Account

### [GET /eapi/v1/bill](https://developers.binance.com/docs/derivatives/options-trading/account/Account-Funding-Flow) - Account Funding Flow

```bash
binance-cli derivatives-options account-funding-flow --currency "currency_example" --record-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /eapi/v1/marginAccount](https://developers.binance.com/docs/derivatives/options-trading/account/Option-Margin-Account-Information) - Option Margin Account Information

```bash
binance-cli derivatives-options option-margin-account-information --recv-window 5000
```

## MarketData

### [GET /eapi/v1/time](https://developers.binance.com/docs/derivatives/options-trading/market-data/Check-Server-Time) - Check Server Time

```bash
binance-cli derivatives-options check-server-time
```

### [GET /eapi/v1/exchangeInfo](https://developers.binance.com/docs/derivatives/options-trading/market-data/Exchange-Information) - Exchange Information

```bash
binance-cli derivatives-options exchange-information
```

### [GET /eapi/v1/exerciseHistory](https://developers.binance.com/docs/derivatives/options-trading/market-data/Historical-Exercise-Records) - Historical Exercise Records

```bash
binance-cli derivatives-options historical-exercise-records --underlying "underlying_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /eapi/v1/index](https://developers.binance.com/docs/derivatives/options-trading/market-data/Symbol-Price-Ticker) - Index Price

```bash
binance-cli derivatives-options index-price --underlying "underlying_example"
```

### [GET /eapi/v1/klines](https://developers.binance.com/docs/derivatives/options-trading/market-data/Kline-Candlestick-Data) - Kline/Candlestick Data

```bash
binance-cli derivatives-options kline-candlestick-data --symbol "symbol_example" --interval "interval_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /eapi/v1/openInterest](https://developers.binance.com/docs/derivatives/options-trading/market-data/Open-Interest) - Open Interest

```bash
binance-cli derivatives-options open-interest --underlying-asset "underlyingAsset_example" --expiration "expiration_example"
```

### [GET /eapi/v1/mark](https://developers.binance.com/docs/derivatives/options-trading/market-data/Option-Mark-Price) - Option Mark Price

```bash
binance-cli derivatives-options option-mark-price --symbol "symbol_example"
```

### [GET /eapi/v1/depth](https://developers.binance.com/docs/derivatives/options-trading/market-data/Order-Book) - Order Book

```bash
binance-cli derivatives-options order-book --symbol "symbol_example" --limit 100
```

### [GET /eapi/v1/blockTrades](https://developers.binance.com/docs/derivatives/options-trading/market-data/Recent-Block-Trade-List) - Recent Block Trades List

```bash
binance-cli derivatives-options recent-block-trades-list --symbol "symbol_example" --limit 100
```

### [GET /eapi/v1/trades](https://developers.binance.com/docs/derivatives/options-trading/market-data/Recent-Trades-List) - Recent Trades List

```bash
binance-cli derivatives-options recent-trades-list --symbol "symbol_example" --limit 100
```

### [GET /eapi/v1/ping](https://developers.binance.com/docs/derivatives/options-trading/market-data/Test-Connectivity) - Test Connectivity

```bash
binance-cli derivatives-options test-connectivity
```

### [GET /eapi/v1/ticker](https://developers.binance.com/docs/derivatives/options-trading/market-data/24hr-Ticker-Price-Change-Statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli derivatives-options ticker24hr-price-change-statistics --symbol "symbol_example"
```

## MarketMakerBlockTrade

### [POST /eapi/v1/block/order/execute](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Accept-Block-Trade-Order) - Accept Block Trade Order

```bash
binance-cli derivatives-options accept-block-trade-order --json {}
```

### [GET /eapi/v1/block/user-trades](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Account-Block-Trade-List) - Account Block Trade List

```bash
binance-cli derivatives-options account-block-trade-list --end-time 1641782889000 --start-time 1623319461670 --underlying "underlying_example" --recv-window 5000
```

### [DELETE /eapi/v1/block/order/create](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Cancel-Block-Trade-Order) - Cancel Block Trade Order

```bash
binance-cli derivatives-options cancel-block-trade-order --block-order-matching-key "blockOrderMatchingKey_example" --recv-window 5000
```

### [PUT /eapi/v1/block/order/create](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Extend-Block-Trade-Order) - Extend Block Trade Order

```bash
binance-cli derivatives-options extend-block-trade-order --json {}
```

### [POST /eapi/v1/block/order/create](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/New-Block-Trade-Order) - New Block Trade Order

```bash
binance-cli derivatives-options new-block-trade-order --json {}
```

### [GET /eapi/v1/block/order/execute](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Query-Block-Trade-Detail) - Query Block Trade Details

```bash
binance-cli derivatives-options query-block-trade-details --block-order-matching-key "blockOrderMatchingKey_example" --recv-window 5000
```

### [GET /eapi/v1/block/order/orders](https://developers.binance.com/docs/derivatives/options-trading/market-maker-block-trade/Query-Block-Trade-Order) - Query Block Trade Order

```bash
binance-cli derivatives-options query-block-trade-order --block-order-matching-key "blockOrderMatchingKey_example" --end-time 1641782889000 --start-time 1623319461670 --underlying "underlying_example" --recv-window 5000
```

## MarketMakerEndpoints

### [POST /eapi/v1/countdownCancelAllHeartBeat](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Auto-Cancel-All-Open-Orders-Heartbeat) - Auto-Cancel All Open Orders (Kill-Switch) Heartbeat

```bash
binance-cli derivatives-options auto-cancel-all-open-orders --json {}
```

### [GET /eapi/v1/countdownCancelAll](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Get-Auto-Cancel-All-Open-Orders-Config) - Get Auto-Cancel All Open Orders (Kill-Switch) Config

```bash
binance-cli derivatives-options get-auto-cancel-all-open-orders --underlying "underlying_example" --recv-window 5000
```

### [GET /eapi/v1/mmp](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Get-Market-Maker-Protection-Config) - Get Market Maker Protection Config

```bash
binance-cli derivatives-options get-market-maker-protection-config --underlying "underlying_example" --recv-window 5000
```

### [POST /eapi/v1/mmpReset](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Reset-Market-Maker-Protection-Config) - Reset Market Maker Protection Config

```bash
binance-cli derivatives-options reset-market-maker-protection-config --json {}
```

### [POST /eapi/v1/countdownCancelAll](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Set-Auto-Cancel-All-Open-Orders-Config) - Set Auto-Cancel All Open Orders (Kill-Switch) Config

```bash
binance-cli derivatives-options set-auto-cancel-all-open-orders --json {}
```

### [POST /eapi/v1/mmpSet](https://developers.binance.com/docs/derivatives/options-trading/market-maker-endpoints/Set-Market-Maker-Protection-Config) - Set Market Maker Protection Config

```bash
binance-cli derivatives-options set-market-maker-protection-config --json {}
```

## Trade

### [GET /eapi/v1/userTrades](https://developers.binance.com/docs/derivatives/options-trading/trade/Account-Trade-List) - Account Trade List

```bash
binance-cli derivatives-options account-trade-list --symbol "symbol_example" --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [DELETE /eapi/v1/allOpenOrdersByUnderlying](https://developers.binance.com/docs/derivatives/options-trading/trade/Cancel-All-Option-Orders-By-Underlying) - Cancel All Option Orders By Underlying

```bash
binance-cli derivatives-options cancel-all-option-orders-by-underlying --underlying "underlying_example" --recv-window 5000
```

### [DELETE /eapi/v1/allOpenOrders](https://developers.binance.com/docs/derivatives/options-trading/trade/Cancel-all-Option-orders-on-specific-symbol) - Cancel all Option orders on specific symbol

```bash
binance-cli derivatives-options cancel-all-option-orders-on-specific-symbol --symbol "symbol_example" --recv-window 5000
```

### [DELETE /eapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/options-trading/trade/Cancel-Multiple-Option-Orders) - Cancel Multiple Option Orders

```bash
binance-cli derivatives-options cancel-multiple-option-orders --symbol "symbol_example" --order-ids 4611875134427365000  --client-order-ids "my_id_1"  --recv-window 5000
```

### [DELETE /eapi/v1/order](https://developers.binance.com/docs/derivatives/options-trading/trade/Cancel-Option-Order) - Cancel Option Order

```bash
binance-cli derivatives-options cancel-option-order --symbol "symbol_example" --order-id 1 --client-order-id "1" --recv-window 5000
```

### [POST /eapi/v1/order](https://developers.binance.com/docs/derivatives/options-trading/trade/New-Order) - New Order

```bash
binance-cli derivatives-options new-order --json {}
```

### [GET /eapi/v1/position](https://developers.binance.com/docs/derivatives/options-trading/trade/Option-Position-Information) - Option Position Information

```bash
binance-cli derivatives-options option-position-information --symbol "symbol_example" --recv-window 5000
```

### [POST /eapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/options-trading/trade/Place-Multiple-Orders) - Place Multiple Orders

```bash
binance-cli derivatives-options place-multiple-orders --json {}
```

### [GET /eapi/v1/openOrders](https://developers.binance.com/docs/derivatives/options-trading/trade/Query-Current-Open-Option-Orders) - Query Current Open Option Orders

```bash
binance-cli derivatives-options query-current-open-option-orders --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /eapi/v1/historyOrders](https://developers.binance.com/docs/derivatives/options-trading/trade/Query-Option-Order-History) - Query Option Order History

```bash
binance-cli derivatives-options query-option-order-history --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /eapi/v1/order](https://developers.binance.com/docs/derivatives/options-trading/trade/Query-Single-Order) - Query Single Order

```bash
binance-cli derivatives-options query-single-order --symbol "symbol_example" --order-id 1 --client-order-id "1" --recv-window 5000
```

### [GET /eapi/v1/commission](https://developers.binance.com/docs/derivatives/options-trading/trade/User-Commission) - User Commission

```bash
binance-cli derivatives-options user-commission --recv-window 5000
```

### [GET /eapi/v1/exerciseRecord](https://developers.binance.com/docs/derivatives/options-trading/trade/User-Exercise-Record) - User Exercise Record

```bash
binance-cli derivatives-options user-exercise-record --symbol "symbol_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

## UserDataStreams

### [DELETE /eapi/v1/listenKey](https://developers.binance.com/docs/derivatives/options-trading/user-data-streams/Close-User-Data-Stream) - Close User Data Stream

```bash
binance-cli derivatives-options close-user-data-stream
```

### [PUT /eapi/v1/listenKey](https://developers.binance.com/docs/derivatives/options-trading/user-data-streams/Keepalive-User-Data-Stream) - Keepalive User Data Stream

```bash
binance-cli derivatives-options keepalive-user-data-stream
```

### [POST /eapi/v1/listenKey](https://developers.binance.com/docs/derivatives/options-trading/user-data-streams/Start-User-Data-Stream) - Start User Data Stream

```bash
binance-cli derivatives-options start-user-data-stream
```
