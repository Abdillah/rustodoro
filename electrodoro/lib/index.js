const {app, BrowserWindow} = require('electron');
const path = require('path');
const addon = require('../native');

function createWindow () {
    // Create the browser window.
    win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            preload: path.join(__dirname, 'bootstrap/preload.js')
        }
    })

    // and load the index.html of the app.
    win.loadURL(`file://${__dirname}/view/index.html`)
}

app.on('ready', createWindow)
