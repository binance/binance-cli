## MarketData

### [GET /sapi/v1/dci/product/list](https://developers.binance.com/docs/advanced_earn/dual-investment/market-data/Get-Dual-Investment-product-list) - Get Dual Investment product list

```bash
binance-cli dual-investment get-dual-investment-product-list --option-type "optionType_example" --exercised-coin "exercisedCoin_example" --invest-coin "investCoin_example" --page-size 10 --page-index 1 --recv-window 5000
```

## Trade

### [POST /sapi/v1/dci/product/auto_compound/edit-status](https://developers.binance.com/docs/advanced_earn/dual-investment/trade/Change-Auto-Compound-status) - Change Auto-Compound status

```bash
binance-cli dual-investment change-auto-compound-status --json {}
```

### [GET /sapi/v1/dci/product/accounts](https://developers.binance.com/docs/advanced_earn/dual-investment/trade/Check-Dual-Investment-accounts) - Check Dual Investment accounts

```bash
binance-cli dual-investment check-dual-investment-accounts --recv-window 5000
```

### [GET /sapi/v1/dci/product/positions](https://developers.binance.com/docs/advanced_earn/dual-investment/trade/Get-Dual-Investment-positions) - Get Dual Investment positions

```bash
binance-cli dual-investment get-dual-investment-positions --status "status_example" --page-size 10 --page-index 1 --recv-window 5000
```

### [POST /sapi/v1/dci/product/subscribe](https://developers.binance.com/docs/advanced_earn/dual-investment/trade/Subscribe-Dual-Investment-products) - Subscribe Dual Investment products

```bash
binance-cli dual-investment subscribe-dual-investment-products --json {}
```
