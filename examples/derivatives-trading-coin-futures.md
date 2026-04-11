## Account

### [GET /dapi/v1/account](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information) - Account Information

```bash
binance-cli futures-coin account-information --recv-window 5000
```

### [GET /dapi/v1/balance](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance) - Futures Account Balance

```bash
binance-cli futures-coin futures-account-balance --recv-window 5000
```

### [GET /dapi/v1/positionSide/dual](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode) - Get Current Position Mode

```bash
binance-cli futures-coin get-current-position-mode --recv-window 5000
```

### [GET /dapi/v1/order/asyn](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History) - Get Download Id For Futures Order History

```bash
binance-cli futures-coin get-download-id-for-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/trade/asyn](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History) - Get Download Id For Futures Trade History

```bash
binance-cli futures-coin get-download-id-for-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/income/asyn](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History) - Get Download Id For Futures Transaction History

```bash
binance-cli futures-coin get-download-id-for-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /dapi/v1/order/asyn/id](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id) - Get Futures Order History Download Link by Id

```bash
binance-cli futures-coin get-futures-order-history-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /dapi/v1/trade/asyn/id](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id) - Get Futures Trade Download Link by Id

```bash
binance-cli futures-coin get-futures-trade-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /dapi/v1/income/asyn/id](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id) - Get Futures Transaction History Download Link by Id

```bash
binance-cli futures-coin get-futures-transaction-history-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /dapi/v1/income](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Income-History) - Get Income History

```bash
binance-cli futures-coin get-income-history --symbol "symbol_example" --income-type "incomeType_example" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /dapi/v1/leverageBracket](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Pair) - Notional Bracket for Pair

```bash
binance-cli futures-coin notional-bracket-for-pair --pair "pair_example" --recv-window 5000
```

### [GET /dapi/v2/leverageBracket](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Notional-Bracket-for-Symbol) - Notional Bracket for Symbol

```bash
binance-cli futures-coin notional-bracket-for-symbol --symbol "symbol_example" --recv-window 5000
```

### [GET /dapi/v1/commissionRate](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate) - User Commission Rate

```bash
binance-cli futures-coin user-commission-rate --symbol "symbol_example" --recv-window 5000
```

## MarketData

### [GET /futures/data/basis](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Basis) - Basis

```bash
binance-cli futures-coin basis --pair "pair_example" --contract-type PERPETUAL --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/time](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Check-Server-time) - Check Server time

```bash
binance-cli futures-coin check-server-time
```

### [GET /dapi/v1/aggTrades](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List) - Compressed/Aggregate Trades List

```bash
binance-cli futures-coin compressed-aggregate-trades-list --symbol "symbol_example" --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/continuousKlines](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data) - Continuous Contract Kline/Candlestick Data

```bash
binance-cli futures-coin continuous-contract-kline-candlestick-data --pair "pair_example" --contract-type PERPETUAL --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/exchangeInfo](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Exchange-Information) - Exchange Information

```bash
binance-cli futures-coin exchange-information
```

### [GET /dapi/v1/fundingRate](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Rate-History-of-Perpetual-Futures) - Get Funding Rate History of Perpetual Futures

```bash
binance-cli futures-coin get-funding-rate-history-of-perpetual-futures --symbol "symbol_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/fundingInfo](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Info) - Get Funding Rate Info

```bash
binance-cli futures-coin get-funding-rate-info
```

### [GET /dapi/v1/premiumIndex](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-and-Mark-Price) - Index Price and Mark Price

```bash
binance-cli futures-coin index-price-and-mark-price --symbol "symbol_example" --pair "pair_example"
```

### [GET /dapi/v1/indexPriceKlines](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data) - Index Price Kline/Candlestick Data

```bash
binance-cli futures-coin index-price-kline-candlestick-data --pair "pair_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/klines](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Kline-Candlestick-Data) - Kline/Candlestick Data

