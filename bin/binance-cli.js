#!/usr/bin/env node

const program = require('commander')
const Market = require('../commands/market')
const Websocket = require('../commands/websocket')
const Trade = require('../commands/trade')

// get account info
//
// Example:
//  - `binance-cli account`
//  - `binance-cli a`
//  - `binance-cli a | jq '.balances[]| select(.asset == "BTC")'`
//  - `binance-cli a | jq '.balances[]| select(.free != "0.00000000")'`
program
  .command('account')
  .alias('a')
  .description('Get account balance.')
  .action(async () => await Trade.account())

// get compressed/aggregate trades list
//
// Example:
//  - `binance-cli at bnbusdt`
//  - `binance-cli at --limit 10 --startTime 1595937694913 --endTime 1595937794913 bnbusdt`
program
  .command('agg_trades')
  .description('Get compressed/aggregate trades list')
  .alias('at')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await Market.aggTrades(symbol, args))

// get current average price
//
// binance-cli ap <symbol>
//
// Example:
//  - `binance-cli ap bnbusdt`
program
  .command('avg_price')
  .alias('ap')
  .description('Get current average price')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .action(async (symbol) => await Market.avgPrice(symbol))

// order book ticker
//
// binance-cli bt
// binance-cli bt -s <symbol>
//
// Example:
//  - `binance-cli bt -s bnbusdt`
program
  .command('book_ticker')
  .alias('bt')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get order book price')
  .action(async (args) => await Market.bookTicker(args))

// buy a new order
//
// Example:
//  - `binance-cli buy -s BNBUSDT -t LIMIT -q 0.05 -p 350 -f GTC`
program
  .command('buy')
  .description('Place a buy order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await Trade.newOrder({ side: 'BUY', ...args }))

// cancel all open orders
//
// Example:
//  - `binance-cli cancel_all bnbusdt`
program
  .command('cancel_all')
  .alias('ca')
  .description('Cancel all open orders.')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .action(async (symbol) => await Trade.cancelAll(symbol))

// cancel an order
//
// Example:
//  - `binance-cli cancel bnbusdt -i 12345`
//  - `binance-cli cancel bnbusdt -c my_order_123`
program
  .command('cancel_order.')
  .alias('cancel')
  .description('Cancel an order')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await Trade.cancelOrder(symbol, args))

// get an order details
//
// Example:
//  - `binance-cli get_order bnbusdt -i 12345`
//  - `binance-cli get bnbusdt -c my_order_123`
program
  .command('get_order.')
  .alias('get')
  .description('Get an order details')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await Trade.getOrder(symbol, args))

// Old trade lookup
//
// Example:
//  - `binance-cli ht bnbusdt`
//  - `binance-cli ht --limit 10 --fromId 1  bnbusdt`
//  - `binance-cli ht -l 10 -f 1 btcusdt`
program
  .command('hist_trades')
  .description('Get historical trades. API key is required.')
  .alias('ht')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await Market.histTrades(symbol, args))

// get exchange info
//
// Example:
//  - `binance-cli i`
program
  .command('info')
  .alias('i')
  .description('Get exchange info')
  .action(async () => await Market.exchangeInfo())

// get Kline/Candlestick data
//
// binance-cli k <symbol> <interval>
//
// Example:
//  - `binance-cli k bnbusdt 1m`
//  - `binance-cli k -l 10 -s 1595937694913 -e 1595937794913 bnbusdt 3m`
program
  .command('klines')
  .description('Get kline/candlestick data')
  .alias('k')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .argument('<interval>', 'kline interval, values: 1m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .action(async (symbol, interval, args) => await Market.klines(symbol, interval, args))

// listen to streams
//
// binance-cli listen <stream> <stream> <listenKey>
//
// Example:
//  - `binance-cli listen bnbusdt@depth bnbusdt@bookTicker`
program
  .command('listen [streams...]')
  .alias('l')
  .description('Listen to a single or multiple streams. API key is required.')
  .action(async (streams) => await Websocket.listen(streams))

// get order book
//
// binance-cli book <symbol>
// binance-cli book -l <limit> <symbol>
//
// Example:
//  - `binance-cli book bnbusdt`
//  - `binance-cli ob -l 10 bnbusdt`
program
  .command('order_book')
  .description('Get order book')
  .alias('ob')
  .alias('book')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 100; max 5000. Valid limits:[5, 10, 20, 50, 100, 500, 1000, 5000]')
  .action(async (symbol, args) => await Market.depth(symbol, args.limit))

// ticker price
//
// binance-cli price
// binance-cli price -s <symbol>
//
// Example:
//  - `binance-cli price -s bnbusdt`
program
  .command('price')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get ticker price')
  .action(async (args) => await Market.tickerPrice(args))

// sell a new order
//
// Example:
//  - `binance-cli sell --symbol=bnbusdt --type=limit --qty=1 --price=100 --tif=GTC`
//  - `binance-cli sell -s bnbusdt -t limit -q 0.03 -p 500 -f GTC`
program
  .command('sell')
  .description('Place a sell order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await Trade.newOrder({ side: 'SELL', ...args }))

// get 24hr ticker data
//
// binance-cli ticker -s <symbol>
//
// Example:
//  - `binance-cli ticker -s bnbusdt`
program
  .command('ticker')
  .description('Get 24hr ticker data')
  .option('-s, --symbol <symbol>', 'trading symbol, e.g. bnbusdt')
  .action(async (args) => await Market.ticker(args))

// get current server time
//
// Example:
//  - `binance-cli time`
program
  .command('time')
  .alias('t')
  .description('Get server time')
  .action(async () => await Market.time())

// Recent trades list
//
// Example:
//  - `binance-cli t bnbusdt`
//  - `binance-cli t -l 10 bnbusdt`
program
  .command('trades')
  .description('Get recent trades')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .action(async (symbol, args) => await Market.trades(symbol, args.limit))

program.version('0.1.0', '-v, --version', 'current version')

// Parse arguments
program.parse(process.argv)
