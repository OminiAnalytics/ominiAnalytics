/**
 * File: browser.js
 * Created Date: 02 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 08:03:22
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
  // User agent
  ua: navigator.userAgent,
  // User agent data (only in modern browsers)
  uag: navigator.userAgentData,
  // navigator
  nav: navigator,
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
    const browser = this.ua.match(
      /(YaBrowser|Vivaldi|OPR|Firefox|Safari|Edge|Trident|Chrome)\/([\S]+)/i
    )
    let name
    if (this.nav.brave && this.nav.brave.isBrave()) {
      name = 'Brave'
    } else {
      name = browser[0]
    }
    const version = browser[1].split('.')[0]
    const fversion = browser[1]
    return {
      name,
      version,
      fversion
    }
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
