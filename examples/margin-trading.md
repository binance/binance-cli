## Account

### [POST /sapi/v1/margin/max-leverage](https://developers.binance.com/docs/margin_trading/account/Adjust-cross-margin-max-leverage) - Adjust cross margin max leverage

```bash
binance-cli margin-trading adjust-cross-margin-max-leverage --json {}
```

### [DELETE /sapi/v1/margin/isolated/account](https://developers.binance.com/docs/margin_trading/account/Disable-Isolated-Margin-Account) - Disable Isolated Margin Account

```bash
binance-cli margin-trading disable-isolated-margin-account --symbol "symbol_example" --recv-window 5000
```

### [POST /sapi/v1/margin/isolated/account](https://developers.binance.com/docs/margin_trading/account/Enable-Isolated-Margin-Account) - Enable Isolated Margin Account

```bash
binance-cli margin-trading enable-isolated-margin-account --json {}
```

### [GET /sapi/v1/bnbBurn](https://developers.binance.com/docs/margin_trading/account/Get-BNB-Burn-Status) - Get BNB Burn Status

```bash
binance-cli margin-trading get-bnb-burn-status --recv-window 5000
```

### [GET /sapi/v1/margin/tradeCoeff](https://developers.binance.com/docs/margin_trading/account/Get-Summary-of-Margin-account) - Get Summary of Margin account

```bash
binance-cli margin-trading get-summary-of-margin-account --recv-window 5000
```

### [GET /sapi/v1/margin/capital-flow](https://developers.binance.com/docs/margin_trading/account/Query-Cross-Isolated-Margin-Capital-Flow) - Query Cross Isolated Margin Capital Flow

```bash
binance-cli margin-trading query-cross-isolated-margin-capital-flow --asset "asset_example" --symbol "symbol_example" --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/account](https://developers.binance.com/docs/margin_trading/account/Query-Cross-Margin-Account-Details) - Query Cross Margin Account Details

```bash
binance-cli margin-trading query-cross-margin-account-details --recv-window 5000
```

### [GET /sapi/v1/margin/crossMarginData](https://developers.binance.com/docs/margin_trading/account/Query-Cross-Margin-Fee-Data) - Query Cross Margin Fee Data

```bash
binance-cli margin-trading query-cross-margin-fee-data --vip-level 1 --coin "coin_example" --recv-window 5000
```

### [GET /sapi/v1/margin/isolated/accountLimit](https://developers.binance.com/docs/margin_trading/account/Query-Enabled-Isolated-Margin-Account-Limit) - Query Enabled Isolated Margin Account Limit

```bash
binance-cli margin-trading query-enabled-isolated-margin-account-limit --recv-window 5000
```

### [GET /sapi/v1/margin/isolated/account](https://developers.binance.com/docs/margin_trading/account/Query-Isolated-Margin-Account-Info) - Query Isolated Margin Account Info

```bash
binance-cli margin-trading query-isolated-margin-account-info --symbols "symbols_example" --recv-window 5000
```

### [GET /sapi/v1/margin/isolatedMarginData](https://developers.binance.com/docs/margin_trading/account/Query-Isolated-Margin-Fee-Data) - Query Isolated Margin Fee Data

```bash
binance-cli margin-trading query-isolated-margin-fee-data --vip-level 1 --symbol "symbol_example" --recv-window 5000
```

## BorrowRepay

### [GET /sapi/v1/margin/next-hourly-interest-rate](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Get-a-future-hourly-interest-rate) - Get future hourly interest rate

```bash
binance-cli margin-trading get-future-hourly-interest-rate --assets "assets_example" --is-isolated "false"
```

### [GET /sapi/v1/margin/interestHistory](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Get-Interest-History) - Get Interest History

```bash
binance-cli margin-trading get-interest-history --asset "asset_example" --isolated-symbol "isolatedSymbol_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/margin/borrow-repay](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Margin-account-borrow-repay) - Margin account borrow/repay

```bash
binance-cli margin-trading margin-account-borrow-repay --json {}
```

### [GET /sapi/v1/margin/borrow-repay](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Query-Borrow-Repay) - Query borrow/repay records in Margin account

