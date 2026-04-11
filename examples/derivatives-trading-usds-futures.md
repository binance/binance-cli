## Account

### [GET /fapi/v2/account](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V2) - Account Information V2

```bash
binance-cli futures-usds account-information-v2 --recv-window 5000
```

### [GET /fapi/v3/account](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V3) - Account Information V3

```bash
binance-cli futures-usds account-information-v3 --recv-window 5000
```

### [GET /fapi/v2/balance](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V2) - Futures Account Balance V2

```bash
binance-cli futures-usds futures-account-balance-v2 --recv-window 5000
```

### [GET /fapi/v3/balance](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V3) - Futures Account Balance V3

```bash
binance-cli futures-usds futures-account-balance-v3 --recv-window 5000
```

### [GET /fapi/v1/accountConfig](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Config) - Futures Account Configuration

```bash
binance-cli futures-usds futures-account-configuration --recv-window 5000
```

### [GET /fapi/v1/apiTradingStatus](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Trading-Quantitative-Rules-Indicators) - Futures Trading Quantitative Rules Indicators

```bash
binance-cli futures-usds futures-trading-quantitative-rules-indicators --symbol "symbol_example" --recv-window 5000
```

### [GET /fapi/v1/feeBurn](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-BNB-Burn-Status) - Get BNB Burn Status

```bash
binance-cli futures-usds get-bnb-burn-status --recv-window 5000
```

### [GET /fapi/v1/multiAssetsMargin](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Current-Multi-Assets-Mode) - Get Current Multi-Assets Mode

```bash
binance-cli futures-usds get-current-multi-assets-mode --recv-window 5000
```

### [GET /fapi/v1/positionSide/dual](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Current-Position-Mode) - Get Current Position Mode

```bash
binance-cli futures-usds get-current-position-mode --recv-window 5000
```

### [GET /fapi/v1/order/asyn](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Order-History) - Get Download Id For Futures Order History

```bash
binance-cli futures-usds get-download-id-for-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/trade/asyn](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Trade-History) - Get Download Id For Futures Trade History

```bash
binance-cli futures-usds get-download-id-for-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/income/asyn](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Download-Id-For-Futures-Transaction-History) - Get Download Id For Futures Transaction History

```bash
binance-cli futures-usds get-download-id-for-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /fapi/v1/order/asyn/id](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Order-History-Download-Link-by-Id) - Get Futures Order History Download Link by Id

```bash
binance-cli futures-usds get-futures-order-history-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /fapi/v1/trade/asyn/id](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Trade-Download-Link-by-Id) - Get Futures Trade Download Link by Id

```bash
binance-cli futures-usds get-futures-trade-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /fapi/v1/income/asyn/id](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Futures-Transaction-History-Download-Link-by-Id) - Get Futures Transaction History Download Link by Id

```bash
binance-cli futures-usds get-futures-transaction-history-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /fapi/v1/income](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Income-History) - Get Income History

```bash
binance-cli futures-usds get-income-history --symbol "symbol_example" --income-type "incomeType_example" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /fapi/v1/leverageBracket](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Notional-and-Leverage-Brackets) - Notional and Leverage Brackets

```bash
binance-cli futures-usds notional-and-leverage-brackets --symbol "symbol_example" --recv-window 5000
```

### [GET /fapi/v1/rateLimit/order](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Query-Rate-Limit) - Query User Rate Limit

```bash
binance-cli futures-usds query-user-rate-limit --recv-window 5000
```

### [GET /fapi/v1/symbolConfig](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Symbol-Config) - Symbol Configuration

```bash
binance-cli futures-usds symbol-configuration --symbol "symbol_example" --recv-window 5000
```

### [POST /fapi/v1/feeBurn](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Toggle-BNB-Burn-On-Futures-Trade) - Toggle BNB Burn On Futures Trade

```bash
binance-cli futures-usds toggle-bnb-burn-on-futures-trade --json {}
```

### [GET /fapi/v1/commissionRate](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/User-Commission-Rate) - User Commission Rate

```bash
binance-cli futures-usds user-commission-rate --symbol "symbol_example" --recv-window 5000
```

## Convert

### [POST /fapi/v1/convert/acceptQuote](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Accept-Quote) - Accept the offered quote

```bash
binance-cli futures-usds accept-the-offered-quote --json {}
```

### [GET /fapi/v1/convert/exchangeInfo](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/) - List All Convert Pairs

```bash
binance-cli futures-usds list-all-convert-pairs --from-asset "fromAsset_example" --to-asset "toAsset_example"
```

### [GET /fapi/v1/convert/orderStatus](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Order-Status) - Order status

```bash
binance-cli futures-usds order-status --order-id 1 --quote-id "1"
```

### [POST /fapi/v1/convert/getQuote](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert/Send-quote-request) - Send Quote Request

```bash
binance-cli futures-usds send-quote-request --json {}
```

## MarketData

### [GET /fapi/v1/symbolAdlRisk](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/ADL-Risk) - ADL Risk

```bash
binance-cli futures-usds adl-risk --symbol "symbol_example"
```

### [GET /futures/data/basis](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Basis) - Basis

```bash
binance-cli futures-usds basis --pair "pair_example" --contract-type PERPETUAL --period 5m --limit 30 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/time](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time) - Check Server Time

