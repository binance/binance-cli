const printout = async (method) => {
  const result = await method()
  console.log(JSON.stringify(result.data))
}

const display = async (result) => {
  console.log(JSON.stringify(result.data))
}

const print = (msg, obj) => {
  if (typeof msg === 'object') return console.log(JSON.stringify(msg))
  if (obj) {
    console.log(msg)
    console.log(JSON.stringify(obj))
  } else {
    console.log(msg)
  }
}

// Print error helper
const printError = (error, obj) => {
  if (error.response) {
    console.log(error.response.data, obj || '')
  } else {
    console.log(error, obj || '')
  }
}

module.exports = {
  printout,
  display,
  print,
  printError
}
