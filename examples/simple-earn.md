## Bfusd

### [GET /sapi/v1/bfusd/account](https://developers.binance.com/docs/simple_earn/bfusd/account/) - Get BFUSD Account

```bash
binance-cli simple-earn get-bfusd-account --recv-window 5000
```

### [GET /sapi/v1/bfusd/quota](https://developers.binance.com/docs/simple_earn/bfusd/account/Get-BFUSD-Quota-Details) - Get BFUSD Quota Details

```bash
binance-cli simple-earn get-bfusd-quota-details --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/rateHistory](https://developers.binance.com/docs/simple_earn/bfusd/history/Get-BFUSD-Rate-History) - Get BFUSD Rate History

```bash
binance-cli simple-earn get-bfusd-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/redemptionHistory](https://developers.binance.com/docs/simple_earn/bfusd/history/Get-BFUSD-Redemption-History) - Get BFUSD Redemption History

```bash
binance-cli simple-earn get-bfusd-redemption-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/rewardsHistory](https://developers.binance.com/docs/simple_earn/bfusd/history/Get-BFUSD-Rewards-History) - Get BFUSD Rewards History

```bash
binance-cli simple-earn get-bfusd-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/bfusd/history/subscriptionHistory](https://developers.binance.com/docs/simple_earn/bfusd/history/Get-BFUSD-subscription-history) - Get BFUSD subscription history

```bash
binance-cli simple-earn get-bfusd-subscription-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/bfusd/redeem](https://developers.binance.com/docs/simple_earn/bfusd/earn/Redeem-BFUSD) - Redeem BFUSD

```bash
binance-cli simple-earn redeem-bfusd --json {}
```

### [POST /sapi/v1/bfusd/subscribe](https://developers.binance.com/docs/simple_earn/bfusd/earn/Subscribe-BFUSD) - Subscribe BFUSD

```bash
binance-cli simple-earn subscribe-bfusd --json {}
```

## FlexibleLocked

### [GET /sapi/v1/simple-earn/flexible/history/collateralRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Collateral-Record) - Get Collateral Record

```bash
binance-cli simple-earn get-collateral-record --product-id "1" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/personalLeftQuota](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Flexible-Personal-Left-Quota) - Get Flexible Personal Left Quota

```bash
binance-cli simple-earn get-flexible-personal-left-quota --product-id "1" --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/position](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Flexible-Product-Position) - Get Flexible Product Position

```bash
binance-cli simple-earn get-flexible-product-position --asset "asset_example" --product-id "1" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/redemptionRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Flexible-Redemption-Record) - Get Flexible Redemption Record

```bash
binance-cli simple-earn get-flexible-redemption-record --product-id "1" --redeem-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/rewardsRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Flexible-Rewards-History) - Get Flexible Rewards History

```bash
binance-cli simple-earn get-flexible-rewards-history --type "s" --product-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/subscriptionPreview](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Get-Flexible-Subscription-Preview) - Get Flexible Subscription Preview

```bash
binance-cli simple-earn get-flexible-subscription-preview --product-id "1" --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/subscriptionRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Flexible-Subscription-Record) - Get Flexible Subscription Record

```bash
binance-cli simple-earn get-flexible-subscription-record --product-id "1" --purchase-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/personalLeftQuota](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Locked-Personal-Left-Quota) - Get Locked Personal Left Quota

```bash
binance-cli simple-earn get-locked-personal-left-quota --project-id "1" --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/position](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Locked-Product-Position) - Get Locked Product Position

```bash
binance-cli simple-earn get-locked-product-position --asset "asset_example" --position-id "1" --project-id "1" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/redemptionRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Locked-Redemption-Record) - Get Locked Redemption Record

```bash
binance-cli simple-earn get-locked-redemption-record --position-id "1" --redeem-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/rewardsRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Locked-Rewards-History) - Get Locked Rewards History

```bash
binance-cli simple-earn get-locked-rewards-history --position-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/subscriptionPreview](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Get-Locked-Subscription-Preview) - Get Locked Subscription Preview

