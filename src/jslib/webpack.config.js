/**
 * File: webpack.config.js
 * Created Date: 30 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 10:09:24
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

/*
 * File: webpack.config.js
 * Created Date: 30 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 10:09:24
 * Modified By: realbacon
 * -----
 * License  : MIT
 * -----
 */

const path = require('path')
const TerserPlugin = require('terser-webpack-plugin')

module.exports = {
  entry: './src/ominiTracker.js',
  target: 'web',
  optimization: {
    minimize: true,
    minimizer: [
      new TerserPlugin({
        extractComments: false,
        terserOptions: {
          format: {
            comments: true
          }
        }
      })
    ]
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
    library: 'OminiTracker',
    libraryTarget: 'umd',
    globalObject: 'this',
    libraryExport: 'default',
    umdNamedDefine: true
  }
}
