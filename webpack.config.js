const path = require('path');

const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: './res/index.js',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
    },
    mode: "development",
    plugins: [
        new CopyPlugin({
            patterns: ["res/index.html"],
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
};