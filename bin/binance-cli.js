#!/usr/bin/env node

const program = require('commander')

const Market = require('../commands/spot/market')
const Websocket = require('../commands/spot/websocket')
const Trade = require('../commands/spot/trade')

const UM_Market = require('../commands/futures/um/market')
const UM_trade = require('../commands/futures/um/trade')

const CM_Market = require('../commands/futures/cm/market')
const CM_trade = require('../commands/futures/cm/trade')

// SPOT

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
  .command('cancel_order')
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
  .command('get_order')
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

// Futures UM

// get current UM futures server time
//
// Example:
//  - `binance-cli um_time`
program
  .command('um_time')
  .alias('um_t')
  .description('Get UM futures server time')
  .action(async () => await UM_Market.time())

// get UM futures exchange info
//
// Example:
//  - `binance-cli um_info`
program
  .command('um_info')
  .alias('um_i')
  .description('Get UM futures exchange info')
  .action(async () => await UM_Market.exchangeInfo())

// get UM futures order book
//
// binance-cli um_order_book <symbol>
// binance-cli um_order_book -l <limit> <symbol>
//
// Example:
//  - `binance-cli um_book bnbusdt`
//  - `binance-cli um_ob -l 10 bnbusdt`
program
  .command('um_order_book')
  .description('Get UM futures order book')
  .alias('um_ob')
  .alias('um_book')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 100; max 5000. Valid limits:[5, 10, 20, 50, 100, 500, 1000, 5000]')
  .action(async (symbol, args) => await UM_Market.depth(symbol, args.limit))

// Recent UM futures trades list
//
// Example:
//  - `binance-cli um_trades bnbusdt`
//  - `binance-cli um_trades -l 10 bnbusdt`
program
  .command('um_trades')
  .description('Get UM futures recent trades')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .action(async (symbol, args) => await UM_Market.trades(symbol, args.limit))

// Old UM futures trade lookup
//
// Example:
//  - `binance-cli um_ht bnbusdt`
//  - `binance-cli um_ht --limit 10 --fromId 1  bnbusdt`
//  - `binance-cli um_ht -l 10 -f 1 btcusdt`
program
  .command('um_hist_trades')
  .description('Get UM futures historical trades. API key is required.')
  .alias('um_ht')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await UM_Market.histTrades(symbol, args))

// get UM futures compressed/aggregate trades list
//
// Example:
//  - `binance-cli um_at bnbusdt`
//  - `binance-cli um_at --limit 10 --startTime 1595937694913 --endTime 1595937794913 bnbusdt`
program
  .command('um_agg_trades')
  .description('Get UM futures compressed/aggregate trades list')
  .alias('um_at')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await UM_Market.aggTrades(symbol, args))

