## Account

### [POST /sapi/v1/margin/max-leverage](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#adjust-cross-margin-max-leverage) - Adjust cross margin max leverage (USER_DATA)

```bash
binance-cli margin-trading adjust-cross-margin-max-leverage --max-leverage 3
```

### [DELETE /sapi/v1/margin/isolated/account](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#disable-isolated-margin-account) - Disable Isolated Margin Account (TRADE)

```bash
binance-cli margin-trading disable-isolated-margin-account --symbol BTCUSDT --recv-window 5000
```

### [POST /sapi/v1/margin/isolated/account](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#enable-isolated-margin-account) - Enable Isolated Margin Account (TRADE)

```bash
binance-cli margin-trading enable-isolated-margin-account --symbol BTCUSDT --recv-window 5000
```

### [GET /sapi/v1/bnbBurn](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-bnb-burn-status) - Get BNB Burn Status (USER_DATA)

```bash
binance-cli margin-trading get-bnb-burn-status --recv-window 5000
```

### [GET /sapi/v1/margin/tradeCoeff](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-summary-of-margin-account) - Get Summary of Margin account (USER_DATA)

```bash
binance-cli margin-trading get-summary-of-margin-account --recv-window 5000
```

### [GET /sapi/v1/margin/capital-flow](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-isolated-margin-capital-flow) - Query Cross Isolated Margin Capital Flow (USER_DATA)

```bash
binance-cli margin-trading query-cross-isolated-margin-capital-flow --asset USDT --symbol BTCUSDT --rtype TRANSFER --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/account](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-account-details) - Query Cross Margin Account Details (USER_DATA)

```bash
binance-cli margin-trading query-cross-margin-account-details --recv-window 5000
```

### [GET /sapi/v1/margin/crossMarginData](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-fee-data) - Query Cross Margin Fee Data (USER_DATA)

```bash
binance-cli margin-trading query-cross-margin-fee-data --vip-level 1 --coin BTC --recv-window 5000
```

### [GET /sapi/v1/margin/isolated/accountLimit](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-enabled-isolated-margin-account-limit) - Query Enabled Isolated Margin Account Limit (USER_DATA)

```bash
binance-cli margin-trading query-enabled-isolated-margin-account-limit --recv-window 5000
```

### [GET /sapi/v1/margin/isolated/account](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-account-info) - Query Isolated Margin Account Info (USER_DATA)

```bash
binance-cli margin-trading query-isolated-margin-account-info --symbols BTCUSDT,BNBUSDT,ADAUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/isolatedMarginData](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-fee-data) - Query Isolated Margin Fee Data (USER_DATA)

```bash
binance-cli margin-trading query-isolated-margin-fee-data --vip-level 1 --symbol BTCUSDT --recv-window 5000
```

## BorrowRepay

### [GET /sapi/v1/margin/next-hourly-interest-rate](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-future-hourly-interest-rate) - Get future hourly interest rate (USER_DATA)

```bash
binance-cli margin-trading get-future-hourly-interest-rate --assets BTC,ETH --is-isolated TRUE
```

### [GET /sapi/v1/margin/interestHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-interest-history) - Get Interest History (USER_DATA)

```bash
binance-cli margin-trading get-interest-history --asset USDT --isolated-symbol BNBUSDT --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/margin/borrow-repay](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#margin-account-borrow-repay) - Margin account borrow/repay (USER_DATA)

```bash
binance-cli margin-trading margin-account-borrow-repay --asset USDT --is-isolated TRUE --amount 1.0 --rtype BORROW --symbol BTCUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/borrow-repay](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-borrow-repay-records-in-margin-account) - Query borrow/repay records in Margin account (USER_DATA)

```bash
binance-cli margin-trading query-borrow-repay-records-in-margin-account --rtype BORROW --asset BNB --isolated-symbol BNBUSDT --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/margin/interestRateHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-margin-interest-rate-history) - Query Margin Interest Rate History (USER_DATA)

```bash
binance-cli margin-trading query-margin-interest-rate-history --asset BTC --vip-level 1 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v1/margin/maxBorrowable](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-max-borrow) - Query Max Borrow (USER_DATA)

```bash
binance-cli margin-trading query-max-borrow --asset BTC --isolated-symbol BTCUSDT --recv-window 5000
```

## MarketData

### [GET /sapi/v1/margin/crossMarginCollateralRatio](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#cross-margin-collateral-ratio) - Cross margin collateral ratio (MARKET_DATA)

```bash
binance-cli margin-trading cross-margin-collateral-ratio
```

### [GET /sapi/v1/margin/allPairs](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-cross-margin-pairs) - Get All Cross Margin Pairs (MARKET_DATA)

