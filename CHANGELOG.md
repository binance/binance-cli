# Change log

## 1.1.1 - 2026-04-13

### Updated

- Argument "--profile" takes precedence over env variables
- Validate profile name
- Use console.error for error messages

## 1.1.0 - 2026-04-11

### Added

- Added command to delete profile.

- Exit with error code on failed command.

- Added support for the following products:
    - Algo Trading
    - Alpha
    - C2C
    - Copy Trading
    - Crypto Loan
    - Derivatives Trading (COIN-M Futures)
    - Derivatives Trading (Options)
    - Derivatives Trading (Portfolio Margin)
    - Derivatives Trading (Portfolio Margin Pro)
    - Dual Investment
    - Fiat
    - Gift Card
    - Margin Trading
    - Mining
    - Pay
    - Rebate
    - Simple Earn
    - Staking
    - Sub Account
    - VIP Loan
    - Wallet

### Updated

- Add alias `profile select` for `profile change`
- Add option `--select` for `profile create` to select the profile after creation
- Add option `--force` for `profile create` to overwrite profile if existing

## 1.0.4 - 2026-04-09

### Updated

- Different User-Agent if used from AI Agent

## 1.0.3 - 2026-04-09

### Updated

- Simplified checks for required params

## 1.0.2 - 2026-04-09

### Added

- Add environment to profile commands

### Updated

- Add the request fields as options for POST/PUT endpoints
- Renamed `BINANCE_API_SECRET` to `BINANCE_SECRET_KEY`

## 1.0.1 - 2026-04-07

### Updated

- Accept json or list of params for POST/PUT endpoints

## 1.0.0 - 2026-04-07

- First release
