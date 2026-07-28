## Account

### [GET /papi/v1/balance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-balance) - Account Balance (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin account-balance --asset USDT --recv-window 5000
```

### [GET /papi/v1/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-information) - Account Information (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin account-information --recv-window 5000
```

### [POST /papi/v1/bnb-transfer](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#bnb-transfer) - BNB transfer (TRADE)

```bash
binance-cli derivatives-portfolio-margin bnb-transfer --amount 1.0 --transfer-side TO_UM --recv-window 5000
```

### [POST /papi/v1/repay-futures-switch](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-auto-repay-futures-status) - Change Auto-repay-futures Status (TRADE)

```bash
binance-cli derivatives-portfolio-margin change-auto-repay-futures-status --auto-repay TRUE --recv-window 5000
```

### [POST /papi/v1/cm/leverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-initial-leverage) - Change CM Initial Leverage (TRADE)

```bash
binance-cli derivatives-portfolio-margin change-cm-initial-leverage --symbol BTCUSD_200925 --leverage 21 --recv-window 5000
```

### [POST /papi/v1/cm/positionSide/dual](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-position-mode) - Change CM Position Mode (TRADE)

```bash
binance-cli derivatives-portfolio-margin change-cm-position-mode --dual-side-position TRUE --recv-window 5000
```

### [POST /papi/v1/um/leverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-initial-leverage) - Change UM Initial Leverage (TRADE)

```bash
binance-cli derivatives-portfolio-margin change-um-initial-leverage --symbol BTCUSDT --leverage 21 --recv-window 5000
```

### [POST /papi/v1/um/positionSide/dual](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-position-mode) - Change UM Position Mode (TRADE)

```bash
binance-cli derivatives-portfolio-margin change-um-position-mode --dual-side-position TRUE --recv-window 5000
```

### [GET /papi/v1/cm/leverageBracket](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#cm-notional-and-leverage-brackets) - CM Notional and Leverage Brackets (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin cm-notional-and-leverage-brackets --symbol BTCUSD_PERP --recv-window 5000
```

### [POST /papi/v1/auto-collection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-auto-collection) - Fund Auto-collection (TRADE)

```bash
binance-cli derivatives-portfolio-margin fund-auto-collection --recv-window 5000
```

### [POST /papi/v1/asset-collection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-collection-by-asset) - Fund Collection by Asset (TRADE)

```bash
binance-cli derivatives-portfolio-margin fund-collection-by-asset --asset BTC --recv-window 5000
```

### [GET /papi/v1/repay-futures-switch](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-auto-repay-futures-status) - Get Auto-repay-futures Status (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-auto-repay-futures-status --recv-window 5000
```

### [GET /papi/v1/cm/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-account-detail) - Get CM Account Detail (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-cm-account-detail --recv-window 5000
```

### [GET /papi/v1/cm/positionSide/dual](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-current-position-mode) - Get CM Current Position Mode (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-cm-current-position-mode --recv-window 5000
```

### [GET /papi/v1/cm/income](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-income-history) - Get CM Income History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-cm-income-history --symbol BTCUSD_200925 --income-type TRANSFER --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/order/asyn](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-order-history) - Get Download Id For UM Futures Order History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/um/trade/asyn](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-trade-history) - Get Download Id For UM Futures Trade History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/um/income/asyn](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-transaction-history) - Get Download Id For UM Futures Transaction History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/margin/marginInterestHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-margin-borrow-loan-interest-history) - Get Margin Borrow/Loan Interest History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-margin-borrow-loan-interest-history --asset USDT --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived TRUE --recv-window 5000
```

### [GET /papi/v1/um/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail) - Get UM Account Detail (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-account-detail --recv-window 5000
```

### [GET /papi/v2/um/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail-v2) - Get UM Account Detail V2 (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-account-detail-v2 --recv-window 5000
```

### [GET /papi/v1/um/positionSide/dual](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-current-position-mode) - Get UM Current Position Mode (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-current-position-mode --recv-window 5000
```

### [GET /papi/v1/um/order/asyn/id](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-order-download-link-by-id) - Get UM Futures Order Download Link by Id (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-futures-order-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /papi/v1/um/trade/asyn/id](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-trade-download-link-by-id) - Get UM Futures Trade Download Link by Id (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-futures-trade-download-link-by-id --download-id 545923594199212032 --recv-window 5000
```

### [GET /papi/v1/um/income/asyn/id](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-transaction-download-link-by-id) - Get UM Futures Transaction Download Link by Id (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-futures-transaction-download-link-by-id --download-id 1 --recv-window 5000
```

