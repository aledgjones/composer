const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const path = require("path");

const config = {
  mode: process.env.NODE_ENV,
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

if (process.env.NODE_ENV === "production") {
  config.optimization = {
    minimizer: [`...`, new CssMinimizerPlugin()],
  };
}

module.exports = config;