```bash
binance-cli margin-trading query-borrow-repay-records-in-margin-account --type "type_example" --asset "asset_example" --isolated-symbol "isolatedSymbol_example" --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/margin/interestRateHistory](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Query-Margin-Interest-Rate-History) - Query Margin Interest Rate History

```bash
binance-cli margin-trading query-margin-interest-rate-history --asset "asset_example" --vip-level 1 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v1/margin/maxBorrowable](https://developers.binance.com/docs/margin_trading/borrow-and-repay/Query-Max-Borrow) - Query Max Borrow

```bash
binance-cli margin-trading query-max-borrow --asset "asset_example" --isolated-symbol "isolatedSymbol_example" --recv-window 5000
```

## MarketData

### [GET /sapi/v1/margin/crossMarginCollateralRatio](https://developers.binance.com/docs/margin_trading/market-data/Cross-margin-collateral-ratio) - Cross margin collateral ratio

```bash
binance-cli margin-trading cross-margin-collateral-ratio
```

### [GET /sapi/v1/margin/allPairs](https://developers.binance.com/docs/margin_trading/market-data/Get-All-Cross-Margin-Pairs) - Get All Cross Margin Pairs

```bash
binance-cli margin-trading get-all-cross-margin-pairs --symbol "symbol_example"
```

### [GET /sapi/v1/margin/isolated/allPairs](https://developers.binance.com/docs/margin_trading/market-data/Get-All-Isolated-Margin-Symbol) - Get All Isolated Margin Symbol

```bash
binance-cli margin-trading get-all-isolated-margin-symbol --symbol "symbol_example" --recv-window 5000
```

### [GET /sapi/v1/margin/allAssets](https://developers.binance.com/docs/margin_trading/market-data/Get-All-Margin-Assets) - Get All Margin Assets

```bash
binance-cli margin-trading get-all-margin-assets --asset "asset_example"
```

### [GET /sapi/v1/margin/delist-schedule](https://developers.binance.com/docs/margin_trading/market-data/Get-Delist-Schedule) - Get Delist Schedule

```bash
binance-cli margin-trading get-delist-schedule --recv-window 5000
```

### [GET /sapi/v1/margin/limit-price-pairs](https://developers.binance.com/docs/margin_trading/market-data/Get-Limit-Price-Pairs) - Get Limit Price Pairs

```bash
binance-cli margin-trading get-limit-price-pairs
```

### [GET /sapi/v1/margin/list-schedule](https://developers.binance.com/docs/margin_trading/market-data/Get-list-Schedule) - Get list Schedule

```bash
binance-cli margin-trading get-list-schedule --recv-window 5000
```

### [GET /sapi/v1/margin/risk-based-liquidation-ratio](https://developers.binance.com/docs/margin_trading/market-data/Get-Margin-Asset-Risk-Based-Liquidation-Ratio) - Get Margin Asset Risk-Based Liquidation Ratio

```bash
binance-cli margin-trading get-margin-asset-risk-based-liquidation-ratio
```

### [GET /sapi/v1/margin/restricted-asset](https://developers.binance.com/docs/margin_trading/market-data/Get-Margin-Restricted-Assets) - Get Margin Restricted Assets

```bash
binance-cli margin-trading get-margin-restricted-assets
```

### [GET /sapi/v1/margin/isolatedMarginTier](https://developers.binance.com/docs/margin_trading/market-data/Query-Isolated-Margin-Tier-Data) - Query Isolated Margin Tier Data

```bash
binance-cli margin-trading query-isolated-margin-tier-data --symbol "symbol_example" --tier 1 --recv-window 5000
```

### [GET /sapi/v1/margin/leverageBracket](https://developers.binance.com/docs/margin_trading/market-data/Query-Liability-Coin-Leverage-Bracket-in-Cross-Margin-Pro-Mode) - Query Liability Coin Leverage Bracket in Cross Margin Pro Mode

```bash
binance-cli margin-trading query-liability-coin-leverage-bracket-in-cross-margin-pro-mode
```

### [GET /sapi/v1/margin/available-inventory](https://developers.binance.com/docs/margin_trading/market-data/Query-margin-avaliable-inventory) - Query Margin Available Inventory

```bash
binance-cli margin-trading query-margin-available-inventory --type "type_example"
```

### [GET /sapi/v1/margin/priceIndex](https://developers.binance.com/docs/margin_trading/market-data/Query-Margin-PriceIndex) - Query Margin PriceIndex

```bash
binance-cli margin-trading query-margin-priceindex --symbol "symbol_example"
```

## RiskDataStream

### [DELETE /sapi/v1/margin/listen-key](https://developers.binance.com/docs/margin_trading/risk-data-stream/Close-User-Data-Stream) - Close User Data Stream

```bash
binance-cli margin-trading close-user-data-stream
```

### [PUT /sapi/v1/margin/listen-key](https://developers.binance.com/docs/margin_trading/risk-data-stream/Keepalive-User-Data-Stream) - Keepalive User Data Stream

```bash
binance-cli margin-trading keepalive-user-data-stream --json {}
```

### [POST /sapi/v1/margin/listen-key](https://developers.binance.com/docs/margin_trading/risk-data-stream/Start-User-Data-Stream) - Start User Data Stream

```bash
binance-cli margin-trading start-user-data-stream
```

## Trade

### [POST /sapi/v1/margin/apiKey](https://developers.binance.com/docs/margin_trading/trade/Create-Special-Key-of-Low-Latency-Trading) - Create Special Key(Low-Latency Trading)

```bash
binance-cli margin-trading create-special-key --json {}
```

### [DELETE /sapi/v1/margin/apiKey](https://developers.binance.com/docs/margin_trading/trade/Delete-Special-Key-of-Low-Latency-Trading) - Delete Special Key(Low-Latency Trading)

```bash
binance-cli margin-trading delete-special-key --api-name "apiName_example" --symbol "symbol_example" --recv-window 5000
```

### [PUT /sapi/v1/margin/apiKey/ip](https://developers.binance.com/docs/margin_trading/trade/Edit-ip-for-Special-Key-of-Low-Latency-Trading) - Edit ip for Special Key(Low-Latency Trading)

```bash
binance-cli margin-trading edit-ip-for-special-key --json {}
```

### [GET /sapi/v1/margin/forceLiquidationRec](https://developers.binance.com/docs/margin_trading/trade/Get-Force-Liquidation-Record) - Get Force Liquidation Record

```bash
binance-cli margin-trading get-force-liquidation-record --start-time 1623319461670 --end-time 1641782889000 --isolated-symbol "isolatedSymbol_example" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/margin/exchange-small-liability](https://developers.binance.com/docs/margin_trading/trade/Get-Small-Liability-Exchange-Coin-List) - Get Small Liability Exchange Coin List

```bash
binance-cli margin-trading get-small-liability-exchange-coin-list --recv-window 5000
```

### [GET /sapi/v1/margin/exchange-small-liability-history](https://developers.binance.com/docs/margin_trading/trade/Get-Small-Liability-Exchange-History) - Get Small Liability Exchange History

```bash
binance-cli margin-trading get-small-liability-exchange-history --current 1 --size 10 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [DELETE /sapi/v1/margin/openOrders](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-Cancel-All-Open-Orders) - Margin Account Cancel all Open Orders on a Symbol

```bash
binance-cli margin-trading margin-account-cancel-all-open-orders-on-a-symbol --symbol "symbol_example" --is-isolated "false" --recv-window 5000
```

### [DELETE /sapi/v1/margin/orderList](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-Cancel-OCO) - Margin Account Cancel OCO

```bash
binance-cli margin-trading margin-account-cancel-oco --symbol "symbol_example" --is-isolated "false" --order-list-id 1 --list-client-order-id "1" --new-client-order-id "1" --recv-window 5000
```

### [DELETE /sapi/v1/margin/order](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-Cancel-Order) - Margin Account Cancel Order

```bash
binance-cli margin-trading margin-account-cancel-order --symbol "symbol_example" --is-isolated "false" --order-id 1 --orig-client-order-id "1" --new-client-order-id "1" --recv-window 5000
```

### [POST /sapi/v1/margin/order/oco](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-New-OCO) - Margin Account New OCO

```bash
binance-cli margin-trading margin-account-new-oco --json {}
```

### [POST /sapi/v1/margin/order](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-New-Order) - Margin Account New Order

```bash
binance-cli margin-trading margin-account-new-order --json {}
```

### [POST /sapi/v1/margin/order/oto](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-New-OTO) - Margin Account New OTO

```bash
binance-cli margin-trading margin-account-new-oto --json {}
```

### [POST /sapi/v1/margin/order/otoco](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-New-OTOCO) - Margin Account New OTOCO

```bash
binance-cli margin-trading margin-account-new-otoco --json {}
```

### [POST /sapi/v1/margin/manual-liquidation](https://developers.binance.com/docs/margin_trading/trade/Margin-Manual-Liquidation) - Margin Manual Liquidation

```bash
binance-cli margin-trading margin-manual-liquidation --json {}
```

### [GET /sapi/v1/margin/rateLimit/order](https://developers.binance.com/docs/margin_trading/trade/Query-Current-Margin-Order-Count-Usage) - Query Current Margin Order Count Usage

```bash
binance-cli margin-trading query-current-margin-order-count-usage --is-isolated "false" --symbol "symbol_example" --recv-window 5000
```

### [GET /sapi/v1/margin/allOrderList](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-all-OCO) - Query Margin Account\'s all OCO

```bash
binance-cli margin-trading query-margin-accounts-all-oco --is-isolated "false" --symbol "symbol_example" --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/allOrders](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-All-Orders) - Query Margin Account\'s All Orders

```bash
binance-cli margin-trading query-margin-accounts-all-orders --symbol "symbol_example" --is-isolated "false" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/orderList](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-OCO) - Query Margin Account\'s OCO