```bash
binance-cli simple-earn get-locked-subscription-preview --project-id "1" --amount 1.0 --auto-subscribe false --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/history/subscriptionRecord](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Locked-Subscription-Record) - Get Locked Subscription Record

```bash
binance-cli simple-earn get-locked-subscription-record --purchase-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/history/rateHistory](https://developers.binance.com/docs/simple_earn/flexible-locked/history/Get-Rate-History) - Get Rate History

```bash
binance-cli simple-earn get-rate-history --product-id "1" --apr-period "DAY" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/flexible/list](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Simple-Earn-Flexible-Product-List) - Get Simple Earn Flexible Product List

```bash
binance-cli simple-earn get-simple-earn-flexible-product-list --asset "asset_example" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/simple-earn/locked/list](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Get-Simple-Earn-Locked-Product-List) - Get Simple Earn Locked Product List

```bash
binance-cli simple-earn get-simple-earn-locked-product-list --asset "asset_example" --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/simple-earn/flexible/redeem](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Redeem-Flexible-Product) - Redeem Flexible Product

```bash
binance-cli simple-earn redeem-flexible-product --json {}
```

### [POST /sapi/v1/simple-earn/locked/redeem](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Redeem-Locked-Product) - Redeem Locked Product

```bash
binance-cli simple-earn redeem-locked-product --json {}
```

### [POST /sapi/v1/simple-earn/flexible/setAutoSubscribe](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Set-Flexible-Auto-Subscribe) - Set Flexible Auto Subscribe

```bash
binance-cli simple-earn set-flexible-auto-subscribe --json {}
```

### [POST /sapi/v1/simple-earn/locked/setAutoSubscribe](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Set-Locked-Auto-Subscribe) - Set Locked Auto Subscribe

```bash
binance-cli simple-earn set-locked-auto-subscribe --json {}
```

### [POST /sapi/v1/simple-earn/locked/setRedeemOption](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Set-Locked-Redeem-Option) - Set Locked Product Redeem Option

```bash
binance-cli simple-earn set-locked-product-redeem-option --json {}
```

### [GET /sapi/v1/simple-earn/account](https://developers.binance.com/docs/simple_earn/flexible-locked/account/Simple-Account) - Simple Account

```bash
binance-cli simple-earn simple-account --recv-window 5000
```

### [POST /sapi/v1/simple-earn/flexible/subscribe](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Subscribe-Flexible-Product) - Subscribe Flexible Product

```bash
binance-cli simple-earn subscribe-flexible-product --json {}
```

### [POST /sapi/v1/simple-earn/locked/subscribe](https://developers.binance.com/docs/simple_earn/flexible-locked/earn/Subscribe-Locked-Product) - Subscribe Locked Product

```bash
binance-cli simple-earn subscribe-locked-product --json {}
```

## Rwusd

### [GET /sapi/v1/rwusd/account](https://developers.binance.com/docs/simple_earn/rwusd/account/) - Get RWUSD Account

```bash
binance-cli simple-earn get-rwusd-account --recv-window 5000
```

### [GET /sapi/v1/rwusd/quota](https://developers.binance.com/docs/simple_earn/rwusd/account/Get-RWUSD-Quota-Details) - Get RWUSD Quota Details

```bash
binance-cli simple-earn get-rwusd-quota-details --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/rateHistory](https://developers.binance.com/docs/simple_earn/rwusd/history/Get-RWUSD-Rate-History) - Get RWUSD Rate History

```bash
binance-cli simple-earn get-rwusd-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/redemptionHistory](https://developers.binance.com/docs/simple_earn/rwusd/history/Get-RWUSD-Redemption-History) - Get RWUSD Redemption History

```bash
binance-cli simple-earn get-rwusd-redemption-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/rewardsHistory](https://developers.binance.com/docs/simple_earn/rwusd/history/Get-RWUSD-Rewards-History) - Get RWUSD Rewards History

```bash
binance-cli simple-earn get-rwusd-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/rwusd/history/subscriptionHistory](https://developers.binance.com/docs/simple_earn/rwusd/history/Get-RWUSD-subscription-history) - Get RWUSD subscription history

```bash
binance-cli simple-earn get-rwusd-subscription-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/rwusd/redeem](https://developers.binance.com/docs/simple_earn/rwusd/earn/Redeem-RWUSD) - Redeem RWUSD

```bash
binance-cli simple-earn redeem-rwusd --json {}
```

### [POST /sapi/v1/rwusd/subscribe](https://developers.binance.com/docs/simple_earn/rwusd/earn/Subscribe-RWUSD) - Subscribe RWUSD

```bash
binance-cli simple-earn subscribe-rwusd --json {}
```
