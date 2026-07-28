## Account

### [GET /fapi/v2/account](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v2) - Account Information V2 (USER_DATA)

```bash
binance-cli futures-usds account-information-v2 --recv-window 5000
```

### [GET /fapi/v3/account](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v3) - Account Information V3 (USER_DATA)

```bash
binance-cli futures-usds account-information-v3 --recv-window 5000
```

### [GET /fapi/v2/balance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v2) - Futures Account Balance V2 (USER_DATA)

```bash
binance-cli futures-usds futures-account-balance-v2 --recv-window 5000
```

### [GET /fapi/v3/balance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v3) - Futures Account Balance V3 (USER_DATA)

```bash
binance-cli futures-usds futures-account-balance-v3 --recv-window 5000
```

### [GET /fapi/v1/accountConfig](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-configuration) - Futures Account Configuration (USER_DATA)

```bash
binance-cli futures-usds futures-account-configuration --recv-window 5000
```

### [GET /fapi/v1/apiTradingStatus](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-trading-quantitative-rules-indicators) - Futures Trading Quantitative Rules Indicators (USER_DATA)

```bash
binance-cli futures-usds futures-trading-quantitative-rules-indicators --symbol BTCUSDT --recv-window 5000
```

### [GET /fapi/v1/feeBurn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-bnb-burn-status) - Get BNB Burn Status (USER_DATA)

```bash
binance-cli futures-usds get-bnb-burn-status --recv-window 5000
```

### [GET /fapi/v1/multiAssetsMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-multi-assets-mode) - Get Current Multi-Assets Mode (USER_DATA)

```bash
binance-cli futures-usds get-current-multi-assets-mode --recv-window 5000
```

### [GET /fapi/v1/positionSide/dual](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-position-mode) - Get Current Position Mode (USER_DATA)

```bash
binance-cli futures-usds get-current-position-mode --recv-window 5000
```

### [GET /fapi/v1/order/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-order-history) - Get Download Id For Futures Order History (USER_DATA)

```bash
binance-cli futures-usds get-download-id-for-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/trade/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history) - Get Download Id For Futures Trade History (USER_DATA)

```bash
binance-cli futures-usds get-download-id-for-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/income/asyn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history) - Get Download Id For Futures Transaction History (USER_DATA)

```bash
binance-cli futures-usds get-download-id-for-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/order/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id) - Get Futures Order History Download Link by Id (USER_DATA)

```bash
binance-cli futures-usds get-futures-order-history-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /fapi/v1/trade/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id) - Get Futures Trade Download Link by Id (USER_DATA)

```bash
binance-cli futures-usds get-futures-trade-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /fapi/v1/income/asyn/id](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id) - Get Futures Transaction History Download Link by Id (USER_DATA)

```bash
binance-cli futures-usds get-futures-transaction-history-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /fapi/v1/income](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-income-history) - Get Income History (USER_DATA)

```bash
binance-cli futures-usds get-income-history --symbol BTCUSDT --income-type TRANSFER --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 30 --recv-window 5000
```

### [GET /fapi/v1/leverageBracket](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#notional-and-leverage-brackets) - Notional and Leverage Brackets (USER_DATA)

```bash
binance-cli futures-usds notional-and-leverage-brackets --symbol ETHUSDT --recv-window 5000
```

### [GET /fapi/v1/rateLimit/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#query-user-rate-limit) - Query User Rate Limit (USER_DATA)

```bash
binance-cli futures-usds query-user-rate-limit --recv-window 5000
```

### [GET /fapi/v1/symbolConfig](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#symbol-configuration) - Symbol Configuration (USER_DATA)

```bash
binance-cli futures-usds symbol-configuration --symbol BTCUSDT --recv-window 5000
```

### [POST /fapi/v1/feeBurn](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#toggle-bnb-burn-on-futures-trade) - Toggle BNB Burn On Futures Trade (TRADE)

```bash
binance-cli futures-usds toggle-bnb-burn-on-futures-trade --fee-burn true --recv-window 5000
```

### [GET /fapi/v1/commissionRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#user-commission-rate) - User Commission Rate (USER_DATA)

```bash
binance-cli futures-usds user-commission-rate --symbol BTCUSDT --recv-window 5000
```

## Convert

### [POST /fapi/v1/convert/acceptQuote](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#accept-the-offered-quote) - Accept the offered quote (USER_DATA)

