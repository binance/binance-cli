# Binance CLI

A simple CLI that interacts with the Binance API


## Installation

```bash

# download the code
git clone git@github.com:binance/binance-cli.git
cd binance-cli
npm install -g

```
## Usage


```bash

# Display help command
binance-cli -h

```

### Market Data
#### get server time
```bash
binance-cli t
binance-cli time
```

#### get exchange infomation
```bash

binance-cli i
# get BTCUSDT pair's filters. jq is required.
binance-cli i | jq '.symbols[] | select(.symbol == "BNBUSDT") |.filters'

```
#### get order book
```bash
# binance-cli book <symbol>
# binance-cli book -l <limit> <symbol>
binance-cli book bnbusdt
binance-cli book --limit 10 bnbusdt
```

#### get trades

```bash
# binance-cli t <symbol>
binance-cli t bnbusdt
binance-cli t -l 10 bnbusdt
```


#### get aggregate Trades List
```bash
# binance-cli at <symbol>
binance-cli at bnbusdt

# get aggregate Trades List with parameters
# binance-cli at <symbol>
binance-cli at --limit 10 --startTime 1595937694913 --endTime 1595937794913 bnbusdt
```


#### get klines data
```bash
# binance-cli k <symbol> <interval>
binance-cli k bnbusdt 1m
binance-cli k -l 1 bnbusdt 1m

```
#### get average price
```bash
# binance-cli ap <symbol>
binance-cli ap bnbusdt
```

#### get 24hr ticker
```bash
# binance-cli ticker -s <symbol>
binance-cli ticker -s bnbusdt
```


#### get ticker price
```bash
binance-cli price
# or with a symbol
binance-cli price -s bnbusdt
```

#### get order book ticker
```bash
# binance-cli bt
# binance-cli bt -s <symbol>
binance-cli bt -s bnbusdt
```

### listen to streams
```bash

# binance-cli listen <stream> <stream> <listenKey>
binance-cli listen bnbusdt@depth bnbusdt@bookTicker
```

### User data and trade
It's required to set the API key and secret

```shell
export BINANCE_API_KEY=<the_api_key>
export BINANCE_API_SECRET=<the_api_secret>
```

#### buy
```bash
# place a limit buy order on BNBUSDT with price=350 and qty=0.05
binance-cli buy -s BNBUSDT -t LIMIT -q 0.05 -p 350 -f GTC
```

#### sell
```bash
# place a limit sell order on BNBUSDT with price=500 and qty=0.03
binance-cli sell -s bnbusdt -t limit -q 0.03 -p 500 -f GTC
```

#### get order details
```bash
binance-cli get bnbusdt -i 12345
binance-cli get bnbusdt -c my_order_123
```

#### cancel an order
```bash
binance-cli cancel bnbusdt -i 12345
binance-cli cancel bnbusdt -c my_order_123
```

#### cancel all open orders
```bash
binance-cli cancel_all bnbusdt
```

Use `--help` to consult all the available commands
```bash
binance-cli --help
```

## LICENSE
MIT
