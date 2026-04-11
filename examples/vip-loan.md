## MarketData

### [GET /sapi/v1/loan/vip/request/interestRate](https://developers.binance.com/docs/vip_loan/market-data/Get-Borrow-Interest-Rate) - Get Borrow Interest Rate

```bash
binance-cli vip-loan get-borrow-interest-rate --loan-coin "loanCoin_example" --recv-window 5000
```

### [GET /sapi/v1/loan/vip/collateral/data](https://developers.binance.com/docs/vip_loan/market-data/Get-Collateral-Asset-Data) - Get Collateral Asset Data

```bash
binance-cli vip-loan get-collateral-asset-data --collateral-coin "collateralCoin_example" --recv-window 5000
```

### [GET /sapi/v1/loan/vip/loanable/data](https://developers.binance.com/docs/vip_loan/market-data/Get-Loanable-Assets-Data) - Get Loanable Assets Data

```bash
binance-cli vip-loan get-loanable-assets-data --loan-coin "loanCoin_example" --vip-level 1 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/interestRateHistory](https://developers.binance.com/docs/vip_loan/market-data/Get-VIP-Loan-Interest-Rate-History) - Get VIP Loan Interest Rate History

```bash
binance-cli vip-loan get-vip-loan-interest-rate-history --coin "coin_example" --recv-window 5000 --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10
```

## Trade

### [POST /sapi/v1/loan/vip/borrow](https://developers.binance.com/docs/vip_loan/trade/VIP-Loan-Borrow) - VIP Loan Borrow

```bash
binance-cli vip-loan vip-loan-borrow --json {}
```

### [POST /sapi/v1/loan/vip/renew](https://developers.binance.com/docs/vip_loan/trade/VIP-Loan-Renew) - VIP Loan Renew

```bash
binance-cli vip-loan vip-loan-renew --json {}
```

### [POST /sapi/v1/loan/vip/repay](https://developers.binance.com/docs/vip_loan/trade/VIP-Loan-Repay) - VIP Loan Repay

```bash
binance-cli vip-loan vip-loan-repay --json {}
```

## UserInformation

### [GET /sapi/v1/loan/vip/collateral/account](https://developers.binance.com/docs/vip_loan/user-information/Check-Locked-Value-of-VIP-Collateral-Account) - Check VIP Loan Collateral Account

```bash
binance-cli vip-loan check-vip-loan-collateral-account --order-id 1 --collateral-account-id 1 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/accruedInterest](https://developers.binance.com/docs/vip_loan/user-information/Get-VIP-Loan-Accrued-Interest) - Get VIP Loan Accrued Interest

```bash
binance-cli vip-loan get-vip-loan-accrued-interest --order-id 1 --loan-coin "loanCoin_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/ongoing/orders](https://developers.binance.com/docs/vip_loan/user-information/Get-VIP-Loan-Ongoing-Orders) - Get VIP Loan Ongoing Orders

```bash
binance-cli vip-loan get-vip-loan-ongoing-orders --order-id 1 --collateral-account-id 1 --loan-coin "loanCoin_example" --collateral-coin "collateralCoin_example" --current 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/loan/vip/request/data](https://developers.binance.com/docs/vip_loan/user-information/Query-Application-Status) - Query Application Status

```bash
binance-cli vip-loan query-application-status --current 1 --limit 10 --recv-window 5000
```