```bash
binance-cli futures-usds check-server-time
```

### [GET /fapi/v1/indexInfo](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Composite-Index-Symbol-Information) - Composite Index Symbol Information

```bash
binance-cli futures-usds composite-index-symbol-information --symbol "symbol_example"
```

### [GET /fapi/v1/aggTrades](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List) - Compressed/Aggregate Trades List

```bash
binance-cli futures-usds compressed-aggregate-trades-list --symbol "symbol_example" --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /fapi/v1/continuousKlines](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data) - Continuous Contract Kline/Candlestick Data

```bash
binance-cli futures-usds continuous-contract-kline-candlestick-data --pair "pair_example" --contract-type PERPETUAL --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /fapi/v1/exchangeInfo](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information) - Exchange Information

```bash
binance-cli futures-usds exchange-information
```

### [GET /fapi/v1/fundingRate](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History) - Get Funding Rate History

```bash
binance-cli futures-usds get-funding-rate-history --symbol "symbol_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /fapi/v1/fundingInfo](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info) - Get Funding Rate Info

```bash
binance-cli futures-usds get-funding-rate-info
```

### [GET /fapi/v1/indexPriceKlines](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data) - Index Price Kline/Candlestick Data

```bash
binance-cli futures-usds index-price-kline-candlestick-data --pair "pair_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /fapi/v1/klines](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data) - Kline/Candlestick Data

```bash
binance-cli futures-usds kline-candlestick-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /futures/data/globalLongShortAccountRatio](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Long-Short-Ratio) - Long/Short Ratio

```bash
binance-cli futures-usds long-short-ratio --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/premiumIndex](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price) - Mark Price

```bash
binance-cli futures-usds mark-price --symbol "symbol_example"
```

### [GET /fapi/v1/markPriceKlines](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price-Kline-Candlestick-Data) - Mark Price Kline/Candlestick Data

```bash
binance-cli futures-usds mark-price-kline-candlestick-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /fapi/v1/assetIndex](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Multi-Assets-Mode-Asset-Index) - Multi-Assets Mode Asset Index

```bash
binance-cli futures-usds multi-assets-mode-asset-index --symbol "symbol_example"
```

### [GET /fapi/v1/historicalTrades](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup) - Old Trades Lookup

```bash
binance-cli futures-usds old-trades-lookup --symbol "symbol_example" --limit 100 --from-id 1
```

### [GET /fapi/v1/openInterest](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest) - Open Interest

```bash
binance-cli futures-usds open-interest --symbol "symbol_example"
```

### [GET /futures/data/openInterestHist](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics) - Open Interest Statistics

```bash
binance-cli futures-usds open-interest-statistics --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/depth](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book) - Order Book

```bash
binance-cli futures-usds order-book --symbol "symbol_example" --limit 100
```

### [GET /fapi/v1/premiumIndexKlines](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Premium-index-Kline-Data) - Premium index Kline Data

```bash
binance-cli futures-usds premium-index-kline-data --symbol "symbol_example" --interval 1m --start-time 1623319461670 --end-time 1641782889000 --limit 100
```

### [GET /futures/data/delivery-price](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price) - Quarterly Contract Settlement Price

```bash
binance-cli futures-usds quarterly-contract-settlement-price --pair "pair_example"
```

### [GET /fapi/v1/constituents](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Index-Constituents) - Query Index Price Constituents

```bash
binance-cli futures-usds query-index-price-constituents --symbol "symbol_example"
```

### [GET /fapi/v1/insuranceBalance](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Query-Insurance-Fund-Balance-Snapshot) - Query Insurance Fund Balance Snapshot

```bash
binance-cli futures-usds query-insurance-fund-balance-snapshot --symbol "symbol_example"
```

### [GET /fapi/v1/trades](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List) - Recent Trades List

