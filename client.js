const WebSocket = require('ws');
const util = require('util');

const connection = new WebSocket('ws://127.0.0.1:3012');

connection.onopen = () => {
    connection.send('hey') 
}

connection.onerror = (error) => {
    console.log(`WebSocket error: ${util.inspect(error)}`)
}

connection.onmessage = (e) => {
    console.log(e.data)
}