```bash
binance-cli futures-usds accept-the-offered-quote --quote-id 1 --recv-window 5000
```

### [GET /fapi/v1/convert/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#list-all-convert-pairs) - List All Convert Pairs

```bash
binance-cli futures-usds list-all-convert-pairs --from-asset BTC --to-asset USDT
```

### [GET /fapi/v1/convert/orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#order-status) - Order status (USER_DATA)

```bash
binance-cli futures-usds order-status --order-id 933256278426274400 --quote-id 1
```

### [POST /fapi/v1/convert/getQuote](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#send-quote-request) - Send Quote Request (USER_DATA)

```bash
binance-cli futures-usds send-quote-request --from-asset BTC --to-asset USDT --from-amount 1.0 --to-amount 1.0 --valid-time 10s --recv-window 5000
```

## MarketData

### [GET /fapi/v1/symbolAdlRisk](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#adl-risk) - ADL Risk

```bash
binance-cli futures-usds adl-risk --symbol BTCUSDT
```

### [GET /fapi/v1/assetIndex](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#asset-index) - Multi-Assets Mode Asset Index

```bash
binance-cli futures-usds asset-index --symbol ADAUSD
```

### [GET /futures/data/basis](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#basis) - Basis

```bash
binance-cli futures-usds basis --pair BTCUSDT --contract-type PERPETUAL --period PERIOD_5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/time](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#check-server-time) - Check Server Time

```bash
binance-cli futures-usds check-server-time
```

### [GET /fapi/v1/indexInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#composite-index-symbol-information) - Composite Index Symbol Information

```bash
binance-cli futures-usds composite-index-symbol-information --symbol DEFIUSDT
```

### [GET /fapi/v1/aggTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list) - Compressed/Aggregate Trades List

```bash
binance-cli futures-usds compressed-aggregate-trades-list --symbol BTCUSDT --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /fapi/v1/continuousKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data) - Continuous Contract Kline/Candlestick Data

```bash
binance-cli futures-usds continuous-contract-kline-candlestick-data --pair BTCUSDT --contract-type PERPETUAL --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /fapi/v1/exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#exchange-information) - Exchange Information

```bash
binance-cli futures-usds exchange-information
```

### [GET /fapi/v1/fundingRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-history) - Get Funding Rate History

```bash
binance-cli futures-usds get-funding-rate-history --symbol BTCUSDT --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /fapi/v1/fundingInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-info) - Get Funding Rate Info

```bash
binance-cli futures-usds get-funding-rate-info
```

### [GET /fapi/v1/indexPriceKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data) - Index Price Kline/Candlestick Data

```bash
binance-cli futures-usds index-price-kline-candlestick-data --pair BTCUSDT --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /fapi/v1/klines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#kline-candlestick-data) - Kline/Candlestick Data

```bash
binance-cli futures-usds kline-candlestick-data --symbol BTCUSDT --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /futures/data/globalLongShortAccountRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#long-short-ratio) - Long/Short Ratio

```bash
binance-cli futures-usds long-short-ratio --symbol BTCUSDT --period PERIOD_5m --limit 50 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/premiumIndex](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price) - Mark Price

```bash
binance-cli futures-usds mark-price --symbol BTCUSDT
```

### [GET /fapi/v1/markPriceKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data) - Mark Price Kline/Candlestick Data

```bash
binance-cli futures-usds mark-price-kline-candlestick-data --symbol BTCUSDT --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /fapi/v1/historicalTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#old-trades-lookup) - Old Trades Lookup (MARKET_DATA)

```bash
binance-cli futures-usds old-trades-lookup --symbol BTCUSDT --limit 50 --from-id 1
```

### [GET /fapi/v1/openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest) - Open Interest

```bash
binance-cli futures-usds open-interest --symbol BTCUSDT
```

### [GET /futures/data/openInterestHist](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest-statistics) - Open Interest Statistics

