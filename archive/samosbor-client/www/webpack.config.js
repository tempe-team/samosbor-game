const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html', 'fonts/iosevka-fixed-slab-extended.ttf']),
    new webpack.EnvironmentPlugin({
      WS_CONNECT_STRING: 'ws://localhost:8000',
    })
  ],
};
