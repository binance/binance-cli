## EthStaking

### [GET /sapi/v2/eth-staking/account](https://developers.binance.com/docs/staking/eth-staking/account/ETH-Staking-account) - ETH Staking account

```bash
binance-cli staking eth-staking-account --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/quota](https://developers.binance.com/docs/staking/eth-staking/account/Get-current-ETH-staking-quota) - Get current ETH staking quota

```bash
binance-cli staking get-current-eth-staking-quota --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/redemptionHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-ETH-redemption-history) - Get ETH redemption history

```bash
binance-cli staking get-eth-redemption-history --redeem-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/stakingHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-ETH-staking-history) - Get ETH staking history

```bash
binance-cli staking get-eth-staking-history --purchase-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/rateHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-BETH-Rate-History) - Get WBETH Rate History

```bash
binance-cli staking get-wbeth-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/eth/history/wbethRewardsHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-WBETH-rewards-history) - Get WBETH rewards history

```bash
binance-cli staking get-wbeth-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/wbeth/history/unwrapHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-WBETH-unwrap-history) - Get WBETH unwrap history

```bash
binance-cli staking get-wbeth-unwrap-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/eth-staking/wbeth/history/wrapHistory](https://developers.binance.com/docs/staking/eth-staking/history/Get-WBETH-wrap-history) - Get WBETH wrap history

```bash
binance-cli staking get-wbeth-wrap-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [POST /sapi/v1/eth-staking/eth/redeem](https://developers.binance.com/docs/staking/eth-staking/staking/Redeem-ETH) - Redeem ETH

```bash
binance-cli staking redeem-eth --json {}
```

### [POST /sapi/v2/eth-staking/eth/stake](https://developers.binance.com/docs/staking/eth-staking/staking/Subscribe-ETH-Staking) - Subscribe ETH Staking

```bash
binance-cli staking subscribe-eth-staking --json {}
```

### [POST /sapi/v1/eth-staking/wbeth/wrap](https://developers.binance.com/docs/staking/eth-staking/staking/Wrap-BETH) - Wrap BETH

```bash
binance-cli staking wrap-beth --json {}
```

## OnChainYields

### [GET /sapi/v1/onchain-yields/locked/personalLeftQuota](https://developers.binance.com/docs/staking/on-chain-yields/account/Get-Onchain-Locked-Personal-Left-Quota) - Get On-chain Yields Locked Personal Left Quota

```bash
binance-cli staking get-on-chain-yields-locked-personal-left-quota --project-id "1" --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/list](https://developers.binance.com/docs/staking/on-chain-yields/account/) - Get On-chain Yields Locked Product List

```bash
binance-cli staking get-on-chain-yields-locked-product-list --asset "asset_example" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/position](https://developers.binance.com/docs/staking/on-chain-yields/account/Get-Onchain-Locked-Product-Position) - Get On-chain Yields Locked Product Position

```bash
binance-cli staking get-on-chain-yields-locked-product-position --asset "asset_example" --position-id "1" --project-id "1" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/redemptionRecord](https://developers.binance.com/docs/staking/on-chain-yields/history/Get-Onchain-Locked-Redemption-Record) - Get On-chain Yields Locked Redemption Record

```bash
binance-cli staking get-on-chain-yields-locked-redemption-record --position-id "1" --redeem-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/rewardsRecord](https://developers.binance.com/docs/staking/on-chain-yields/history/Get-Onchain-Locked-Rewards-History) - Get On-chain Yields Locked Rewards History

```bash
binance-cli staking get-on-chain-yields-locked-rewards-history --position-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/subscriptionPreview](https://developers.binance.com/docs/staking/on-chain-yields/earn/) - Get On-chain Yields Locked Subscription Preview

```bash
binance-cli staking get-on-chain-yields-locked-subscription-preview --project-id "1" --amount 1.0 --auto-subscribe false --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/locked/history/subscriptionRecord](https://developers.binance.com/docs/staking/on-chain-yields/history/) - Get On-chain Yields Locked Subscription Record

```bash
binance-cli staking get-on-chain-yields-locked-subscription-record --purchase-id "1" --client-id "1" --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/onchain-yields/account](https://developers.binance.com/docs/staking/on-chain-yields/account/Onchain-Account) - On-chain Yields Account

```bash
binance-cli staking on-chain-yields-account --recv-window 5000
```

### [POST /sapi/v1/onchain-yields/locked/redeem](https://developers.binance.com/docs/staking/on-chain-yields/earn/Redeem-Onchain-Locked-Product) - Redeem On-chain Yields Locked Product

```bash
binance-cli staking redeem-on-chain-yields-locked-product --json {}
```

### [POST /sapi/v1/onchain-yields/locked/setAutoSubscribe](https://developers.binance.com/docs/staking/on-chain-yields/earn/Set-Onchain-Locked-Auto-Subscribe) - Set On-chain Yields Locked Auto Subscribe

```bash
binance-cli staking set-on-chain-yields-locked-auto-subscribe --json {}
```

### [POST /sapi/v1/onchain-yields/locked/setRedeemOption](https://developers.binance.com/docs/staking/on-chain-yields/earn/Set-Onchain-Locked-Redeem-Option) - Set On-chain Yields Locked Product Redeem Option

```bash
binance-cli staking set-on-chain-yields-locked-product-redeem-option --json {}
```

### [POST /sapi/v1/onchain-yields/locked/subscribe](https://developers.binance.com/docs/staking/on-chain-yields/earn/Subscribe-Onchain-Locked-Product) - Subscribe On-chain Yields Locked Product

```bash
binance-cli staking subscribe-on-chain-yields-locked-product --json {}
```

## SoftStaking

### [GET /sapi/v1/soft-staking/list](https://developers.binance.com/docs/staking/soft-staking/) - Get Soft Staking Product List

```bash
binance-cli staking get-soft-staking-product-list --asset "asset_example" --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/soft-staking/history/rewardsRecord](https://developers.binance.com/docs/staking/soft-staking/Get-Soft-Staking-Rewards-History) - Get Soft Staking Rewards History

```bash
binance-cli staking get-soft-staking-rewards-history --asset "asset_example" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/soft-staking/set](https://developers.binance.com/docs/staking/soft-staking/Set-Soft-Staking) - Set Soft Staking

```bash
binance-cli staking set-soft-staking --soft-staking true --recv-window 5000
```

## SolStaking

### [POST /sapi/v1/sol-staking/sol/claim](https://developers.binance.com/docs/staking/sol-staking/staking/Claim-Boost-Rewards) - Claim Boost Rewards

```bash
binance-cli staking claim-boost-rewards --json {}
```

### [GET /sapi/v1/sol-staking/sol/history/rateHistory](https://developers.binance.com/docs/staking/sol-staking/history/Get-BNSOL-Rate-History) - Get BNSOL Rate History

```bash
binance-cli staking get-bnsol-rate-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/bnsolRewardsHistory](https://developers.binance.com/docs/staking/sol-staking/history/Get-BNSOL-rewards-history) - Get BNSOL rewards history

```bash
binance-cli staking get-bnsol-rewards-history --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/boostRewardsHistory](https://developers.binance.com/docs/staking/sol-staking/history/Get-Boost-Rewards-History) - Get Boost Rewards History

```bash
binance-cli staking get-boost-rewards-history --type "CLAIM" --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/redemptionHistory](https://developers.binance.com/docs/staking/sol-staking/history/Get-SOL-redemption-history) - Get SOL redemption history

```bash
binance-cli staking get-sol-redemption-history --redeem-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/stakingHistory](https://developers.binance.com/docs/staking/sol-staking/history/Get-SOL-staking-history) - Get SOL staking history

```bash
binance-cli staking get-sol-staking-history --purchase-id 1 --start-time 1623319461670 --end-time 1641782889000 --current 1 --size 10 --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/quota](https://developers.binance.com/docs/staking/sol-staking/account/Get-SOL-staking-quota-details) - Get SOL staking quota details

```bash
binance-cli staking get-sol-staking-quota-details --recv-window 5000
```

### [GET /sapi/v1/sol-staking/sol/history/unclaimedRewards](https://developers.binance.com/docs/staking/sol-staking/history/Get-Unclaimed-Rewards) - Get Unclaimed Rewards

```bash
binance-cli staking get-unclaimed-rewards --recv-window 5000
```

### [POST /sapi/v1/sol-staking/sol/redeem](https://developers.binance.com/docs/staking/sol-staking/staking/Redeem-SOL) - Redeem SOL

```bash
binance-cli staking redeem-sol --json {}
```

### [GET /sapi/v1/sol-staking/account](https://developers.binance.com/docs/staking/sol-staking/account/SOL-Staking-account) - SOL Staking account

```bash
binance-cli staking sol-staking-account --recv-window 5000
```

### [POST /sapi/v1/sol-staking/sol/stake](https://developers.binance.com/docs/staking/sol-staking/staking/Subscribe-SOL-Staking) - Subscribe SOL Staking

```bash
binance-cli staking subscribe-sol-staking --json {}
```
