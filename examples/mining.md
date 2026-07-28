## Default

### [GET /sapi/v1/mining/statistics/user/list](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#account-list) - Account List (USER_DATA)

```bash
binance-cli mining account-list --algo sha256 --user-name test --recv-window 5000
```

### [GET /sapi/v1/mining/pub/algoList](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#acquiring-algorithm) - Acquiring Algorithm (MARKET_DATA)

```bash
binance-cli mining acquiring-algorithm
```

### [GET /sapi/v1/mining/pub/coinList](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#acquiring-coinname) - Acquiring CoinName (MARKET_DATA)

```bash
binance-cli mining acquiring-coinname
```

### [POST /sapi/v1/mining/hash-transfer/config/cancel](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#cancel-hashrate-resale-configuration) - Cancel hashrate resale configuration (USER_DATA)

```bash
binance-cli mining cancel-hashrate-resale-configuration --config-id 168 --user-name test --recv-window 5000
```

### [GET /sapi/v1/mining/payment/list](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#earnings-list) - Earnings List (USER_DATA)

```bash
binance-cli mining earnings-list --algo sha256 --user-name test --coin BTC --start-date 1770736694138 --end-date 1770736694138 --page-index 1 --page-size 10 --recv-window 5000
```

### [GET /sapi/v1/mining/payment/other](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#extra-bonus-list) - Extra Bonus List (USER_DATA)

```bash
binance-cli mining extra-bonus-list --algo sha256 --user-name test --coin BTC --start-date 1770736694138 --end-date 1770736694138 --page-index 1 --page-size 10 --recv-window 5000
```

### [GET /sapi/v1/mining/hash-transfer/profit/details](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#hashrate-resale-detail) - Hashrate Resale Detail (USER_DATA)

```bash
binance-cli mining hashrate-resale-detail --config-id 168 --page-index 1 --page-size 10 --recv-window 5000
```

### [GET /sapi/v1/mining/hash-transfer/config/details/list](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#hashrate-resale-list) - Hashrate Resale List (USER_DATA)

```bash
binance-cli mining hashrate-resale-list --page-index 1 --page-size 10 --recv-window 5000
```

### [POST /sapi/v1/mining/hash-transfer/config](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#hashrate-resale-request) - Hashrate Resale Request (USER_DATA)

```bash
binance-cli mining hashrate-resale-request --user-name test --algo sha256 --end-date 1770736694138 --start-date 1770736694138 --to-pool-user S19pro --hash-rate 100000000 --recv-window 5000
```

### [GET /sapi/v1/mining/payment/uid](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#mining-account-earning) - Mining Account Earning (USER_DATA)

```bash
binance-cli mining mining-account-earning --algo sha256 --start-date 1770736694138 --end-date 1770736694138 --page-index 1 --page-size 10 --recv-window 5000
```

### [GET /sapi/v1/mining/worker/detail](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#request-for-detail-miner-list) - Request for Detail Miner List (USER_DATA)

```bash
binance-cli mining request-for-detail-miner-list --algo sha256 --user-name test --worker-name bhdc1.16A10404B --recv-window 5000
```

### [GET /sapi/v1/mining/worker/list](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#request-for-miner-list) - Request for Miner List (USER_DATA)

```bash
binance-cli mining request-for-miner-list --algo sha256 --user-name test --page-index 1 --sort 0 --sort-column 1 --worker-status 0 --recv-window 5000
```

### [GET /sapi/v1/mining/statistics/user/status](https://developers.binance.com/en/docs/catalog/investment-and-services-mining/api/rest-api/~#statistic-list) - Statistic List (USER_DATA)

```bash
binance-cli mining statistic-list --algo sha256 --user-name test --recv-window 5000
```
