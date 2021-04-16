const WebSocket = require('ws');
const express = require('express');
const http = require('http');
const fs = require('fs');
const path = require('path');
const { static } = require('express');

// Config
const port = 3001;

// Setup USB

// Setup the server
const app = express();
const server = http.createServer(app);

// Setup express
if (process.env.NODE_ENV === 'production') {
    app.use(express.static('public')); // Serve the built application 
}

// Setup the websocket
const wss = new WebSocket.Server({ noServer: true });

// Upgrade the connections and emit a event
server.on('upgrade', (request, socket, head) => {
    wss.handleUpgrade(request, socket, head, ws => {
        wss.emit('connection', ws, request);
    });
});

// Start the server
server.listen(port);

// setInterval(() => {
//     const time = Date.now();
//     const entry = {time: time, acceleration: {x: Math.random() * 50, y: Math.random() * 50, z: Math.random() * 50}}
//     wss.clients.forEach((ws) => {
//         ws.send(JSON.stringify(entry));
//     });
//     console.log('Sent:', entry);
// }, 100);
