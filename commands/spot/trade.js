const { Spot } = require('@binance/connector')
const { print, printError } = require('../../helpers/prettyPrint')
const { checkKeyAndSecret, removeEmptyValue } = require('../../helpers/util')

const apiKey = process.env.BINANCE_API_KEY
const apiSecret = process.env.BINANCE_API_SECRET
const server = process.env.SERVER || "https://api.binance.com"

const client = new Spot(apiKey, apiSecret, { baseURL: server})

const getOrder = async (symbol, { orderId, origClientOrderId }) => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
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
    client.getOrder(symbol, param).then(response => print(response.data))
      .catch(error => printError(error))
  }
}

const cancelOrder = async (symbol, { orderId, origClientOrderId }) => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
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
    client.cancelOrder(symbol, param).then(response => print(response.data))
      .catch(error => printError(error))
  }
}

const cancelAll = async (symbol) => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
    client.cancelOpenOrders(symbol).then(response => print(response.data))
      .catch(error => printError(error))
  }
}

const account = async () => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
    client.account().then(response => print(response.data))
      .catch(error => printError(error))
  }
}

const newOrder = async ({ symbol, side, type, qty, price, tif, quoteOrderQty }) => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
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
    client.newOrder(symbol, side.toUpperCase(), type.toUpperCase(), parameters).then(response => print(response.data))
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
