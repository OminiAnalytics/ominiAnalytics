/**
 * File: webpack.config.js
 * Created Date: 24 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 24/09/2022 07:16:52
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
**/


module.exports = {
  module: {
    rules: [
      {
        test: /\.(svg|png|jpe?g|gif)$/i,
        use: [
          {
            loader: 'file-loader'
          }
        ]
      }
    ]
  }
};
