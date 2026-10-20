## MarketData

### [GET /sapi/v1/w3w/wallet/prediction/market/detail](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#get-market-detail) - Get Market Detail

```bash
binance-cli w3w-prediction get-market-detail --market-topic-id 4229564
```

### [GET /sapi/v1/w3w/wallet/prediction/category/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#list-prediction-categories) - List Prediction Categories

```bash
binance-cli w3w-prediction list-prediction-categories
```

### [GET /sapi/v1/w3w/wallet/prediction/market/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#list-prediction-markets) - List Prediction Markets

```bash
binance-cli w3w-prediction list-prediction-markets --l1-category crypto --l2-category up-down --sort-by VOLUME --order-by DESC --offset 0 --limit 20
```

### [GET /sapi/v1/w3w/wallet/prediction/market/search](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#market-search) - Market Search

```bash
binance-cli w3w-prediction market-search --query BTC price --top-k 20
```

### [GET /sapi/v1/w3w/wallet/prediction/order-book/last-trade-price](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#query-last-trade-price) - Query Last Trade Price

```bash
binance-cli w3w-prediction query-last-trade-price --market-id 5567895
```

### [GET /sapi/v1/w3w/wallet/prediction/order-book](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/market-data#query-order-book) - Query Order Book

```bash
binance-cli w3w-prediction query-order-book --vendor predict_fun --market-id 5567895 --token-id 112233
```

