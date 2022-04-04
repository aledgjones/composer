const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
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
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
      {
        test: /\.(png|jpe?g|gif|svg)$/i,
        use: [
          {
            loader: "file-loader",
          },
        ],
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
    new CopyWebpackPlugin({
      patterns: [
        {
          from: "./static/**/*",
          to: "[name][ext]",
        },
      ],
    }),
    new MiniCssExtractPlugin(),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
