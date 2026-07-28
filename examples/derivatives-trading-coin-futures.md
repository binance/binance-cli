## Account

### [GET /dapi/v1/account](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#account-information) - Account Information (USER_DATA)

```bash
binance-cli futures-coin account-information --recv-window 5000
```

### [GET /dapi/v1/balance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#futures-account-balance) - Futures Account Balance (USER_DATA)

```bash
binance-cli futures-coin futures-account-balance --recv-window 5000
```

### [GET /dapi/v1/positionSide/dual](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-current-position-mode) - Get Current Position Mode (USER_DATA)

```bash
binance-cli futures-coin get-current-position-mode --recv-window 5000
```

### [GET /dapi/v1/order/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-order-history) - Get Download Id For Futures Order History (USER_DATA)

```bash
binance-cli futures-coin get-download-id-for-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/trade/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history) - Get Download Id For Futures Trade History (USER_DATA)

```bash
binance-cli futures-coin get-download-id-for-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/income/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history) - Get Download Id For Futures Transaction History (USER_DATA)

```bash
binance-cli futures-coin get-download-id-for-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/order/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id) - Get Futures Order History Download Link by Id (USER_DATA)

```bash
binance-cli futures-coin get-futures-order-history-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /dapi/v1/trade/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id) - Get Futures Trade Download Link by Id (USER_DATA)

```bash
binance-cli futures-coin get-futures-trade-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /dapi/v1/income/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id) - Get Futures Transaction History Download Link by Id (USER_DATA)

```bash
binance-cli futures-coin get-futures-transaction-history-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /dapi/v1/income](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-income-history) - Get Income History (USER_DATA)

```bash
binance-cli futures-coin get-income-history --symbol BTCUSDT --income-type TRANSFER --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 30 --recv-window 5000
```

### [GET /dapi/v1/leverageBracket](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-pair) - Notional Bracket for Pair (USER_DATA)

```bash
binance-cli futures-coin notional-bracket-for-pair --pair BTCUSD --recv-window 5000
```

### [GET /dapi/v2/leverageBracket](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-symbol) - Notional Bracket for Symbol (USER_DATA)

```bash
binance-cli futures-coin notional-bracket-for-symbol --symbol BTCUSD_PERP --recv-window 5000
```

### [GET /dapi/v1/commissionRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#user-commission-rate) - User Commission Rate (USER_DATA)

```bash
binance-cli futures-coin user-commission-rate --symbol BTCUSD_PERP --recv-window 5000
```

## MarketData

### [GET /futures/data/basis](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#basis) - Basis

```bash
binance-cli futures-coin basis --pair pair_example --contract-type PERPETUAL --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/time](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#check-server-time) - Check Server time

```bash
binance-cli futures-coin check-server-time
```

### [GET /dapi/v1/aggTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list) - Compressed/Aggregate Trades List

```bash
binance-cli futures-coin compressed-aggregate-trades-list --symbol symbol_example --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/continuousKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data) - Continuous Contract Kline/Candlestick Data

```bash
binance-cli futures-coin continuous-contract-kline-candlestick-data --pair BTCUSD --contract-type PERPETUAL --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#exchange-information) - Exchange Information

```bash
binance-cli futures-coin exchange-information
```

### [GET /dapi/v1/fundingRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-history-of-perpetual-futures) - Get Funding Rate History of Perpetual Futures

```bash
binance-cli futures-coin get-funding-rate-history-of-perpetual-futures --symbol symbol_example --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/fundingInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-info) - Get Funding Rate Info

```bash
binance-cli futures-coin get-funding-rate-info
```

### [GET /dapi/v1/premiumIndex](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-and-mark-price) - Index Price and Mark Price

```bash
binance-cli futures-coin index-price-and-mark-price --symbol BTCUSD_PERP --pair BTCUSD
```

### [GET /dapi/v1/indexPriceKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data) - Index Price Kline/Candlestick Data

```bash
binance-cli futures-coin index-price-kline-candlestick-data --pair BTCUSD --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/klines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#kline-candlestick-data) - Kline/Candlestick Data

```bash
binance-cli futures-coin kline-candlestick-data --symbol BTCUSD --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /futures/data/globalLongShortAccountRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#long-short-ratio) - Long/Short Ratio

```bash
binance-cli futures-coin long-short-ratio --pair pair_example --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/markPriceKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data) - Mark Price Kline/Candlestick Data

```bash
binance-cli futures-coin mark-price-kline-candlestick-data --symbol BTCUSD --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/historicalTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#old-trades-lookup) - Old Trades Lookup (MARKET_DATA)

