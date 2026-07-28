## Account

### [GET /sapi/v1/account/apiTradingStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-api-trading-status) - Account API Trading Status (USER_DATA)

```bash
binance-cli wallet account-api-trading-status --recv-window 5000
```

### [GET /sapi/v1/account/info](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-info) - Account info (USER_DATA)

```bash
binance-cli wallet account-info --recv-window 5000
```

### [GET /sapi/v1/account/status](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-status) - Account Status (USER_DATA)

```bash
binance-cli wallet account-status --recv-window 5000
```

### [GET /sapi/v1/accountSnapshot](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#daily-account-snapshot) - Daily Account Snapshot (USER_DATA)

```bash
binance-cli wallet daily-account-snapshot --rtype SPOT --start-time 1623319461670 --end-time 1641782889000 --limit 7 --recv-window 5000
```

### [POST /sapi/v1/account/disableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#disable-fast-withdraw-switch) - Disable Fast Withdraw Switch (USER_DATA)

```bash
binance-cli wallet disable-fast-withdraw-switch --recv-window 5000
```

### [POST /sapi/v1/account/enableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#enable-fast-withdraw-switch) - Enable Fast Withdraw Switch (USER_DATA)

```bash
binance-cli wallet enable-fast-withdraw-switch --recv-window 5000
```

### [GET /sapi/v1/account/apiRestrictions](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#get-api-key-permission) - Get API Key Permission (USER_DATA)

```bash
binance-cli wallet get-api-key-permission --recv-window 5000
```

## Asset

### [GET /sapi/v1/asset/assetDetail](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-detail) - Asset Detail (USER_DATA)

```bash
binance-cli wallet asset-detail --asset BTC --recv-window 5000
```

### [GET /sapi/v1/asset/assetDividend](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-dividend-record) - Asset Dividend Record (USER_DATA)

```bash
binance-cli wallet asset-dividend-record --asset BTC --start-time 1623319461670 --end-time 1641782889000 --limit 20 --recv-window 5000
```

### [POST /sapi/v1/asset/dust-convert/convert](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convert) - Dust Convert (USER_DATA)

```bash
binance-cli wallet dust-convert --asset USDT --account-type SPOT --client-id 1 --target-asset BTC --third-party-client-id 1 --dust-quota-asset-to-target-asset-price 1.0
```

### [POST /sapi/v1/asset/dust-convert/query-convertible-assets](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convertible-assets) - Dust Convertible Assets (USER_DATA)

```bash
binance-cli wallet dust-convertible-assets --target-asset BTC --account-type SPOT --dust-quota-asset-to-target-asset-price 1.0
```

### [POST /sapi/v1/asset/dust](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-transfer) - Dust Transfer (USER_DATA)

```bash
binance-cli wallet dust-transfer --asset BTC --account-type SPOT --recv-window 5000
```

### [GET /sapi/v1/asset/dribblet](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dustlog) - DustLog (USER_DATA)

```bash
binance-cli wallet dustlog --account-type SPOT --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [POST /sapi/v1/asset/get-funding-asset](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#funding-wallet) - Funding Wallet (USER_DATA)

```bash
binance-cli wallet funding-wallet --asset BTC --need-btc-valuation true --recv-window 5000
```

### [POST /sapi/v1/asset/dust-btc](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-assets-that-can-be-converted-into-bnb) - Get Assets That Can Be Converted Into BNB (USER_DATA)

```bash
binance-cli wallet get-assets-that-can-be-converted-into-bnb --account-type SPOT --recv-window 5000
```

### [GET /sapi/v1/asset/ledger-transfer/cloud-mining/queryByPage](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history) - Get Cloud-Mining payment and refund history (USER_DATA)

```bash
binance-cli wallet get-cloud-mining-payment-and-refund-history --start-time 1623319461670 --end-time 1641782889000 --tran-id 1 --client-tran-id 1 --asset BTC --current 1 --size 10
```

### [GET /sapi/v1/spot/open-symbol-list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-open-symbol-list) - Get Open Symbol List (MARKET_DATA)

```bash
binance-cli wallet get-open-symbol-list
```

### [GET /sapi/v1/asset/custody/transfer-history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-delegation-history) - Query User Delegation History(For Master Account) (USER_DATA)

```bash
binance-cli wallet query-user-delegation-history --email abc@test.com --start-time 1623319461670 --end-time 1641782889000 --rtype Delegate --asset BTC --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/asset/transfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-universal-transfer-history) - Query User Universal Transfer History (USER_DATA)