### [GET /papi/v1/um/income](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-income-history) - Get UM Income History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-income-history --symbol symbol_example --income-type TRANSFER --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/commissionRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-cm) - Get User Commission Rate for CM (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-user-commission-rate-for-cm --symbol BTCUSD_PERP --recv-window 5000
```

### [GET /papi/v1/um/commissionRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-um) - Get User Commission Rate for UM (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-user-commission-rate-for-um --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/margin/maxBorrowable](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#margin-max-borrow) - Margin Max Borrow (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin margin-max-borrow --asset USDT --recv-window 5000
```

### [GET /papi/v1/um/apiTradingStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#portfolio-margin-um-trading-quantitative-rules-indicators) - Portfolio Margin UM Trading Quantitative Rules Indicators (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin portfolio-margin-um-trading-quantitative-rules-indicators --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/cm/positionRisk](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-cm-position-information) - Query CM Position Information (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-cm-position-information --margin-asset USDT --pair BTCUSD_201225 --recv-window 5000
```

### [GET /papi/v1/margin/marginLoan](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-loan-record) - Query Margin Loan Record (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-loan-record --asset USDT --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived TRUE --recv-window 5000
```

### [GET /papi/v1/margin/maxWithdraw](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-max-withdraw) - Query Margin Max Withdraw (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-max-withdraw --asset USDT --recv-window 5000
```

### [GET /papi/v1/margin/repayLoan](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-repay-record) - Query Margin repay Record (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-repay-record --asset USDT --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived TRUE --recv-window 5000
```

### [GET /papi/v1/portfolio/interest-history](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-portfolio-margin-negative-balance-interest-history) - Query Portfolio Margin Negative Balance Interest History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-portfolio-margin-negative-balance-interest-history --asset USDT --start-time 1623319461670 --end-time 1641782889000 --size 10 --recv-window 5000
```

### [GET /papi/v1/um/positionRisk](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-um-position-information) - Query UM Position Information (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-um-position-information --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/portfolio/negative-balance-exchange-record](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-negative-balance-auto-exchange-record) - Query User Negative Balance Auto Exchange Record (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-user-negative-balance-auto-exchange-record --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/rateLimit/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-rate-limit) - Query User Rate Limit (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-user-rate-limit --recv-window 5000
```

### [POST /papi/v1/repay-futures-negative-balance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#repay-futures-negative-balance) - Repay futures Negative Balance (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin repay-futures-negative-balance --recv-window 5000
```

### [GET /papi/v1/um/accountConfig](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-account-configuration) - UM Futures Account Configuration (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin um-futures-account-configuration --recv-window 5000
```

### [GET /papi/v1/um/symbolConfig](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-symbol-configuration) - UM Futures Symbol Configuration (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin um-futures-symbol-configuration --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/um/leverageBracket](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-notional-and-leverage-brackets) - UM Notional and Leverage Brackets (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin um-notional-and-leverage-brackets --symbol ETHUSDT --recv-window 5000
```

## MarketData

### [GET /papi/v1/ping](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/market-data#test-connectivity) - Test Connectivity

```bash
binance-cli derivatives-portfolio-margin test-connectivity
```

## Trade

### [DELETE /papi/v1/cm/conditional/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-conditional-orders) - Cancel All CM Open Conditional Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-all-cm-open-conditional-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /papi/v1/cm/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-orders) - Cancel All CM Open Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-all-cm-open-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /papi/v1/um/algo/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-algo-open-orders) - Cancel All UM Algo Open Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-all-um-algo-open-orders --symbol BNBUSDT --recv-window 5000
```

### [DELETE /papi/v1/um/conditional/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-open-conditional-orders) - Cancel All UM Open Conditional Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-all-um-open-conditional-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /papi/v1/um/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-open-orders) - Cancel All UM Open Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-all-um-open-orders --symbol BTCUSDT --recv-window 5000
```

### [DELETE /papi/v1/cm/conditional/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-conditional-order) - Cancel CM Conditional Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-cm-conditional-order --symbol BTCUSDT --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [DELETE /papi/v1/cm/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-order) - Cancel CM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-cm-order --symbol BTCUSD_200925 --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [DELETE /papi/v1/margin/allOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-all-open-orders-on-asymbol) - Cancel Margin Account All Open Orders on a Symbol (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-all-open-orders-on-a-symbol --symbol BTCUSDT --recv-window 5000
```

