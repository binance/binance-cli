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

# Set the API key and secret as global variables
# For SPOT
export BINANCE_API_KEY=<the_api_key>
export BINANCE_API_SECRET=<the_api_secret>

# For Futures
export BINANCE_FUTURES_API_KEY=<the_api_key>
export BINANCE_FUTURES_API_SECRET=<the_api_secret>

# It's possible to change the base URL to connect to testnet
# For SPOT
export SERVER=https://testnet.binance.vision

# For Futures
export FUTURES_SERVER=https://testnet.binancefuture.com
# then the command request will be sent to the testnet.
```



### SPOT

#### Market Data

##### Get Server Time
```bash
binance-cli t
binance-cli time
```

##### Get Exchange Infomation
```bash
binance-cli i
# get BTCUSDT pair's filters. jq is required.
binance-cli i | jq '.symbols[] | select(.symbol == "BNBUSDT") |.filters'
```

##### Get Order Book
```bash
# binance-cli book <symbol>
# binance-cli book -l <limit> <symbol>
binance-cli book bnbusdt
binance-cli book --limit 10 bnbusdt
```

##### Get Trades
```bash
# binance-cli trades <symbol>
binance-cli trades bnbusdt
binance-cli trades -l 10 bnbusdt
```

##### Get Aggregate Trades List
```bash
# binance-cli at <symbol>
binance-cli at bnbusdt

# get aggregate Trades List with parameters
# binance-cli at <symbol>
binance-cli at --limit 10 --startTime 1595937694913 --endTime 1595937794913 bnbusdt
```

##### Get Klines Data
```bash
# binance-cli k <symbol> <interval>
binance-cli k bnbusdt 1m
binance-cli k -l 1 bnbusdt 1m

```
##### Get Average Price
```bash
# binance-cli ap <symbol>
binance-cli ap bnbusdt
```

##### Get 24hr Ticker
```bash
# binance-cli ticker -s <symbol>
binance-cli ticker -s bnbusdt
```

##### Get Ticker Price
```bash
binance-cli price
# or with a symbol
binance-cli price -s bnbusdt
```

##### Get Order Book Ticker
```bash
# binance-cli bt
# binance-cli bt -s <symbol>
binance-cli bt -s bnbusdt
```

#### Listen To Streams
```bash
# binance-cli listen <stream> <stream> <listenKey>
binance-cli listen bnbusdt@depth bnbusdt@bookTicker
```

#### User Data And Trade

##### Buy
```bash
# place a limit buy order on BNBUSDT with price=350 and qty=0.05
binance-cli buy -s BNBUSDT -t LIMIT -q 0.05 -p 350 -f GTC
```

##### Sell
```bash
# place a limit sell order on BNBUSDT with price=500 and qty=0.03
binance-cli sell -s bnbusdt -t limit -q 0.03 -p 500 -f GTC
```

##### Get Order Details
```bash
binance-cli get bnbusdt -i 12345
binance-cli get bnbusdt -c my_order_123
```

##### Cancel An Order
```bash
binance-cli cancel bnbusdt -i 12345
binance-cli cancel bnbusdt -c my_order_123
```

##### Cancel All Open Orders
```bash
binance-cli cancel_all bnbusdt
```



### UM Futures

#### Market Data

##### Get Server Time
```bash
binance-cli um_t
binance-cli um_time
```

##### Get Exchange Infomation
```bash
binance-cli um_i
# get BTCUSDT pair's filters. jq is required.
binance-cli um_info | jq '.symbols[] | select(.symbol == "BNBUSDT") |.filters'

```

##### Get Order Book

```bash
# binance-cli um_book <symbol>
# binance-cli um_book -l <limit> <symbol>
binance-cli um_book bnbusdt
binance-cli um_book --limit 10 bnbusdt
```

##### Get Trades
```bash
# binance-cli um_trades <symbol>
binance-cli um_trades bnbusdt
binance-cli um_trades -l 10 bnbusdt
```

##### Get Aggregate Trades List
```bash
# binance-cli um_at <symbol>
binance-cli um_at bnbusdt