```bash
binance-cli margin-trading get-all-cross-margin-pairs --symbol BNBBTC
```

### [GET /sapi/v1/margin/isolated/allPairs](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-isolated-margin-symbol) - Get All Isolated Margin Symbol (MARKET_DATA)

```bash
binance-cli margin-trading get-all-isolated-margin-symbol --symbol BNBBTC --recv-window 5000
```

### [GET /sapi/v1/margin/allAssets](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-margin-assets) - Get All Margin Assets (MARKET_DATA)

```bash
binance-cli margin-trading get-all-margin-assets --asset USDC
```

### [GET /sapi/v1/margin/delist-schedule](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-delist-schedule) - Get Delist Schedule (MARKET_DATA)

```bash
binance-cli margin-trading get-delist-schedule --recv-window 5000
```

### [GET /sapi/v1/margin/limit-price-pairs](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-limit-price-pairs) - Get Limit Price Pairs (MARKET_DATA)

```bash
binance-cli margin-trading get-limit-price-pairs
```

### [GET /sapi/v1/margin/list-schedule](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-list-schedule) - Get list Schedule (MARKET_DATA)

```bash
binance-cli margin-trading get-list-schedule --recv-window 5000
```

### [GET /sapi/v1/margin/risk-based-liquidation-ratio](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-asset-risk-based-liquidation-ratio) - Get Margin Asset Risk-Based Liquidation Ratio (MARKET_DATA)

```bash
binance-cli margin-trading get-margin-asset-risk-based-liquidation-ratio
```

### [GET /sapi/v1/margin/restricted-asset](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-restricted-assets) - Get Margin Restricted Assets (MARKET_DATA)

```bash
binance-cli margin-trading get-margin-restricted-assets
```

### [GET /sapi/v1/margin/isolatedMarginTier](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-isolated-margin-tier-data) - Query Isolated Margin Tier Data (USER_DATA)

```bash
binance-cli margin-trading query-isolated-margin-tier-data --symbol BTCUSDT --tier 1 --recv-window 5000
```

### [GET /sapi/v1/margin/leverageBracket](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-liability-coin-leverage-bracket-in-cross-margin-pro-mode) - Query Liability Coin Leverage Bracket in Cross Margin Pro Mode (MARKET_DATA)

```bash
binance-cli margin-trading query-liability-coin-leverage-bracket-in-cross-margin-pro-mode
```

### [GET /sapi/v1/margin/available-inventory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-available-inventory) - Query Margin Available Inventory (USER_DATA)

```bash
binance-cli margin-trading query-margin-available-inventory --rtype MARGIN
```

### [GET /sapi/v1/margin/priceIndex](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-priceindex) - Query Margin PriceIndex (MARKET_DATA)

```bash
binance-cli margin-trading query-margin-priceindex --symbol BNBBTC
```

## Trade

### [POST /sapi/v1/margin/apiKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#create-special-key) - Create Special Key(Low-Latency Trading) (TRADE)

```bash
binance-cli margin-trading create-special-key --api-name apiName --symbol BTCUSDT --ip 69.210.67.14,69.210.67.15 --public-key publicKey --permission-mode READ --recv-window 5000
```

### [DELETE /sapi/v1/margin/apiKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#delete-special-key) - Delete Special Key(Low-Latency Trading) (TRADE)

```bash
binance-cli margin-trading delete-special-key --api-name apiName --symbol BTCUSDT --recv-window 5000
```

### [PUT /sapi/v1/margin/apiKey/ip](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#edit-ip-for-special-key) - Edit ip for Special Key(Low-Latency Trading) (TRADE)

```bash
binance-cli margin-trading edit-ip-for-special-key --ip 24.156.99.202 --symbol BTCUSDT --recv-window 5000
```

### [POST /sapi/v1/margin/exit-special-key-mode](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#exit-special-key-mode) - Exit Special Key Mode (TRADE)

```bash
binance-cli margin-trading exit-special-key-mode --recv-window 5000
```

### [GET /sapi/v1/margin/forceLiquidationRec](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-force-liquidation-record) - Get Force Liquidation Record (USER_DATA)

