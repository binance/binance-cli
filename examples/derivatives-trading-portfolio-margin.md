## Account

### [GET /papi/v1/balance](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Account-Balance) - Account Balance

```bash
binance-cli derivatives-portfolio-margin account-balance --asset "asset_example" --recv-window 5000
```

### [GET /papi/v1/account](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Account-Information) - Account Information

```bash
binance-cli derivatives-portfolio-margin account-information --recv-window 5000
```

### [POST /papi/v1/bnb-transfer](https://developers.binance.com/docs/derivatives/portfolio-margin/account/BNB-transfer) - BNB transfer

```bash
binance-cli derivatives-portfolio-margin bnb-transfer --json {}
```

### [POST /papi/v1/repay-futures-switch](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Change-Auto-repay-futures-Status) - Change Auto-repay-futures Status

```bash
binance-cli derivatives-portfolio-margin change-auto-repay-futures-status --json {}
```

### [POST /papi/v1/cm/leverage](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Change-CM-Initial-Leverage) - Change CM Initial Leverage

```bash
binance-cli derivatives-portfolio-margin change-cm-initial-leverage --json {}
```

### [POST /papi/v1/cm/positionSide/dual](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Change-CM-Position-Mode) - Change CM Position Mode

```bash
binance-cli derivatives-portfolio-margin change-cm-position-mode --json {}
```

### [POST /papi/v1/um/leverage](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Change-UM-Initial-Leverage) - Change UM Initial Leverage

```bash
binance-cli derivatives-portfolio-margin change-um-initial-leverage --json {}
```

### [POST /papi/v1/um/positionSide/dual](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Change-UM-Position-Mode) - Change UM Position Mode

```bash
binance-cli derivatives-portfolio-margin change-um-position-mode --json {}
```

### [GET /papi/v1/cm/leverageBracket](https://developers.binance.com/docs/derivatives/portfolio-margin/account/CM-Notional-and-Leverage-Brackets) - CM Notional and Leverage Brackets

```bash
binance-cli derivatives-portfolio-margin cm-notional-and-leverage-brackets --symbol "symbol_example" --recv-window 5000
```

### [POST /papi/v1/auto-collection](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Fund-Auto-collection) - Fund Auto-collection

```bash
binance-cli derivatives-portfolio-margin fund-auto-collection --json {}
```

### [POST /papi/v1/asset-collection](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Fund-Collection-by-Asset) - Fund Collection by Asset

```bash
binance-cli derivatives-portfolio-margin fund-collection-by-asset --json {}
```

### [GET /papi/v1/repay-futures-switch](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-Auto-repay-futures-Status) - Get Auto-repay-futures Status

```bash
binance-cli derivatives-portfolio-margin get-auto-repay-futures-status --recv-window 5000
```

### [GET /papi/v1/cm/account](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-CM-Account-Detail) - Get CM Account Detail

```bash
binance-cli derivatives-portfolio-margin get-cm-account-detail --recv-window 5000
```

### [GET /papi/v1/cm/positionSide/dual](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-CM-Current-Position-Mode) - Get CM Current Position Mode

```bash
binance-cli derivatives-portfolio-margin get-cm-current-position-mode --recv-window 5000
```

### [GET /papi/v1/cm/income](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-CM-Income-History) - Get CM Income History

```bash
binance-cli derivatives-portfolio-margin get-cm-income-history --symbol "symbol_example" --income-type "incomeType_example" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/order/asyn](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-Download-Id-For-UM-Futures-Order-History) - Get Download Id For UM Futures Order History

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-order-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/um/trade/asyn](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-Download-Id-For-UM-Futures-Trade-History) - Get Download Id For UM Futures Trade History

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-trade-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/um/income/asyn](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-Download-Id-For-UM-Futures-Transaction-History) - Get Download Id For UM Futures Transaction History

```bash
binance-cli derivatives-portfolio-margin get-download-id-for-um-futures-transaction-history --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/margin/marginInterestHistory](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-Margin-BorrowLoan-Interest-History) - Get Margin Borrow/Loan Interest History

```bash
binance-cli derivatives-portfolio-margin get-margin-borrow-loan-interest-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived "" --recv-window 5000
```

### [GET /papi/v1/um/account](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Account-Detail) - Get UM Account Detail

```bash
binance-cli derivatives-portfolio-margin get-um-account-detail --recv-window 5000
```

### [GET /papi/v2/um/account](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Account-Detail-V2) - Get UM Account Detail V2

```bash
binance-cli derivatives-portfolio-margin get-um-account-detail-v2 --recv-window 5000
```

