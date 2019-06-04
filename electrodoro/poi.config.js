const path = require('path');

module.exports = {
    entry: 'lib/ts/index.tsx',
    output: {
        publicUrl: './',
    },
    babel: {
        jsx: 'require("deku").element',
        transpileModules: [ 'deku' ],
    },

    configureWebpack(config) {
        config.output.path = path.resolve(__dirname, 'lib/view');
        return config;
    },
}
