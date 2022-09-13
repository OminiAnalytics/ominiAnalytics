/**
 * File: utilities.js
 * Created Date: 12 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 12/09/2022 10:07:20
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/
export const utilities = {
  /**
   * Get the match group from a regex - User Agent
   * @param {string} str The string to search in
   * @param {string} match The string to match
   * @return {Result} A result object containing the match group
   * */
  matchUA: function (str, match) {
    // eslint-disable-next-line no-useless-escape
    const r = str.match(new RegExp(`${match}`, 'i'))
    if (r !== null) {
      return new this.Result('ok', r.slice(1, 3))
    } else {
      return new this.Result('error', null)
    }
  },
  /**
   * Result object
   * @param {string} status The status of the result 'ok' || 'error'
   * @param {string} data The data of the result if is ok
   * @return {Result} The result object
   * */
  Result: class {
    constructor (status, data) {
      this.data = data
      this.status = status
    }

    /**
     * Return the status of the result
     * @returns {boolean} True if the result is ok
     */
    isOk () {
      return this.status === 'ok'
    }

    /**
     * Returns the data of the result if is ok
     * @returns {any} The data of the result if is ok
     */
    unwrap () {
      if (this.isOk()) {
        return this.data
      }
      return null
    }
  }
}
