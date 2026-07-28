## AccountManagement

### [POST /sapi/v1/sub-account/virtualSubAccount](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#create-avirtual-sub-account) - Create a Virtual Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account create-a-virtual-sub-account --sub-account-string testSubAccount --recv-window 5000
```

### [POST /sapi/v1/sub-account/futures/enable](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#enable-futures-for-sub-account) - Enable Futures for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account enable-futures-for-sub-account --email 123@test.com --recv-window 5000
```

### [POST /sapi/v1/sub-account/eoptions/enable](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#enable-options-for-sub-account) - Enable Options for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account enable-options-for-sub-account --email 123@test.com --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/positionRisk](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#get-futures-position-risk-of-sub-account) - Get Futures Position-Risk of Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-futures-position-risk-of-sub-account --email 123@test.com --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/positionRisk](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#get-futures-position-risk-of-sub-account-v2) - Get Futures Position-Risk of Sub-account V2 (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-futures-position-risk-of-sub-account-v2 --email 123@test.com --futures-type 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/status](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#get-sub-accounts-status-on-margin-or-futures) - Get Sub-account's Status on Margin Or Futures (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-sub-accounts-status-on-margin-or-futures --email 123@test.com --recv-window 5000
```

### [GET /sapi/v1/sub-account/list](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#query-sub-account-list) - Query Sub-account List (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-list --email 123@test.com --is-freeze true --page 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/transaction-statistics](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/account-management#query-sub-account-transaction-statistics) - Query Sub-account Transaction Statistics (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-transaction-statistics --email abc@test.com --recv-window 5000
```

## ApiManagement

### [POST /sapi/v2/sub-account/subAccountApi/ipRestriction](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#add-ip-restriction-for-sub-account-api-key) - Add IP Restriction for Sub-Account API key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account add-ip-restriction-for-sub-account-api-key --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --status 1 --ip-address 69.210.67.14 --recv-window 5000
```

### [POST /sapi/v1/sub-account/subAccountApi](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#create-sub-account-api-key) - Create Sub-account API Key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account create-sub-account-api-key --email 123@test.com --api-name myKey --status 2 --can-trade true --can-margin-loan-repay false --can-futures-trade false --can-universal-transfer false --can-vanilla-options false --ip-address 69.210.67.14 --third-party-name thirdParty --public-key  --recv-window 5000
```

### [DELETE /sapi/v1/sub-account/subAccountApi/ipRestriction/ipList](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#delete-ip-list-for-asub-account-api-key) - Delete IP List For a Sub-account API Key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account delete-ip-list-for-a-sub-account-api-key --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --ip-address 69.210.67.14 --recv-window 5000
```

### [DELETE /sapi/v1/sub-account/subAccountApi](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#delete-sub-account-api-key) - Delete Sub-account API Key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account delete-sub-account-api-key --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --recv-window 5000
```

### [GET /sapi/v1/sub-account/subAccountApi/ipRestriction](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#get-ip-restriction-for-asub-account-api-key) - Get IP Restriction for a Sub-account API Key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-ip-restriction-for-a-sub-account-api-key --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --recv-window 5000
```

### [POST /sapi/v1/sub-account/subAccountApiPermission](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#modify-sub-account-api-key-permission) - Modify Sub-account API Key Permission (For Master Account) (USER_DATA)

```bash
binance-cli sub-account modify-sub-account-api-key-permission --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --can-trade true --can-margin-loan-repay false --can-futures-trade true --can-universal-transfer false --can-vanilla-options false --recv-window 5000
```

### [GET /sapi/v1/sub-account/subAccountApi](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/api-management#query-sub-account-api-key) - Query Sub-account API Key (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-api-key --email 123@test.com --sub-account-api-key k5V49ldtn4tszj6W3hystegdfvmGbqDzjmkCtpTvC0G74WhK7yd4rfCTo4lShf --page 1 --size 30 --recv-window 5000
```

## AssetManagement

