/**
 * File: browser.js
 * Created Date: 02 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 8/09/2022 11:41:19
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

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
    let name
    let version
    let fversion
    if (this.ua.indexOf('Trident') !== -1) {
      name = 'IE'
      version = this.match('Trident')[1].split('.')[0]
      fversion = this.match('Trident')[1]
    } else if (this.ua.indexOf('YaBrowser') !== -1) {
      name = 'Yandex'
      version = this.match('YaBrowser')[1].split('.')[0]
      fversion = this.match('YaBrowser')[1]
    } else if (this.ua.indexOf('Vivaldi') !== -1) {
      name = 'Vivaldi'
      version = this.match('Vivaldi')[1].split('.')[0]
      fversion = this.match('Vivaldi')[1]
    } else if (this.ua.indexOf('OPR') !== -1) {
      name = 'Opera'
      version = this.match('OPR')[1].split('.')[0]
      fversion = this.match('OPR')[1]
    } else if (this.ua.indexOf('Version') !== -1) {
      name = 'Safari'
      version = this.match('Safari')[1].split('.')[0]
      fversion = this.match('Safari')[1]
    } else if (this.ua.indexOf('Edge') !== -1) {
      name = 'Edge'
      version = this.match('Edge')[1].split('.')[0]
      fversion = this.match('Edge')[1]
    } else if (this.ua.indexOf('Firefox') !== -1) {
      name = 'Firefox'
      version = this.match('Firefox')[1].split('.')[0]
      fversion = this.match('Firefox')[1]
    } else if (this.ua.indexOf('Chrome') !== -1) {
      name = 'Chrome'
      version = this.match('Chrome')[1].split('.')[0]
      fversion = this.match('Chrome')[1]
    }

    return {
      name,
      version,
      fversion
    }
  },
  /**
   * Return the browser version
   * @returns {string} Browser version
   * @example
   * const version = ominiBrowser.getVersion();
   * console.log(version);
   * // => "92"
   **/
  match: function (regex) {
    // eslint-disable-next-line no-useless-escape
    return this.ua.match(new RegExp(`${regex}\\\/([\\S]+)`, 'i'))
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
