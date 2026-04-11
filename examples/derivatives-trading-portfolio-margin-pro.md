## Account

### [POST /sapi/v1/portfolio/bnb-transfer](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/BNB-transfer) - BNB transfer

```bash
binance-cli derivatives-portfolio-margin-pro bnb-transfer --json {}
```

### [POST /sapi/v1/portfolio/repay-futures-switch](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Change-Auto-repay-futures-Status) - Change Auto-repay-futures Status

```bash
binance-cli derivatives-portfolio-margin-pro change-auto-repay-futures-status --json {}
```

### [POST /sapi/v1/portfolio/auto-collection](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Fund-Auto-collection) - Fund Auto-collection

```bash
binance-cli derivatives-portfolio-margin-pro fund-auto-collection --json {}
```

### [POST /sapi/v1/portfolio/asset-collection](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Fund-Collection-by-Asset) - Fund Collection by Asset

```bash
binance-cli derivatives-portfolio-margin-pro fund-collection-by-asset --json {}
```

### [GET /sapi/v1/portfolio/repay-futures-switch](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Auto-repay-futures-Status) - Get Auto-repay-futures Status

```bash
binance-cli derivatives-portfolio-margin-pro get-auto-repay-futures-status --recv-window 5000
```

### [GET /sapi/v1/portfolio/delta-mode](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Delta-Mode-Status) - Get Delta Mode Status

```bash
binance-cli derivatives-portfolio-margin-pro get-delta-mode-status --recv-window 5000
```

### [GET /sapi/v1/portfolio/balance](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Classic-Portfolio-Margin-Balance-Info) - Get Portfolio Margin Pro Account Balance

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-account-balance --asset "asset_example" --recv-window 5000
```

### [GET /sapi/v1/portfolio/account](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Classic-Portfolio-Margin-Account-Info) - Get Portfolio Margin Pro Account Info

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-account-info --recv-window 5000
```

### [GET /sapi/v2/portfolio/account](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Classic-Portfolio-Margin-Account-Info-V2) - Get Portfolio Margin Pro SPAN Account Info

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-pro-span-account-info --recv-window 5000
```

### [GET /sapi/v1/portfolio/earn-asset-balance](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Get-Transferable-Earn-Asset-Balance-for-Portfolio-Margin) - Get Transferable Earn Asset Balance for Portfolio Margin

```bash
binance-cli derivatives-portfolio-margin-pro get-transferable-earn-asset-balance-for-portfolio-margin --asset "asset_example" --transfer-type "transferType_example" --recv-window 5000
```

### [POST /sapi/v1/portfolio/repay](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Classic-Portfolio-Margin-Bankruptcy-Loan-Repay) - Portfolio Margin Pro Bankruptcy Loan Repay

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-pro-bankruptcy-loan-repay --json {}
```

### [GET /sapi/v1/portfolio/pmLoan](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Query-Classic-Portfolio-Margin-Bankruptcy-Loan-Amount) - Query Portfolio Margin Pro Bankruptcy Loan Amount

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-bankruptcy-loan-amount --recv-window 5000
```

### [GET /sapi/v1/portfolio/pmloan-history](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Query-Portfolio-Margin-Pro-Bankruptcy-Loan-Repay-History) - Query Portfolio Margin Pro Bankruptcy Loan Repay History

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-bankruptcy-loan-repay-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/portfolio/interest-history](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Query-Classic-Portfolio-Margin-Negative-Balance-Interest-History) - Query Portfolio Margin Pro Negative Balance Interest History

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-pro-negative-balance-interest-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --size 10 --recv-window 5000
```

### [POST /sapi/v1/portfolio/repay-futures-negative-balance](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Repay-futures-Negative-Balance) - Repay futures Negative Balance

```bash
binance-cli derivatives-portfolio-margin-pro repay-futures-negative-balance --json {}
```

### [POST /sapi/v1/portfolio/delta-mode](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Switch-Delta-Mode) - Switch Delta Mode

```bash
binance-cli derivatives-portfolio-margin-pro switch-delta-mode --json {}
```

### [POST /sapi/v1/portfolio/earn-asset-transfer](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/account/Transfer-LDUSDT-Portfolio-Margin) - Transfer LDUSDT/RWUSD for Portfolio Margin

```bash
binance-cli derivatives-portfolio-margin-pro transfer-ldusdt-rwusd-for-portfolio-margin --json {}
```

## MarketData

### [GET /sapi/v1/portfolio/margin-asset-leverage](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/market-data/Get-Portfolio-Margin-Asset-Leverage) - Get Portfolio Margin Asset Leverage

```bash
binance-cli derivatives-portfolio-margin-pro get-portfolio-margin-asset-leverage
```

### [GET /sapi/v1/portfolio/collateralRate](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/market-data/Classic-Portfolio-Margin-Collateral-Rate) - Portfolio Margin Collateral Rate

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-collateral-rate
```

### [GET /sapi/v2/portfolio/collateralRate](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/market-data/Portfolio-Margin-Pro-Tiered-Collateral-Rate) - Portfolio Margin Pro Tiered Collateral Rate

```bash
binance-cli derivatives-portfolio-margin-pro portfolio-margin-pro-tiered-collateral-rate --recv-window 5000
```

### [GET /sapi/v1/portfolio/asset-index-price](https://developers.binance.com/docs/derivatives/portfolio-margin-pro/market-data/Query-Portfolio-Margin-Asset-Index-Price) - Query Portfolio Margin Asset Index Price

```bash
binance-cli derivatives-portfolio-margin-pro query-portfolio-margin-asset-index-price --asset "asset_example"
```