```bash
binance-cli futures-usds recent-trades-list --symbol "symbol_example" --limit 100
```

### [GET /fapi/v1/rpiDepth](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book-RPI) - RPI Order Book

```bash
binance-cli futures-usds rpi-order-book --symbol "symbol_example" --limit 100
```

### [GET /fapi/v1/ticker/bookTicker](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Order-Book-Ticker) - Symbol Order Book Ticker

```bash
binance-cli futures-usds symbol-order-book-ticker --symbol "symbol_example"
```

### [GET /fapi/v1/ticker/price](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker) - Symbol Price Ticker

```bash
binance-cli futures-usds symbol-price-ticker --symbol "symbol_example"
```

### [GET /fapi/v2/ticker/price](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Symbol-Price-Ticker-V2) - Symbol Price Ticker V2

```bash
binance-cli futures-usds symbol-price-ticker-v2 --symbol "symbol_example"
```

### [GET /futures/data/takerlongshortRatio](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Taker-BuySell-Volume) - Taker Buy/Sell Volume

```bash
binance-cli futures-usds taker-buy-sell-volume --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/ping](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Test-Connectivity) - Test Connectivity

```bash
binance-cli futures-usds test-connectivity
```

### [GET /fapi/v1/ticker/24hr](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics) - 24hr Ticker Price Change Statistics

```bash
binance-cli futures-usds ticker24hr-price-change-statistics --symbol "symbol_example"
```

### [GET /futures/data/topLongShortAccountRatio](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Long-Short-Account-Ratio) - Top Trader Long/Short Ratio (Accounts)

```bash
binance-cli futures-usds top-trader-long-short-ratio-accounts --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /futures/data/topLongShortPositionRatio](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Top-Trader-Long-Short-Ratio) - Top Trader Long/Short Ratio (Positions)

```bash
binance-cli futures-usds top-trader-long-short-ratio-positions --symbol "symbol_example" --period 5m --limit 100 --start-time 1623319461670 --end-time 1641782889000
```

### [GET /fapi/v1/tradingSchedule](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Trading-Schedule) - Trading Schedule

```bash
binance-cli futures-usds trading-schedule
```

## PortfolioMarginEndpoints

### [GET /fapi/v1/pmAccountInfo](https://developers.binance.com/docs/derivatives/usds-margined-futures/portfolio-margin-endpoints/Classic-Portfolio-Margin-Account-Information) - Classic Portfolio Margin Account Information

```bash
binance-cli futures-usds classic-portfolio-margin-account-information --asset "asset_example" --recv-window 5000
```

## Trade

### [GET /fapi/v1/userTrades](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Account-Trade-List) - Account Trade List

```bash
binance-cli futures-usds account-trade-list --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 100 --recv-window 5000
```

### [GET /fapi/v1/allOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/All-Orders) - All Orders

```bash
binance-cli futures-usds all-orders --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [POST /fapi/v1/countdownCancelAll](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders) - Auto-Cancel All Open Orders

```bash
binance-cli futures-usds auto-cancel-all-open-orders --json {}
```

### [DELETE /fapi/v1/algoOrder](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Algo-Order) - Cancel Algo Order

```bash
binance-cli futures-usds cancel-algo-order --algo-id 1 --client-algo-id "1" --recv-window 5000
```

### [DELETE /fapi/v1/algoOpenOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-All-Algo-Open-Orders) - Cancel All Algo Open Orders

```bash
binance-cli futures-usds cancel-all-algo-open-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /fapi/v1/allOpenOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-All-Open-Orders) - Cancel All Open Orders

```bash
binance-cli futures-usds cancel-all-open-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /fapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Multiple-Orders) - Cancel Multiple Orders

```bash
binance-cli futures-usds cancel-multiple-orders --symbol "symbol_example" --order-id-list 1234567  --orig-client-order-id-list "my_id_1"  --recv-window 5000
```

### [DELETE /fapi/v1/order](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-Order) - Cancel Order

```bash
binance-cli futures-usds cancel-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [POST /fapi/v1/leverage](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Initial-Leverage) - Change Initial Leverage

```bash
binance-cli futures-usds change-initial-leverage --json {}
```

### [POST /fapi/v1/marginType](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Margin-Type) - Change Margin Type

```bash
binance-cli futures-usds change-margin-type --json {}
```

### [POST /fapi/v1/multiAssetsMargin](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Multi-Assets-Mode) - Change Multi-Assets Mode

```bash
binance-cli futures-usds change-multi-assets-mode --json {}
```