# get aggregate Trades List with parameters
# binance-cli um_at <symbol>
binance-cli um_at --limit 10 --startTime 1595937694913 --endTime 1595937794913 bnbusdt
```

##### Get Klines Data
```bash
# binance-cli um_k <symbol> <interval>
binance-cli um_k bnbusdt 1m
binance-cli um_k -l 1 bnbusdt 1m
```

##### Get 24hr Ticker
```bash
# binance-cli um_ticker -s <symbol>
binance-cli um_ticker -s bnbusdt
```

##### Get Ticker Price
```bash
binance-cli um_price
# or with a symbol
binance-cli um_price -s bnbusdt
```

##### Get Order Book Ticker
```bash
# binance-cli um_bt
# binance-cli um_bt -s <symbol>
binance-cli um_bt -s bnbusdt
```

#### User Data And Trade

##### Buy
```bash
# place a limit buy order on BNBUSDT with price=350 and qty=0.05
binance-cli um_buy -s BNBUSDT -t LIMIT -q 0.05 -p 350 -f GTC
```

##### Sell
```bash
# place a limit sell order on BNBUSDT with price=600 and qty=0.03
binance-cli um_sell -s bnbusdt -t limit -q 0.03 -p 600 -f GTC
```

##### Get Order Details
```bash
binance-cli um_get bnbusdt -i 12345
binance-cli um_get bnbusdt -c my_order_123
```

##### Cancel An Order
```bash
binance-cli um_cancel bnbusdt -i 12345
binance-cli um_cancel bnbusdt -c my_order_123
```

##### Cancel All Open Orders
```bash
binance-cli um_cancel_all bnbusdt
```



### CM Futures

#### Market Data

##### Get Server Time
```bash
binance-cli cm_t
binance-cli cm_time
```

##### Get Exchange Infomation
```bash
binance-cli cm_i
# get BTCUSDT pair's filters. jq is required.
binance-cli cm_info | jq '.symbols[] | select(.symbol == "BNBUSD_PERP") |.filters'
```
##### Get Order Book
```bash
# binance-cli cm_book <symbol>
# binance-cli cm_book -l <limit> <symbol>
binance-cli cm_book BNBUSD_PERP
binance-cli cm_book --limit 10 BNBUSD_PERP
```

##### Get Trades
```bash
# binance-cli cm_t <symbol>
binance-cli cm_trades BNBUSD_PERP
binance-cli cm_trades -l 10 BNBUSD_PERP
```

##### Get Aggregate Trades List
```bash
# binance-cli cm_at <symbol>
binance-cli cm_at BNBUSD_PERP

# get aggregate Trades List with parameters
# binance-cli cm_at <symbol>
binance-cli cm_at --limit 10 --startTime 1701367424920 --endTime 1702037809993 BNBUSD_PERP
```

##### Get Klines Data
```bash
# binance-cli cm_k <symbol> <interval>
binance-cli cm_k BNBUSD_PERP 1m
binance-cli cm_k -l 1 BNBUSD_PERP 1m

```

##### Get 24hr Ticker
```bash
# binance-cli cm_ticker -s <symbol>
binance-cli cm_ticker -s BNBUSD_PERP
```

##### Get Ticker Price
```bash
binance-cli cm_price
# or with a symbol
binance-cli cm_price -s BNBUSD_PERP
```

##### Get Order Book Ticker
```bash
# binance-cli cm_bt
# binance-cli cm_bt -s <symbol>
binance-cli cm_bt -s BNBUSD_PERP
```

#### User Data And Trade

##### Buy
```bash
# place a limit buy order on BNBUSDT with price=350 and qty=1
binance-cli cm_buy -s BNBUSD_PERP -t LIMIT -q 1 -p 350 -f GTC
```

##### Sell
```bash
# place a limit sell order on BNBUSDT with price=600 and qty=1
binance-cli cm_sell -s BNBUSD_PERP -t limit -q 1 -p 600 -f GTC
```

##### Get Order Details
```bash
binance-cli cm_get BNBUSD_PERP -i 12345
binance-cli cm_get BNBUSD_PERP -c my_order_123
```

##### Cancel An Order
```bash
binance-cli cm_cancel BNBUSD_PERP -i 12345
binance-cli cm_cancel BNBUSD_PERP -c my_order_123
```

##### Cancel All Open Orders
```bash
binance-cli cm_cancel_all BNBUSD_PERP
```

### Annex

Use `--help` to consult all the available commands
```bash
binance-cli --help
```

## LICENSE
MIT
