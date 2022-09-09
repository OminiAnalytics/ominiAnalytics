/**
 * File: .eslintrc.js
 * Created Date: 07 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 8/09/2022 09:54:52
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: 'standard',
  overrides: [],
  parserOptions: {
    ecmaVersion: 'latest'
  },
  ignorePatterns: ['dist/*', 'webpack.config.js'],
  rules: {
    quotes: ['error', 'single']
  }
}
