## EthStaking

### [GET /sapi/v2/eth-staking/account](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#eth-staking-account) - ETH Staking account (USER_DATA)

```bash
binance-cli staking eth-staking-account --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/quota](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-current-eth-staking-quota) - Get current ETH staking quota (USER_DATA)

```bash
binance-cli staking get-current-eth-staking-quota --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/redemptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-eth-redemption-history) - Get ETH redemption history (USER_DATA)

```bash
binance-cli staking get-eth-redemption-history --redeem-id 1234567 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/stakingHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-eth-staking-history) - Get ETH staking history (USER_DATA)

```bash
binance-cli staking get-eth-staking-history --purchase-id 1234567 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/rateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-wbeth-rate-history) - Get WBETH Rate History (USER_DATA)

```bash
binance-cli staking get-wbeth-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/wbethRewardsHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-wbeth-rewards-history) - Get WBETH rewards history (USER_DATA)

```bash
binance-cli staking get-wbeth-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/wbeth/history/unwrapHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-wbeth-unwrap-history) - Get WBETH unwrap history (USER_DATA)

```bash
binance-cli staking get-wbeth-unwrap-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/wbeth/history/wrapHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#get-wbeth-wrap-history) - Get WBETH wrap history (USER_DATA)

```bash
binance-cli staking get-wbeth-wrap-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/eth-staking/eth/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#redeem-eth) - Redeem ETH (TRADE)

```bash
binance-cli staking redeem-eth --amount 1.0 --asset BETH --recv-window 5000
```

### [POST /sapi/v2/eth-staking/eth/stake](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#subscribe-eth-staking) - Subscribe ETH Staking (TRADE)

```bash
binance-cli staking subscribe-eth-staking --amount 1.0 --recv-window 5000
```

### [POST /sapi/v1/eth-staking/wbeth/wrap](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/eth-staking#wrap-beth) - Wrap BETH (TRADE)

```bash
binance-cli staking wrap-beth --amount 1.0 --recv-window 5000
```

## OnChainYields

### [GET /sapi/v1/onchain-yields/locked/personalLeftQuota](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-personal-left-quota) - Get On-chain Yields Locked Personal Left Quota (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-personal-left-quota --project-id 1 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/list](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-product-list) - Get On-chain Yields Locked Product List (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-product-list --asset SOL --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/position](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-product-position) - Get On-chain Yields Locked Product Position (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-product-position --asset BTC --position-id 1 --project-id 1 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/redemptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-redemption-record) - Get On-chain Yields Locked Redemption Record (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-redemption-record --position-id 1 --redeem-id 1 --asset BTC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/rewardsRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-rewards-history) - Get On-chain Yields Locked Rewards History (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-rewards-history --position-id 1 --asset BTC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/subscriptionPreview](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-subscription-preview) - Get On-chain Yields Locked Subscription Preview (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-subscription-preview --project-id 1 --amount 1.0 --auto-subscribe true --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/subscriptionRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#get-on-chain-yields-locked-subscription-record) - Get On-chain Yields Locked Subscription Record (USER_DATA)

```bash
binance-cli staking get-on-chain-yields-locked-subscription-record --purchase-id 1 --client-id 1 --asset BTC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/account](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#on-chain-yields-account) - On-chain Yields Account (USER_DATA)

```bash
binance-cli staking on-chain-yields-account --recv-window 5000
```

### [POST /sapi/v1/onchain-yields/locked/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#redeem-on-chain-yields-locked-product) - Redeem On-chain Yields Locked Product (TRADE)

```bash
binance-cli staking redeem-on-chain-yields-locked-product --position-id 1 --channel-id 1 --recv-window 5000
```

### [POST /sapi/v1/onchain-yields/locked/setAutoSubscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#set-on-chain-yields-locked-auto-subscribe) - Set On-chain Yields Locked Auto Subscribe (USER_DATA)

```bash
binance-cli staking set-on-chain-yields-locked-auto-subscribe --position-id 1 --auto-subscribe true --recv-window 5000
```

### [POST /sapi/v1/onchain-yields/locked/setRedeemOption](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#set-on-chain-yields-locked-product-redeem-option) - Set On-chain Yields Locked Product Redeem Option (USER_DATA)

```bash
binance-cli staking set-on-chain-yields-locked-product-redeem-option --position-id 1 --redeem-to SPOT --recv-window 5000
```

### [POST /sapi/v1/onchain-yields/locked/subscribe](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/on-chain-yields#subscribe-on-chain-yields-locked-product) - Subscribe On-chain Yields Locked Product (TRADE)

```bash
binance-cli staking subscribe-on-chain-yields-locked-product --project-id 1 --amount 1.0 --auto-subscribe false --source-account SPOT --redeem-to FLEXIBLE --channel-id 1 --client-id 1 --recv-window 5000
```

## SoftStaking

### [GET /sapi/v1/soft-staking/list](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/soft-staking#get-soft-staking-product-list) - Get Soft Staking Product List (USER_DATA)

```bash
binance-cli staking get-soft-staking-product-list --asset BTC --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/soft-staking/history/rewardsRecord](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/soft-staking#get-soft-staking-rewards-history) - Get Soft Staking Rewards History (USER_DATA)

```bash
binance-cli staking get-soft-staking-rewards-history --asset BTC --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/soft-staking/set](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/soft-staking#set-soft-staking) - Set Soft Staking (USER_DATA)

```bash
binance-cli staking set-soft-staking --soft-staking true --recv-window 5000
```

## SolStaking

### [POST /sapi/v1/sol-staking/sol/claim](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#claim-boost-rewards) - Claim Boost Rewards (TRADE)

```bash
binance-cli staking claim-boost-rewards --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/rateHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-bnsol-rate-history) - Get BNSOL Rate History (USER_DATA)

```bash
binance-cli staking get-bnsol-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/bnsolRewardsHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-bnsol-rewards-history) - Get BNSOL rewards history (USER_DATA)

```bash
binance-cli staking get-bnsol-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/boostRewardsHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-boost-rewards-history) - Get Boost Rewards History (USER_DATA)

```bash
binance-cli staking get-boost-rewards-history --rtype CLAIM --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/redemptionHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-sol-redemption-history) - Get SOL redemption history (USER_DATA)

```bash
binance-cli staking get-sol-redemption-history --redeem-id 1234567 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/stakingHistory](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-sol-staking-history) - Get SOL staking history (USER_DATA)

```bash
binance-cli staking get-sol-staking-history --purchase-id 1234567 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/quota](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-sol-staking-quota-details) - Get SOL staking quota details (USER_DATA)

```bash
binance-cli staking get-sol-staking-quota-details --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/unclaimedRewards](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#get-unclaimed-rewards) - Get Unclaimed Rewards (USER_DATA)

```bash
binance-cli staking get-unclaimed-rewards --recv-window 5000
```

### [POST /sapi/v1/sol-staking/sol/redeem](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#redeem-sol) - Redeem SOL (TRADE)

```bash
binance-cli staking redeem-sol --amount 1.0 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/account](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#sol-staking-account) - SOL Staking account (USER_DATA)

```bash
binance-cli staking sol-staking-account --recv-window 5000
```

### [POST /sapi/v1/sol-staking/sol/stake](https://developers.binance.com/en/docs/catalog/investment-and-services-staking/api/rest-api/sol-staking#subscribe-sol-staking) - Subscribe SOL Staking (TRADE)

```bash
binance-cli staking subscribe-sol-staking --amount 1.0 --recv-window 5000
```