```bash
binance-cli futures-coin kline-candlestick-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /futures/data/globalLongShortAccountRatio](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Long-Short-Ratio) - Long/Short Ratio

```bash
binance-cli futures-coin long-short-ratio --pair "pair_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/markPriceKlines](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data) - Mark Price Kline/Candlestick Data

```bash
binance-cli futures-coin mark-price-kline-candlestick-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/historicalTrades](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Old-Trades-Lookup) - Old Trades Lookup

```bash
binance-cli futures-coin old-trades-lookup --symbol "symbol_example" --limit 100 --from-id 1
```

### [GET /dapi/v1/openInterest](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Open-Interest) - Open Interest

```bash
binance-cli futures-coin open-interest --symbol "symbol_example"
```

### [GET /futures/data/openInterestHist](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Open-Interest-Statistics) - Open Interest Statistics

```bash
binance-cli futures-coin open-interest-statistics --pair "pair_example" --contract-type PERPETUAL --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/depth](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Order-Book) - Order Book

```bash
binance-cli futures-coin order-book --symbol "symbol_example" --limit 100
```

### [GET /dapi/v1/premiumIndexKlines](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Premium-index-Kline-Data) - Premium index Kline Data

```bash
binance-cli futures-coin premium-index-kline-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /dapi/v1/constituents](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Constituents) - Query Index Price Constituents

```bash
binance-cli futures-coin query-index-price-constituents --symbol "symbol_example"
```

### [GET /dapi/v1/trades](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Recent-Trades-List) - Recent Trades List

```bash
binance-cli futures-coin recent-trades-list --symbol "symbol_example" --limit 100
```

### [GET /dapi/v1/ticker/bookTicker](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker) - Symbol Order Book Ticker

```bash
binance-cli futures-coin symbol-order-book-ticker --symbol "symbol_example" --pair "pair_example"
```

### [GET /dapi/v1/ticker/price](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Symbol-Price-Ticker) - Symbol Price Ticker

```bash
binance-cli futures-coin symbol-price-ticker --symbol "symbol_example" --pair "pair_example"
```

### [GET /futures/data/takerBuySellVol](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Taker-Buy-Sell-Volume) - Taker Buy/Sell Volume

```bash
binance-cli futures-coin taker-buy-sell-volume --pair "pair_example" --contract-type PERPETUAL --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /dapi/v1/ping](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Test-Connectivity) - Test Connectivity

```bash
binance-cli futures-coin test-connectivity
```

### [GET /dapi/v1/ticker/24hr](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli futures-coin ticker24hr-price-change-statistics --symbol "symbol_example" --pair "pair_example"
```

### [GET /futures/data/topLongShortAccountRatio](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio) - Top Trader Long/Short Ratio (Accounts)

```bash
binance-cli futures-coin top-trader-long-short-ratio-accounts --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /futures/data/topLongShortPositionRatio](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio) - Top Trader Long/Short Ratio (Positions)

```bash
binance-cli futures-coin top-trader-long-short-ratio-positions --pair "pair_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

## PortfolioMarginEndpoints

### [GET /dapi/v1/pmAccountInfo](https://developers.binance.com/docs/derivatives/coin-margined-futures/portfolio-margin-endpoints/Classic-Portfolio-Margin-Account-Information) - Classic Portfolio Margin Account Information

```bash
binance-cli futures-coin classic-portfolio-margin-account-information --asset "asset_example" --recv-window 5000
```

## Trade

### [GET /dapi/v1/userTrades](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List) - Account Trade List

```bash
binance-cli futures-coin account-trade-list --symbol "symbol_example" --pair "pair_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 100 --recv-window 5000
```

### [GET /dapi/v1/allOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders) - All Orders

```bash
binance-cli futures-coin all-orders --symbol "symbol_example" --pair "pair_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [POST /dapi/v1/countdownCancelAll](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders) - Auto-Cancel All Open Orders