```bash
binance-cli futures-coin old-trades-lookup --symbol symbol_example --limit 30 --from-id 595103
```

### [GET /dapi/v1/openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest) - Open Interest

```bash
binance-cli futures-coin open-interest --symbol BTCUSD_200626
```

### [GET /futures/data/openInterestHist](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest-statistics) - Open Interest Statistics

```bash
binance-cli futures-coin open-interest-statistics --pair BTCUSD --contract-type PERPETUAL --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/depth](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#order-book) - Order Book

```bash
binance-cli futures-coin order-book --symbol BTCUSD_PERP --limit 500
```

### [GET /dapi/v1/premiumIndexKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#premium-index-kline-data) - Premium index Kline Data

```bash
binance-cli futures-coin premium-index-kline-data --symbol BTCUSD --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 30
```

### [GET /dapi/v1/constituents](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#query-index-price-constituents) - Query Index Price Constituents

```bash
binance-cli futures-coin query-index-price-constituents --symbol BTCUSD
```

### [GET /dapi/v1/trades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#recent-trades-list) - Recent Trades List

```bash
binance-cli futures-coin recent-trades-list --symbol BTCUSD --limit 30
```

### [GET /dapi/v1/ticker/bookTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-order-book-ticker) - Symbol Order Book Ticker

```bash
binance-cli futures-coin symbol-order-book-ticker --symbol BTCUSD_200626 --pair BTCUSD
```

### [GET /dapi/v1/ticker/price](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-price-ticker) - Symbol Price Ticker

```bash
binance-cli futures-coin symbol-price-ticker --symbol BTCUSD_200626 --pair BTCUSD
```

### [GET /futures/data/takerBuySellVol](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#taker-buy-sell-volume) - Taker Buy/Sell Volume

```bash
binance-cli futures-coin taker-buy-sell-volume --pair BTCUSD --contract-type PERPETUAL --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/ping](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#test-connectivity) - Test Connectivity

```bash
binance-cli futures-coin test-connectivity
```

### [GET /dapi/v1/ticker/24hr](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli futures-coin ticker24hr-price-change-statistics --symbol BTCUSD_200925 --pair BTCUSD
```

### [GET /futures/data/topLongShortAccountRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts) - Top Trader Long/Short Account Ratio

```bash
binance-cli futures-coin top-trader-long-short-ratio-accounts --symbol symbol_example --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /futures/data/topLongShortPositionRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions) - Top Trader Long/Short Position Ratio