// get UM futures Kline/Candlestick data
//
// binance-cli um_k <symbol> <interval>
//
// Example:
//  - `binance-cli um_k bnbusdt 1m`
//  - `binance-cli um_k -l 10 -s 1595937694913 -e 1595937794913 bnbusdt 3m`
program
  .command('um_klines')
  .description('Get UM futures kline/candlestick data')
  .alias('um_k')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .argument('<interval>', 'kline interval, values: 1m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .action(async (symbol, interval, args) => await UM_Market.klines(symbol, interval, args))

// get UM futures 24hr ticker data
//
// binance-cli um_ticker -s <symbol>
//
// Example:
//  - `binance-cli um_ticker -s bnbusdt`
program
  .command('um_ticker')
  .description('Get UM futures 24hr ticker data')
  .option('-s, --symbol <symbol>', 'trading symbol, e.g. bnbusdt')
  .action(async (args) => await UM_Market.ticker(args))

// UM futures ticker price
//
// binance-cli um_price
// binance-cli um_price -s <symbol>
//
// Example:
//  - `binance-cli um_price -s bnbusdt`
program
  .command('um_price')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get ticker price')
  .action(async (args) => await UM_Market.tickerPrice(args))

// UM futures order book ticker
//
// binance-cli um_bt
// binance-cli um_bt -s <symbol>
//
// Example:
//  - `binance-cli um_bt -s bnbusdt`
program
  .command('um_book_ticker')
  .alias('um_bt')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get UM futures order book price')
  .action(async (args) => await UM_Market.bookTicker(args))

// get UM futures account info
//
// Example:
//  - `binance-cli um_account`
//  - `binance-cli um_a`
program
  .command('um_account')
  .alias('um_a')
  .description('Get UM futures account balance.')
  .action(async () => await UM_trade.account())

// buy a new UM futures order
//
// Example:
//  - `binance-cli um_buy -s BNBUSDT -t LIMIT -q 0.05 -p 350 -f GTC`
program
  .command('um_buy')
  .description('Place a UM futures buy order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await UM_trade.newOrder({ side: 'BUY', ...args }))

// sell a new UM futures order
//
// Example:
//  - `binance-cli um_sell --symbol=bnbusdt --type=limit --qty=1 --price=600 --tif=GTC`
//  - `binance-cli um_sell -s bnbusdt -t limit -q 0.03 -p 600 -f GTC`
program
  .command('um_sell')
  .description('Place a UM futures sell order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await UM_trade.newOrder({ side: 'SELL', ...args }))

// get an UM futures order details
//
// Example:
//  - `binance-cli um_get_order bnbusdt -i 12345`
//  - `binance-cli um_get bnbusdt -c my_order_123`
program
  .command('um_get_order')
  .alias('um_get')
  .description('Get an UM futures order details')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await UM_trade.getOrder(symbol, args))

// cancel an UM futures order
//
// Example:
//  - `binance-cli um_cancel bnbusdt -i 12345`
//  - `binance-cli um_cancel bnbusdt -c my_order_123`
program
  .command('um_cancel_order')
  .alias('um_cancel')
  .description('Cancel an UM futures order')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await UM_trade.cancelOrder(symbol, args))

// cancel all open UM futures orders
//
// Example:
//  - `binance-cli um_cancel_all bnbusdt`
program
  .command('um_cancel_all')
  .alias('um_ca')
  .description('Cancel all open orders.')
  .argument('<symbol>', 'trading symbol, e.g. bnbusdt')
  .action(async (symbol) => await UM_trade.cancelAll(symbol))


// Futures CM

// get current CM futures server time
//
// Example:
//  - `binance-cli cm_time`
program
  .command('cm_time')
  .alias('cm_t')
  .description('Get CM futures server time')
  .action(async () => await CM_Market.time())

// get CM futures exchange info
//
// Example:
//  - `binance-cli cm_info`
program
  .command('cm_info')
  .alias('cm_i')
  .description('Get CM futures exchange info')
  .action(async () => await CM_Market.exchangeInfo())

// get CM futures order book
//
// binance-cli cm_order_book <symbol>
// binance-cli cm_order_book -l <limit> <symbol>
//
// Example:
//  - `binance-cli cm_book BNBUSD_PERP`
//  - `binance-cli cm_ob -l 10 BNBUSD_PERP`
program
  .command('cm_order_book')
  .description('Get CM futures order book')
  .alias('cm_ob')
  .alias('cm_book')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-l, --limit <limit>', 'Default 100; max 5000. Valid limits:[5, 10, 20, 50, 100, 500, 1000, 5000]')
  .action(async (symbol, args) => await CM_Market.depth(symbol, args.limit))

// Recent CM futures trades list
//
// Example:
//  - `binance-cli cm_trades BNBUSD_PERP`
//  - `binance-cli cm_trades -l 10 BNBUSD_PERP`
program
  .command('cm_trades')
  .description('Get CM futures recent trades')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .action(async (symbol, args) => await CM_Market.trades(symbol, args.limit))

// Old CM futures trade lookup
//
// Example:
//  - `binance-cli cm_ht BNBUSD_PERP`
//  - `binance-cli cm_ht --limit 10 --fromId 1  BNBUSD_PERP`
//  - `binance-cli cm_ht -l 10 -f 1 BNBUSD_PERP`
program
  .command('cm_hist_trades')
  .description('Get CM futures historical trades. API key is required.')
  .alias('cm_ht')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await CM_Market.histTrades(symbol, args))

// get CM futures compressed/aggregate trades list
//
// Example:
//  - `binance-cli cm_at BNBUSD_PERP`
//  - `binance-cli cm_at --limit 10 --startTime 1701367424920 --endTime 1702037809993 BNBUSD_PERP`
program
  .command('cm_agg_trades')
  .description('Get CM futures compressed/aggregate trades list')
  .alias('cm_at')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .option('-f, --fromId <fromId>', 'Trade id to fetch from. Default gets most recent trades.')
  .action(async (symbol, args) => await CM_Market.aggTrades(symbol, args))

// get CM futures Kline/Candlestick data
//
// binance-cli cm_k <symbol> <interval>
//
// Example:
//  - `binance-cli cm_k BNBUSD_PERP 1m`
//  - `binance-cli cm_k -l 10 -s 1701367424920 -e 1702037809993 BNBUSD_PERP 3m`
program
  .command('cm_klines')
  .description('Get CM futures kline/candlestick data')
  .alias('cm_k')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .argument('<interval>', 'kline interval, values: 1m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M')
  .option('-l, --limit <limit>', 'Default 500; max 1000.')
  .option('-s, --startTime <startTime>', 'Timestamp in ms to get aggregate trades from INCLUSIVE.')
  .option('-e, --endTime <endTime>', 'Timestamp in ms to get aggregate trades until INCLUSIVE.')
  .action(async (symbol, interval, args) => await CM_Market.klines(symbol, interval, args))

