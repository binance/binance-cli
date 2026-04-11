# Binance CLI

[![Latest Release](https://img.shields.io/github/release/binance/binance-cli.svg)](https://github.com/binance/binance-cli/releases)
[![npm Downloads](https://img.shields.io/npm/dm/@binance/binance-cli.svg)](https://www.npmjs.com/package/@binance/binance-cli)

A simple CLI that interacts with the Binance API

<img src="./assets/demo.gif" alt="binance-cli demo" width="1400" align="center">

## Installation

```bash
# Install from npm
npm install -g @binance/binance-cli
```

## Usage

### Help

Use `--help` to consult all the available commands

```bash
binance-cli --help
```

`--help` is supported for each product and each command.

<img src="./assets/help.gif" alt="binance-cli demo" width="1400" align="center">

### Authentication

#### Using Environment variable

```bash

# Display help command
binance-cli -h

# Set the API key and secret as global variables
export BINANCE_API_KEY=<the_api_key>
export BINANCE_SECRET_KEY=<the_api_secret> # Can be secret key, path to private key or content of private key

# Set the environment (prod by default, valid options: prod, demo and testnet)
export BINANCE_API_ENV=testnet

# It is possible to have a custom base URL
export BINANCE_SPOT_BASE_PATH=https://testnet.binance.vision
export BINANCE_FUTURES_USDS_BASE_PATH=https://testnet.binancefuture.com
```

#### Using profile (non-interactive)

##### Create a new profile

```bash
binance-cli profile create --name new-name --api-key <the_api_key> --api-secret <the_api_secret> --env <prod|demo|testnet>
```

##### Select a profile

```bash
binance-cli profile select --name new-name
```

#### View the current active profile

```bash
binance-cli profile view
```

#### List all the existing profiles

```bash
binance-cli profile list
```

The active profile will be shown with an asterisk.

#### Delete an existing profile

```bash
binance-cli profile delete --name profile-name
```

#### Specify profile

It is also possible to specify the profile for a single command.

```bash
binance-cli spot get-account --profile my_profile
```

#### Using profile (interactive)

##### Create a new profile

```bash
binance-cli profile create -i # binance-cli will prompt the questions to create a new profile
```

It can also be used to update an existing profile.

##### Select a profile

```bash
binance-cli profile select -i # select the profile to activate
```

##### View the current active profile

```bash
binance-cli profile view
```

### Commands

#### Available commands

- [Algo Trading](./examples/algo.md)
- [Alpha](./examples/alpha.md)
- [C2C](./examples/c2c.md)
- [Convert](./examples/convert.md)
- [Copy Trading](./examples/copy-trading.md)
- [Crypto Loan](./examples/crypto-loan.md)
- [Derivatives Trading (COIN-M Futures)](./examples/derivatives-trading-coin-futures.md)
- [Derivatives Trading (Options)](./examples/derivatives-trading-options.md)
- [Derivatives Trading (Portfolio Margin)](./examples/derivatives-trading-portfolio-margin.md)
- [Derivatives Trading (Portfolio Margin Pro)](./examples/derivatives-trading-portfolio-margin-pro.md)
- [Derivatives Trading (USDS-M Futures)](./examples/derivatives-trading-usds-futures.md)
- [Dual Investment](./examples/dual-investment.md)
- [Fiat](./examples/fiat.md)
- [Gift Card](./examples/gift-card.md)
- [Margin Trading](./examples/margin-trading.md)
- [Mining](./examples/mining.md)
- [Pay](./examples/pay.md)
- [Rebate](./examples/rebate.md)
- [Simple Earn](./examples/simple-earn.md)
- [Spot Trading](./examples/spot.md)
- [Staking](./examples/staking.md)
- [Sub Account](./examples/sub-account.md)
- [VIP Loan](./examples/vip-loan.md)
- [Wallet](./examples/wallet.md)

#### Parameters

##### CLI Argument

```bash
binance-cli spot klines --symbol "BNBUSDT" --interval "1s"
```

##### Using pipe

```bash
echo '{"symbol": "BNBUSDT", "interval": "1s"}' | binance-cli spot klines
```

#### Using interactive mode

```bash
binance-cli spot klines -i
```

#### Completion

In order to add completion, the following steps can be followed:

##### For Bash:

Append the output to the .bashrc file:

```bash
binance-cli completion >> ~/.bashrc
```

##### For Zsh:

Append the output to the .zshrc file:

```bash
binance-cli completion >> ~/.zshrc
```

After updating the file, the terminal needs to be restarted or activate the completion with `source ~/.bashrc (or .zshrc)`.

#### Custom request

Custom request can also be sent using binance-cli, any parameter can be added to the command and will be sent in the request.

```bash
binance-cli request GET https://api.binance.com/api/v3/trades --symbol BNBUSDT --limit 5
```

Signed endpoints are also supported.

```bash
binance-cli request GET https://testnet.binance.vision/api/v3/account --signed
```

## LICENSE

MIT
