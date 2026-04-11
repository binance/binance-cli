## AccountManagement

### [POST /sapi/v1/sub-account/virtualSubAccount](https://developers.binance.com/docs/sub_account/account-management/Create-a-Virtual-Sub-account) - Create a Virtual Sub-account (For Master Account)

```bash
binance-cli sub-account create-a-virtual-sub-account --json {}
```

### [POST /sapi/v1/sub-account/futures/enable](https://developers.binance.com/docs/sub_account/account-management/Enable-Futures-for-Sub-account) - Enable Futures for Sub-account (For Master Account)

```bash
binance-cli sub-account enable-futures-for-sub-account --json {}
```

### [POST /sapi/v1/sub-account/eoptions/enable](https://developers.binance.com/docs/sub_account/account-management/Enable-Options-for-Sub-account) - Enable Options for Sub-account (For Master Account)

```bash
binance-cli sub-account enable-options-for-sub-account --json {}
```

### [GET /sapi/v1/sub-account/futures/positionRisk](https://developers.binance.com/docs/sub_account/account-management/Get-Futures-Position-Risk-of-Sub-account) - Get Futures Position-Risk of Sub-account (For Master Account)

```bash
binance-cli sub-account get-futures-position-risk-of-sub-account --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/positionRisk](https://developers.binance.com/docs/sub_account/account-management/Get-Futures-Position-Risk-of-Sub-account-V2) - Get Futures Position-Risk of Sub-account V2 (For Master Account)

```bash
binance-cli sub-account get-futures-position-risk-of-sub-account-v2 --email "sub-account-email@email.com" --futures-type 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/status](https://developers.binance.com/docs/sub_account/account-management/Get-Sub-accounts-Status-on-Margin-Or-Futures) - Get Sub-account\'s Status on Margin Or Futures (For Master Account)

```bash
binance-cli sub-account get-sub-accounts-status-on-margin-or-futures --email "email_example" --recv-window 5000
```

### [GET /sapi/v1/sub-account/list](https://developers.binance.com/docs/sub_account/account-management/Query-Sub-account-List) - Query Sub-account List (For Master Account)

```bash
binance-cli sub-account query-sub-account-list --email "email_example" --is-freeze "isFreeze_example" --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/transaction-statistics](https://developers.binance.com/docs/sub_account/account-management/Query-Sub-account-Transaction-Statistics) - Query Sub-account Transaction Statistics (For Master Account)

```bash
binance-cli sub-account query-sub-account-transaction-statistics --email "email_example" --recv-window 5000
```

## ApiManagement

### [POST /sapi/v2/sub-account/subAccountApi/ipRestriction](https://developers.binance.com/docs/sub_account/api-management/Add-IP-Restriction-for-Sub-Account-API-key) - Add IP Restriction for Sub-Account API key (For Master Account)

```bash
binance-cli sub-account add-ip-restriction-for-sub-account-api-key --json {}
```

### [DELETE /sapi/v1/sub-account/subAccountApi/ipRestriction/ipList](https://developers.binance.com/docs/sub_account/api-management/Delete-IP-List-For-a-Sub-account-API-Key) - Delete IP List For a Sub-account API Key (For Master Account)

```bash
binance-cli sub-account delete-ip-list-for-a-sub-account-api-key --email "sub-account-email@email.com" --sub-account-api-key "subAccountApiKey_example" --ip-address "ipAddress_example" --recv-window 5000
```

### [GET /sapi/v1/sub-account/subAccountApi/ipRestriction](https://developers.binance.com/docs/sub_account/api-management/Get-IP-Restriction-for-a-Sub-account-API-Key) - Get IP Restriction for a Sub-account API Key (For Master Account)

```bash
binance-cli sub-account get-ip-restriction-for-a-sub-account-api-key --email "sub-account-email@email.com" --sub-account-api-key "subAccountApiKey_example" --recv-window 5000
```

## AssetManagement

### [POST /sapi/v1/sub-account/futures/transfer](https://developers.binance.com/docs/sub_account/asset-management/Futures-Transfer-for-Sub-account) - Futures Transfer for Sub-account (For Master Account)

```bash
binance-cli sub-account futures-transfer-for-sub-account --json {}
```

### [GET /sapi/v1/sub-account/futures/account](https://developers.binance.com/docs/sub_account/asset-management/Get-Detail-on-Sub-accounts-Futures-Account) - Get Detail on Sub-account\'s Futures Account (For Master Account)

```bash
binance-cli sub-account get-detail-on-sub-accounts-futures-account --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/account](https://developers.binance.com/docs/sub_account/asset-management/Get-Detail-on-Sub-accounts-Futures-Account-V2) - Get Detail on Sub-account\'s Futures Account V2 (For Master Account)

```bash
binance-cli sub-account get-detail-on-sub-accounts-futures-account-v2 --email "sub-account-email@email.com" --futures-type 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/margin/account](https://developers.binance.com/docs/sub_account/asset-management/Get-Detail-on-Sub-accounts-Margin-Account) - Get Detail on Sub-account\'s Margin Account (For Master Account)

```bash
binance-cli sub-account get-detail-on-sub-accounts-margin-account --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/move-position](https://developers.binance.com/docs/sub_account/asset-management/Get-Move-Position-History-for-Sub-account) - Get Move Position History for Sub-account (For Master Account)

```bash
binance-cli sub-account get-move-position-history-for-sub-account --symbol "symbol_example" --page 1 --row 1 --start-time 1623319461670 --end-time 1641782889000 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/subAddress](https://developers.binance.com/docs/sub_account/asset-management/Get-Sub-account-Deposit-Address) - Get Sub-account Deposit Address (For Master Account)

```bash
binance-cli sub-account get-sub-account-deposit-address --email "sub-account-email@email.com" --coin "coin_example" --network "network_example" --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/capital/deposit/subHisrec](https://developers.binance.com/docs/sub_account/asset-management/Get-Sub-account-Deposit-History) - Get Sub-account Deposit History (For Master Account)

```bash
binance-cli sub-account get-sub-account-deposit-history --email "sub-account-email@email.com" --coin "coin_example" --status 1 --start-time 1623319461670 --end-time 1641782889000 --limit 1 --offset 0 --recv-window 5000 --tx-id "1"
```

### [GET /sapi/v1/sub-account/futures/accountSummary](https://developers.binance.com/docs/sub_account/asset-management/Get-Summary-of-Sub-accounts-Futures-Account) - Get Summary of Sub-account\'s Futures Account (For Master Account)

```bash
binance-cli sub-account get-summary-of-sub-accounts-futures-account --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v2/sub-account/futures/accountSummary](https://developers.binance.com/docs/sub_account/asset-management/Get-Summary-of-Sub-accounts-Futures-Account-V2) - Get Summary of Sub-account\'s Futures Account V2 (For Master Account)

```bash
binance-cli sub-account get-summary-of-sub-accounts-futures-account-v2 --futures-type 1 --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/margin/accountSummary](https://developers.binance.com/docs/sub_account/asset-management/Get-Summary-of-Sub-accounts-Margin-Account) - Get Summary of Sub-account\'s Margin Account (For Master Account)

```bash
binance-cli sub-account get-summary-of-sub-accounts-margin-account --recv-window 5000
```

### [POST /sapi/v1/sub-account/margin/transfer](https://developers.binance.com/docs/sub_account/asset-management/Margin-Transfer-for-Sub-account) - Margin Transfer for Sub-account (For Master Account)

```bash
binance-cli sub-account margin-transfer-for-sub-account --json {}
```

### [POST /sapi/v1/sub-account/futures/move-position](https://developers.binance.com/docs/sub_account/asset-management/Move-Position-for-Sub-account) - Move Position for Sub-account (For Master Account)

```bash
binance-cli sub-account move-position-for-sub-account --json {}
```

### [GET /sapi/v3/sub-account/assets](https://developers.binance.com/docs/sub_account/asset-management/Query-Sub-account-Assets-V4) - Query Sub-account Assets (For Master Account)

```bash
binance-cli sub-account query-sub-account-assets --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v4/sub-account/assets](https://developers.binance.com/docs/sub_account/asset-management/Query-Sub-account-Assets-V4) - Query Sub-account Assets (For Master Account)

```bash
binance-cli sub-account query-sub-account-assets-asset-management --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v1/sub-account/futures/internalTransfer](https://developers.binance.com/docs/sub_account/asset-management/Query-Sub-account-Futures-Asset-Transfer-History) - Query Sub-account Futures Asset Transfer History (For Master Account)

```bash
binance-cli sub-account query-sub-account-futures-asset-transfer-history --email "sub-account-email@email.com" --futures-type 1 --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/sub/transfer/history](https://developers.binance.com/docs/sub_account/asset-management/Query-Sub-account-Spot-Asset-Transfer-History) - Query Sub-account Spot Asset Transfer History (For Master Account)

```bash
binance-cli sub-account query-sub-account-spot-asset-transfer-history --from-email "fromEmail_example" --to-email "toEmail_example" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/sub-account/spotSummary](https://developers.binance.com/docs/sub_account/asset-management/Query-Sub-account-Spot-Assets-Summary) - Query Sub-account Spot Assets Summary (For Master Account)

```bash
binance-cli sub-account query-sub-account-spot-assets-summary --email "email_example" --page 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sub-account/universalTransfer](https://developers.binance.com/docs/sub_account/asset-management/Query-Universal-Transfer-History) - Query Universal Transfer History (For Master Account)

```bash
binance-cli sub-account query-universal-transfer-history --from-email "fromEmail_example" --to-email "toEmail_example" --client-tran-id "1" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --recv-window 5000
```

### [POST /sapi/v1/sub-account/futures/internalTransfer](https://developers.binance.com/docs/sub_account/asset-management/Sub-account-Futures-Asset-Transfer) - Sub-account Futures Asset Transfer (For Master Account)

```bash
binance-cli sub-account sub-account-futures-asset-transfer --json {}
```

### [GET /sapi/v1/sub-account/transfer/subUserHistory](https://developers.binance.com/docs/sub_account/asset-management/Sub-account-Transfer-History) - Sub-account Transfer History (For Sub-account)

```bash
binance-cli sub-account sub-account-transfer-history --asset "asset_example" --type 1 --start-time 1623319461670 --end-time 1641782889000 --limit 1 --return-fail-history false --recv-window 5000
```

### [POST /sapi/v1/sub-account/transfer/subToMaster](https://developers.binance.com/docs/sub_account/asset-management/Transfer-to-Master) - Transfer to Master (For Sub-account)

```bash
binance-cli sub-account transfer-to-master --json {}
```

### [POST /sapi/v1/sub-account/transfer/subToSub](https://developers.binance.com/docs/sub_account/asset-management/Transfer-to-Sub-account-of-Same-Master) - Transfer to Sub-account of Same Master (For Sub-account)

```bash
binance-cli sub-account transfer-to-sub-account-of-same-master --json {}
```

### [POST /sapi/v1/sub-account/universalTransfer](https://developers.binance.com/docs/sub_account/asset-management/Universal-Transfer) - Universal Transfer (For Master Account)

```bash
binance-cli sub-account universal-transfer --json {}
```

## ManagedSubAccount

### [POST /sapi/v1/managed-subaccount/deposit](https://developers.binance.com/docs/sub_account/managed-sub-account/Deposit-Assets-Into-The-Managed-Sub-account) - Deposit Assets Into The Managed Sub-account (For Investor Master Account)

```bash
binance-cli sub-account deposit-assets-into-the-managed-sub-account --json {}
```

### [GET /sapi/v1/managed-subaccount/deposit/address](https://developers.binance.com/docs/sub_account/managed-sub-account/Get-Managed-Sub-account-Deposit-Address) - Get Managed Sub-account Deposit Address (For Investor Master Account)

```bash
binance-cli sub-account get-managed-sub-account-deposit-address --email "sub-account-email@email.com" --coin "coin_example" --network "network_example" --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/asset](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-account-Asset-Details) - Query Managed Sub-account Asset Details (For Investor Master Account)