```bash
binance-cli futures-usds open-interest-statistics --symbol BTCUSDT --period PERIOD_5m --limit 50 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/depth](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#order-book) - Order Book

```bash
binance-cli futures-usds order-book --symbol BTCUSDT --limit 50
```

### [GET /fapi/v1/premiumIndexKlines](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#premium-index-kline-data) - Premium index Kline Data

```bash
binance-cli futures-usds premium-index-kline-data --symbol BTCUSDT --interval INTERVAL_1m --start-time 1623319461670 --end-time 1641782889000 --limit 50
```

### [GET /futures/data/delivery-price](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#quarterly-contract-settlement-price) - Quarterly Contract Settlement Price

```bash
binance-cli futures-usds quarterly-contract-settlement-price --pair BTCUSDT
```

### [GET /fapi/v1/constituents](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-index-price-constituents) - Query Index Price Constituents

```bash
binance-cli futures-usds query-index-price-constituents --symbol BTCUSDT
```

### [GET /fapi/v1/insuranceBalance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-insurance-fund-balance-snapshot) - Query Insurance Fund Balance Snapshot

```bash
binance-cli futures-usds query-insurance-fund-balance-snapshot --symbol BNBUSDT
```

### [GET /fapi/v1/trades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#recent-trades-list) - Recent Trades List

```bash
binance-cli futures-usds recent-trades-list --symbol BTCUSDT --limit 50
```

### [GET /fapi/v1/rpiDepth](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#rpi-order-book) - RPI Order Book

```bash
binance-cli futures-usds rpi-order-book --symbol BTCUSDT --limit 1000
```

### [GET /fapi/v1/ticker/bookTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-order-book-ticker) - Symbol Order Book Ticker

```bash
binance-cli futures-usds symbol-order-book-ticker --symbol BTCUSDT
```

### [GET /fapi/v1/ticker/price](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker) - Symbol Price Ticker

```bash
binance-cli futures-usds symbol-price-ticker --symbol BTCUSDT
```

### [GET /fapi/v2/ticker/price](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker-v2) - Symbol Price Ticker V2

```bash
binance-cli futures-usds symbol-price-ticker-v2 --symbol BTCUSDT
```

### [GET /futures/data/takerlongshortRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#taker-buy-sell-volume) - Taker Buy/Sell Volume

```bash
binance-cli futures-usds taker-buy-sell-volume --symbol BTCUSDT --period PERIOD_5m --limit 50 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/ping](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#test-connectivity) - Test Connectivity

```bash
binance-cli futures-usds test-connectivity
```

### [GET /fapi/v1/ticker/24hr](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli futures-usds ticker24hr-price-change-statistics --symbol BTCUSDT
```

### [GET /futures/data/topLongShortAccountRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts) - Top Trader Long/Short Account Ratio (MARKET_DATA)

```bash
binance-cli futures-usds top-trader-long-short-ratio-accounts --symbol BTCUSDT --period PERIOD_5m --limit 50 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /futures/data/topLongShortPositionRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions) - Top Trader Long/Short Position Ratio (MARKET_DATA)

```bash
binance-cli futures-usds top-trader-long-short-ratio-positions --symbol BTCUSDT --period PERIOD_5m --limit 50 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/tradingSchedule](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#trading-schedule) - Trading Schedule

```bash
binance-cli futures-usds trading-schedule
```

## PortfolioMarginEndpoints

### [GET /fapi/v1/pmAccountInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/portfolio-margin-endpoints#classic-portfolio-margin-account-information) - Classic Portfolio Margin Account Information (USER_DATA)

```bash
binance-cli futures-usds classic-portfolio-margin-account-information --asset BTC --recv-window 5000
```

## Trade

### [GET /fapi/v1/userTrades](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#account-trade-list) - Account Trade List (USER_DATA)

```bash
binance-cli futures-usds account-trade-list --symbol BTCUSDT --order-id 25851813 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 50 --recv-window 5000
```

### [GET /fapi/v1/allOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#all-orders) - All Orders (USER_DATA)

```bash
binance-cli futures-usds all-orders --symbol BTCUSDT --order-id 1917641 --start-time 1623319461670 --end-time 1641782889000 --limit 50 --recv-window 5000
```

### [POST /fapi/v1/countdownCancelAll](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#auto-cancel-all-open-orders) - Auto-Cancel All Open Orders (TRADE)

```bash
binance-cli futures-usds auto-cancel-all-open-orders --symbol BTCUSDT --countdown-time 1000 --recv-window 5000
```

### [DELETE /fapi/v1/algoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-algo-order) - Cancel Algo Order (TRADE)

```bash
binance-cli futures-usds cancel-algo-order --algo-id 2146760 --client-algo-id 6B2I9XVcJpCjqPAJ4YoFX7 --recv-window 5000
```

### [DELETE /fapi/v1/algoOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-algo-open-orders) - Cancel All Algo Open Orders (TRADE)

```bash
binance-cli futures-usds cancel-all-algo-open-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /fapi/v1/allOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-open-orders) - Cancel All Open Orders (TRADE)