```bash
binance-cli margin-trading get-force-liquidation-record --start-time 1623319461670 --end-time 1641782889000 --isolated-symbol BTCUSDT --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/margin/exchange-small-liability](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-coin-list) - Get Small Liability Exchange Coin List (USER_DATA)

```bash
binance-cli margin-trading get-small-liability-exchange-coin-list --recv-window 5000
```

### [GET /sapi/v1/margin/exchange-small-liability-history](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-history) - Get Small Liability Exchange History (USER_DATA)

```bash
binance-cli margin-trading get-small-liability-exchange-history --current 1 --size 10 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [POST /sapi/v1/margin/liquidation-loan/repay](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#liquidation-loan-repay) - Liquidation Loan Repay (MARGIN)

```bash
binance-cli margin-trading liquidation-loan-repay --asset USDT --amount 300.00 --recv-window 5000
```

### [DELETE /sapi/v1/margin/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-all-open-orders-on-asymbol) - Margin Account Cancel all Open Orders on a Symbol (TRADE)

```bash
binance-cli margin-trading margin-account-cancel-all-open-orders-on-a-symbol --symbol BTCUSDT --is-isolated FALSE --recv-window 5000
```

### [DELETE /sapi/v1/margin/orderList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-oco) - Margin Account Cancel OCO (TRADE)

```bash
binance-cli margin-trading margin-account-cancel-oco --symbol BTCUSDT --is-isolated FALSE --order-list-id 1 --list-client-order-id 1 --new-client-order-id 1 --recv-window 5000
```

### [DELETE /sapi/v1/margin/order](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-order) - Margin Account Cancel Order (TRADE)

```bash
binance-cli margin-trading margin-account-cancel-order --symbol LTCBTC --is-isolated FALSE --order-id 1 --orig-client-order-id 1 --new-client-order-id 1 --recv-window 5000
```

### [POST /sapi/v1/margin/order/oco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oco) - Margin Account New OCO (TRADE)

```bash
binance-cli margin-trading margin-account-new-oco --symbol LTCBTC --side BUY --quantity 1.0 --price 1.0 --stop-price 1.0 --is-isolated FALSE --list-client-order-id 1 --limit-client-order-id 1 --limit-iceberg-qty 1.0 --stop-client-order-id 1 --stop-limit-price 1.0 --stop-iceberg-qty 1.0 --stop-limit-time-in-force GTC --new-order-resp-type ACK --side-effect-type NO_SIDE_EFFECT --self-trade-prevention-mode NONE --auto-repay-at-cancel false --recv-window 5000
```

### [POST /sapi/v1/margin/order](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-order) - Margin Account New Order (TRADE)

```bash
binance-cli margin-trading margin-account-new-order --symbol BTCUSDT --side BUY --rtype LIMIT --is-isolated FALSE --quantity 1.0 --quote-order-qty 1.0 --price 1.0 --stop-price 1.0 --new-client-order-id 1 --iceberg-qty 1.0 --new-order-resp-type ACK --side-effect-type NO_SIDE_EFFECT --time-in-force GTC --self-trade-prevention-mode NONE --trailing-delta 100 --auto-repay-at-cancel true --recv-window 5000
```

### [POST /sapi/v1/margin/order/oto](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oto) - Margin Account New OTO (TRADE)

```bash
binance-cli margin-trading margin-account-new-oto --symbol BTCUSDT --working-type LIMIT --working-side SELL --working-price 1.0 --working-quantity 1.0 --working-iceberg-qty 1.0 --pending-type LIMIT --pending-side BUY --pending-quantity 1.0 --is-isolated FALSE --list-client-order-id 1 --new-order-resp-type ACK --side-effect-type NO_SIDE_EFFECT --self-trade-prevention-mode NONE --auto-repay-at-cancel true --working-client-order-id 1 --working-time-in-force GTC --pending-client-order-id 1 --pending-price 1.0 --pending-stop-price 1.0 --pending-trailing-delta 1.0 --pending-iceberg-qty 1.0 --pending-time-in-force GTC
```

### [POST /sapi/v1/margin/order/otoco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-otoco) - Margin Account New OTOCO (TRADE)

```bash
binance-cli margin-trading margin-account-new-otoco --symbol BTCUSDT --working-type LIMIT --working-side SELL --working-price 1.0 --working-quantity 1.0 --pending-side BUY --pending-quantity 1.0 --pending-above-type LIMIT_MAKER --is-isolated FALSE --side-effect-type NO_SIDE_EFFECT --auto-repay-at-cancel true --list-client-order-id 1 --new-order-resp-type ACK --self-trade-prevention-mode NONE --working-client-order-id 1 --working-iceberg-qty 1.0 --working-time-in-force FOK --pending-above-client-order-id 1 --pending-above-price 1.0 --pending-above-stop-price 1.0 --pending-above-trailing-delta 1.0 --pending-above-iceberg-qty 1.0 --pending-above-time-in-force FOK --pending-below-type LIMIT_MAKER --pending-below-client-order-id 1 --pending-below-price 1.0 --pending-below-stop-price 1.0 --pending-below-trailing-delta 1.0 --pending-below-iceberg-qty 1.0 --pending-below-time-in-force FOK
```

### [POST /sapi/v1/margin/manual-liquidation](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-manual-liquidation) - Margin Manual Liquidation (TRADE)

```bash
binance-cli margin-trading margin-manual-liquidation --rtype MARGIN --symbol ETHUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/rateLimit/order](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-current-margin-order-count-usage) - Query Current Margin Order Count Usage (TRADE)

```bash
binance-cli margin-trading query-current-margin-order-count-usage --is-isolated FALSE --symbol BTCUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/liquidation-loan](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan) - Query Liquidation Loan (USER_DATA)

