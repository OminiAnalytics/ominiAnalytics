/**
 * File: ominiTracker.js
 * Created Date: 30 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 8/09/2022 06:38:50
 * Modified By: realbacon
 * -----
 * @license MIT
 * -----
 **/
/* @License MIT - OminiTracker */
'use strict'

import { ApiAsync } from './api.js'
import { User } from './userTracker.js'
import { ominiCollector } from './collect/collector.js'

class OminiTracker {
  constructor (url) {
    this.api = new ApiAsync(url)
    this.user = new User(this.api)
  }

  async initTracker () {
    ominiCollector.hash().then((a) => {
      const jsond = a[0]
      const hash = a[1]
      this.user.collect = { dt: jsond, hash }
      this.user.init()
    })
  }
}
export default OminiTracker