### [POST /sapi/v1/sub-account/futures/transfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#futures-transfer-for-sub-account) - Futures Transfer for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account futures-transfer-for-sub-account --email 123@test.com --asset USDT --amount 1.0 --rtype 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/account](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-detail-on-sub-accounts-futures-account) - Get Detail on Sub-account's Futures Account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-detail-on-sub-accounts-futures-account --email 123@test.com --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/account](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-detail-on-sub-accounts-futures-account-v2) - Get Detail on Sub-account's Futures Account V2 (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-detail-on-sub-accounts-futures-account-v2 --email 123@test.com --futures-type 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/margin/account](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-detail-on-sub-accounts-margin-account) - Get Detail on Sub-account's Margin Account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-detail-on-sub-accounts-margin-account --email 123@test.com --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/move-position](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-move-position-history-for-sub-account) - Get Move Position History for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-move-position-history-for-sub-account --symbol BTCUSDT --page 1 --rows 1 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/subAddress](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-sub-account-deposit-address) - Get Sub-account Deposit Address (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-sub-account-deposit-address --email 123@test.com --coin BTC --network network_example --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/subHisrec](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-sub-account-deposit-history) - Get Sub-account Deposit History (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-sub-account-deposit-history --email 123@test.com --include-source false --coin BTC --status 0 --start-time 1623319461670 --end-time 1641782889000 --limit 1 --offset 0 --recv-window 5000 --tx-id 1
```

### [GET /sapi/v1/sub-account/futures/accountSummary](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-summary-of-sub-accounts-futures-account) - Get Summary of Sub-account's Futures Account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-summary-of-sub-accounts-futures-account --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/accountSummary](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-summary-of-sub-accounts-futures-account-v2) - Get Summary of Sub-account's Futures Account V2 (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-summary-of-sub-accounts-futures-account-v2 --futures-type 1 --page 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/margin/accountSummary](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#get-summary-of-sub-accounts-margin-account) - Get Summary of Sub-account's Margin Account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account get-summary-of-sub-accounts-margin-account --recv-window 5000
```

### [POST /sapi/v1/sub-account/margin/transfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#margin-transfer-for-sub-account) - Margin Transfer for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account margin-transfer-for-sub-account --email 123@test.com --asset BTC --amount 1.0 --rtype 1 --recv-window 5000
```

### [POST /sapi/v1/sub-account/futures/move-position](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#move-position-for-sub-account) - Move Position for Sub-account (For Master Account) (USER_DATA)

```bash
binance-cli sub-account move-position-for-sub-account --from-user-email testFrom@google.com --to-user-email testTo@google.com --product-type UM --order-args [] --recv-window 5000
```

### [GET /sapi/v3/sub-account/assets](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-sub-account-assets) - Query Sub-account Assets (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-assets --email 123@test.com --recv-window 5000
```

### [GET /sapi/v4/sub-account/assets](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-sub-account-assets-asset-management) - Query Sub-account Assets V4 (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-assets-asset-management --email 123@test.com --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/internalTransfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-sub-account-futures-asset-transfer-history) - Query Sub-account Futures Asset Transfer History (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-futures-asset-transfer-history --email 123@test.com --futures-type 1 --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/sub/transfer/history](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-sub-account-spot-asset-transfer-history) - Query Sub-account Spot Asset Transfer History (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-spot-asset-transfer-history --from-email aaa@test.com --to-email bbb@test.com --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/spotSummary](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-sub-account-spot-assets-summary) - Query Sub-account Spot Assets Summary (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-sub-account-spot-assets-summary --email 123@test.com --page 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/universalTransfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#query-universal-transfer-history) - Query Universal Transfer History (For Master Account) (USER_DATA)

```bash
binance-cli sub-account query-universal-transfer-history --from-email abctest@gmail.com --to-email deftest@gmail.com --client-tran-id 1 --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 10 --recv-window 5000
```

### [POST /sapi/v1/sub-account/futures/internalTransfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#sub-account-futures-asset-transfer) - Sub-account Futures Asset Transfer (For Master Account) (USER_DATA)

