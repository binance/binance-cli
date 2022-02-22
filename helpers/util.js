const { printError } = require('./prettyPrint')

const isEmptyValue = input => {
  /**
   * Scope of empty value: falsy value (except for false and 0),
   * string with white space characters only, empty object, empty array
   */
  return (!input && input !== false && input !== 0) ||
    ((typeof input === 'string' || input instanceof String) && /^\s+$/.test(input)) ||
    (input instanceof Object && !Object.keys(input).length) ||
    (Array.isArray(input) && !input.length)
}

const removeEmptyValue = obj => {
  if (!(obj instanceof Object)) return {}
  Object.keys(obj).forEach(key => isEmptyValue(obj[key]) && delete obj[key])
  return obj
}

const checkKeyAndSecret = (apiKey, apiSecret) => {
  const hasApiKeyAndSecret = Boolean(apiKey) && Boolean(apiSecret)
  if (!hasApiKeyAndSecret) printError('API Key or Secret is not set, please set BINANCE_API_KEY and BINANCE_API_SECRET')
  return hasApiKeyAndSecret
}

const checkAPIKey = (apiKey) => {
  const hasApiKey = Boolean(apiKey)
  if (!hasApiKey) printError('API Key is not set, please set BINANCE_API_KEY')
  return hasApiKey
}

module.exports = {
  removeEmptyValue,
  checkKeyAndSecret,
  checkAPIKey
}
