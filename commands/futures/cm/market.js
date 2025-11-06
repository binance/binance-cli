const { 
    DerivativesTradingCoinFutures,
    DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL 
} = require("@binance/derivatives-trading-coin-futures")
const { print, printError } = require('../../../helpers/prettyPrint')
const { checkAPIKey } = require('../../../helpers/util')

const configurationRestAPI = {
  apiKey: process.env.BINANCE_FUTURES_API_KEY,
  basePath: process.env.FUTURES_SERVER || DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL
}

const client = new DerivativesTradingCoinFutures({ configurationRestAPI })

const time = async () => client.restAPI.checkServerTime().then(async response => print(await response.data()))
      .catch(error => print(error))

const exchangeInfo = async () =>
  client.restAPI.exchangeInformation().then(async response => print(await response.data()))
    .catch(error => print(error))

const depth = async (symbol, limit = 100) => {
  try {
    const result = await client.restAPI.orderBook({ symbol, limit: Number.parseInt(limit, 10) })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const trades = async (symbol, limit = 500) => {
  try {
    const result = await client.restAPI.recentTradesList({ symbol, limit: Number.parseInt(limit, 10) })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const histTrades = async (symbol, {limit, fromId}) => {
  if (checkAPIKey(apiKey)) {
    try {
      const result = await client.restAPI.oldTradesLookup({symbol, limit: Number.parseInt(limit, 10), fromId})
      const data = await result.data()
      print(data)
    } catch (e) {
      printError(e)
    }
  }
}

const aggTrades = async (symbol, { fromId, startTime, endTime, limit }) => {
  try {
    let param = {}
    if (limit) param.limit = Number.parseInt(limit)

    const result = await client.restAPI.compressedAggregateTradesList(
        { symbol, fromId, startTime, endTime, ...param }
    )
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const klines = async (symbol, interval, { startTime, endTime, limit }) => {
  try {
    let param = {}
    if (limit) param.limit = Number.parseInt(limit)

    const result = await client.restAPI.klineCandlestickData({ symbol, interval, startTime, endTime, ...param })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const ticker = async ({ symbol }) => {
  try {
    const result = await client.restAPI.ticker24hrPriceChangeStatistics({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const tickerPrice = async ({ symbol }) => {
  try {
    const result = await client.restAPI.symbolPriceTicker({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const bookTicker = async ({ symbol }) => {
  try {
    const result = await client.restAPI.symbolOrderBookTicker({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

module.exports = {
  time,
  exchangeInfo,
  depth,
  trades,
  histTrades,
  aggTrades,
  klines,
  ticker,
  tickerPrice,
  bookTicker
}