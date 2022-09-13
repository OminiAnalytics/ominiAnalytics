/**
 * File: browser.js
 * Created Date: 02 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 12/09/2022 10:46:29
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

import { utilities } from '../utilities.js'

/*
Supported browser:

- Chrome
- Firefox
- Safari
- Opera
- Edge
- IE
- Vivaldi
- Yandex
*/

export const ominiBrowser = {
  /**
   * Set environment to browser
   * @param {string} ua
   * @param {*} uag
   * @param {*} nav
   * @returns {ominiBrowser} The ominiBrowser object
   */
  setEnv: function (ua, uag, nav) {
    this.ua = ua
    this.uag = uag
    this.nav = nav
    return this
  },
  /**
   * Return the browser name, formatted-version and full-version
   * @returns {{name: string, version: string, fversion: string}} Browser name, formatted-version and full-version
   * @example
   * const browser = ominiBrowser.getBrowser();
   * console.log(browser);
   * // => { name : "Chrome", version : "92", fversion : "92.0.4515.159"}
   */
  gbrowser: function () {
    if (this.uag !== undefined) {
      const fbrands = this.uag.brands.filter(
        (brand) =>
          brand.brand.indexOf('Brand') === -1 &&
          brand.brand.indexOf('Chromium') === -1
      )[0]
      return {
        name: fbrands.brand,
        version: fbrands.version.split('.')[0],
        fversion: fbrands.version
      }
    }
    // Order is important
    if (this.isYandex().isOk()) {
      return this.isYandex().unwrap()
    }
    if (this.isTrident().isOk()) {
      return this.isTrident().unwrap()
    }
    if (this.isEdge().isOk()) {
      return this.isEdge().unwrap()
    }
    if (this.isChrome().isOk()) {
      return this.isChrome().unwrap()
    }
    if (this.isFirefox().isOk()) {
      return this.isFirefox().unwrap()
    }
    if (this.isSafari().isOk()) {
      return this.isSafari().unwrap()
    }
    if (this.isOpera().isOk()) {
      return this.isOpera().unwrap()
    }
    if (this.isVivaldi().isOk()) {
      return this.isVivaldi().unwrap()
    }
    return { name: 'Unknown', version: 'Unknown', fversion: 'Unknown' }
  },
  isTrident: function () {
    const browserRegex = '(Trident)/([\\d.]+)'
    return this.handleMatch(browserRegex)
  },
  isYandex: function () {
    const browserRegex = '(YaBrowser)/([\\d.]+)'

    const r = this.handleMatch(browserRegex)
    if (r.isOk()) r.data.name = 'Yandex' // Change the name to Yandex
    return r
  },
  isVivaldi: function () {
    const browserRegex = '(Vivaldi)/([\\d.]+)'
    return this.handleMatch(browserRegex)
  },
  isOpera: function () {
    const browserRegex = '(OPR|Opera)/([\\d.]+)'
    const r = this.handleMatch(browserRegex)
    if (r.isOk()) r.data.name = 'Opera' // Change the name to Opera
    return r
  },
  isSafari: function () {
    const browserRegex = 'Version/[\\d.]+ (Safari)/([\\d.]+)'
    return this.handleMatch(browserRegex)
  },
  isEdge: function () {
    const browserRegex = '(Edg[\\S]{0,})/([\\d.]+)'
    const r = this.handleMatch(browserRegex)
    if (r.isOk()) r.data.name = 'Edge' // Change the name to Edge
    return r
  },
  isFirefox: function () {
    const browserRegex = '(Firefox)/([\\d.]+)'
    return this.handleMatch(browserRegex)
  },
  isChrome: function () {
    const browserRegex = '(Chrome|CriOS)/([\\d.]+)[\\S\\s]{0,} Safari/[\\d.]+$'
    const r = this.handleMatch(browserRegex)
    if (r.isOk()) r.data.name = 'Chrome' // Change the name to Chrome
    return r
  },
  /**
   * Return a Result object with browser name, major-version and full-version
   * @param {object} match The match object
   * @returns {utilities.Result} Result object with browser name, major-version and full-version
   * */
  handleMatch: function (reg) {
    const match = utilities.matchUA(this.ua, reg)
    if (match.isOk()) {
      const [browser, fversion] = match.unwrap()
      const version = fversion.split('.')[0] || fversion
      return new utilities.Result('ok', { name: browser, version, fversion })
    }
    return new utilities.Result('error', null)
  },
  /**
   * Wrap the browser information
   * And returns { name, formatted-version, full-version }
   * @returns {{name: string, version: string, fversion: string}} Browser name, formatted-version and full-version
   * @example
   * const browser = ominiBrowser.wrap();
   * console.log(browser);
   * // => { name : "Chrome", version : "92", fversion : "92.0.4515.159"}
   * */
  wrap: function () {
    return this.gbrowser()
  }
}
