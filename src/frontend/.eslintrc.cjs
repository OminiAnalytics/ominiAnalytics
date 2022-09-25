/**
 * File: .eslintrc.cjs
 * Created Date: 17 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 24/09/2022 07:17:41
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
**/





module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: ['./tsconfig.json'],
  },
  plugins: ['@typescript-eslint'],
  extends: [
    'prettier',
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
  ],
};