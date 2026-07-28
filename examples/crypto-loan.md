## FlexibleRate

### [GET /sapi/v2/loan/flexible/repay/rate](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#check-collateral-repay-rate) - Check Collateral Flexible Repay Rate (USER_DATA)

```bash
binance-cli crypto-loan check-collateral-repay-rate --loan-coin BUSD --collateral-coin BNB --recv-window 5000
```

### [POST /sapi/v2/loan/flexible/adjust/ltv](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#flexible-loan-adjust-ltv) - Flexible Loan Adjust LTV (TRADE)

```bash
binance-cli crypto-loan flexible-loan-adjust-ltv --loan-coin BUSD --collateral-coin BNB --adjustment-amount 1 --direction ADDITIONAL --recv-window 5000
```

### [POST /sapi/v2/loan/flexible/borrow](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#flexible-loan-borrow) - Flexible Loan Borrow (TRADE)

```bash
binance-cli crypto-loan flexible-loan-borrow --loan-coin BUSD --collateral-coin BNB --loan-amount 1 --collateral-amount 1 --recv-window 5000
```

### [POST /sapi/v2/loan/flexible/repay](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#flexible-loan-repay) - Flexible Loan Repay (TRADE)

```bash
binance-cli crypto-loan flexible-loan-repay --loan-coin BUSD --collateral-coin BNB --repay-amount 1 --collateral-return true --full-repayment false --repayment-type 789 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/loanable/data](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-assets-data) - Get Flexible Loan Assets Data (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-assets-data --loan-coin BUSD --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/borrow/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-borrow-history) - Get Flexible Loan Borrow History (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-borrow-history --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/collateral/data](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-collateral-assets-data) - Get Flexible Loan Collateral Assets Data (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-collateral-assets-data --collateral-coin BNB --recv-window 5000
```

### [GET /sapi/v2/loan/interestRateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-interest-rate-history) - Get Flexible Loan Interest Rate History (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-interest-rate-history --coin USDT --recv-window 5000 --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10
```

### [GET /sapi/v2/loan/flexible/liquidation/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-liquidation-history) - Get Flexible Loan Liquidation History (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-liquidation-history --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/ltv/adjustment/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-ltv-adjustment-history) - Get Flexible Loan LTV Adjustment History (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-ltv-adjustment-history --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/ongoing/orders](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-ongoing-orders) - Get Flexible Loan Ongoing Orders (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-ongoing-orders --loan-coin BUSD --collateral-coin BNB --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/repay/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/flexible-rate#get-flexible-loan-repayment-history) - Get Flexible Loan Repayment History (USER_DATA)

```bash
binance-cli crypto-loan get-flexible-loan-repayment-history --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

## StableRate

### [GET /sapi/v1/loan/income](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/stable-rate#get-crypto-loans-income-history) - Get Crypto Loans Income History (USER_DATA)

```bash
binance-cli crypto-loan get-crypto-loans-income-history --asset BUSD --rtype borrowIn --start-time 1623319461670 --end-time 1641782889000 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/borrow/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/stable-rate#get-loan-borrow-history) - Get Loan Borrow History (USER_DATA)

```bash
binance-cli crypto-loan get-loan-borrow-history --order-id 1 --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/ltv/adjustment/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/stable-rate#get-loan-ltv-adjustment-history) - Get Loan LTV Adjustment History (USER_DATA)

```bash
binance-cli crypto-loan get-loan-ltv-adjustment-history --order-id 1 --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/repay/history](https://developers.binance.com/en/docs/catalog/investment-and-services-crypto-loan/api/rest-api/stable-rate#get-loan-repayment-history) - Get Loan Repayment History (USER_DATA)

```bash
binance-cli crypto-loan get-loan-repayment-history --order-id 1 --loan-coin BUSD --collateral-coin BNB --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```
