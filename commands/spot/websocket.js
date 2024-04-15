const { Spot } = require('@binance/connector')
const { print } = require('../../helpers/prettyPrint')
const server = process.env.SERVER || "wss://stream.binance.com:443"


const client = new Spot(null, null, { baseURL: server })

const listen = (streams) => {
  const callbacks = {
    open: () => print('open'),
    close: () => print('closed'),
    message: (data) => print(data)
  }

  if (streams.length > 1) {
    client.combinedStreams(streams, callbacks)
  }else{
    // borrow the ability of userData that can do raw subscription.
    client.userData(streams[0], callbacks)
  }
}

module.exports = {
  listen
}