```bash
binance-cli margin-trading query-liquidation-loan --recv-window 5000
```

### [GET /sapi/v1/margin/liquidation-loan/repay-history](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan-repay-history) - Query Liquidation Loan Repay History (USER_DATA)

```bash
binance-cli margin-trading query-liquidation-loan-repay-history --start-time 1714492800000 --end-time 1714579200000 --current 1 --size 50 --recv-window 5000
```

### [GET /sapi/v1/margin/allOrderList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-oco) - Query Margin Account's all OCO (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-all-oco --is-isolated FALSE --symbol LTCBTC --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /sapi/v1/margin/allOrders](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-orders) - Query Margin Account's All Orders (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-all-orders --symbol BNBBTC --is-isolated FALSE --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /sapi/v1/margin/orderList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-oco) - Query Margin Account's OCO (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-oco --is-isolated FALSE --symbol LTCBTC --order-list-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /sapi/v1/margin/openOrderList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-oco) - Query Margin Account's Open OCO (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-open-oco --is-isolated FALSE --symbol LTCBTC --recv-window 5000
```

### [GET /sapi/v1/margin/openOrders](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-orders) - Query Margin Account's Open Orders (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-open-orders --symbol BNBBTC --is-isolated FALSE --recv-window 5000
```

### [GET /sapi/v1/margin/order](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-order) - Query Margin Account's Order (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-order --symbol BNBBTC --is-isolated FALSE --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /sapi/v1/margin/myTrades](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-trade-list) - Query Margin Account's Trade List (USER_DATA)

```bash
binance-cli margin-trading query-margin-accounts-trade-list --symbol BNBBTC --is-isolated FALSE --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [GET /sapi/v1/margin/myPreventedMatches](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-prevented-matches) - Query Prevented Matches (USER_DATA)

```bash
binance-cli margin-trading query-prevented-matches --symbol BTCUSDT --prevented-match-id 1 --order-id 1 --from-prevented-match-id 1 --is-isolated FALSE --recv-window 5000
```

### [GET /sapi/v1/margin/apiKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key) - Query Special key(Low Latency Trading) (TRADE)

```bash
binance-cli margin-trading query-special-key --symbol BTCUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/api-key-list](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key-list) - Query Special key List(Low Latency Trading) (TRADE)

```bash
binance-cli margin-trading query-special-key-list --symbol BTCUSDT --recv-window 5000
```

### [POST /sapi/v1/margin/exchange-small-liability](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#small-liability-exchange) - Small Liability Exchange (MARGIN)

```bash
binance-cli margin-trading small-liability-exchange --asset-names BTC,ETH --recv-window 5000
```

## Transfer

### [GET /sapi/v1/margin/transfer](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#get-cross-margin-transfer-history) - Get Cross Margin Transfer History (USER_DATA)

```bash
binance-cli margin-trading get-cross-margin-transfer-history --asset BNB --rtype ROLL_IN --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --isolated-symbol BNBUSDT --recv-window 5000
```

### [GET /sapi/v1/margin/maxTransferable](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#query-max-transfer-out-amount) - Query Max Transfer-Out Amount (USER_DATA)

```bash
binance-cli margin-trading query-max-transfer-out-amount --asset BTC --isolated-symbol BTCUSDT --recv-window 5000
```

## UserDataStream

### [DELETE /sapi/v1/margin/listen-key](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#close-user-data-stream) - Close User Data Stream (USER_STREAM)

```bash
binance-cli margin-trading close-user-data-stream
```

### [PUT /sapi/v1/margin/listen-key](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#keepalive-user-data-stream) - Keepalive User Data Stream (USER_STREAM)

```bash
binance-cli margin-trading keepalive-user-data-stream --listen-key listen_key_example
```

### [POST /sapi/v1/margin/listen-key](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#start-user-data-stream) - Start User Data Stream (USER_STREAM)

```bash
binance-cli margin-trading start-user-data-stream
```
