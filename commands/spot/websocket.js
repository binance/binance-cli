const { Spot } = require('@binance/spot')
const { print } = require('../../helpers/prettyPrint')


const listen = async (streams) => {
  if (streams.length === 0) {
    try {
      const configurationWebsocketAPI = {
        wsURL: process.env.SERVER || "wss://ws-api.binance.com:443/ws-api/v3",
      };

      const client = new Spot({ configurationWebsocketAPI })

      connection = await client.websocketAPI.connect();
      await connection.sessionLogon();

      const subscription = await connection.userDataStreamSubscribe();
      const stream = subscription.stream
      stream.on('message', (data) => {
        print(data)
      })

      while (true) {
        await new Promise(resolve => setTimeout(resolve, 30000))
      }
    } catch (e) {
      print(e)
    }
  } else {
    const configurationWebsocketStreams = {
      wsURL: process.env.SERVER || "wss://stream.binance.com:443",
    };

    const client = new Spot({ configurationWebsocketStreams })

    connection = await client.websocketStreams.connect()
    await connection.subscribe(streams)
    connection.on('message', (data) => {
      print(data)
    })
  }
}

module.exports = {
  listen
}
