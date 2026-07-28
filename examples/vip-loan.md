## MarketData

### [GET /sapi/v1/loan/vip/request/interestRate](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/market-data#get-borrow-interest-rate) - Get Borrow Interest Rate (USER_DATA)

```bash
binance-cli vip-loan get-borrow-interest-rate --loan-coin BTC --recv-window 5000
```

### [GET /sapi/v1/loan/vip/collateral/data](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/market-data#get-collateral-asset-data) - Get Collateral Asset Data (USER_DATA)

```bash
binance-cli vip-loan get-collateral-asset-data --collateral-coin BUSD --recv-window 5000
```

### [GET /sapi/v1/loan/vip/loanable/data](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/market-data#get-loanable-assets-data) - Get Loanable Assets Data (USER_DATA)

```bash
binance-cli vip-loan get-loanable-assets-data --loan-coin BUSD --vip-level 1 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/interestRateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/market-data#get-viploan-interest-rate-history) - Get VIP Loan Interest Rate History (USER_DATA)

```bash
binance-cli vip-loan get-vip-loan-interest-rate-history --coin USDT --recv-window 5000 --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10
```

### [GET /sapi/v1/loan/vip/fixed/market](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/market-data#query-viploan-fixed-rate-market) - Query VIP Loan Fixed Rate Market (USER_DATA)

```bash
binance-cli vip-loan query-vip-loan-fixed-rate-market --loan-coin USDT --duration 30 --current 1 --size 10 --recv-window 5000
```

## Trade

### [POST /sapi/v1/loan/vip/borrow](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/trade#vip-loan-borrow) - VIP Loan Borrow (TRADE)

```bash
binance-cli vip-loan vip-loan-borrow --loan-account-id 1 --loan-coin BTC --loan-amount 1.0 --collateral-account-id 12345678,12345678,12345678 --collateral-coin BUSD,USDT,ETH --is-flexible-rate true --loan-term 30 --recv-window 5000
```

### [POST /sapi/v1/loan/vip/fixed/borrow](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/trade#vip-loan-fixed-rate-borrow) - VIP Loan Fixed Rate Borrow (TRADE)

```bash
binance-cli vip-loan vip-loan-fixed-rate-borrow --supply-request 1212:0.12:100;3434:0.13:50 --borrow-coin BUSD --loan-term 30 --borrow-uid 12345678 --collateral-coin BNB,ETH,BTC --collateral-account-id 12345,67890,13579 --auto-repay true --recv-window 5000
```

### [POST /sapi/v1/loan/vip/renew](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/trade#vip-loan-renew) - VIP Loan Renew (TRADE)

```bash
binance-cli vip-loan vip-loan-renew --order-id 1 --loan-term 30 --recv-window 5000
```

### [POST /sapi/v1/loan/vip/repay](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/trade#vip-loan-repay) - VIP Loan Repay (TRADE)

```bash
binance-cli vip-loan vip-loan-repay --order-id 1 --amount 1.0 --recv-window 5000
```

## UserInformation

### [GET /sapi/v1/loan/vip/collateral/account](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/user-information#check-viploan-collateral-account) - Check VIP Loan Collateral Account (USER_DATA)

```bash
binance-cli vip-loan check-vip-loan-collateral-account --order-id 1 --collateral-account-id 1 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/accruedInterest](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/user-information#get-viploan-accrued-interest) - Get VIP Loan Accrued Interest (USER_DATA)

```bash
binance-cli vip-loan get-vip-loan-accrued-interest --order-id 1 --loan-coin BTC --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/ongoing/orders](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/user-information#get-viploan-ongoing-orders) - Get VIP Loan Ongoing Orders (USER_DATA)

```bash
binance-cli vip-loan get-vip-loan-ongoing-orders --order-id 1 --collateral-account-id 1 --loan-coin BUSD --collateral-coin BNB,BTC,ETH --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/repay/history](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/user-information#get-viploan-repayment-history) - Get VIP Loan Repayment History (USER_DATA)

```bash
binance-cli vip-loan get-vip-loan-repayment-history --order-id 1 --loan-coin BUSD --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/request/data](https://developers.binance.com/en/docs/catalog/investment-and-services-vip-loan/api/rest-api/user-information#query-application-status) - Query Application Status (USER_DATA)

```bash
binance-cli vip-loan query-application-status --current 1 --limit 10 --recv-window 5000
```