```bash
binance-cli futures-usds cancel-all-open-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /fapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-multiple-orders) - Cancel Multiple Orders (TRADE)

```bash
binance-cli futures-usds cancel-multiple-orders --symbol BTCUSDT --order-id-list 1234567  --orig-client-order-id-list my_id_1  --recv-window 5000
```

### [DELETE /fapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-order) - Cancel Order (TRADE)

```bash
binance-cli futures-usds cancel-order --symbol BTCUSDT --order-id 283194212 --orig-client-order-id myOrder1 --recv-window 5000
```

### [POST /fapi/v1/leverage](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-initial-leverage) - Change Initial Leverage (TRADE)

```bash
binance-cli futures-usds change-initial-leverage --symbol BTCUSDT --leverage 1 --recv-window 5000
```

### [POST /fapi/v1/marginType](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-margin-type) - Change Margin Type (TRADE)

```bash
binance-cli futures-usds change-margin-type --symbol BTCUSDT --margin-type ISOLATED --recv-window 5000
```

### [POST /fapi/v1/multiAssetsMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-multi-assets-mode) - Change Multi-Assets Mode (TRADE)

```bash
binance-cli futures-usds change-multi-assets-mode --multi-assets-margin true --recv-window 5000
```

### [POST /fapi/v1/positionSide/dual](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-position-mode) - Change Position Mode (TRADE)

```bash
binance-cli futures-usds change-position-mode --dual-side-position true --recv-window 5000
```

### [GET /fapi/v1/openAlgoOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-algo-open-orders) - Current All Algo Open Orders (USER_DATA)

```bash
binance-cli futures-usds current-all-algo-open-orders --algo-type CONDITIONAL --symbol BTCUSDT --algo-id 2148627 --recv-window 5000
```

### [GET /fapi/v1/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-open-orders) - Current All Open Orders (USER_DATA)

```bash
binance-cli futures-usds current-all-open-orders --symbol BTCUSDT --recv-window 5000
```

### [POST /fapi/v1/stock/contract](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#futures-tradfi-perps-contract) - Futures TradFi Perps Contract (USER_DATA)

```bash
binance-cli futures-usds futures-tradfi-perps-contract --recv-window 5000
```

### [GET /fapi/v1/orderAmendment](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-order-modify-history) - Get Order Modify History (USER_DATA)

```bash
binance-cli futures-usds get-order-modify-history --symbol BTCUSDT --order-id 20072994037 --orig-client-order-id LJ9R4QZDihCaS8UAOOLpgW --start-time 1623319461670 --end-time 1641782889000 --limit 50 --recv-window 5000
```

### [GET /fapi/v1/positionMargin/history](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-position-margin-change-history) - Get Position Margin Change History (TRADE)

```bash
binance-cli futures-usds get-position-margin-change-history --symbol BTCUSDT --rtype 1 --start-time 1623319461670 --end-time 1641782889000 --limit 50 --recv-window 5000
```

### [POST /fapi/v1/positionMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-isolated-position-margin) - Modify Isolated Position Margin (TRADE)

```bash
binance-cli futures-usds modify-isolated-position-margin --symbol BTCUSDT --amount 1.0 --rtype 1 --position-side BOTH --recv-window 5000
```

### [PUT /fapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-multiple-orders) - Modify Multiple Orders (TRADE)

```bash
binance-cli futures-usds modify-multiple-orders --batch-orders [] --recv-window 5000
```

### [PUT /fapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-order) - Modify Order (TRADE)

```bash
binance-cli futures-usds modify-order --symbol BTCUSDT --side BUY --quantity 1.0 --price 30005 --order-id 20072994037 --orig-client-order-id LJ9R4QZDihCaS8UAOOLpgW --price-match OPPONENT --modify-id 1 --recv-window 5000
```

### [POST /fapi/v1/algoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-algo-order) - New Algo Order (TRADE)

```bash
binance-cli futures-usds new-algo-order --algo-type CONDITIONAL --symbol BNBUSDT --side BUY --rtype STOP_MARKET --position-side BOTH --time-in-force GTC --quantity 1.0 --price 1.0 --trigger-price 1.0 --working-type MARK_PRICE --price-match OPPONENT --close-position TRUE --price-protect TRUE --reduce-only TRUE --activate-price 1.0 --callback-rate 1 --client-algo-id 1 --new-order-resp-type ACK --self-trade-prevention-mode EXPIRE_TAKER --good-till-date 1770736694138 --recv-window 5000
```

### [POST /fapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-order) - New Order (TRADE)

```bash
binance-cli futures-usds new-order --symbol BTCUSDT --side BUY --rtype LIMIT --position-side BOTH --time-in-force GTC --reduce-only TRUE --quantity 1.0 --price 1.0 --new-client-order-id 1 --new-order-resp-type ACK --price-match OPPONENT --self-trade-prevention-mode EXPIRE_TAKER --good-till-date 1770736694138 --recv-window 5000
```

### [POST /fapi/v1/batchOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#place-multiple-orders) - Place Multiple Orders (TRADE)

```bash
binance-cli futures-usds place-multiple-orders --batch-orders [] --recv-window 5000
```

### [GET /fapi/v1/adlQuantile](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-adl-quantile-estimation) - Position ADL Quantile Estimation (USER_DATA)

```bash
binance-cli futures-usds position-adl-quantile-estimation --symbol BTCUSDT --recv-window 5000
```

### [GET /fapi/v2/positionRisk](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v2) - Position Information V2 (USER_DATA)

```bash
binance-cli futures-usds position-information-v2 --symbol BTCUSDT --recv-window 5000
```

### [GET /fapi/v3/positionRisk](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v3) - Position Information V3 (USER_DATA)

```bash
binance-cli futures-usds position-information-v3 --symbol BTCUSDT --recv-window 5000
```

### [GET /fapi/v1/algoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-algo-order) - Query Algo Order (USER_DATA)

```bash
binance-cli futures-usds query-algo-order --algo-id 1 --client-algo-id 1 --recv-window 5000
```

### [GET /fapi/v1/allAlgoOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-all-algo-orders) - Query All Algo Orders (USER_DATA)

```bash
binance-cli futures-usds query-all-algo-orders --symbol BTCUSDT --algo-id 2146760 --start-time 1623319461670 --end-time 1641782889000 --limit 50 --recv-window 5000
```

### [GET /fapi/v1/openOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-current-open-order) - Query Current Open Order (USER_DATA)

```bash
binance-cli futures-usds query-current-open-order --symbol BTCUSDT --order-id 1917641 --orig-client-order-id abc --recv-window 5000
```

### [GET /fapi/v1/order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-order) - Query Order (USER_DATA)

```bash
binance-cli futures-usds query-order --symbol BTCUSDT --order-id 1917641 --orig-client-order-id abc --recv-window 5000
```

### [POST /fapi/v1/order/test](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#test-order) - Test Order (TRADE)

```bash
binance-cli futures-usds test-order --symbol BTCUSDT --side BUY --rtype LIMIT --position-side BOTH --reduce-only TRUE --quantity 1.0 --price 1.0 --new-client-order-id 1 --stop-price 1.0 --close-position TRUE --activation-price 1.0 --callback-rate 1 --time-in-force GTC --working-type MARK_PRICE --price-protect TRUE --new-order-resp-type ACK --price-match OPPONENT --self-trade-prevention-mode EXPIRE_TAKER --good-till-date 1770736694138 --recv-window 5000
```

### [GET /fapi/v1/forceOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#users-force-orders) - User's Force Orders (USER_DATA)

```bash
binance-cli futures-usds users-force-orders --symbol BTCUSDT --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 50 --recv-window 5000
```

## UserDataStreams

### [DELETE /fapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#close-user-data-stream) - Close User Data Stream (USER_STREAM)

```bash
binance-cli futures-usds close-user-data-stream
```

### [PUT /fapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream) - Keepalive User Data Stream (USER_STREAM)

```bash
binance-cli futures-usds keepalive-user-data-stream
```

### [POST /fapi/v1/listenKey](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#start-user-data-stream) - Start User Data Stream (USER_STREAM)

```bash
binance-cli futures-usds start-user-data-stream
```
