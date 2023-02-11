const path = require("path");
const HTMLWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: "./client/index.ts",
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    static: dist,
  },
  performance: {
    hints: false,
  },
  resolve: {
    extensions: [".ts", ".js", ".wasm"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyPlugin({patterns: [{
      from: path.resolve(__dirname, "static/assets"),
      to: path.resolve(dist, 'assets')
    }]}),
    new HTMLWebpackPlugin({
      template: path.resolve(__dirname, "static/index.html"),
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