```bash
binance-cli futures-coin top-trader-long-short-ratio-positions --pair BTCUSD --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

## Trade

### [GET /dapi/v1/userTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#account-trade-list) - Account Trade List (USER_DATA)

```bash
binance-cli futures-coin account-trade-list --symbol BTCUSD_200626 --pair BTCUSD --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 6 --limit 30 --recv-window 5000
```

### [GET /dapi/v1/allOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#all-orders) - All Orders (USER_DATA)

```bash
binance-cli futures-coin all-orders --symbol BTCUSD_200925 --pair BTCUSD --order-id 1917641 --start-time 1623319461670 --end-time 1641782889000 --limit 30 --recv-window 5000
```

### [POST /dapi/v1/countdownCancelAll](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#auto-cancel-all-open-orders) - Auto-Cancel All Open Orders (TRADE)

```bash
binance-cli futures-coin auto-cancel-all-open-orders --symbol BTCUSD_200925 --countdown-time 1000 --recv-window 5000
```

### [DELETE /dapi/v1/allOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-all-open-orders) - Cancel All Open Orders (TRADE)

```bash
binance-cli futures-coin cancel-all-open-orders --symbol BTCUSD_200925 --recv-window 5000
```

### [DELETE /dapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-multiple-orders) - Cancel Multiple Orders (TRADE)

```bash
binance-cli futures-coin cancel-multiple-orders --symbol BTCUSD_200925 --order-id-list 1234567  --orig-client-order-id-list my_id_1  --recv-window 5000
```

### [DELETE /dapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-order) - Cancel Order (TRADE)

```bash
binance-cli futures-coin cancel-order --symbol BTCUSD_200925 --order-id 283194212 --orig-client-order-id myOrder1 --recv-window 5000
```

### [POST /dapi/v1/leverage](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-initial-leverage) - Change Initial Leverage (TRADE)

```bash
binance-cli futures-coin change-initial-leverage --symbol BTCUSD_200925 --leverage 1 --recv-window 5000
```

### [POST /dapi/v1/marginType](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-margin-type) - Change Margin Type (TRADE)

```bash
binance-cli futures-coin change-margin-type --symbol BTCUSD_200925 --margin-type ISOLATED --recv-window 5000
```

### [POST /dapi/v1/positionSide/dual](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-position-mode) - Change Position Mode (TRADE)

```bash
binance-cli futures-coin change-position-mode --dual-side-position true --recv-window 5000
```

### [GET /dapi/v1/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#current-all-open-orders) - Current All Open Orders (USER_DATA)

```bash
binance-cli futures-coin current-all-open-orders --symbol BTCUSD_200925 --pair BTCUSD --recv-window 5000
```

### [GET /dapi/v1/orderAmendment](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-order-modify-history) - Get Order Modify History (USER_DATA)

```bash
binance-cli futures-coin get-order-modify-history --symbol BTCUSD_PERP --order-id 20072994037 --orig-client-order-id LJ9R4QZDihCaS8UAOOLpgW --start-time 1623319461670 --end-time 1641782889000 --limit 30 --recv-window 5000
```

### [GET /dapi/v1/positionMargin/history](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-position-margin-change-history) - Get Position Margin Change History (TRADE)

```bash
binance-cli futures-coin get-position-margin-change-history --symbol BTCUSD --rtype 1 --start-time 1623319461670 --end-time 1641782889000 --limit 30 --recv-window 5000
```

### [POST /dapi/v1/positionMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-isolated-position-margin) - Modify Isolated Position Margin (TRADE)

```bash
binance-cli futures-coin modify-isolated-position-margin --symbol BTCUSDT --amount 1.0 --rtype 1 --position-side BOTH --recv-window 5000
```

### [PUT /dapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-multiple-orders) - Modify Multiple Orders (TRADE)

```bash
binance-cli futures-coin modify-multiple-orders --batch-orders [] --recv-window 5000
```

### [PUT /dapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-order) - Modify Order (TRADE)

```bash
binance-cli futures-coin modify-order --symbol BTCUSD_PERP --side BUY --order-id 1 --orig-client-order-id 1 --quantity 1.0 --price 1.0 --price-match OPPONENT --modify-id 1 --recv-window 5000
```

### [POST /dapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#new-order) - New Order (TRADE)

```bash
binance-cli futures-coin new-order --symbol BTCUSD_200925 --side BUY --rtype LIMIT --position-side BOTH --reduce-only TRUE --quantity 1.0 --price 1.0 --new-client-order-id 1 --stop-price 1.0 --close-position true --activation-price 1.0 --callback-rate 5000.0 --time-in-force GTC --working-type MARK_PRICE --price-protect TRUE --new-order-resp-type ACK --price-match OPPONENT --self-trade-prevention-mode EXPIRE_TAKER --recv-window 5000
```

### [POST /dapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#place-multiple-orders) - Place Multiple Orders (TRADE)

```bash
binance-cli futures-coin place-multiple-orders --batch-orders [] --recv-window 5000
```

### [GET /dapi/v1/adlQuantile](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-adl-quantile-estimation) - Position ADL Quantile Estimation (USER_DATA)

```bash
binance-cli futures-coin position-adl-quantile-estimation --symbol BTCUSD_200925 --recv-window 5000
```

### [GET /dapi/v1/positionRisk](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-information) - Position Information (USER_DATA)

```bash
binance-cli futures-coin position-information --margin-asset USDT --pair BTCUSDT --recv-window 5000
```

### [GET /dapi/v1/openOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-current-open-order) - Query Current Open Order (USER_DATA)

```bash
binance-cli futures-coin query-current-open-order --symbol BTCUSD_200925 --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /dapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-order) - Query Order (USER_DATA)

```bash
binance-cli futures-coin query-order --symbol BTCUSD_200925 --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /dapi/v1/forceOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#users-force-orders) - User's Force Orders (USER_DATA)

```bash
binance-cli futures-coin users-force-orders --symbol BTCUSD_200925 --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 30 --recv-window 5000
```

## UserDataStreams

### [DELETE /dapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#close-user-data-stream) - Close User Data Stream (USER_STREAM)

```bash
binance-cli futures-coin close-user-data-stream
```

### [PUT /dapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream) - Keepalive User Data Stream (USER_STREAM)

```bash
binance-cli futures-coin keepalive-user-data-stream
```

### [POST /dapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#start-user-data-stream) - Start User Data Stream (USER_STREAM)

```bash
binance-cli futures-coin start-user-data-stream
```