### [DELETE /papi/v1/margin/orderList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-oco-orders) - Cancel Margin Account OCO Orders (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-oco-orders --symbol LTCBTC --order-list-id 1 --list-client-order-id 1 --new-client-order-id 1 --recv-window 5000
```

### [DELETE /papi/v1/margin/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-order) - Cancel Margin Account Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-order --symbol LTCBTC --order-id 1 --orig-client-order-id 1 --new-client-order-id 1 --recv-window 5000
```

### [DELETE /papi/v1/um/algo/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-algo-order) - Cancel UM Algo Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-um-algo-order --algo-id 2146760 --client-algo-id 6B2I9XVcJpCjqPAJ4YoFX7 --recv-window 5000
```

### [DELETE /papi/v1/um/conditional/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-conditional-order) - Cancel UM Conditional Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-um-conditional-order --symbol BTCUSDT --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [DELETE /papi/v1/um/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-order) - Cancel UM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin cancel-um-order --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/cm/userTrades](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-account-trade-list) - CM Account Trade List (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin cm-account-trade-list --symbol BTCUSD_200626 --pair BTCUSD --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 50 --recv-window 5000
```

### [GET /papi/v1/cm/adlQuantile](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-position-adl-quantile-estimation) - CM Position ADL Quantile Estimation (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin cm-position-adl-quantile-estimation --symbol BTCUSD_201225 --recv-window 5000
```

### [POST /papi/v1/um/stock/contract](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#futures-tradfi-perps-contract) - Futures TradFi Perps Contract (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin futures-tradfi-perps-contract --recv-window 5000
```

### [GET /papi/v1/um/feeBurn](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#get-um-futures-bnb-burn-status) - Get UM Futures BNB Burn Status (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin get-um-futures-bnb-burn-status --recv-window 5000
```

### [POST /papi/v1/marginLoan](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-borrow) - Margin Account Borrow (MARGIN)

```bash
binance-cli derivatives-portfolio-margin margin-account-borrow --asset USDT --amount 1.0 --recv-window 5000
```

### [POST /papi/v1/margin/order/oco](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-new-oco) - Margin Account New OCO (TRADE)

```bash
binance-cli derivatives-portfolio-margin margin-account-new-oco --symbol LTCBTC --side BUY --quantity 1.0 --price 1.0 --stop-price 1.0 --list-client-order-id 1 --limit-client-order-id 1 --limit-iceberg-qty 1.0 --stop-client-order-id 1 --stop-limit-price 1.0 --stop-iceberg-qty 1.0 --stop-limit-time-in-force GTC --new-order-resp-type ACK --side-effect-type NO_SIDE_EFFECT --recv-window 5000
```

### [POST /papi/v1/repayLoan](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay) - Margin Account Repay (MARGIN)

```bash
binance-cli derivatives-portfolio-margin margin-account-repay --asset USDT --amount 1.0 --recv-window 5000
```

### [POST /papi/v1/margin/repay-debt](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay-debt) - Margin Account Repay Debt (TRADE)

```bash
binance-cli derivatives-portfolio-margin margin-account-repay-debt --asset USDT --amount 1.0 --specify-repay-assets BNB --recv-window 5000
```

### [GET /papi/v1/margin/myTrades](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-trade-list) - Margin Account Trade List (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin margin-account-trade-list --symbol BTCUSDT --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [PUT /papi/v1/cm/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-cm-order) - Modify CM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin modify-cm-order --symbol BTCUSD_PERP --side BUY --quantity 1.0 --price 1.0 --order-id 1 --orig-client-order-id 1 --price-match OPPONENT --modify-id 1 --recv-window 5000
```

### [PUT /papi/v1/um/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-um-order) - Modify UM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin modify-um-order --symbol BTCUSDT --side BUY --quantity 1.0 --price 1.0 --order-id 1 --orig-client-order-id 1 --price-match OPPONENT --modify-id 1 --recv-window 5000
```

### [POST /papi/v1/cm/conditional/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-conditional-order) - New CM Conditional Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-cm-conditional-order --symbol BTCUSDT --side BUY --strategy-type STOP --position-side BOTH --time-in-force GTC --quantity 1.0 --reduce-only true --price 1.0 --working-type MARK_PRICE --price-protect TRUE --new-client-strategy-id 1 --stop-price 1.0 --activation-price 1.0 --callback-rate 1.0 --recv-window 5000
```