// get CM futures 24hr ticker data
//
// binance-cli cm_ticker -s <symbol>
//
// Example:
//  - `binance-cli cm_ticker -s BNBUSD_PERP`
program
  .command('cm_ticker')
  .description('Get CM futures 24hr ticker data')
  .option('-s, --symbol <symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .action(async (args) => await CM_Market.ticker(args))

// CM futures ticker price
//
// binance-cli cm_price
// binance-cli cm_price -s <symbol>
//
// Example:
//  - `binance-cli cm_price -s BNBUSD_PERP`
program
  .command('cm_price')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get ticker price')
  .action(async (args) => await CM_Market.tickerPrice(args))

// CM futures order book ticker
//
// binance-cli cm_bt
// binance-cli cm_bt -s <symbol>
//
// Example:
//  - `binance-cli cm_bt -s BNBUSD_PERP`
program
  .command('cm_book_ticker')
  .alias('cm_bt')
  .option('-s, --symbol <symbol>', 'trading symbol')
  .description('Get CM futures order book price')
  .action(async (args) => await CM_Market.bookTicker(args))

// get CM futures account info
//
// Example:
//  - `binance-cli cm_account`
//  - `binance-cli cm_a`
program
  .command('cm_account')
  .alias('cm_a')
  .description('Get CM futures account balance.')
  .action(async () => await CM_trade.account())

// buy a new CM futures order
//
// Example:
//  - `binance-cli cm_buy -s BNBUSD_PERP -t LIMIT -q 1 -p 350 -f GTC`
program
  .command('cm_buy')
  .description('Place a CM futures buy order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await CM_trade.newOrder({ side: 'BUY', ...args }))

// sell a new CM futures order
//
// Example:
//  - `binance-cli cm_sell --symbol=BNBUSD_PERP --type=limit --qty=1 --price=600 --tif=GTC`
//  - `binance-cli cm_sell -s BNBUSD_PERP -t limit -q 1 -p 600 -f GTC`
program
  .command('cm_sell')
  .description('Place a CM futures sell order.')
  .option('-s, --symbol [symbol]', 'trading symbol, e.g. btcusdt, all symbols are available from exchangeInfo command')
  .option('-t, --type [orderType]', 'order type, e.g. LIMIT, MARKET, LIMIT_MAKER. Different order type may require different parameters, check the document for details.')
  .option('-q, --qty [quantity]', 'order quantity. Check the rule under exchangeInfo command if you see filter error like LOT_SIZE')
  .option('-p, --price [price]', 'order price. Not required for MARKET order. Check the filters if you also see errors.')
  .option('-f, --tif [timeInForce]', 'GTC, IOC or FOK')
  .option('-u, --quoteOrderQty [quoteOrderQty]', 'For market order, please see details from API document.')
  .action(async (args) => await CM_trade.newOrder({ side: 'SELL', ...args }))

// get an CM futures order details
//
// Example:
//  - `binance-cli cm_get_order BNBUSD_PERP -i 12345`
//  - `binance-cli cm_get BNBUSD_PERP -c my_order_123`
program
  .command('cm_get_order')
  .alias('cm_get')
  .description('Get an CM futures order details')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await CM_trade.getOrder(symbol, args))

// cancel an CM futures order
//
// Example:
//  - `binance-cli cm_cancel BNBUSD_PERP -i 12345`
//  - `binance-cli cm_cancel BNBUSD_PERP -c my_order_123`
program
  .command('cm_cancel_order')
  .alias('cm_cancel')
  .description('Cancel an CM futures order')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .option('-i, --orderId <orderId>', 'order id')
  .option('-c, --origClientOrderId <origClientOrderId>', 'original client order id that used for placing the order')
  .action(async (symbol, args) => await CM_trade.cancelOrder(symbol, args))

// cancel all open CM futures orders
//
// Example:
//  - `binance-cli cm_cancel_all BNBUSD_PERP`
program
  .command('cm_cancel_all')
  .alias('cm_ca')
  .description('Cancel all open orders.')
  .argument('<symbol>', 'trading symbol, e.g. BNBUSD_PERP')
  .action(async (symbol) => await CM_trade.cancelAll(symbol))


program.version('1.2.0', '-v, --version', 'current version')

// Parse arguments
program.parse(process.argv)