```bash
binance-cli sub-account query-managed-sub-account-asset-details --email "sub-account-email@email.com" --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/fetch-future-asset](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-account-Futures-Asset-Details) - Query Managed Sub-account Futures Asset Details (For Investor Master Account)

```bash
binance-cli sub-account query-managed-sub-account-futures-asset-details --email "sub-account-email@email.com" --account-type "accountType_example"
```

### [GET /sapi/v1/managed-subaccount/info](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-account-List) - Query Managed Sub-account List (For Investor)

```bash
binance-cli sub-account query-managed-sub-account-list --email "email_example" --page 1 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/marginAsset](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-account-Margin-Asset-Details) - Query Managed Sub-account Margin Asset Details (For Investor Master Account)

```bash
binance-cli sub-account query-managed-sub-account-margin-asset-details --email "sub-account-email@email.com" --account-type "accountType_example"
```

### [GET /sapi/v1/managed-subaccount/accountSnapshot](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-account-Snapshot) - Query Managed Sub-account Snapshot (For Investor Master Account)

```bash
binance-cli sub-account query-managed-sub-account-snapshot --email "sub-account-email@email.com" --type "type_example" --start-time 1623319461670 --end-time 1641782889000 --limit 1 --recv-window 5000
```

### [GET /sapi/v1/managed-subaccount/queryTransLogForInvestor](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-Account-Transfer-Log-Investor) - Query Managed Sub Account Transfer Log (For Investor Master Account)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-master-account-investor --email "sub-account-email@email.com" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --transfers "transfers_example" --transfer-function-account-type "transferFunctionAccountType_example"
```

### [GET /sapi/v1/managed-subaccount/queryTransLogForTradeParent](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-Account-Transfer-Log-Trading-Team-Master) - Query Managed Sub Account Transfer Log (For Trading Team Master Account)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-master-account-trading --email "sub-account-email@email.com" --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --transfers "transfers_example" --transfer-function-account-type "transferFunctionAccountType_example"
```

### [GET /sapi/v1/managed-subaccount/query-trans-log](https://developers.binance.com/docs/sub_account/managed-sub-account/Query-Managed-Sub-Account-Transfer-Log-Trading-Team-Sub) - Query Managed Sub Account Transfer Log (For Trading Team Sub Account)

```bash
binance-cli sub-account query-managed-sub-account-transfer-log-sub-account-trading --start-time 1623319461670 --end-time 1641782889000 --page 1 --limit 1 --transfers "transfers_example" --transfer-function-account-type "transferFunctionAccountType_example" --recv-window 5000
```

### [POST /sapi/v1/managed-subaccount/withdraw](https://developers.binance.com/docs/sub_account/managed-sub-account/Withdrawl-Assets-From-The-Managed-Sub-account) - Withdrawl Assets From The Managed Sub-account (For Investor Master Account)

```bash
binance-cli sub-account withdrawl-assets-from-the-managed-sub-account --json {}
```