```bash
binance-cli wallet query-user-universal-transfer-history --rtype r#type_example --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --from-symbol ISOLATEDMARGIN_MARGIN --to-symbol MARGIN_ISOLATEDMARGIN --recv-window 5000
```

### [GET /sapi/v1/asset/wallet/balance](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-wallet-balance) - Query User Wallet Balance (USER_DATA)

```bash
binance-cli wallet query-user-wallet-balance --quote-asset BTC --recv-window 5000
```

### [POST /sapi/v1/bnbBurn](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#toggle-bnb-burn-on-spot-trade-and-margin-interest) - Toggle BNB Burn On Spot Trade And Margin Interest (USER_DATA)

```bash
binance-cli wallet toggle-bnb-burn-on-spot-trade-and-margin-interest --spot-bnb-burn true --interest-bnb-burn true --recv-window 5000
```

### [GET /sapi/v1/asset/tradeFee](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#trade-fee) - Trade Fee (USER_DATA)

```bash
binance-cli wallet trade-fee --symbol ADABNB --recv-window 5000
```

### [POST /sapi/v3/asset/getUserAsset](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-asset) - User Asset (USER_DATA)

```bash
binance-cli wallet user-asset --asset BTC --need-btc-valuation true --recv-window 5000
```

### [POST /sapi/v1/asset/transfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer) - User Universal Transfer (USER_DATA)

```bash
binance-cli wallet user-universal-transfer --rtype MAIN_UMFUTURE --asset BTC --amount 1.0 --from-symbol ISOLATEDMARGIN_MARGIN --to-symbol MARGIN_ISOLATEDMARGIN --recv-window 5000
```

## Capital

### [GET /sapi/v1/capital/config/getall](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#all-coins-information) - All Coins' Information (USER_DATA)

```bash
binance-cli wallet all-coins-information --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/address](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-address) - Deposit Address(supporting network) (USER_DATA)

```bash
binance-cli wallet deposit-address --coin BTC --network network_example --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/hisrec](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-history) - Deposit History (supporting network) (USER_DATA)

```bash
binance-cli wallet deposit-history --include-source false --coin BTC --status 789 --start-time 1623319461670 --end-time 1641782889000 --offset 0 --limit 1000 --recv-window 5000 --tx-id 1
```

### [GET /sapi/v1/capital/deposit/address/list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-deposit-address-list-with-network) - Fetch deposit address list with network (USER_DATA)

```bash
binance-cli wallet fetch-deposit-address-list-with-network --coin BTC --network network_example
```

### [GET /sapi/v1/capital/withdraw/address/list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-address-list) - Fetch withdraw address list (USER_DATA)

```bash
binance-cli wallet fetch-withdraw-address-list
```

### [GET /sapi/v1/capital/withdraw/quota](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-quota) - Fetch withdraw quota (USER_DATA)

```bash
binance-cli wallet fetch-withdraw-quota
```

### [POST /sapi/v1/capital/deposit/credit-apply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#one-click-arrival-deposit-apply) - One click arrival deposit apply (for expired address deposit) (USER_DATA)

```bash
binance-cli wallet one-click-arrival-deposit-apply --deposit-id 1 --tx-id 1 --sub-account-id 1 --sub-user-id 1
```

### [POST /sapi/v1/capital/withdraw/apply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw) - Withdraw (USER_DATA)

```bash
binance-cli wallet withdraw --coin BTC --address address_example --amount 1.0 --withdraw-order-id 1 --network network_example --address-tag address_tag_example --transaction-fee-flag true --name name_example --wallet-type 0 --recv-window 5000
```

### [GET /sapi/v1/capital/withdraw/history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw-history) - Withdraw History (supporting network) (USER_DATA)

