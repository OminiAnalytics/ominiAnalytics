/**
 * File: tailwind.config.js
 * Created Date: 17 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 17/09/2022 03:31:32
 * Last Modified: 17/09/2022 03:31:32
 * Modified By: realbacon
 * @license MIT
 * -----
 **/

/* eslint-disable no-undef */

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,tsx}'],
  theme: {
    extend: {}
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: 'dark'
  }
}
