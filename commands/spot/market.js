const { Spot, SPOT_REST_API_PROD_URL } = require('@binance/spot')
const { print, printError } = require('../../helpers/prettyPrint')
const { checkAPIKey } = require('../../helpers/util')

const configurationRestAPI = {
  apiKey: process.env.BINANCE_API_KEY,
  basePath: process.env.SERVER || SPOT_REST_API_PROD_URL
}

const client = new Spot({ configurationRestAPI })

const time = async () =>
  await client.restAPI.time().then(async response => print(await response.data()))
    .catch(error => print(error))

const exchangeInfo = async () =>
  await client.restAPI.exchangeInfo().then(async response => print(await response.data()))
    .catch(error => print(error))

const depth = async (symbol, limit = 100) => {
  try {
    const result = await client.restAPI.depth({ symbol, limit })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const trades = async (symbol, limit = 100) => {
  try {
    const result = await client.restAPI.getTrades({ symbol, limit })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const histTrades = async (symbol, { limit, fromId }) => {
  if (checkAPIKey(apiKey)) {
    try {
      const result = await client.restAPI.historicalTrades({ symbol, limit, fromId })
      const data = await result.data()
      print(data)
    } catch (e) {
      printError(e)
    }
  }
}

const aggTrades = async (symbol, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await client.restAPI.aggTrades({ symbol, fromId, startTime, endTime, limit })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const klines = async (symbol, interval, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await client.restAPI.klines({ symbol, interval, fromId, startTime, endTime, limit })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const avgPrice = async (symbol) => {
  try {
    const result = await client.restAPI.avgPrice({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const ticker = async ({ symbol }) => {
  try {
    const result = await client.restAPI.ticker({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const ticker24hr = async ({ symbol }) => {
  try {
    const result = await client.restAPI.ticker24hr({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const tickerPrice = async ({ symbol }) => {
  try {
    const result = await client.restAPI.tickerPrice({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const bookTicker = async ({ symbol }) => {
  try {
    const result = await client.restAPI.tickerBookTicker({ symbol })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const tickerTradingDay = async ({ symbol, symbols }) => {
  try {
    const result = await client.restAPI.tickerTradingDay({ symbol, symbols })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

const uiKlines = async (symbol, interval, { fromId, startTime, endTime, limit }) => {
  try {
    const result = await client.restAPI.uiKlines({ symbol, interval, fromId, startTime, endTime, limit })
    const data = await result.data()
    print(data)
  } catch (e) {
    printError(e)
  }
}

module.exports = {
  time,
  exchangeInfo,
  aggTrades,
  avgPrice,
  depth,
  trades,
  histTrades,
  klines,
  ticker,
  ticker24hr,
  bookTicker,
  tickerPrice,
  tickerTradingDay,
  uiKlines
}
