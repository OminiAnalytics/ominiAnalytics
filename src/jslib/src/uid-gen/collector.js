/**
 * File: collector.js
 * Created Date: 04 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 10:40:8
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

import Base64 from 'crypto-js/enc-base64'
import parse from 'crypto-js/enc-utf8'
import HmacSHA256 from 'crypto-js/hmac-sha256'
import { ominiHardware } from './hardware.js'
import { ominiBrowser } from './browser.js'
import { ominiOs } from './opersys.js'
import { ominiLocation } from './location.js'

export const ominiCollector = {
  /*
    Each trackable element has an importance level.
    The importance level is used to calculate the weight of each element.
    Here is the table of importance levels (from 1 to 10), in right order:
    Country : 8
    Browser.name : 6
    Browser.version : 4
    Browser.fversion : 2
    Os[0].name : 10
    Os[0].version : 9
    Os.type: 10
    Os.arch: 10
    Hardware.memory: 7
    Hardware.cpu: 7
    Hardware.gpu: 5
    Hardware.colorBufferFloat: 3
    Hardware.screen[0]: 8
    Hardware.screen[1]: 8
    Hardware.screen[2]: 8
    IP adress: 10 (server side)
    */
  /**
   * Generate the shecksum of the device and his json object in base64
   * @returns {[string,string]} A json object in base64 and the hash of the device
   * @example
   * const hash = ominiGenerator.generate();
   * console.log(hash);
   * // => ["W29iamVjdCBPYmplY3Rd...", "87cd08503f063d2..."}]
   * */
  hash: async function () {
    // Get hardware information
    const Hardware = ominiHardware.wrap()
    // Get browser information
    const Browser = ominiBrowser.wrap()
    // Get OS information
    const Os = await ominiOs.wrap().then((os) => os)
    // Get location information
    const Country = ominiLocation.wrap()
    // Get json object
    const json = {
      Country,
      Browser,
      Os,
      Hardware,
      UserAgent: navigator.userAgent
    }
    // Get json string
    const jsonStr = JSON.stringify(json)
    const sign = HmacSHA256(jsonStr, navigator.userAgent)
    return [Base64.stringify(parse.parse(jsonStr)), sign.toString()]
  }
}