```bash
binance-cli sub-account sub-account-futures-asset-transfer --from-email abc@test.com --to-email def@test.com --futures-type 1 --asset BTC --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/sub-account/transfer/subUserHistory](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#sub-account-transfer-history) - Sub-account Transfer History (For Sub-account) (USER_DATA)

```bash
binance-cli sub-account sub-account-transfer-history --asset BTC --rtype 1 --start-time 1623319461670 --end-time 1641782889000 --limit 10 --return-fail-history false --recv-window 5000
```

### [POST /sapi/v1/sub-account/transfer/subToMaster](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#transfer-to-master) - Transfer to Master (For Sub-account) (USER_DATA)

```bash
binance-cli sub-account transfer-to-master --asset BTC --amount 1.0 --recv-window 5000
```

### [POST /sapi/v1/sub-account/transfer/subToSub](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#transfer-to-sub-account-of-same-master) - Transfer to Sub-account of Same Master (For Sub-account) (USER_DATA)

```bash
binance-cli sub-account transfer-to-sub-account-of-same-master --to-email abc@test.com --asset BTC --amount 1.0 --recv-window 5000
```

### [POST /sapi/v1/sub-account/universalTransfer](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/asset-management#universal-transfer) - Universal Transfer (For Master Account) (USER_DATA)

```bash
binance-cli sub-account universal-transfer --from-account-type SPOT --to-account-type SPOT --asset BTC --amount 1.0 --from-email abc@test.com --to-email def@test.com --client-tran-id 1 --symbol BTCUSDT --recv-window 5000
```

## ManagedSubAccount

### [POST /sapi/v1/managed-subaccount/deposit](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#deposit-assets-into-the-managed-sub-account) - Deposit Assets Into The Managed Sub-account (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account deposit-assets-into-the-managed-sub-account --to-email abc@test.com --asset BTC --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/deposit/address](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#get-managed-sub-account-deposit-address) - Get Managed Sub-account Deposit Address (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account get-managed-sub-account-deposit-address --email abc@test.com --coin USDT --network LIGHTNING --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/asset](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-asset-details) - Query Managed Sub-account Asset Details (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-asset-details --email abc@test.com --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/fetch-future-asset](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-futures-asset-details) - Query Managed Sub-account Futures Asset Details (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-futures-asset-details --email abc@test.com --account-type MARGIN
```

### [GET /sapi/v1/managed-subaccount/info](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-list) - Query Managed Sub-account List (For Investor) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-list --email abc@test.com --page 1 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/marginAsset](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-margin-asset-details) - Query Managed Sub-account Margin Asset Details (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-margin-asset-details --email abc@test.com --account-type MARGIN
```

### [GET /sapi/v1/managed-subaccount/accountSnapshot](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-snapshot) - Query Managed Sub-account Snapshot (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-snapshot --email abc@test.com --rtype SPOT --start-time 1623319461670 --end-time 1641782889000 --limit 10 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/queryTransLogForInvestor](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-transfer-log-master-account-investor) - Query Managed Sub Account Transfer Log For Investor Master Account (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-master-account-investor --email abc@test.com --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --transfers transfers_example --transfer-function-account-type SPOT
```

### [GET /sapi/v1/managed-subaccount/queryTransLogForTradeParent](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-transfer-log-master-account-trading) - Query Managed Sub Account Transfer Log For Trading Team Master Account (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-master-account-trading --email abc@test.com --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 10 --transfers transfers_example --transfer-function-account-type SPOT
```

### [GET /sapi/v1/managed-subaccount/query-trans-log](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#query-managed-sub-account-transfer-log-sub-account-trading) - Query Managed Sub Account Transfer Log (For Trading Team Sub Account) (USER_DATA)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-sub-account-trading --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 10 --transfers transfers_example --transfer-function-account-type SPOT --recv-window 5000
```

### [POST /sapi/v1/managed-subaccount/withdraw](https://developers.binance.com/en/docs/catalog/vip-and-institutional-sub-account/api/rest-api/managed-sub-account#withdrawl-assets-from-the-managed-sub-account) - Withdrawl Assets From The Managed Sub-account (For Investor Master Account) (USER_DATA)

```bash
binance-cli sub-account withdrawl-assets-from-the-managed-sub-account --from-email from@test.com --asset BTC --amount 1.0 --transfer-date 789 --recv-window 5000
```
