## FlexibleRate

### [GET /sapi/v2/loan/flexible/repay/rate](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Check-Collateral-Repay-Rate) - Check Collateral Repay Rate

```bash
binance-cli crypto-loan check-collateral-repay-rate --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --recv-window 5000
```

### [POST /sapi/v2/loan/flexible/adjust/ltv](https://developers.binance.com/docs/crypto_loan/flexible-rate/trade/Flexible-Loan-Adjust-LTV) - Flexible Loan Adjust LTV

```bash
binance-cli crypto-loan flexible-loan-adjust-ltv --json {}
```

### [POST /sapi/v2/loan/flexible/borrow](https://developers.binance.com/docs/crypto_loan/flexible-rate/trade/Flexible-Loan-Borrow) - Flexible Loan Borrow

```bash
binance-cli crypto-loan flexible-loan-borrow --json {}
```

### [POST /sapi/v2/loan/flexible/repay](https://developers.binance.com/docs/crypto_loan/flexible-rate/trade/Flexible-Loan-Repay) - Flexible Loan Repay

```bash
binance-cli crypto-loan flexible-loan-repay --json {}
```

### [GET /sapi/v2/loan/flexible/loanable/data](https://developers.binance.com/docs/crypto_loan/flexible-rate/market-data/Get-Flexible-Loan-Assets-Data) - Get Flexible Loan Assets Data

```bash
binance-cli crypto-loan get-flexible-loan-assets-data --loan-coin "loanCoin_example" --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/borrow/history](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Get-Flexible-Loan-Borrow-History) - Get Flexible Loan Borrow History

```bash
binance-cli crypto-loan get-flexible-loan-borrow-history --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/collateral/data](https://developers.binance.com/docs/crypto_loan/flexible-rate/market-data/Get-Flexible-Loan-Collateral-Assets-Data) - Get Flexible Loan Collateral Assets Data

```bash
binance-cli crypto-loan get-flexible-loan-collateral-assets-data --collateral-coin "collateralCoin_example" --recv-window 5000
```

### [GET /sapi/v2/loan/interestRateHistory](https://developers.binance.com/docs/crypto_loan/flexible-rate/market-data/Get-Flexible-Loan-Interest-Rate-History) - Get Flexible Loan Interest Rate History

```bash
binance-cli crypto-loan get-flexible-loan-interest-rate-history --coin "coin_example" --recv-window 5000 --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10
```

### [GET /sapi/v2/loan/flexible/liquidation/history](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Get-Flexible-Loan-Liquidation-History) - Get Flexible Loan Liquidation History

```bash
binance-cli crypto-loan get-flexible-loan-liquidation-history --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/ltv/adjustment/history](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Get-Flexible-Loan-LTV-Adjustment-History) - Get Flexible Loan LTV Adjustment History

```bash
binance-cli crypto-loan get-flexible-loan-ltv-adjustment-history --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/ongoing/orders](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Get-Flexible-Loan-Ongoing-Orders) - Get Flexible Loan Ongoing Orders

```bash
binance-cli crypto-loan get-flexible-loan-ongoing-orders --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v2/loan/flexible/repay/history](https://developers.binance.com/docs/crypto_loan/flexible-rate/user-information/Get-Flexible-Loan-Repayment-History) - Get Flexible Loan Repayment History

```bash
binance-cli crypto-loan get-flexible-loan-repayment-history --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

## StableRate

### [GET /sapi/v1/loan/repay/collateral/rate](https://developers.binance.com/docs/crypto_loan/stable-rate/market-data/Check-Collateral-Repay-Rate) - Check Collateral Repay Rate

```bash
binance-cli crypto-loan check-collateral-repay-rate-stable-rate --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --repay-amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/loan/income](https://developers.binance.com/docs/crypto_loan/stable-rate/market-data/Get-Crypto-Loans-Income-History) - Get Crypto Loans Income History

```bash
binance-cli crypto-loan get-crypto-loans-income-history --asset "asset_example" --type "0" --start-time 1623319461670 --end-time 1641782889000 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/borrow/history](https://developers.binance.com/docs/crypto_loan/stable-rate/user-information/Get-Loan-Borrow-History) - Get Loan Borrow History

```bash
binance-cli crypto-loan get-loan-borrow-history --order-id 1 --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/ltv/adjustment/history](https://developers.binance.com/docs/crypto_loan/stable-rate/user-information/Get-Loan-LTV-Adjustment-History) - Get Loan LTV Adjustment History

```bash
binance-cli crypto-loan get-loan-ltv-adjustment-history --order-id 1 --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/repay/history](https://developers.binance.com/docs/crypto_loan/stable-rate/user-information/Get-Loan-Repayment-History) - Get Loan Repayment History

```bash
binance-cli crypto-loan get-loan-repayment-history --order-id 1 --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```
