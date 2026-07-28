## MarketData

### [GET /sapi/v1/dci/product/list](https://developers.binance.com/en/docs/catalog/investment-and-services-dual-investment/api/rest-api/market-data#get-dual-investment-product-list) - Get Dual Investment product list

```bash
binance-cli dual-investment get-dual-investment-product-list --option-type CALL --exercised-coin USDT --invest-coin BNB --page-size 10 --page-index 1 --recv-window 5000
```

## Trade

### [POST /sapi/v1/dci/product/auto_compound/edit-status](https://developers.binance.com/en/docs/catalog/investment-and-services-dual-investment/api/rest-api/trade#change-auto-compound-status) - Change Auto-Compound status (USER_DATA)

```bash
binance-cli dual-investment change-auto-compound-status --position-id 741590 --auto-compound-plan NONE --recv-window 5000
```

### [GET /sapi/v1/dci/product/accounts](https://developers.binance.com/en/docs/catalog/investment-and-services-dual-investment/api/rest-api/trade#check-dual-investment-accounts) - Check Dual Investment accounts (USER_DATA)

```bash
binance-cli dual-investment check-dual-investment-accounts --recv-window 5000
```

### [GET /sapi/v1/dci/product/positions](https://developers.binance.com/en/docs/catalog/investment-and-services-dual-investment/api/rest-api/trade#get-dual-investment-positions) - Get Dual Investment positions (USER_DATA)

```bash
binance-cli dual-investment get-dual-investment-positions --status PENDING --page-size 10 --page-index 1 --recv-window 5000
```

### [POST /sapi/v1/dci/product/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-dual-investment/api/rest-api/trade#subscribe-dual-investment-products) - Subscribe Dual Investment products (USER_DATA)

```bash
binance-cli dual-investment subscribe-dual-investment-products --id 741590 --order-id 8257205859 --deposit-amount 1 --auto-compound-plan NONE --recv-window 5000
```
