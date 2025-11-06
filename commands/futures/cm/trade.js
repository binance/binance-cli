const {
  DerivativesTradingCoinFutures,
  DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL
} = require("@binance/derivatives-trading-coin-futures")
const { print, printError } = require('../../../helpers/prettyPrint')
const { checkKeyAndSecret, removeEmptyValue } = require('../../../helpers/util')

const configurationRestAPI = {
  apiKey: process.env.BINANCE_FUTURES_API_KEY,
  apiSecret: process.env.BINANCE_FUTURES_API_SECRET,
  basePath: process.env.FUTURES_SERVER || DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL
}

const client = new DerivativesTradingCoinFutures({ configurationRestAPI })

const account = async () => {
  if (checkKeyAndSecret(configurationRestAPI.apiKey, configurationRestAPI.apiSecret)) {
    client.restAPI.accountInformation().then(response => print(response.data()))
      .catch(error => printError(error))
  }
}

const newOrder = async ({ symbol, side, type, qty, price, tif, quoteOrderQty }) => {
  if (checkKeyAndSecret(configurationRestAPI.apiKey, configurationRestAPI.apiSecret)) {
    console.log(side, type, qty)
    if (!symbol) {
      printError('symbol is required. you can set it like: --symbol=BNBUSDT')
      return
    }

    if (!side) {
      printError('side is required. you can set it like: --side=BUY')
      return
    }

    if (!type) {
      printError('order type is required. you can set it like: --type=LIMIT')
      return
    }

    let parameters = {
      quantity: qty,
      price,
      timeInForce: tif,
      quoteOrderQty
    }
    parameters = removeEmptyValue(parameters)
    client.restAPI.newOrder({ symbol, side: side.toUpperCase(), type: type.toUpperCase(), ...parameters })
      .then(async response => print(await response.data()))
      .catch(error => printError(error))
  }
}

const getOrder = async (symbol, { orderId, origClientOrderId }) => {
  if (checkKeyAndSecret(configurationRestAPI.apiKey, configurationRestAPI.apiSecret)) {
    let param = {}
    if (orderId) {
      param = {
        orderId
      }
    }

    if (origClientOrderId) {
      param = {
        origClientOrderId
      }
    }

    if (!param) {
      printError('Either orderId or origClientOrderId must be sent.')
      return
    }
    client.restAPI.queryOrder({ symbol, ...param }).then(async response => print(await response.data()))
      .catch(error => printError(error))
  }
}

const cancelOrder = async (symbol, { orderId, origClientOrderId }) => {
  if (checkKeyAndSecret(configurationRestAPI.apiKey, configurationRestAPI.apiSecret)) {
    let param = {}
    if (orderId) {
      param = {
        orderId
      }
    }

    if (origClientOrderId) {
      param = {
        origClientOrderId
      }
    }

    if (!param) {
      printError('Either orderId or origClientOrderId must be sent.')
      return
    }
    client.restAPI.cancelOrder({ symbol, ...param }).then(async response => print(await response.data()))
      .catch(error => printError(error))
  }
}

const cancelAll = async (symbol) => {
  if (checkKeyAndSecret(configurationRestAPI.apiKey, configurationRestAPI.apiSecret)) {
    client.restAPI.cancelAllOpenOrders({ symbol }).then(async response => print(await response.data()))
      .catch(error => printError(error))
  }
}

module.exports = {
  account,
  newOrder,
  getOrder,
  cancelOrder,
  cancelAll
}
