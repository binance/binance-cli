## Mining

### [GET /sapi/v1/mining/statistics/user/list](https://developers.binance.com/docs/mining/rest-api/Account-List) - Account List

```bash
binance-cli mining account-list --algo "algo_example" --user-name "userName_example" --recv-window 5000
```

### [GET /sapi/v1/mining/pub/algoList](https://developers.binance.com/docs/mining/rest-api/Acquiring-Algorithm) - Acquiring Algorithm

```bash
binance-cli mining acquiring-algorithm
```

### [GET /sapi/v1/mining/pub/coinList](https://developers.binance.com/docs/mining/rest-api/Acquiring-CoinName) - Acquiring CoinName

```bash
binance-cli mining acquiring-coinname
```

### [POST /sapi/v1/mining/hash-transfer/config/cancel](https://developers.binance.com/docs/mining/rest-api/Cancel-hashrate-resale-configuration) - Cancel hashrate resale configuration

```bash
binance-cli mining cancel-hashrate-resale-configuration --json {}
```

### [GET /sapi/v1/mining/payment/list](https://developers.binance.com/docs/mining/rest-api/Earnings-List) - Earnings List

```bash
binance-cli mining earnings-list --algo "algo_example" --user-name "userName_example" --coin "coin_example" --start-date 1 --end-date 1 --page-index 1 --page-size 1 --recv-window 5000
```

### [GET /sapi/v1/mining/payment/other](https://developers.binance.com/docs/mining/rest-api/Extra-Bonus-List) - Extra Bonus List

```bash
binance-cli mining extra-bonus-list --algo "algo_example" --user-name "userName_example" --coin "coin_example" --start-date 1 --end-date 1 --page-index 1 --page-size 1 --recv-window 5000
```

### [GET /sapi/v1/mining/hash-transfer/profit/details](https://developers.binance.com/docs/mining/rest-api/Hashrate-Resale-Detail) - Hashrate Resale Detail

```bash
binance-cli mining hashrate-resale-detail --config-id 1 --page-index 1 --page-size 1 --recv-window 5000
```

### [GET /sapi/v1/mining/hash-transfer/config/details/list](https://developers.binance.com/docs/mining/rest-api/Hashrate-Resale-List) - Hashrate Resale List

```bash
binance-cli mining hashrate-resale-list --page-index 1 --page-size 1 --recv-window 5000
```

### [POST /sapi/v1/mining/hash-transfer/config](https://developers.binance.com/docs/mining/rest-api/Hashrate-Resale-Request) - Hashrate Resale Request

```bash
binance-cli mining hashrate-resale-request --json {}
```

### [GET /sapi/v1/mining/payment/uid](https://developers.binance.com/docs/mining/rest-api/Mining-Account-Earning) - Mining Account Earning

```bash
binance-cli mining mining-account-earning --algo "algo_example" --start-date 1 --end-date 1 --page-index 1 --page-size 1 --recv-window 5000
```

### [GET /sapi/v1/mining/worker/detail](https://developers.binance.com/docs/mining/rest-api/Request-for-Detail-Miner-List) - Request for Detail Miner List

```bash
binance-cli mining request-for-detail-miner-list --algo "algo_example" --user-name "userName_example" --worker-name "workerName_example" --recv-window 5000
```

### [GET /sapi/v1/mining/worker/list](https://developers.binance.com/docs/mining/rest-api/Request-for-Miner-List) - Request for Miner List

```bash
binance-cli mining request-for-miner-list --algo "algo_example" --user-name "userName_example" --page-index 1 --sort 0 --sort-column 1 --worker-status 0 --recv-window 5000
```

### [GET /sapi/v1/mining/statistics/user/status](https://developers.binance.com/docs/mining/rest-api/Statistic-List) - Statistic List

```bash
binance-cli mining statistic-list --algo "algo_example" --user-name "userName_example" --recv-window 5000
```
