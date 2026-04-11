## Account

### [GET /sapi/v1/account/apiTradingStatus](https://developers.binance.com/docs/wallet/account/Account-API-Trading-Status) - Account API Trading Status

```bash
binance-cli wallet account-api-trading-status --recv-window 5000
```

### [GET /sapi/v1/account/info](https://developers.binance.com/docs/wallet/account/Account-info) - Account info

```bash
binance-cli wallet account-info --recv-window 5000
```

### [GET /sapi/v1/account/status](https://developers.binance.com/docs/wallet/account/Account-Status) - Account Status

```bash
binance-cli wallet account-status --recv-window 5000
```

### [GET /sapi/v1/accountSnapshot](https://developers.binance.com/docs/wallet/account/daily-account-snapshoot) - Daily Account Snapshot

```bash
binance-cli wallet daily-account-snapshot --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --limit 7 --recv-window 5000
```

### [POST /sapi/v1/account/disableFastWithdrawSwitch](https://developers.binance.com/docs/wallet/account/Disable-Fast-Withdraw-Switch) - Disable Fast Withdraw Switch

```bash
binance-cli wallet disable-fast-withdraw-switch --json {}
```

### [POST /sapi/v1/account/enableFastWithdrawSwitch](https://developers.binance.com/docs/wallet/account/Enable-Fast-Withdraw-Switch) - Enable Fast Withdraw Switch

```bash
binance-cli wallet enable-fast-withdraw-switch --json {}
```

### [GET /sapi/v1/account/apiRestrictions](https://developers.binance.com/docs/wallet/account/api-key-permission) - Get API Key Permission

```bash
binance-cli wallet get-api-key-permission --recv-window 5000
```

## Asset

### [GET /sapi/v1/asset/assetDetail](https://developers.binance.com/docs/wallet/asset/Asset-Detail) - Asset Detail

```bash
binance-cli wallet asset-detail --asset "asset_example" --recv-window 5000
```

### [GET /sapi/v1/asset/assetDividend](https://developers.binance.com/docs/wallet/asset/assets-divided-record) - Asset Dividend Record

```bash
binance-cli wallet asset-dividend-record --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --limit 7 --recv-window 5000
```

### [POST /sapi/v1/asset/dust-convert/convert](https://developers.binance.com/docs/wallet/asset/Dust-Convert) - Dust Convert

```bash
binance-cli wallet dust-convert --json {}
```

### [POST /sapi/v1/asset/dust-convert/query-convertible-assets](https://developers.binance.com/docs/wallet/asset/Dust-Convertible-Assets) - Dust Convertible Assets

```bash
binance-cli wallet dust-convertible-assets --json {}
```

### [POST /sapi/v1/asset/dust](https://developers.binance.com/docs/wallet/asset/Dust-Transfer) - Dust Transfer

```bash
binance-cli wallet dust-transfer --json {}
```

### [GET /sapi/v1/asset/dribblet](https://developers.binance.com/docs/wallet/asset/dust-log) - DustLog