```bash
binance-cli margin-trading query-margin-accounts-oco --is-isolated "false" --symbol "symbol_example" --order-list-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /sapi/v1/margin/openOrderList](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-Open-OCO) - Query Margin Account\'s Open OCO

```bash
binance-cli margin-trading query-margin-accounts-open-oco --is-isolated "false" --symbol "symbol_example" --recv-window 5000
```

### [GET /sapi/v1/margin/openOrders](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-Open-Orders) - Query Margin Account\'s Open Orders

```bash
binance-cli margin-trading query-margin-accounts-open-orders --symbol "symbol_example" --is-isolated "false" --recv-window 5000
```

### [GET /sapi/v1/margin/order](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-Order) - Query Margin Account\'s Order

```bash
binance-cli margin-trading query-margin-accounts-order --symbol "symbol_example" --is-isolated "false" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /sapi/v1/margin/myTrades](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Account-Trade-List) - Query Margin Account\'s Trade List

```bash
binance-cli margin-trading query-margin-accounts-trade-list --symbol "symbol_example" --is-isolated "false" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/myPreventedMatches](https://developers.binance.com/docs/margin_trading/trade/Query-Margin-Prevented-Matches) - Query Prevented Matches

```bash
binance-cli margin-trading query-prevented-matches --symbol "symbol_example" --prevented-match-id 1 --order-id 1 --from-prevented-match-id 1 --recv-window 5000 --is-isolated "false"
```

### [GET /sapi/v1/margin/apiKey](https://developers.binance.com/docs/margin_trading/trade/Query-Special-Key-of-Low-Latency-Trading) - Query Special key(Low Latency Trading)

```bash
binance-cli margin-trading query-special-key --symbol "symbol_example" --recv-window 5000
```

### [GET /sapi/v1/margin/api-key-list](https://developers.binance.com/docs/margin_trading/trade/Query-Special-Key-List-of-Low-Latency-Trading) - Query Special key List(Low Latency Trading)

```bash
binance-cli margin-trading query-special-key-list --symbol "symbol_example" --recv-window 5000
```

### [POST /sapi/v1/margin/exchange-small-liability](https://developers.binance.com/docs/margin_trading/trade/Small-Liability-Exchange) - Small Liability Exchange

```bash
binance-cli margin-trading small-liability-exchange --json {}
```

## Transfer

### [GET /sapi/v1/margin/transfer](https://developers.binance.com/docs/margin_trading/transfer/Get-Cross-Margin-Transfer-History) - Get Cross Margin Transfer History

```bash
binance-cli margin-trading get-cross-margin-transfer-history --asset "asset_example" --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --isolated-symbol "isolatedSymbol_example" --recv-window 5000
```

### [GET /sapi/v1/margin/maxTransferable](https://developers.binance.com/docs/margin_trading/transfer/Query-Max-Transfer-Out-Amount) - Query Max Transfer-Out Amount

```bash
binance-cli margin-trading query-max-transfer-out-amount --asset "asset_example" --isolated-symbol "isolatedSymbol_example" --recv-window 5000
```
