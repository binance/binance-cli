const { Spot } = require('@binance/connector')
const { print, printError } = require('../../helpers/prettyPrint')
const { checkAPIKey } = require('../../helpers/util')

const apiKey = process.env.BINANCE_API_KEY
const server = process.env.SERVER || "https://api.binance.com"

const client = new Spot(apiKey, null, { baseURL: server})

const time = async () =>
  client.time().then(response => print(response.data))
    .catch(error => print(error))

const exchangeInfo = async () =>
  client.exchangeInfo().then(response => print(response.data))
    .catch(error => print(error))

const depth = async (symbol, limit = 100) => {
  try {
    const result = await client.depth(symbol, { limit })
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const trades = async (symbol, limit = 100) => {
  try {
    const result = await client.trades(symbol, { limit })
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const histTrades = async (symbol, { limit, fromId }) => {
  if (checkAPIKey(apiKey)) {
    const client = new Spot(apiKey)
    try {
      const result = await client.historicalTrades(symbol, { limit, fromId })
      print(result.data)
    } catch (e) {
      printError(e)
    }
  }
}

const aggTrades = async (symbol, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await client.aggTrades(symbol, { fromId, startTime, endTime, limit })
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const klines = async (symbol, interval, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await client.klines(symbol, interval, { fromId, startTime, endTime, limit })
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const avgPrice = async (symbol) => {
  try {
    const result = await client.avgPrice(symbol)
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const ticker = async ({ symbol }) => {
  try {
    const result = await client.ticker24hr(symbol)
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const tickerPrice = async ({ symbol }) => {
  try {
    const result = await client.tickerPrice(symbol)
    print(result.data)
  } catch (e) {
    printError(e)
  }
}

const bookTicker = async ({ symbol }) => {
  try {
    const result = await client.bookTicker(symbol)
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
  avgPrice,
  ticker,
  tickerPrice,
  bookTicker
}