### [GET /papi/v1/um/positionSide/dual](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Current-Position-Mode) - Get UM Current Position Mode

```bash
binance-cli derivatives-portfolio-margin get-um-current-position-mode --recv-window 5000
```

### [GET /papi/v1/um/order/asyn/id](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Futures-Order-Download-Link-by-Id) - Get UM Futures Order Download Link by Id

```bash
binance-cli derivatives-portfolio-margin get-um-futures-order-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /papi/v1/um/trade/asyn/id](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Futures-Trade-Download-Link-by-Id) - Get UM Futures Trade Download Link by Id

```bash
binance-cli derivatives-portfolio-margin get-um-futures-trade-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /papi/v1/um/income/asyn/id](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Futures-Transaction-Download-Link-by-Id) - Get UM Futures Transaction Download Link by Id

```bash
binance-cli derivatives-portfolio-margin get-um-futures-transaction-download-link-by-id --download-id "1" --recv-window 5000
```

### [GET /papi/v1/um/income](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Income-History) - Get UM Income History

```bash
binance-cli derivatives-portfolio-margin get-um-income-history --symbol "symbol_example" --income-type "incomeType_example" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/commissionRate](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-User-Commission-Rate-for-CM) - Get User Commission Rate for CM

```bash
binance-cli derivatives-portfolio-margin get-user-commission-rate-for-cm --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/um/commissionRate](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-User-Commission-Rate-for-UM) - Get User Commission Rate for UM

```bash
binance-cli derivatives-portfolio-margin get-user-commission-rate-for-um --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/margin/maxBorrowable](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Margin-Max-Borrow) - Margin Max Borrow

```bash
binance-cli derivatives-portfolio-margin margin-max-borrow --asset "asset_example" --recv-window 5000
```

### [GET /papi/v1/um/apiTradingStatus](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Portfolio-Margin-UM-Trading-Quantitative-Rules-Indicators) - Portfolio Margin UM Trading Quantitative Rules Indicators

```bash
binance-cli derivatives-portfolio-margin portfolio-margin-um-trading-quantitative-rules-indicators --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/cm/positionRisk](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-CM-Position-Information) - Query CM Position Information

```bash
binance-cli derivatives-portfolio-margin query-cm-position-information --margin-asset "marginAsset_example" --pair "pair_example" --recv-window 5000
```

### [GET /papi/v1/margin/marginLoan](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-Margin-Loan-Record) - Query Margin Loan Record

```bash
binance-cli derivatives-portfolio-margin query-margin-loan-record --asset "asset_example" --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived "" --recv-window 5000
```

### [GET /papi/v1/margin/maxWithdraw](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-Margin-Max-Withdraw) - Query Margin Max Withdraw

```bash
binance-cli derivatives-portfolio-margin query-margin-max-withdraw --asset "asset_example" --recv-window 5000
```

### [GET /papi/v1/margin/repayLoan](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-Margin-repay-Record) - Query Margin repay Record

```bash
binance-cli derivatives-portfolio-margin query-margin-repay-record --asset "asset_example" --tx-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --archived "" --recv-window 5000
```

### [GET /papi/v1/portfolio/interest-history](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-Portfolio-Margin-Negative-Balance-Interest-History) - Query Portfolio Margin Negative Balance Interest History

```bash
binance-cli derivatives-portfolio-margin query-portfolio-margin-negative-balance-interest-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --size 10 --recv-window 5000
```

### [GET /papi/v1/um/positionRisk](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-UM-Position-Information) - Query UM Position Information

```bash
binance-cli derivatives-portfolio-margin query-um-position-information --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/portfolio/negative-balance-exchange-record](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-User-Negative-Balance-Auto-Exchange-Record) - Query User Negative Balance Auto Exchange Record

```bash
binance-cli derivatives-portfolio-margin query-user-negative-balance-auto-exchange-record --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /papi/v1/rateLimit/order](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Query-User-Rate-Limit) - Query User Rate Limit

```bash
binance-cli derivatives-portfolio-margin query-user-rate-limit --recv-window 5000
```

### [POST /papi/v1/repay-futures-negative-balance](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Repay-futures-Negative-Balance) - Repay futures Negative Balance

```bash
binance-cli derivatives-portfolio-margin repay-futures-negative-balance --json {}
```

### [GET /papi/v1/um/accountConfig](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Futures-Account-Config) - UM Futures Account Configuration

```bash
binance-cli derivatives-portfolio-margin um-futures-account-configuration --recv-window 5000
```