```bash
binance-cli futures-coin auto-cancel-all-open-orders --json {}
```

### [DELETE /dapi/v1/allOpenOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders) - Cancel All Open Orders

```bash
binance-cli futures-coin cancel-all-open-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /dapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Multiple-Orders) - Cancel Multiple Orders

```bash
binance-cli futures-coin cancel-multiple-orders --symbol "symbol_example" --order-id-list 1234567  --orig-client-order-id-list "my_id_1"  --recv-window 5000
```

### [DELETE /dapi/v1/order](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order) - Cancel Order

```bash
binance-cli futures-coin cancel-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [POST /dapi/v1/leverage](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage) - Change Initial Leverage

```bash
binance-cli futures-coin change-initial-leverage --json {}
```

### [POST /dapi/v1/marginType](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type) - Change Margin Type

```bash
binance-cli futures-coin change-margin-type --json {}
```

### [POST /dapi/v1/positionSide/dual](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode) - Change Position Mode

```bash
binance-cli futures-coin change-position-mode --json {}
```

### [GET /dapi/v1/openOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders) - Current All Open Orders

```bash
binance-cli futures-coin current-all-open-orders --symbol "symbol_example" --pair "pair_example" --recv-window 5000
```

### [GET /dapi/v1/orderAmendment](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History) - Get Order Modify History

```bash
binance-cli futures-coin get-order-modify-history --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /dapi/v1/positionMargin/history](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History) - Get Position Margin Change History

```bash
binance-cli futures-coin get-position-margin-change-history --symbol "symbol_example" --type LIMIT --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [POST /dapi/v1/positionMargin](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin) - Modify Isolated Position Margin

```bash
binance-cli futures-coin modify-isolated-position-margin --json {}
```

### [PUT /dapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders) - Modify Multiple Orders

```bash
binance-cli futures-coin modify-multiple-orders --json {}
```

### [PUT /dapi/v1/order](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order) - Modify Order

```bash
binance-cli futures-coin modify-order --json {}
```

### [POST /dapi/v1/order](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/New-Order) - New Order

```bash
binance-cli futures-coin new-order --json {}
```

### [POST /dapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Place-Multiple-Orders) - Place Multiple Orders

```bash
binance-cli futures-coin place-multiple-orders --json {}
```

### [GET /dapi/v1/adlQuantile](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation) - Position ADL Quantile Estimation

```bash
binance-cli futures-coin position-adl-quantile-estimation --symbol "symbol_example" --recv-window 5000
```

### [GET /dapi/v1/positionRisk](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Position-Information) - Position Information

```bash
binance-cli futures-coin position-information --margin-asset "marginAsset_example" --pair "pair_example" --recv-window 5000
```

### [GET /dapi/v1/openOrder](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order) - Query Current Open Order

```bash
binance-cli futures-coin query-current-open-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /dapi/v1/order](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order) - Query Order

```bash
binance-cli futures-coin query-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /dapi/v1/forceOrders](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders) - User\'s Force Orders

```bash
binance-cli futures-coin users-force-orders --symbol "symbol_example" --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

## UserDataStreams

### [DELETE /dapi/v1/listenKey](https://developers.binance.com/docs/derivatives/coin-margined-futures/user-data-streams/Close-User-Data-Stream) - Close User Data Stream

```bash
binance-cli futures-coin close-user-data-stream
```

### [PUT /dapi/v1/listenKey](https://developers.binance.com/docs/derivatives/coin-margined-futures/user-data-streams/Keepalive-User-Data-Stream) - Keepalive User Data Stream

```bash
binance-cli futures-coin keepalive-user-data-stream
```

### [POST /dapi/v1/listenKey](https://developers.binance.com/docs/derivatives/coin-margined-futures/user-data-streams/Start-User-Data-Stream) - Start User Data Stream

```bash
binance-cli futures-coin start-user-data-stream
```
