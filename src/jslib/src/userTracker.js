/**
 * File: userTracker.js
 * Created Date: 30 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 7/09/2022 10:03:33
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/

// User class
export class User {
  constructor (api) {
    this.uid = undefined // User id
    this.api = api // Api instance
    this.userIsValid = false // User is valid
    this.collect = undefined // Collected data in base64
  }

  /**
   * Try to get uid cookie or get an empty one.
   * Then validate it to the server or get a new uid if not valid.
   * @returns {[string,string]} Result => [success?,data] @array
   */
  async getUid () {
    // Check if we can access cookie
    if (navigator.cookieEnabled) {
      // Check if user has cookie "Om_uid"
      let result
      let uidcookie
      if (document.cookie.indexOf('Om_uid') > -1) {
        // If yes, check if it is valid
        const guidcookie = document.cookie.split('Om_uid=')[1].split(';')[0]
        ;[result, uidcookie] = await this.validateUidFetch(guidcookie).then()
      } else {
        // If no, create new uid cookie
        ;[result, uidcookie] = this.validateUidFetch('')
      }
      console.log(uidcookie)
      if (result === 'success') {
        this.setCookie(uidcookie)
        return ['success', uidcookie]
      } else {
        return ['error', false]
      }
    } else {
      return ['error', false]
    }
  }

  /**
   *Create new uid cookie
   * @param {string} uid The cookie uid
   */
  setCookie (uid) {
    document.cookie =
      'Om_uid=' + uid + '; max-age=31536000; samesite=strict; Path=/'
  }

  /** Init the user class.
   * If we succeed to get a uid and create a cookie, we set the user as valid.
   **/
  async init () {
    // Set uid cookie
    const [result, d] = await this.getUid()
    if (result === 'success') {
      this.uid = d
      this.userIsValid = true
    }
  }

  /**
   * Fetch uid endpoint to validate uid
   * If not valid, we get a new uid from the server.
   * Then we try to deserialize the uid.
   *@param {string} uidcookie : Uid to validate
   *@returns {[Result]} Result => [success?,data]
   **/
  async validateUidFetch (uidcookie) {
    if (this.collect === undefined) return
    const response = await this.api.postDataJSON({
      endpoint: 'uid',
      data: {
        uid: uidcookie,
        bp: this.collect
      }
    })
    if (response.isOk()) {
      try {
        await response.toJson()
        // eslint-disable-next-line no-unused-vars
        const [uid, _] = response.data
        return ['success', uid]
      } catch (_) {
        return ['error', false]
      }
    }
    return ['error', false]
  }
}
