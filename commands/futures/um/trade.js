const { UMFutures } = require("@binance/futures-connector")
const { print, printError } = require('../../../helpers/prettyPrint')
const { checkKeyAndSecret, removeEmptyValue } = require('../../../helpers/util')

const apiKey = process.env.BINANCE_FUTURES_API_KEY
const apiSecret = process.env.BINANCE_FUTURES_API_SECRET
const server = process.env.FUTURES_SERVER || "https://fapi.binance.com"

const umFuturesClient = new UMFutures(apiKey, apiSecret, { baseURL: server })

const account = async () => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
    umFuturesClient.getAccountInformationV2().then(response => print(response.data))
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
    umFuturesClient.newOrder(symbol, side.toUpperCase(), type.toUpperCase(), parameters).then(response => print(response.data))
      .catch(error => printError(error))
  }
}

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
    umFuturesClient.queryOrder(symbol, param).then(response => print(response.data))
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
    umFuturesClient.cancelOrder(symbol, param).then(response => print(response.data))
      .catch(error => printError(error))
  }
}

const cancelAll = async (symbol) => {
  if (checkKeyAndSecret(apiKey, apiSecret)) {
    umFuturesClient.cancelAllOpenOrders(symbol).then(response => print(response.data))
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
