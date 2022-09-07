/**
 * File: opersys.js
 * Created Date: 02 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 09:01:51
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

/*
Supported OS :

- Windows
- Linux
- Mac
- Android
- iOS
- ChromeOS
- Linux
- SymianOS
- Windows Phone
- X11
- BlackBerry
*/

export const ominiOs = {
  ua: navigator.userAgent,
  uag: navigator.userAgentData,
  /**
   * Wrap the OS information
   * Returns an array [{ OS name, OS version}, Device type, Arch ]
   * @returns {[{name : String, version : String}, String, String]}
   * @example
   * console.log(ominiOS.wrap());
   * // => [{name: "Windows", version: "10"}, "desktop", "x64"]
   */
  wrap: async function () {
    const arch = this.getArch()
    const res = await this.getOsAndTypeExp().then((a) => {
      return a
    })
    const os = res[0]
    const type = res[1]
    return [os, type, arch]
  },

  /**
   * Get the device architecture
   * @returns {String} Device architecture
   * @example
   * console.log(ominiOS.getArch());
   * // => "x86_64"
   * */
  getArch: function () {
    let arch = 'unknown'
    if (this.os === 'Windows') {
      if (this.ua.indexOf('WOW64') !== -1 || this.ua.indexOf('Win64') !== -1) {
        arch = 'x64'
      } else {
        arch = 'x32'
      }
    } else if (this.os === 'Linux') {
      if (this.ua.indexOf('x86_64') !== -1) {
        arch = 'x64'
      } else if (this.ua.indexOf('i686') !== -1) {
        arch = 'x32'
      }
    } else if (this.ua.indexOf('adm64') !== -1 || this.ua.indexOf('arm64')) {
      arch = 'x64'
    }
    return arch
  },
  /**
   * Returns an array of The OS name, the OS version and the device type (mobile|desktop|console)
   * @exp
   * This function uses an experimental feature
   * that is not yet supported in all browsers
   * It fall back to getOsAndType() if not working
   * @returns {[{name : String, version : String},type: String]} Array [Object {String, String }, String]
   * @example
   * console.log(ominiOS.getOsAndType());
   * // => [{name: "Windows", version: "10"}, "desktop"]
   **/
  getOsAndTypeExp: function () {
    let os, type
    // Test if userAgentData api is available
    if (this.uag === undefined) {
      const [os, type] = this.getOsAndType()
      return [os, type]
    }
    const deviceInfo = this.uag.getHighEntropyValues([
      'model',
      'platform',
      'platformVersion',
      'fullVersionList'
    ])
    return deviceInfo.then((device) => {
      if (device.platform === 'Windows') {
        let v = 'unknown'
        switch (device.platformVersion.split('.').slice(0, 2).join('.')) {
          case '11.0':
            v = '11'
            break
          case '12.0':
            v = '11'
            break
          case '13.0':
            v = '11'
            break
          case '10.0':
            v = '10'
            break
          case '6.3':
            v = '8.1'
            break
          case '6.2':
            v = '8'
            break
          case '6.1':
            v = '7'
            break
          case '6.0':
            v = 'Vista'
            break
          case '5.2':
            v = 'XP'
            break
          case '5.0':
            v = '2000'
            break
          default:
            v = 'unknown'
        }
        type = 'desktop'
        os = {
          name: 'Windows',
          version: v
        }
      } else {
        os = {
          name: device.platform,
          version: device.platformVersion
        }
        type = device.mobile ? 'mobile' : 'desktop'
      }
      os = os || { name: 'unknown', version: 'unknown' }
      type = type || 'unknown'
      return [os, type]
    })
  },
  /**
   * Returns an array of The OS name, the OS version and the device type (mobile|desktop|console)
   * @returns {[{name : String, version : String},type: String]} Array [Object {String, String }, String]
   * @example
   * console.log(ominiOS.getOsAndType());
   * // => [{name: "Windows", version: "10"}, "desktop"]
   * */
  getOsAndType: function () {
    // init ua
    // Test very test[os] function
    if (this.testWindows()[0]) {
      const ser = this.testWindows()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testLinux()[0]) {
      const ser = this.testLinux()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testMac()[0]) {
      const ser = this.testMac()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testAndroid()[0]) {
      const ser = this.testAndroid()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testIphone()[0]) {
      const ser = this.testIphone()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testChromeOS()[0]) {
      const ser = this.testChromeOS()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testBlackBerry()[0]) {
      const ser = this.testBlackBerry()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testSymbian()[0]) {
      const ser = this.testSymbian()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testIpod()[0]) {
      const ser = this.testIpod()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testIpad()[0]) {
      const ser = this.testIpad()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    if (this.testWindowsPhone()[0]) {
      const ser = this.testWindowsPhone()
      return [{ name: ser[1], version: ser[2] }, ser[3]]
    }
    return [{ name: 'unknown', version: 'unknown' }, 'unknown']
  },
  testWindows: function () {
    if (this.ua.indexOf('Windows NT') > -1) {
      const p = 'desktop'
      let v = this.ua.match(/Windows NT (\d+\.\d+)/i)
      switch (v[1]) {
        case '10.0':
          v = '10'
          break

        case '6.3':
          v = '8.1'
          break

        case '6.2':
          v = '8'
          break

        case '6.1':
          v = '7'
          break

        case '6.0':
          v = 'Vista'
          break

        case '5.2':
          v = 'XP'
          break

        case '5.0':
          v = '2000'
          break

        default:
          v = 'unknown'
      }
      return [true, 'Windows', v, p]
    }
    return [false, '', '', '']
  },
  testMac: function () {
    if (this.ua.indexOf('Macintosh') > -1 && this.ua.indexOf('iPad') === -1) {
      const p = 'desktop'
      let v = this.ua.match(/Mac OS X (\d+_\d+)/i)
      if (v) {
        v = v[1].replace('_', '.')
      } else {
        v = 'unknown'
      }
      return [true, 'macOS', v, p]
    }
    return [false, '', '', '']
  },
  testLinux: function () {
    if (this.ua.indexOf('Linux') > -1 && this.ua.indexOf('Android') === -1) {
      const p = 'desktop'
      let v = this.ua.match(/Linux ([\d_\w]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Linux', v, p]
    }
    return [false, '', '', '']
  },
  testNintendo: function () {
    if (this.ua.indexOf('Nintendo') > -1) {
      const p = 'console'
      let v = this.ua.match(/Nintendo ([\w]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Nintendo', v, p]
    }
    return [false, '', '', '']
  },
  testPlayStation: function () {
    if (this.ua.indexOf('PlayStation') > -1) {
      const p = 'console'
      let v = this.ua.match(/PlayStation ([\s\w.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'PlayStation', v, p]
    }
    return [false, '', '', '']
  },
  testBlackBerry: function () {
    if (this.ua.indexOf('BlackBerry') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/BlackBerry ([\d.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'BlackBerry', v, p]
    }
    return [false, '', '', '']
  },
  testAndroid: function () {
    if (this.ua.indexOf('Android') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/Android ([\d.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Android', v, p]
    }
    return [false, '', '', '']
  },
  testIpad: function () {
    if (this.ua.indexOf('iPad') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/iPad; CPU OS ([\d_]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'iPad', v, p]
    }
    return [false, '', '', '']
  },
  testIpod: function () {
    if (this.ua.indexOf('iPod') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/OS ([\d_]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'iPod', v, p]
    }
    return [false, '', '', '']
  },
  testIphone: function () {
    if (this.ua.indexOf('iPhone') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/iPhone OS ([\d_]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'iPhone', v, p]
    }
    return [false, '', '', '']
  },
  testUnix: function () {
    if (this.ua.indexOf('X11') > -1) {
      const p = 'desktop'
      let v = this.ua.match(/X11;(?:[ ^U;]+)([\w]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Unix', v, p]
    }
    return [false, '', '', '']
  },
  testChromeOS: function () {
    if (this.ua.indexOf('CrOS') > -1) {
      const p = 'desktop'
      let v = this.ua.match(/CrOS ([\w.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'ChromeOS', v, p]
    }
    return [false, '', '', '']
  },
  testSymbian: function () {
    if (this.ua.indexOf('Symbian') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/SymbianOS\/([\d.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Symbian', v, p]
    }
    return [false, '', '', '']
  },
  testWindowsPhone: function () {
    if (this.ua.indexOf('Windows Phone') > -1) {
      const p = 'mobile'
      let v = this.ua.match(/Windows Phone ([\d.]+)/i)
      if (v) {
        v = v[1]
      } else {
        v = 'unknown'
      }
      return [true, 'Windows Phone', v, p]
    }
    return [false, '', '', '']
  }
}