### [POST /fapi/v1/positionSide/dual](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Position-Mode) - Change Position Mode

```bash
binance-cli futures-usds change-position-mode --json {}
```

### [GET /fapi/v1/openAlgoOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Current-All-Algo-Open-Orders) - Current All Algo Open Orders

```bash
binance-cli futures-usds current-all-algo-open-orders --algo-type "algoType_example" --symbol "symbol_example" --algo-id 1 --recv-window 5000
```

### [GET /fapi/v1/openOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Current-All-Open-Orders) - Current All Open Orders

```bash
binance-cli futures-usds current-all-open-orders --symbol "symbol_example" --recv-window 5000
```

### [POST /fapi/v1/stock/contract](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Futures-TradFi-Perps-Contract) - Futures TradFi Perps Contract

```bash
binance-cli futures-usds futures-tradfi-perps-contract --json {}
```

### [GET /fapi/v1/orderAmendment](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Get-Order-Modify-History) - Get Order Modify History

```bash
binance-cli futures-usds get-order-modify-history --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /fapi/v1/positionMargin/history](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Get-Position-Margin-Change-History) - Get Position Margin Change History

```bash
binance-cli futures-usds get-position-margin-change-history --symbol "symbol_example" --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [POST /fapi/v1/positionMargin](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin) - Modify Isolated Position Margin

```bash
binance-cli futures-usds modify-isolated-position-margin --json {}
```

### [PUT /fapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Multiple-Orders) - Modify Multiple Orders

```bash
binance-cli futures-usds modify-multiple-orders --json {}
```

### [PUT /fapi/v1/order](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Order) - Modify Order

```bash
binance-cli futures-usds modify-order --json {}
```

### [POST /fapi/v1/algoOrder](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Algo-Order) - New Algo Order

```bash
binance-cli futures-usds new-algo-order --json {}
```

### [POST /fapi/v1/order](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Order) - New Order

```bash
binance-cli futures-usds new-order --json {}
```

### [POST /fapi/v1/batchOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Place-Multiple-Orders) - Place Multiple Orders

```bash
binance-cli futures-usds place-multiple-orders --json {}
```

### [GET /fapi/v1/adlQuantile](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-ADL-Quantile-Estimation) - Position ADL Quantile Estimation

```bash
binance-cli futures-usds position-adl-quantile-estimation --symbol "symbol_example" --recv-window 5000
```

### [GET /fapi/v2/positionRisk](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-Information-V2) - Position Information V2

```bash
binance-cli futures-usds position-information-v2 --symbol "symbol_example" --recv-window 5000
```

### [GET /fapi/v3/positionRisk](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Position-Information-V3) - Position Information V3

```bash
binance-cli futures-usds position-information-v3 --symbol "symbol_example" --recv-window 5000
```

### [GET /fapi/v1/algoOrder](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-Algo-Order) - Query Algo Order

```bash
binance-cli futures-usds query-algo-order --algo-id 1 --client-algo-id "1" --recv-window 5000
```

### [GET /fapi/v1/allAlgoOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-All-Algo-Orders) - Query All Algo Orders

```bash
binance-cli futures-usds query-all-algo-orders --symbol "symbol_example" --algo-id 1 --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /fapi/v1/openOrder](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-Current-Open-Order) - Query Current Open Order

```bash
binance-cli futures-usds query-current-open-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /fapi/v1/order](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-Order) - Query Order

```bash
binance-cli futures-usds query-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [POST /fapi/v1/order/test](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/New-Order-Test) - Test Order

```bash
binance-cli futures-usds test-order --json {}
```

### [GET /fapi/v1/forceOrders](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Users-Force-Orders) - User\'s Force Orders

```bash
binance-cli futures-usds users-force-orders --symbol "symbol_example" --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

## UserDataStreams

### [DELETE /fapi/v1/listenKey](https://developers.binance.com/docs/derivatives/usds-margined-futures/user-data-streams/Close-User-Data-Stream) - Close User Data Stream

```bash
binance-cli futures-usds close-user-data-stream
```

### [PUT /fapi/v1/listenKey](https://developers.binance.com/docs/derivatives/usds-margined-futures/user-data-streams/Keepalive-User-Data-Stream) - Keepalive User Data Stream

```bash
binance-cli futures-usds keepalive-user-data-stream
```

### [POST /fapi/v1/listenKey](https://developers.binance.com/docs/derivatives/usds-margined-futures/user-data-streams/Start-User-Data-Stream) - Start User Data Stream

```bash
binance-cli futures-usds start-user-data-stream
```
