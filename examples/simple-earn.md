## Bfusd

### [GET /sapi/v1/bfusd/account](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-account) - Get BFUSD Account (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-account --recv-window 5000
```

### [GET /sapi/v1/bfusd/quota](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-quota-details) - Get BFUSD Quota Details (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-quota-details --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/rateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-rate-history) - Get BFUSD Rate History (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/redemptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-redemption-history) - Get BFUSD Redemption History (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-redemption-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/rewardsHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-rewards-history) - Get BFUSD Rewards History (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/subscriptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#get-bfusd-subscription-history) - Get BFUSD subscription history (USER_DATA)

```bash
binance-cli simple-earn get-bfusd-subscription-history --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/bfusd/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#redeem-bfusd) - Redeem BFUSD (TRADE)

```bash
binance-cli simple-earn redeem-bfusd --amount 1.0 --rtype STANDARD --recv-window 5000
```

### [POST /sapi/v1/bfusd/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/bfusd#subscribe-bfusd) - Subscribe BFUSD (TRADE)

```bash
binance-cli simple-earn subscribe-bfusd --asset USDT --amount 1.0 --recv-window 5000
```

## FlexibleLocked

### [GET /sapi/v1/simple-earn/flexible/history/collateralRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-collateral-record) - Get Collateral Record (USER_DATA)

```bash
binance-cli simple-earn get-collateral-record --product-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/personalLeftQuota](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-personal-left-quota) - Get Flexible Personal Left Quota (USER_DATA)

```bash
binance-cli simple-earn get-flexible-personal-left-quota --product-id 1 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/position](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-product-position) - Get Flexible Product Position (USER_DATA)

```bash
binance-cli simple-earn get-flexible-product-position --asset USDC --product-id 1 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/redemptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-redemption-record) - Get Flexible Redemption Record (USER_DATA)

```bash
binance-cli simple-earn get-flexible-redemption-record --product-id 1 --redeem-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/rewardsRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-rewards-history) - Get Flexible Rewards History (USER_DATA)

```bash
binance-cli simple-earn get-flexible-rewards-history --product-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --rtype ALL --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/subscriptionPreview](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-subscription-preview) - Get Flexible Subscription Preview (USER_DATA)

```bash
binance-cli simple-earn get-flexible-subscription-preview --product-id 1 --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/subscriptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-flexible-subscription-record) - Get Flexible Subscription Record (USER_DATA)

```bash
binance-cli simple-earn get-flexible-subscription-record --product-id 1 --purchase-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/personalLeftQuota](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-personal-left-quota) - Get Locked Personal Left Quota (USER_DATA)

```bash
binance-cli simple-earn get-locked-personal-left-quota --project-id 1 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/position](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-product-position) - Get Locked Product Position (USER_DATA)

```bash
binance-cli simple-earn get-locked-product-position --asset USDC --position-id 1 --project-id 1 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/redemptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-redemption-record) - Get Locked Redemption Record (USER_DATA)

```bash
binance-cli simple-earn get-locked-redemption-record --position-id 1 --redeem-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/rewardsRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-rewards-history) - Get Locked Rewards History (USER_DATA)

```bash
binance-cli simple-earn get-locked-rewards-history --position-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/subscriptionPreview](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-subscription-preview) - Get Locked Subscription Preview (USER_DATA)

```bash
binance-cli simple-earn get-locked-subscription-preview --project-id 1 --amount 1.0 --auto-subscribe true --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/subscriptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-locked-subscription-record) - Get Locked Subscription Record (USER_DATA)

```bash
binance-cli simple-earn get-locked-subscription-record --purchase-id 1 --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/rateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-rate-history) - Get Rate History (USER_DATA)

```bash
binance-cli simple-earn get-rate-history --product-id 1 --apr-period DAY --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/list](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-simple-earn-flexible-product-list) - Get Simple Earn Flexible Product List (USER_DATA)

