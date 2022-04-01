const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/bootstrap.ts",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "./bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({ patterns: [{ from: "./src/index.html" }] }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