### [POST /papi/v1/cm/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-order) - New CM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-cm-order --symbol BTCUSDT --side BUY --rtype LIMIT --position-side BOTH --time-in-force GTC --quantity 1.0 --reduce-only TRUE --price 1.0 --price-match OPPONENT --new-client-order-id 1 --new-order-resp-type ACK --recv-window 5000
```

### [POST /papi/v1/margin/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-margin-order) - New Margin Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-margin-order --symbol BTCUSDT --side BUY --rtype LIMIT --quantity 1.0 --quote-order-qty 1.0 --price 1.0 --stop-price 1.0 --new-client-order-id 1 --new-order-resp-type ACK --iceberg-qty 1.0 --side-effect-type NO_SIDE_EFFECT --time-in-force GTC --self-trade-prevention-mode NONE --auto-repay-at-cancel true --recv-window 5000
```

### [POST /papi/v1/um/algo/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-algo-order) - New UM Algo Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-um-algo-order --algo-type CONDITIONAL --symbol BNBUSDT --side SELL --rtype TAKE_PROFIT --quantity 0.01 --position-side BOTH --time-in-force GTC --price 750.000 --trigger-price 750.000 --working-type CONTRACT_PRICE --price-match NONE --price-protect TRUE --reduce-only TRUE --activate-price 700 --callback-rate 1 --client-algo-id 6B2I9XVcJpCjqPAJ4YoFX7 --new-order-resp-type ACK --self-trade-prevention-mode NONE --good-till-date 0 --recv-window 5000
```

### [POST /papi/v1/um/conditional/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-conditional-order) - New UM Conditional Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-um-conditional-order --symbol BTCUSDT --side BUY --strategy-type STOP --position-side BOTH --time-in-force GTC --quantity 1.0 --reduce-only TRUE --price 1.0 --working-type MARK_PRICE --price-protect TRUE --new-client-strategy-id 1 --stop-price 1.0 --activation-price 1.0 --callback-rate 1.0 --price-match OPPONENT --self-trade-prevention-mode NONE --good-till-date 1770736694138 --recv-window 5000
```

### [POST /papi/v1/um/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-order) - New UM Order (TRADE)

```bash
binance-cli derivatives-portfolio-margin new-um-order --symbol BTCUSDT --side BUY --rtype LIMIT --position-side BOTH --time-in-force GTC --quantity 1.0 --reduce-only TRUE --price 1.0 --new-client-order-id 1 --new-order-resp-type ACK --price-match OPPONENT --self-trade-prevention-mode NONE --good-till-date 1770736694138 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/allOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-conditional-orders) - Query All CM Conditional Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-cm-conditional-orders --symbol BTCUSDT --strategy-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/cm/allOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-orders) - Query All CM Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-cm-orders --symbol BTCUSD_200925 --pair BTCUSD --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-conditional-orders) - Query All Current CM Open Conditional Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-current-cm-open-conditional-orders --symbol BTCUSD --recv-window 5000
```

### [GET /papi/v1/cm/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-orders) - Query All Current CM Open Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-current-cm-open-orders --symbol BTCUSD_200925 --pair BTCUSD --recv-window 5000
```

### [GET /papi/v1/um/algo/openAlgoOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-algo-orders) - Query All Current UM Open Algo Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-current-um-open-algo-orders --algo-type CONDITIONAL --symbol BNBUSDT --algo-id 2146760 --recv-window 5000
```

### [GET /papi/v1/um/conditional/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-conditional-orders) - Query All Current UM Open Conditional Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-current-um-open-conditional-orders --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/um/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-orders) - Query All Current UM Open Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-current-um-open-orders --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/margin/allOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-margin-account-orders) - Query All Margin Account Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-margin-account-orders --symbol BTCUSDT --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/um/conditional/allOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-um-conditional-orders) - Query All UM Conditional Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-um-conditional-orders --symbol BTCUSDT --strategy-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/um/allOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-um-orders) - Query All UM Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-all-um-orders --symbol BTCUSDT --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/orderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-conditional-order-history) - Query CM Conditional Order History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-cm-conditional-order-history --symbol BTCUSDT --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [GET /papi/v1/cm/orderAmendment](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-modify-order-history) - Query CM Modify Order History (TRADE)

