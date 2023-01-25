const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "index.html")
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        })
    ],
    mode: 'development',
    devtool: 'nosources-source-map',
    experiments: {
        asyncWebAssembly: true
    }
};
