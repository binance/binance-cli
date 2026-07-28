## MarketData

### [POST /sapi/v1/giftcard/buyCode](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#create-adual-token-gift-card) - Create a dual-token gift card (fixed value, discount feature) (TRADE)

```bash
binance-cli gift-card create-a-dual-token-gift-card --base-token BUSD --face-token BNB --base-token-amount 1 --recv-window 5000
```

### [POST /sapi/v1/giftcard/createCode](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#create-asingle-token-gift-card) - Create a single-token gift card (USER_DATA)

```bash
binance-cli gift-card create-a-single-token-gift-card --token BNB --amount 1 --recv-window 5000
```

### [GET /sapi/v1/giftcard/cryptography/rsa-public-key](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#fetch-rsa-public-key) - Fetch RSA Public Key (USER_DATA)

```bash
binance-cli gift-card fetch-rsa-public-key --recv-window 5000
```

### [GET /sapi/v1/giftcard/buyCode/token-limit](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#fetch-token-limit) - Fetch Token Limit (USER_DATA)

```bash
binance-cli gift-card fetch-token-limit --base-token BUSD --recv-window 5000
```

### [POST /sapi/v1/giftcard/redeemCode](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#redeem-abinance-gift-card) - Redeem a Binance Gift Card (USER_DATA)

```bash
binance-cli gift-card redeem-a-binance-gift-card --code 6H9EKF5ECCWFBHGE --external-uid user-123 --recv-window 5000
```

### [GET /sapi/v1/giftcard/verify](https://developers.binance.com/en/docs/catalog/investment-and-services-gift-card/api/rest-api/market-data#verify-binance-gift-card-by-gift-card-number) - Verify Binance Gift Card by Gift Card Number (USER_DATA)

```bash
binance-cli gift-card verify-binance-gift-card-by-gift-card-number --reference-no 0033002328060227 --recv-window 5000
```
