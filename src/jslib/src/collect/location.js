/**
 * File: location.js
 * Created Date: 30 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 07:55:6
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

import { countries, zones } from 'moment-timezone/data/meta/latest.json'
export const ominiLocation = {
  /**
   * Return the country name
   * @returns {string}
   */
  getCountry: function () {
    // Get timezone from browser and set it to moment
    const timeZoneToCountry = {}
    const userTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone

    Object.keys(zones).forEach((z) => {
      timeZoneToCountry[z] = countries[zones[z].countries[0]].name
    })

    return timeZoneToCountry[userTimeZone] || 'unknown'
  },
  /**
   * Return the country name
   * @returns {string}
   * @example
   * const country = ominiLocation.getCountry();
   * console.log(country);
   * // => "Spain"
   */
  wrap: function () {
    return this.getCountry()
  }
}