### [GET /papi/v1/um/symbolConfig](https://developers.binance.com/docs/derivatives/portfolio-margin/account/Get-UM-Futures-Symbol-Config) - UM Futures Symbol Configuration

```bash
binance-cli derivatives-portfolio-margin um-futures-symbol-configuration --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/um/leverageBracket](https://developers.binance.com/docs/derivatives/portfolio-margin/account/UM-Notional-and-Leverage-Brackets) - UM Notional and Leverage Brackets

```bash
binance-cli derivatives-portfolio-margin um-notional-and-leverage-brackets --symbol "symbol_example" --recv-window 5000
```

## MarketData

### [GET /papi/v1/ping](https://developers.binance.com/docs/derivatives/portfolio-margin/market-data/Test-Connectivity) - Test Connectivity

```bash
binance-cli derivatives-portfolio-margin test-connectivity
```

## Trade

### [DELETE /papi/v1/cm/conditional/allOpenOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-All-CM-Open-Conditional-Orders) - Cancel All CM Open Conditional Orders

```bash
binance-cli derivatives-portfolio-margin cancel-all-cm-open-conditional-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /papi/v1/cm/allOpenOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-All-CM-Open-Orders) - Cancel All CM Open Orders

```bash
binance-cli derivatives-portfolio-margin cancel-all-cm-open-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /papi/v1/um/conditional/allOpenOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-All-UM-Open-Conditional-Orders) - Cancel All UM Open Conditional Orders

```bash
binance-cli derivatives-portfolio-margin cancel-all-um-open-conditional-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /papi/v1/um/allOpenOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-All-UM-Open-Orders) - Cancel All UM Open Orders

```bash
binance-cli derivatives-portfolio-margin cancel-all-um-open-orders --symbol "symbol_example" --recv-window 5000
```

### [DELETE /papi/v1/cm/conditional/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-CM-Conditional-Order) - Cancel CM Conditional Order

```bash
binance-cli derivatives-portfolio-margin cancel-cm-conditional-order --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [DELETE /papi/v1/cm/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-CM-Order) - Cancel CM Order

```bash
binance-cli derivatives-portfolio-margin cancel-cm-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [DELETE /papi/v1/margin/allOpenOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-Margin-Account-All-Open-Orders-on-a-Symbol) - Cancel Margin Account All Open Orders on a Symbol

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-all-open-orders-on-a-symbol --symbol "symbol_example" --recv-window 5000
```

### [DELETE /papi/v1/margin/orderList](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-Margin-Account-OCO-Orders) - Cancel Margin Account OCO Orders

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-oco-orders --symbol "symbol_example" --order-list-id 1 --list-client-order-id "1" --new-client-order-id "1" --recv-window 5000
```

### [DELETE /papi/v1/margin/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-Margin-Account-Order) - Cancel Margin Account Order

```bash
binance-cli derivatives-portfolio-margin cancel-margin-account-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --new-client-order-id "1" --recv-window 5000
```

### [DELETE /papi/v1/um/conditional/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-UM-Conditional-Order) - Cancel UM Conditional Order

```bash
binance-cli derivatives-portfolio-margin cancel-um-conditional-order --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [DELETE /papi/v1/um/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Cancel-UM-Order) - Cancel UM Order

```bash
binance-cli derivatives-portfolio-margin cancel-um-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/cm/userTrades](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/CM-Account-Trade-List) - CM Account Trade List

```bash
binance-cli derivatives-portfolio-margin cm-account-trade-list --symbol "symbol_example" --pair "pair_example" --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/adlQuantile](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/CM-Position-ADL-Quantile-Estimation) - CM Position ADL Quantile Estimation

```bash
binance-cli derivatives-portfolio-margin cm-position-adl-quantile-estimation --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/um/feeBurn](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Get-UM-Futures-BNB-Burn-Status) - Get UM Futures BNB Burn Status

```bash
binance-cli derivatives-portfolio-margin get-um-futures-bnb-burn-status --recv-window 5000
```

### [POST /papi/v1/marginLoan](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Margin-Account-Borrow) - Margin Account Borrow

```bash
binance-cli derivatives-portfolio-margin margin-account-borrow --json {}
```

### [POST /papi/v1/margin/order/oco](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Margin-Account-New-OCO) - Margin Account New OCO

```bash
binance-cli derivatives-portfolio-margin margin-account-new-oco --json {}
```

### [POST /papi/v1/repayLoan](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Margin-Account-Repay) - Margin Account Repay

```bash
binance-cli derivatives-portfolio-margin margin-account-repay --json {}
```