```bash
binance-cli wallet withdraw-history --coin BTC --withdraw-order-id 1 --status 0 --offset 0 --limit 1000 --id-list id_list_example --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

## Others

### [GET /sapi/v1/spot/delist-schedule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#get-symbols-delist-schedule-for-spot) - Get Spot Delist Schedule (MARKET_DATA)

```bash
binance-cli wallet get-symbols-delist-schedule-for-spot --recv-window 5000
```

### [GET /sapi/v1/system/status](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#system-status) - System Status

```bash
binance-cli wallet system-status
```

## TravelRule

### [POST /sapi/v1/localentity/broker/withdraw/apply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#broker-withdraw) - Broker Withdraw (for brokers of local entities that require travel rule) (USER_DATA)

```bash
binance-cli wallet broker-withdraw --address address_example --coin BTC --amount 1.0 --withdraw-order-id 1 --questionnaire questionnaire_example --originator-pii originator_pii_example --address-tag address_tag_example --network network_example --address-name address_name_example --transaction-fee-flag true --wallet-type 0
```

### [GET /sapi/v1/localentity/questionnaire-requirements](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#check-questionnaire-requirements) - Check Questionnaire Requirements (for local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet check-questionnaire-requirements --recv-window 5000
```

### [GET /sapi/v1/localentity/deposit/history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule) - Deposit History Travel Rule (for local entities that required travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet deposit-history-travel-rule --tr-id 1 --tx-id 1 --tran-id 1 --network network_example --coin BTC --travel-rule-status 0 --pending-questionnaire true --start-time 1623319461670 --end-time 1641782889000 --offset 789 --limit 1000
```

### [GET /sapi/v2/localentity/deposit/history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-v2) - Deposit History V2 (for local entities that required travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet deposit-history-v2 --deposit-id 1 --tx-id 1 --network network_example --coin BTC --retrieve-questionnaire true --start-time 1623319461670 --end-time 1641782889000 --offset 0 --limit 1000
```

### [GET /sapi/v1/addressVerify/list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#fetch-address-verification-list) - Fetch address verification list (USER_DATA)

```bash
binance-cli wallet fetch-address-verification-list --recv-window 5000
```

### [GET /sapi/v1/localentity/country/list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-country-list) - Get Country List (USER_DATA)

```bash
binance-cli wallet get-country-list --recv-window 5000
```

### [GET /sapi/v1/localentity/region/list](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-region-list) - Get Region List (USER_DATA)

```bash
binance-cli wallet get-region-list --country-code au --recv-window 5000
```

### [PUT /sapi/v1/localentity/broker/deposit/provide-info](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire) - Submit Deposit Questionnaire Broker (For local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet submit-deposit-questionnaire --sub-account-id 1 --deposit-id 1 --questionnaire questionnaire_example --beneficiary-pii beneficiary_pii_example --network network_example --coin BTC --amount 1.0 --address address_example --address-tag address_tag_example
```

### [PUT /sapi/v1/localentity/deposit/provide-info](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-travel-rule) - Submit Deposit Questionnaire (For local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet submit-deposit-questionnaire-travel-rule --tran-id 1 --questionnaire questionnaire_example
```

### [PUT /sapi/v2/localentity/deposit/provide-info](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-v2) - Submit Deposit Questionnaire V2 (For local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet submit-deposit-questionnaire-v2 --deposit-id 1 --questionnaire questionnaire_example
```

### [GET /sapi/v1/localentity/vasp](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#vasp-list) - VASP list (for local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet vasp-list --recv-window 5000
```

### [GET /sapi/v1/localentity/withdraw/history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1) - Withdraw History Travel Rule (supporting network) (USER_DATA)

```bash
binance-cli wallet withdraw-history-v1 --tr-id 1 --tx-id 1 --withdraw-order-id 1 --network network_example --coin BTC --travel-rule-status 0 --offset 0 --limit 1000 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v2/localentity/withdraw/history](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v2) - Withdraw History V2 (for local entities that require travel rule) (supporting network) (USER_DATA)

```bash
binance-cli wallet withdraw-history-v2 --tr-id 1 --tx-id 1 --withdraw-order-id 1 --network network_example --coin coin_example --travel-rule-status 0 --offset 0 --limit 1000 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [POST /sapi/v1/localentity/withdraw/apply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-travel-rule) - Withdraw Travel Rule (USER_DATA)

```bash
binance-cli wallet withdraw-travel-rule --coin BTC --address address_example --amount 1.0 --questionnaire questionnaire_example --withdraw-order-id 1 --network network_example --address-tag address_tag_example --transaction-fee-flag true --name name_example --wallet-type 0 --recv-window 5000
```
