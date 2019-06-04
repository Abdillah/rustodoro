const path = require('path')
const addon = require('../../native')

process.once('loaded', function() {
    global.wasm = addon;
    global.exports = {};
});
