## MarketData

### [POST /sapi/v1/giftcard/buyCode](https://developers.binance.com/docs/gift_card/market-data/Create-a-dual-token-gift-card) - Create a dual-token gift card(fixed value, discount feature)

```bash
binance-cli gift-card create-a-dual-token-gift-card --json {}
```

### [POST /sapi/v1/giftcard/createCode](https://developers.binance.com/docs/gift_card/market-data/Create-a-single-token-gift-card) - Create a single-token gift card

```bash
binance-cli gift-card create-a-single-token-gift-card --json {}
```

### [GET /sapi/v1/giftcard/cryptography/rsa-public-key](https://developers.binance.com/docs/gift_card/market-data/Fetch-RSA-Public-Key) - Fetch RSA Public Key

```bash
binance-cli gift-card fetch-rsa-public-key --recv-window 5000
```

### [GET /sapi/v1/giftcard/buyCode/token-limit](https://developers.binance.com/docs/gift_card/market-data/Fetch-Token-Limit) - Fetch Token Limit

```bash
binance-cli gift-card fetch-token-limit --base-token "baseToken_example" --recv-window 5000
```

### [POST /sapi/v1/giftcard/redeemCode](https://developers.binance.com/docs/gift_card/market-data/Redeem-a-Binance-Gift-Card) - Redeem a Binance Gift Card

```bash
binance-cli gift-card redeem-a-binance-gift-card --json {}
```

### [GET /sapi/v1/giftcard/verify](https://developers.binance.com/docs/gift_card/market-data/Verify-Binance-Gift-Card-by-Gift-Card-Number) - Verify Binance Gift Card by Gift Card Number

```bash
binance-cli gift-card verify-binance-gift-card-by-gift-card-number --reference-no "referenceNo_example" --recv-window 5000
```