```bash
binance-cli wallet dustlog --account-type "SPOT" --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [POST /sapi/v1/asset/get-funding-asset](https://developers.binance.com/docs/wallet/asset/Funding-Wallet) - Funding Wallet

```bash
binance-cli wallet funding-wallet --json {}
```

### [POST /sapi/v1/asset/dust-btc](https://developers.binance.com/docs/wallet/asset/assets-can-convert-bnb) - Get Assets That Can Be Converted Into BNB

```bash
binance-cli wallet get-assets-that-can-be-converted-into-bnb --json {}
```

### [GET /sapi/v1/asset/ledger-transfer/cloud-mining/queryByPage](https://developers.binance.com/docs/wallet/asset/cloud-mining-payment-and-refund-history) - Get Cloud-Mining payment and refund history

```bash
binance-cli wallet get-cloud-mining-payment-and-refund-history --start-time 1623319461670 --end-time 1641782889000 --tran-id 1 --client-tran-id "1" --asset "asset_example" --current 1 --size 10
```

### [GET /sapi/v1/spot/open-symbol-list](https://developers.binance.com/docs/wallet/asset/open-symbol-list) - Get Open Symbol List

```bash
binance-cli wallet get-open-symbol-list
```

### [GET /sapi/v1/asset/custody/transfer-history](https://developers.binance.com/docs/wallet/asset/query-user-delegation) - Query User Delegation History(For Master Account)

```bash
binance-cli wallet query-user-delegation-history --email "email_example" --start-time 1623319461670 --end-time 1641782889000 --type "type_example" --asset "asset_example" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/asset/transfer](https://developers.binance.com/docs/wallet/asset/query-user-universal-transfer) - Query User Universal Transfer History

```bash
binance-cli wallet query-user-universal-transfer-history --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --from-symbol "fromSymbol_example" --to-symbol "toSymbol_example" --recv-window 5000
```

### [GET /sapi/v1/asset/wallet/balance](https://developers.binance.com/docs/wallet/asset/Query-User-Wallet-Balance) - Query User Wallet Balance

```bash
binance-cli wallet query-user-wallet-balance --quote-asset "BTC" --recv-window 5000
```

### [POST /sapi/v1/bnbBurn](https://developers.binance.com/docs/wallet/asset/Toggle-BNB-Burn-On-Spot-Trade-And-Margin-Interest) - Toggle BNB Burn On Spot Trade And Margin Interest

```bash
binance-cli wallet toggle-bnb-burn-on-spot-trade-and-margin-interest --json {}
```

### [GET /sapi/v1/asset/tradeFee](https://developers.binance.com/docs/wallet/asset/Trade-Fee) - Trade Fee

```bash
binance-cli wallet trade-fee --symbol "symbol_example" --recv-window 5000
```

### [POST /sapi/v3/asset/getUserAsset](https://developers.binance.com/docs/wallet/asset/user-assets) - User Asset

```bash
binance-cli wallet user-asset --json {}
```

### [POST /sapi/v1/asset/transfer](https://developers.binance.com/docs/wallet/asset/User-Universal-Transfer) - User Universal Transfer

```bash
binance-cli wallet user-universal-transfer --json {}
```

## Capital

### [GET /sapi/v1/capital/config/getall](https://developers.binance.com/docs/wallet/capital/all-coins-info) - All Coins\' Information

```bash
binance-cli wallet all-coins-information --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/address](https://developers.binance.com/docs/wallet/capital/deposite-address) - Deposit Address(supporting network)

```bash
binance-cli wallet deposit-address --coin "coin_example" --network "network_example" --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/hisrec](https://developers.binance.com/docs/wallet/capital/deposite-history) - Deposit History (supporting network)

```bash
binance-cli wallet deposit-history --include-source false --coin "coin_example" --status 1 --start-time 1623319461670 --end-time 1641782889000 --offset 0 --limit 7 --recv-window 5000 --tx-id "1"
```

### [GET /sapi/v1/capital/deposit/address/list](https://developers.binance.com/docs/wallet/capital/Fetch-deposit-address-list-with-network) - Fetch deposit address list with network

```bash
binance-cli wallet fetch-deposit-address-list-with-network --coin "coin_example" --network "network_example"
```

### [GET /sapi/v1/capital/withdraw/address/list](https://developers.binance.com/docs/wallet/capital/fetch-withdraw-address) - Fetch withdraw address list

```bash
binance-cli wallet fetch-withdraw-address-list
```

### [GET /sapi/v1/capital/withdraw/quota](https://developers.binance.com/docs/wallet/capital/Fetch-withdraw-quota) - Fetch withdraw quota

```bash
binance-cli wallet fetch-withdraw-quota
```

### [POST /sapi/v1/capital/deposit/credit-apply](https://developers.binance.com/docs/wallet/capital/one-click-arrival-deposite-apply) - One click arrival deposit apply (for expired address deposit)

```bash
binance-cli wallet one-click-arrival-deposit-apply --json {}
```

### [POST /sapi/v1/capital/withdraw/apply](https://developers.binance.com/docs/wallet/capital/Withdraw) - Withdraw

```bash
binance-cli wallet withdraw --json {}
```

### [GET /sapi/v1/capital/withdraw/history](https://developers.binance.com/docs/wallet/capital/Withdraw-History) - Withdraw History (supporting network)

```bash
binance-cli wallet withdraw-history --coin "coin_example" --withdraw-order-id "1" --status 1 --offset 0 --limit 7 --id-list "idList_example" --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

## Others

### [GET /sapi/v1/spot/delist-schedule](https://developers.binance.com/docs/wallet/others/delist-schedule) - Get symbols delist schedule for spot

```bash
binance-cli wallet get-symbols-delist-schedule-for-spot --recv-window 5000
```

### [GET /sapi/v1/system/status](https://developers.binance.com/docs/wallet/others/System-Status) - System Status (System)

```bash
binance-cli wallet system-status
```

## TravelRule

### [POST /sapi/v1/localentity/broker/withdraw/apply](https://developers.binance.com/docs/wallet/travel-rule/Broker-Withdraw) - Broker Withdraw (for brokers of local entities that require travel rule)

```bash
binance-cli wallet broker-withdraw --json {}
```

### [GET /sapi/v1/localentity/questionnaire-requirements](https://developers.binance.com/docs/wallet/travel-rule/questionnaire-requirements) - Check Questionnaire Requirements (for local entities that require travel rule) (supporting network)

```bash
binance-cli wallet check-questionnaire-requirements --recv-window 5000
```

### [GET /sapi/v1/localentity/deposit/history](https://developers.binance.com/docs/wallet/travel-rule/Deposit-History) - Deposit History (for local entities that required travel rule) (supporting network)

```bash
binance-cli wallet deposit-history-travel-rule --tr-id "1" --tx-id "1" --tran-id "1" --network "network_example" --coin "coin_example" --travel-rule-status 1 --pending-questionnaire true --start-time 1623319461670 --end-time 1641782889000 --offset 0 --limit 7
```

### [GET /sapi/v2/localentity/deposit/history](https://developers.binance.com/docs/wallet/travel-rule/Deposit-History-V2) - Deposit History V2 (for local entities that required travel rule) (supporting network)

```bash
binance-cli wallet deposit-history-v2 --deposit-id 1 --tx-id "1" --network "network_example" --coin "coin_example" --retrieve-questionnaire true --start-time 1623319461670 --end-time 1641782889000 --offset 0 --limit 7
```

### [GET /sapi/v1/addressVerify/list](https://developers.binance.com/docs/wallet/travel-rule/address-verification-list) - Fetch address verification list

```bash
binance-cli wallet fetch-address-verification-list --recv-window 5000
```

### [PUT /sapi/v1/localentity/broker/deposit/provide-info](https://developers.binance.com/docs/wallet/travel-rule/deposit-provide-info) - Submit Deposit Questionnaire (For local entities that require travel rule) (supporting network)

```bash
binance-cli wallet submit-deposit-questionnaire --json {}
```

### [PUT /sapi/v1/localentity/deposit/provide-info](https://developers.binance.com/docs/wallet/travel-rule/deposit-provide-info) - Submit Deposit Questionnaire (For local entities that require travel rule) (supporting network)

```bash
binance-cli wallet submit-deposit-questionnaire-travel-rule --json {}
```

### [PUT /sapi/v2/localentity/deposit/provide-info](https://developers.binance.com/docs/wallet/travel-rule/deposit-provide-info-v2) - Submit Deposit Questionnaire V2 (For local entities that require travel rule) (supporting network)

```bash
binance-cli wallet submit-deposit-questionnaire-v2 --json {}
```

### [GET /sapi/v1/localentity/vasp](https://developers.binance.com/docs/wallet/travel-rule/onboarded-vasp-list) - VASP list (for local entities that require travel rule) (supporting network)

```bash
binance-cli wallet vasp-list --recv-window 5000
```

### [GET /sapi/v1/localentity/withdraw/history](https://developers.binance.com/docs/wallet/travel-rule/Withdraw-History) - Withdraw History (for local entities that require travel rule) (supporting network)

```bash
binance-cli wallet withdraw-history-v1 --tr-id "1" --tx-id "1" --withdraw-order-id "1" --network "network_example" --coin "coin_example" --travel-rule-status 1 --offset 0 --limit 7 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v2/localentity/withdraw/history](https://developers.binance.com/docs/wallet/travel-rule/Withdraw-History-V2) - Withdraw History V2 (for local entities that require travel rule) (supporting network)

```bash
binance-cli wallet withdraw-history-v2 --tr-id "1" --tx-id "1" --withdraw-order-id "1" --network "network_example" --coin "coin_example" --travel-rule-status 1 --offset 0 --limit 7 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [POST /sapi/v1/localentity/withdraw/apply](https://developers.binance.com/docs/wallet/travel-rule/Withdraw) - Withdraw (for local entities that require travel rule)

```bash
binance-cli wallet withdraw-travel-rule --json {}
```
