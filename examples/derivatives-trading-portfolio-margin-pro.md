## Account

### [POST /sapi/v1/portfolio/bnb-transfer](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#bnb-transfer) - BNB transfer (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro bnb-transfer --amount 1.0 --transfer-side TO_UM --recv-window 5000
```

### [POST /sapi/v1/portfolio/repay-futures-switch](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#change-auto-repay-futures-status) - Change Auto-repay-futures Status (TRADE)

```bash
binance-cli derivatives-portfolio-margin-pro change-auto-repay-futures-status --auto-repay TRUE --recv-window 5000
```

### [DELETE /sapi/v1/portfolio/margin-call-level](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#delete-margin-call-level) - Delete Margin Call Level (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro delete-margin-call-level --recv-window 5000
```

### [POST /sapi/v1/portfolio/auto-collection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-auto-collection) - Fund Auto-collection (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro fund-auto-collection --recv-window 5000
```

### [POST /sapi/v1/portfolio/asset-collection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-collection-by-asset) - Fund Collection by Asset (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro fund-collection-by-asset --asset USDT --recv-window 5000
```

### [GET /sapi/v1/portfolio/repay-futures-switch](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-auto-repay-futures-status) - Get Auto-repay-futures Status (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-auto-repay-futures-status --recv-window 5000
```

### [GET /sapi/v1/portfolio/delta-mode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-delta-mode-status) - Get Delta Mode Status (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-delta-mode-status --recv-window 5000
```

### [GET /sapi/v1/portfolio/margin-call-level](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-margin-call-level) - Get Margin Call Level (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-margin-call-level --recv-window 5000
```

### [GET /sapi/v1/portfolio/balance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-balance) - Get Portfolio Margin Pro Account Balance (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-account-balance --asset BTC --recv-window 5000
```

### [GET /sapi/v1/portfolio/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-info) - Get Portfolio Margin Pro Account Info (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-account-info --recv-window 5000
```

### [GET /sapi/v2/portfolio/account](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-span-account-info) - Get Portfolio Margin Pro SPAN Account Info (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-span-account-info --recv-window 5000
```

### [GET /sapi/v1/portfolio/earn-asset-balance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-transferable-earn-asset-balance-for-portfolio-margin) - Get Transferable Earn Asset Balance for Portfolio Margin (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-transferable-earn-asset-balance-for-portfolio-margin --asset LDUSDT --transfer-type EARN_TO_FUTURE --recv-window 5000
```

### [POST /sapi/v1/portfolio/repay](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#portfolio-margin-pro-bankruptcy-loan-repay) - Portfolio Margin Pro Bankruptcy Loan Repay (TRADE)

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-pro-bankruptcy-loan-repay --from SPOT --recv-window 5000
```

### [GET /sapi/v1/portfolio/pmLoan](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-amount) - Query Portfolio Margin Pro Bankruptcy Loan Amount (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-bankruptcy-loan-amount --recv-window 5000
```

### [GET /sapi/v1/portfolio/pmloan-history](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-repay-history) - Query Portfolio Margin Pro Bankruptcy Loan Repay History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-bankruptcy-loan-repay-history --start-time 1623319461670 --end-time 1641782889000 --size 10 --current 1 --recv-window 5000
```

### [GET /sapi/v1/portfolio/interest-history](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-negative-balance-interest-history) - Query Portfolio Margin Pro Negative Balance Interest History (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-negative-balance-interest-history --asset USDT --start-time 1623319461670 --end-time 1641782889000 --size 10 --recv-window 5000
```

### [POST /sapi/v1/portfolio/repay-futures-negative-balance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#repay-futures-negative-balance) - Repay futures Negative Balance (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro repay-futures-negative-balance --from SPOT --recv-window 5000
```

### [POST /sapi/v1/portfolio/margin-call-level](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#set-margin-call-level) - Set Margin Call Level (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro set-margin-call-level --margin-call-level 1.5 --recv-window 5000
```

### [POST /sapi/v1/portfolio/delta-mode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#switch-delta-mode) - Switch Delta Mode (TRADE)

```bash
binance-cli derivatives-portfolio-margin-pro switch-delta-mode --delta-enabled TRUE --recv-window 5000
```

### [POST /sapi/v1/portfolio/earn-asset-transfer](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#transfer-ldusdt-rwusd-for-portfolio-margin) - Transfer LDUSDT/RWUSD for Portfolio Margin (TRADE)

```bash
binance-cli derivatives-portfolio-margin-pro transfer-ldusdt-rwusd-for-portfolio-margin --asset LDUSDT --transfer-type EARN_TO_FUTURE --amount 1 --recv-window 5000
```

## MarketData

### [GET /sapi/v1/portfolio/margin-asset-leverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#get-portfolio-margin-asset-leverage) - Get Portfolio Margin Asset Leverage (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-asset-leverage
```

### [GET /sapi/v1/portfolio/collateralRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-collateral-rate) - Portfolio Margin Collateral Rate (MARKET_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-collateral-rate
```

### [GET /sapi/v2/portfolio/collateralRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-pro-tiered-collateral-rate) - Portfolio Margin Pro Tiered Collateral Rate (USER_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-pro-tiered-collateral-rate --recv-window 5000
```

### [GET /sapi/v1/portfolio/asset-index-price](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#query-portfolio-margin-asset-index-price) - Query Portfolio Margin Asset Index Price (MARKET_DATA)

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-asset-index-price --asset BTC
```
