## Default

### [POST /sapi/v1/fiat/deposit](https://developers.binance.com/en/docs/catalog/investment-and-services-fiat/api/rest-api/~#deposit) - Deposit (TRADE)

```bash
binance-cli fiat deposit --json {}--recv-window 5000
```

### [POST /sapi/v2/fiat/withdraw](https://developers.binance.com/en/docs/catalog/investment-and-services-fiat/api/rest-api/~#fiat-withdraw) - Fiat Withdraw (TRADE)

```bash
binance-cli fiat fiat-withdraw --json {}--recv-window 5000
```

### [GET /sapi/v1/fiat/orders](https://developers.binance.com/en/docs/catalog/investment-and-services-fiat/api/rest-api/~#get-fiat-deposit-withdraw-history) - Get Fiat Deposit/Withdraw History (USER_DATA)

```bash
binance-cli fiat get-fiat-deposit-withdraw-history --transaction-type 0 --begin-time 1641782889000 --end-time 1641782889000 --page 1 --rows 100 --recv-window 5000
```

### [GET /sapi/v1/fiat/payments](https://developers.binance.com/en/docs/catalog/investment-and-services-fiat/api/rest-api/~#get-fiat-payments-history) - Get Fiat Payments History (USER_DATA)

```bash
binance-cli fiat get-fiat-payments-history --transaction-type 0 --begin-time 1641782889000 --end-time 1641782889000 --page 1 --rows 100 --recv-window 5000
```

### [GET /sapi/v1/fiat/get-order-detail](https://developers.binance.com/en/docs/catalog/investment-and-services-fiat/api/rest-api/~#get-order-detail) - Get Order Detail (USER_DATA)

```bash
binance-cli fiat get-order-detail --order-no 036752*678 --recv-window 5000
```
