const { CMFutures } = require("@binance/futures-connector")
const { print, printError } = require('../../../helpers/prettyPrint')
const { checkAPIKey } = require('../../../helpers/util')

const apiKey = process.env.BINANCE_FUTURES_API_KEY
const server = process.env.FUTURES_SERVER || "https://dapi.binance.com"

const cmFuturesClient = new CMFutures(apiKey, null, { baseURL: server })

const time = async () =>
  cmFuturesClient.getTime().then(response => print(response.data))
    .catch(error => print(error))

const exchangeInfo = async () =>
  cmFuturesClient.getExchangeInfo().then(response => print(response.data))
    .catch(error => print(error))

const depth = async (symbol, limit = 100) => {
  try {
    const result = await cmFuturesClient.getDepth(symbol, Number.parseInt(limit, 10))
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const trades = async (symbol, limit = 500) => {
  try {
    const result = await cmFuturesClient.getTrades(symbol, Number.parseInt(limit, 10))
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const histTrades = async (symbol, {limit, fromId}) => {
  if (checkAPIKey(apiKey)) {
    try {
      const result = await cmFuturesClient.getHistoricalTrades(symbol, Number.parseInt(limit, 10), fromId)
      print(result.data)
    } catch (e) {
      printError(e)
    }
  }
}

const aggTrades = async (symbol, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await cmFuturesClient.getAggTrades(symbol, fromId, startTime, endTime, Number.parseInt(limit) )
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const klines = async (symbol, interval, { startTime, endTime, limit }) => {
  try {
    const result = await cmFuturesClient.getKlines(symbol, interval, startTime, endTime, Number.parseInt(limit))
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const ticker = async ({ symbol }) => {
  try {
    const result = await cmFuturesClient.get24hrTicker(symbol)
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const tickerPrice = async ({ symbol }) => {
  try {
    const result = await cmFuturesClient.getPriceTicker(symbol)
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const bookTicker = async ({ symbol }) => {
  try {
    const result = await cmFuturesClient.getBookTicker(symbol)
    print(result.data)
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