```bash
binance-cli derivatives-portfolio-margin query-cm-modify-order-history --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/cm/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-order) - Query CM Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-cm-order --symbol BTCUSD_200925 --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/openOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-conditional-order) - Query Current CM Open Conditional Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-cm-open-conditional-order --symbol BTCUSD_200925 --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [GET /papi/v1/cm/openOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-order) - Query Current CM Open Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-cm-open-order --symbol BTCUSDT --order-id 1917641 --orig-client-order-id abc --recv-window 5000
```

### [GET /papi/v1/margin/openOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-margin-open-order) - Query Current Margin Open Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-margin-open-order --symbol BTCUSDT --recv-window 5000
```

### [GET /papi/v1/um/algo/algoOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-algo-order) - Query Current UM Open Algo Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-um-open-algo-order --algo-id 2146760 --client-algo-id 6B2I9XVcJpCjqPAJ4YoFX7 --recv-window 5000
```

### [GET /papi/v1/um/conditional/openOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-conditional-order) - Query Current UM Open Conditional Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-um-open-conditional-order --symbol BTCUSDT --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [GET /papi/v1/um/openOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-order) - Query Current UM Open Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-current-um-open-order --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/margin/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-account-order) - Query Margin Account Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-account-order --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/margin/allOrderList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-all-oco) - Query Margin Account's all OCO (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-all-oco --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/margin/orderList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-oco) - Query Margin Account's OCO (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-oco --order-list-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/margin/openOrderList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-open-oco) - Query Margin Account's Open OCO (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-open-oco --recv-window 5000
```

### [GET /papi/v1/um/algo/allAlgoOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-algo-order-history) - Query UM Algo Order History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-um-algo-order-history --symbol BNBUSDT --algo-id 2146760 --start-time 1770130294138 --end-time 1770736694138 --limit 500 --recv-window 5000
```

### [GET /papi/v1/um/conditional/orderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-conditional-order-history) - Query UM Conditional Order History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-um-conditional-order-history --symbol BTCUSDT --strategy-id 1 --new-client-strategy-id 1 --recv-window 5000
```

### [GET /papi/v1/um/orderAmendment](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-modify-order-history) - Query UM Modify Order History (TRADE)

```bash
binance-cli derivatives-portfolio-margin query-um-modify-order-history --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/um/order](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-order) - Query UM Order (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-um-order --symbol BTCUSDT --order-id 1 --orig-client-order-id 1 --recv-window 5000
```

### [GET /papi/v1/cm/forceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-cm-force-orders) - Query User's CM Force Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-users-cm-force-orders --symbol BTCUSDT --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [GET /papi/v1/margin/forceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-margin-force-orders) - Query User's Margin Force Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-users-margin-force-orders --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /papi/v1/um/forceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-um-force-orders) - Query User's UM Force Orders (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin query-users-um-force-orders --symbol BTCUSDT --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 500 --recv-window 5000
```

### [POST /papi/v1/um/feeBurn](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#toggle-bnb-burn-on-um-futures-trade) - Toggle BNB Burn On UM Futures Trade (TRADE)

```bash
binance-cli derivatives-portfolio-margin toggle-bnb-burn-on-um-futures-trade --fee-burn TRUE --recv-window 5000
```

### [GET /papi/v1/um/userTrades](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-account-trade-list) - UM Account Trade List (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin um-account-trade-list --symbol BTCUSDT --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 500 --recv-window 5000
```

### [GET /papi/v1/um/adlQuantile](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-position-adl-quantile-estimation) - UM Position ADL Quantile Estimation (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin um-position-adl-quantile-estimation --symbol BTCUSDT --recv-window 5000
```

## UserDataStreams

### [DELETE /papi/v1/listenKey](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#close-user-data-stream) - Close User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-portfolio-margin close-user-data-stream
```

### [PUT /papi/v1/listenKey](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#keepalive-user-data-stream) - Keepalive User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-portfolio-margin keepalive-user-data-stream
```

### [POST /papi/v1/listenKey](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#start-user-data-stream) - Start User Data Stream (USER_STREAM)

```bash
binance-cli derivatives-portfolio-margin start-user-data-stream
```