```bash
binance-cli simple-earn get-simple-earn-flexible-product-list --asset USDC --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/list](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#get-simple-earn-locked-product-list) - Get Simple Earn Locked Product List (USER_DATA)

```bash
binance-cli simple-earn get-simple-earn-locked-product-list --asset USDC --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/simple-earn/flexible/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#redeem-flexible-product) - Redeem Flexible Product (TRADE)

```bash
binance-cli simple-earn redeem-flexible-product --product-id 1 --redeem-all false --amount 1.0 --dest-account SPOT --recv-window 5000
```

### [POST /sapi/v1/simple-earn/locked/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#redeem-locked-product) - Redeem Locked Product (TRADE)

```bash
binance-cli simple-earn redeem-locked-product --position-id 1 --recv-window 5000
```

### [POST /sapi/v1/simple-earn/flexible/setAutoSubscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#set-flexible-auto-subscribe) - Set Flexible Auto Subscribe (USER_DATA)

```bash
binance-cli simple-earn set-flexible-auto-subscribe --product-id 1 --auto-subscribe true --recv-window 5000
```

### [POST /sapi/v1/simple-earn/locked/setAutoSubscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#set-locked-auto-subscribe) - Set Locked Auto Subscribe (USER_DATA)

```bash
binance-cli simple-earn set-locked-auto-subscribe --position-id 1 --auto-subscribe true --recv-window 5000
```

### [POST /sapi/v1/simple-earn/locked/setRedeemOption](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#set-locked-product-redeem-option) - Set Locked Product Redeem Option (USER_DATA)

```bash
binance-cli simple-earn set-locked-product-redeem-option --position-id 1 --redeem-to SPOT --recv-window 5000
```

### [GET /sapi/v1/simple-earn/account](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#simple-account) - Simple Account (USER_DATA)

```bash
binance-cli simple-earn simple-account --recv-window 5000
```

### [POST /sapi/v1/simple-earn/flexible/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#subscribe-flexible-product) - Subscribe Flexible Product (TRADE)

```bash
binance-cli simple-earn subscribe-flexible-product --product-id 1 --amount 1.0 --auto-subscribe true --source-account SPOT --recv-window 5000
```

### [POST /sapi/v1/simple-earn/locked/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/flexible-locked#subscribe-locked-product) - Subscribe Locked Product (TRADE)

```bash
binance-cli simple-earn subscribe-locked-product --project-id 1 --amount 1.0 --auto-subscribe false --source-account SPOT --redeem-to SPOT --recv-window 5000
```

## Rwusd

### [GET /sapi/v1/rwusd/account](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-account) - Get RWUSD Account (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-account --recv-window 5000
```

### [GET /sapi/v1/rwusd/quota](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-quota-details) - Get RWUSD Quota Details (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-quota-details --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/rateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-rate-history) - Get RWUSD Rate History (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/redemptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-redemption-history) - Get RWUSD Redemption History (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-redemption-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/rewardsHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-rewards-history) - Get RWUSD Rewards History (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/subscriptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#get-rwusd-subscription-history) - Get RWUSD subscription history (USER_DATA)

```bash
binance-cli simple-earn get-rwusd-subscription-history --asset USDC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/rwusd/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#redeem-rwusd) - Redeem RWUSD (TRADE)

```bash
binance-cli simple-earn redeem-rwusd --amount 1.0 --rtype STANDARD --recv-window 5000
```

### [POST /sapi/v1/rwusd/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/rwusd#subscribe-rwusd) - Subscribe RWUSD (TRADE)

```bash
binance-cli simple-earn subscribe-rwusd --asset USDC --amount 1.0 --recv-window 5000
```

## YieldArena

### [GET /sapi/v1/earn/arena/activities](https://developers.binance.com/en/docs/catalog/investment-and-services-simple-earn/api/rest-api/yield-arena#get-yield-arena-activities) - Get Yield Arena Activities (USER_DATA)

```bash
binance-cli simple-earn get-yield-arena-activities --lang en --recv-window 5000
```