### [POST /papi/v1/margin/repay-debt](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Margin-Account-Repay-Debt) - Margin Account Repay Debt

```bash
binance-cli derivatives-portfolio-margin margin-account-repay-debt --json {}
```

### [GET /papi/v1/margin/myTrades](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Margin-Account-Trade-List) - Margin Account Trade List

```bash
binance-cli derivatives-portfolio-margin margin-account-trade-list --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 100 --recv-window 5000
```

### [PUT /papi/v1/cm/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Modify-CM-Order) - Modify CM Order

```bash
binance-cli derivatives-portfolio-margin modify-cm-order --json {}
```

### [PUT /papi/v1/um/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Modify-UM-Order) - Modify UM Order

```bash
binance-cli derivatives-portfolio-margin modify-um-order --json {}
```

### [POST /papi/v1/cm/conditional/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/New-CM-Conditional-Order) - New CM Conditional Order

```bash
binance-cli derivatives-portfolio-margin new-cm-conditional-order --json {}
```

### [POST /papi/v1/cm/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/New-CM-Order) - New CM Order

```bash
binance-cli derivatives-portfolio-margin new-cm-order --json {}
```

### [POST /papi/v1/margin/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/New-Margin-Order) - New Margin Order

```bash
binance-cli derivatives-portfolio-margin new-margin-order --json {}
```

### [POST /papi/v1/um/conditional/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/New-UM-Conditional-Order) - New UM Conditional Order

```bash
binance-cli derivatives-portfolio-margin new-um-conditional-order --json {}
```

### [POST /papi/v1/um/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/New-UM-Order) - New UM Order

```bash
binance-cli derivatives-portfolio-margin new-um-order --json {}
```

### [GET /papi/v1/cm/conditional/allOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-CM-Conditional-Orders) - Query All CM Conditional Orders

```bash
binance-cli derivatives-portfolio-margin query-all-cm-conditional-orders --symbol "symbol_example" --strategy-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/allOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-CM-Orders) - Query All CM Orders

```bash
binance-cli derivatives-portfolio-margin query-all-cm-orders --symbol "symbol_example" --pair "pair_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/openOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-Current-CM-Open-Conditional-Orders) - Query All Current CM Open Conditional Orders

```bash
binance-cli derivatives-portfolio-margin query-all-current-cm-open-conditional-orders --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/cm/openOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-Current-CM-Open-Orders) - Query All Current CM Open Orders

```bash
binance-cli derivatives-portfolio-margin query-all-current-cm-open-orders --symbol "symbol_example" --pair "pair_example" --recv-window 5000
```

### [GET /papi/v1/um/conditional/openOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-Current-UM-Open-Conditional-Orders) - Query All Current UM Open Conditional Orders

```bash
binance-cli derivatives-portfolio-margin query-all-current-um-open-conditional-orders --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/um/openOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-Current-UM-Open-Orders) - Query All Current UM Open Orders

```bash
binance-cli derivatives-portfolio-margin query-all-current-um-open-orders --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/margin/allOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-Margin-Account-Orders) - Query All Margin Account Orders

```bash
binance-cli derivatives-portfolio-margin query-all-margin-account-orders --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/conditional/allOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-UM-Conditional-Orders) - Query All UM Conditional Orders

```bash
binance-cli derivatives-portfolio-margin query-all-um-conditional-orders --symbol "symbol_example" --strategy-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/allOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-All-UM-Orders) - Query All UM Orders

```bash
binance-cli derivatives-portfolio-margin query-all-um-orders --symbol "symbol_example" --order-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/conditional/orderHistory](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-CM-Conditional-Order-History) - Query CM Conditional Order History

```bash
binance-cli derivatives-portfolio-margin query-cm-conditional-order-history --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [GET /papi/v1/cm/orderAmendment](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-CM-Modify-Order-History) - Query CM Modify Order History

```bash
binance-cli derivatives-portfolio-margin query-cm-modify-order-history --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/cm/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-CM-Order) - Query CM Order

```bash
binance-cli derivatives-portfolio-margin query-cm-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/cm/conditional/openOrder](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Current-CM-Open-Conditional-Order) - Query Current CM Open Conditional Order

```bash
binance-cli derivatives-portfolio-margin query-current-cm-open-conditional-order --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [GET /papi/v1/cm/openOrder](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Current-CM-Open-Order) - Query Current CM Open Order

