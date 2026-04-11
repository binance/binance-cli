## Fiat

### [POST /sapi/v1/fiat/deposit](https://developers.binance.com/docs/fiat/rest-api/Fiat-Deposit) - Deposit

```bash
binance-cli fiat deposit --json {}
```

### [POST /sapi/v2/fiat/withdraw](https://developers.binance.com/docs/fiat/rest-api/Fiat-Withdraw) - Fiat Withdraw

```bash
binance-cli fiat fiat-withdraw --json {}
```

### [GET /sapi/v1/fiat/orders](https://developers.binance.com/docs/fiat/rest-api/Get-Fiat-Deposit-Withdraw-History) - Get Fiat Deposit/Withdraw History

```bash
binance-cli fiat get-fiat-deposit-withdraw-history --transaction-type "transactionType_example" --begin-time 1 --end-time 1641782889000 --page 1 --rows 100 --recv-window 5000
```

### [GET /sapi/v1/fiat/payments](https://developers.binance.com/docs/fiat/rest-api/Get-Fiat-Payments-History) - Get Fiat Payments History

```bash
binance-cli fiat get-fiat-payments-history --transaction-type "transactionType_example" --begin-time 1 --end-time 1641782889000 --page 1 --rows 100 --recv-window 5000
```

### [GET /sapi/v1/fiat/get-order-detail](https://developers.binance.com/docs/fiat/rest-api/Get-Order-Detail) - Get Order Detail

```bash
binance-cli fiat get-order-detail --order-no "orderNo_example" --recv-window 5000
```
