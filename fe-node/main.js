const {app, BrowserWindow} = require('electron')

function createWindow () {
    // Create the browser window.
    win = new BrowserWindow({width: 800, height: 600})

    // and load the index.html of the app.
    win.loadURL(`file://${__dirname}/index.html`)
}

// Init blackbox state manager

// Define callback
function updateUI(state) {
    // Update UI
}

// Register callback function


app.on('ready', createWindow)