## Otc

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/create](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#create-otc-blocktrade) - Create OTC Blocktrade (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction create-otc-blocktrade --market-id 123 --token-id 71321045679252212594626385532706912750332728571942532289631379312455583992563 --side BUY --maker-amount 600000000000000000000 --taker-amount 1000000000000000000000 --price-per-share 0.65 --expiration 1790000000
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/fulfil](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#fulfil-otc-blocktrade) - Fulfil OTC Blocktrade (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction fulfil-otc-blocktrade --order-id 26080500000001234567 --secret-token a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/detail](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#get-otc-blocktrade-detail) - Get OTC Blocktrade Detail (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-otc-blocktrade-detail --order-id 26080500000001234567
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/events](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#get-otc-blocktrade-events) - Get OTC Blocktrade Events (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-otc-blocktrade-events --first 20 --after next-cursor-string --event-types null  --market-id 123
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/reserved-balances](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#get-otc-reserved-balances) - Get OTC Reserved Balances (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-otc-reserved-balances --assets []
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#list-otc-blocktrades) - List OTC Blocktrades (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction list-otc-blocktrades --first 20 --after next-cursor-string --status OPEN
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/preview](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#preview-otc-blocktrade) - Preview OTC Blocktrade (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction preview-otc-blocktrade --secret-token a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### [POST /sapi/v1/w3w/wallet/prediction/otc/blocktrade/remove](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/otc#remove-otc-blocktrades) - Remove OTC Blocktrades (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction remove-otc-blocktrades --order-ids null
```

## Position

### [GET /sapi/v1/w3w/wallet/prediction/position/token](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/position#get-position-by-token) - Get Position by Token (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-position-by-token --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --token-id 112233 --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/pnl/query](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/position#query-pn-l) - Query PnL (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-pn-l --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --token-id 112233 --market-id 5567895 --market-topic-id 4229564 --active-only false --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/position/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/position#query-positions) - Query Positions (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-positions --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --tab ONGOING --offset 0 --limit 20 --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/position/filter](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/position#query-positions-by-filter) - Query Positions by Filter (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-positions-by-filter --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --market-topic-id 4229564 --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/position/settled-history](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/position#query-settled-position-history) - Query Settled Position History (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-settled-position-history --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --l1-category crypto --result 1 --start-date 2026-05-01 --end-date 2026-05-25 --offset 0 --limit 20 --recv-window 5000
```

## Redeem

### [POST /sapi/v1/w3w/wallet/prediction/batch-redeem](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/redeem#batch-redeem) - Batch Redeem (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction batch-redeem --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --wallet-id 5b5c1ec3be4e4416a5872b21c1ca5d20 --token-ids 112233  --chain-id 56
```

### [GET /sapi/v1/w3w/wallet/prediction/redeem/status](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/redeem#get-redeem-status) - Get Redeem Status (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-redeem-status --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --tx-hash 0xabc123def456789abcdef123456789abcdef123456789abcdef123456789abcd --recv-window 5000
```

## Trade

### [POST /sapi/v1/w3w/wallet/prediction/trade/batch-cancel](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/trade#batch-cancel-orders) - Batch Cancel Orders (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction batch-cancel-orders --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --wallet-id 5b5c1ec3be4e4416a5872b21c1ca5d20 --cancel-info-list []
```

### [POST /sapi/v1/w3w/wallet/prediction/trade/get-quote](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/trade#get-quote) - Get Quote (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-quote --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --token-id 112233 --side BUY --amount-in 1000000000000000000 --order-type MARKET --slippage-bps 1200 --price-limit 0.5 --chain-id 56 --fee-rate-bps 200 --funding-source MPC --fund-transfer-amount 1000000000000000000
```

### [POST /sapi/v1/w3w/wallet/prediction/trade/place-order-bundle](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/trade#place-order) - Place Order (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction place-order --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --wallet-id 5b5c1ec3be4e4416a5872b21c1ca5d20 --quote-id q_20260525_abc123xyz --time-in-force FOK --account-type SPOT --order-type MARKET --slippage-bps 1200 --price-limit 0.5 --funding-source MPC --fund-transfer-amount 1000000000000000000
```

### [GET /sapi/v1/w3w/wallet/prediction/order/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/trade#query-active-orders) - Query Active Orders (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-active-orders --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --trade-side BUY --l1-category crypto --market-id 5567895 --offset 0 --limit 20 --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/order/history](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/trade#query-order-history) - Query Order History (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-order-history --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --l1-category crypto --order-type MARKET --status CLOSED --start-date 2026-05-01 --end-date 2026-05-25 --offset 0 --limit 20 --recv-window 5000
```

## Transfer

### [POST /sapi/v1/w3w/wallet/prediction/deposit/apply](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#apply-mm-deposit) - Apply MM Deposit (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction apply-mm-deposit --from-token USDT --from-token-amount 1000000000000000000 --to-token USDT --account-type SPOT --chain-id 56
```

### [POST /sapi/v1/w3w/wallet/prediction/withdraw/apply](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#apply-mm-withdraw) - Apply MM Withdraw (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction apply-mm-withdraw --coin USDT --network BEP20 --amount 100.00 --withdraw-order-id wd_20260814_001 --wallet-type 56 --name MM withdraw
```

### [POST /sapi/v1/w3w/wallet/prediction/transfer/inbound](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#create-inbound-transfer) - Create Inbound Transfer (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction create-inbound-transfer --wallet-id 5b5c1ec3be4e4416a5872b21c1ca5d20 --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --from-token-amount 1000000000000000000 --account-type SPOT --from-token USDT --to-token USDT --chain-id 56
```

### [POST /sapi/v1/w3w/wallet/prediction/transfer/outbound](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#create-outbound-transfer) - Create Outbound Transfer (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction create-outbound-transfer --wallet-id 5b5c1ec3be4e4416a5872b21c1ca5d20 --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --from-token-amount 1000000000000000000 --account-type SPOT --source-biz USER_TRANSFER --from-token USDT --to-token USDT --chain-id 56
```

### [GET /sapi/v1/w3w/wallet/prediction/transfer/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#query-transfer-list) - Query Transfer List (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-transfer-list --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --start-date 2026-05-01 --end-date 2026-05-25 --token-symbol USDT --direction OUTBOUND --offset 0 --limit 20 --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/transfer/status](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/transfer#query-transfer-status) - Query Transfer Status (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-transfer-status --transfer-id tf_20260525_out_001 --recv-window 5000
```

## Wallet

### [GET /sapi/v1/w3w/wallet/prediction/pnl/portfolio](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/wallet#get-portfolio) - Get Portfolio (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-portfolio --wallet-address 0x12e32db8817e292508c34111cbc4b23340df542c --token-id 112233 --market-id 5567895 --market-topic-id 4229564 --active-only false --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/quota/limit/status](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/wallet#get-quota-status) - Get Quota Status (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction get-quota-status --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/wallet/list](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/wallet#list-prediction-wallets) - List Prediction Wallets (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction list-prediction-wallets --recv-window 5000
```

### [GET /sapi/v1/w3w/wallet/prediction/balance/payment-options](https://developers.binance.com/en/docs/catalog/web3-wallet-prediction-trading/api/rest-api/wallet#query-payment-option-balances) - Query Payment Option Balances (PREDICTION_TRADE)

```bash
binance-cli w3w-prediction query-payment-option-balances --recv-window 5000
```
