# Binance CLI

[![Latest Release](https://img.shields.io/github/release/binance/binance-cli.svg)](https://github.com/binance/binance-cli/releases)

A simple CLI that interacts with the Binance API

<img src="./assets/demo.gif" alt="binance-cli demo" width="1400" align="center">

## Installation

Download the correct binary from [Release](https://github.com/binance/binance-cli/releases) 

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

- **Algo Trading**: [Rest API](./examples/algo.md)
- **Alpha**: [Rest API](./examples/alpha.md) - [WebSocket Streams](./examples/alpha-ws-streams.md)
- **C2C**: [Rest API](./examples/c2c.md)
- **CMS**: [WebSocket Streams](./examples/alpha-ws-streams.md)
- **Convert**: [Rest API](./examples/convert.md)
- **Copy Trading**: [Rest API](./examples/copy-trading.md)
- **Crypto Loan**: [Rest API](./examples/crypto-loan.md)
- **Derivatives Trading (COIN-M Futures)**: [Rest API](./examples/derivatives-trading-coin-futures.md) - [WebSocket Streams](./examples/derivatives-trading-coin-futures-ws-streams.md)
- **Derivatives Trading (Options)**: [Rest API](./examples/derivatives-trading-options.md) - [WebSocket Streams](./examples/derivatives-trading-options-ws-streams.md)
- **Derivatives Trading (Portfolio Margin)**: [Rest API](./examples/derivatives-trading-portfolio-margin.md) - [WebSocket Streams](./examples/derivatives-trading-portfolio-margin-ws-streams.md)
- **Derivatives Trading (Portfolio Margin Pro)**: [Rest API](./examples/derivatives-trading-portfolio-margin-pro.md) - [WebSocket Streams](./examples/derivatives-trading-portfolio-margin-pro-ws-streams.md)
- **Derivatives Trading (USDS-M Futures)**: [Rest API](./examples/derivatives-trading-usds-futures.md) - [WebSocket Streams](./examples/derivatives-trading-usds-futures-ws-streams.md)
- **Dual Investment**: [Rest API](./examples/dual-investment.md)
- **Fiat**: [Rest API](./examples/fiat.md)
- **Gift Card**: [Rest API](./examples/gift-card.md)
- **Margin Trading**: [Rest API](./examples/margin-trading.md)
- **Mining**: [Rest API](./examples/mining.md)
- **Pay**: [Rest API](./examples/pay.md)
- **Rebate**: [Rest API](./examples/rebate.md)
- **Simple Earn**: [Rest API](./examples/simple-earn.md)
- **Spot Trading**: [Rest API](./examples/spot.md) - [WebSocket Streams](./examples/spot-ws-streams.md)
- **Stocks**: [Rest API](./examples/stocks.md) - [WebSocket Streams](./examples/stocks-ws-streams.md)
- **Staking**: [Rest API](./examples/staking.md)
- **Sub Account**: [Rest API](./examples/sub-account.md)
- **VIP Loan**: [Rest API](./examples/vip-loan.md)
- **Web3 Prediction**: [Rest API](./examples/w3w-prediction.md) - [WebSocket Streams](./examples/w3w-prediction-ws-streams.md)
- **Wallet**: [Rest API](./examples/wallet.md)

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
binance-cli completion bash >> ~/.bashrc
```

##### For Zsh:

Append the output to the .zshrc file:

```bash
binance-cli completion zsh >> ~/.zshrc
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