```bash
binance-cli derivatives-portfolio-margin query-current-cm-open-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/margin/openOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Current-Margin-Open-Order) - Query Current Margin Open Order

```bash
binance-cli derivatives-portfolio-margin query-current-margin-open-order --symbol "symbol_example" --recv-window 5000
```

### [GET /papi/v1/um/conditional/openOrder](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Current-UM-Open-Conditional-Order) - Query Current UM Open Conditional Order

```bash
binance-cli derivatives-portfolio-margin query-current-um-open-conditional-order --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [GET /papi/v1/um/openOrder](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Current-UM-Open-Order) - Query Current UM Open Order

```bash
binance-cli derivatives-portfolio-margin query-current-um-open-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/margin/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Margin-Account-Order) - Query Margin Account Order

```bash
binance-cli derivatives-portfolio-margin query-margin-account-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/margin/allOrderList](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Margin-Account-all-OCO) - Query Margin Account\'s all OCO

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-all-oco --from-id 1 --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/margin/orderList](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Margin-Account-OCO) - Query Margin Account\'s OCO

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-oco --order-list-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/margin/openOrderList](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Margin-Account-Open-OCO) - Query Margin Account\'s Open OCO

```bash
binance-cli derivatives-portfolio-margin query-margin-accounts-open-oco --recv-window 5000
```

### [GET /papi/v1/um/conditional/orderHistory](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-UM-Conditional-Order-History) - Query UM Conditional Order History

```bash
binance-cli derivatives-portfolio-margin query-um-conditional-order-history --symbol "symbol_example" --strategy-id 1 --new-client-strategy-id "1" --recv-window 5000
```

### [GET /papi/v1/um/orderAmendment](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-UM-Modify-Order-History) - Query UM Modify Order History

```bash
binance-cli derivatives-portfolio-margin query-um-modify-order-history --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/order](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-UM-Order) - Query UM Order

```bash
binance-cli derivatives-portfolio-margin query-um-order --symbol "symbol_example" --order-id 1 --orig-client-order-id "1" --recv-window 5000
```

### [GET /papi/v1/cm/forceOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Users-CM-Force-Orders) - Query User\'s CM Force Orders

```bash
binance-cli derivatives-portfolio-margin query-users-cm-force-orders --symbol "symbol_example" --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [GET /papi/v1/margin/forceOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Users-Margin-Force-Orders) - Query User\'s Margin Force Orders

```bash
binance-cli derivatives-portfolio-margin query-users-margin-force-orders --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /papi/v1/um/forceOrders](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Query-Users-UM-Force-Orders) - Query User\'s UM Force Orders

```bash
binance-cli derivatives-portfolio-margin query-users-um-force-orders --symbol "symbol_example" --auto-close-type LIQUIDATION --start-time 1623319461670 --end-time 1641782889000 --limit 100 --recv-window 5000
```

### [POST /papi/v1/um/feeBurn](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/Toggle-BNB-Burn-On-UM-Futures-Trade) - Toggle BNB Burn On UM Futures Trade

```bash
binance-cli derivatives-portfolio-margin toggle-bnb-burn-on-um-futures-trade --json {}
```

### [GET /papi/v1/um/userTrades](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/UM-Account-Trade-List) - UM Account Trade List

```bash
binance-cli derivatives-portfolio-margin um-account-trade-list --symbol "symbol_example" --start-time 1623319461670 --end-time 1641782889000 --from-id 1 --limit 100 --recv-window 5000
```

### [GET /papi/v1/um/adlQuantile](https://developers.binance.com/docs/derivatives/portfolio-margin/trade/UM-Position-ADL-Quantile-Estimation) - UM Position ADL Quantile Estimation

```bash
binance-cli derivatives-portfolio-margin um-position-adl-quantile-estimation --symbol "symbol_example" --recv-window 5000
```

## UserDataStreams

### [DELETE /papi/v1/listenKey](https://developers.binance.com/docs/derivatives/portfolio-margin/user-data-streams/Close-User-Data-Stream) - Close User Data Stream

```bash
binance-cli derivatives-portfolio-margin close-user-data-stream
```

### [PUT /papi/v1/listenKey](https://developers.binance.com/docs/derivatives/portfolio-margin/user-data-streams/Keepalive-User-Data-Stream) - Keepalive User Data Stream

```bash
binance-cli derivatives-portfolio-margin keepalive-user-data-stream
```

### [POST /papi/v1/listenKey](https://developers.binance.com/docs/derivatives/portfolio-margin/user-data-streams/Start-User-Data-Stream) - Start User Data Stream

```bash
binance-cli derivatives-portfolio-margin start-user-data-stream
```
