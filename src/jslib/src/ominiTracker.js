/*
 * File: ominiTracker.js
 * Created Date: 28 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 29/08/2022 03:12:41
 * Modified By: realbacon
 * -----
 * Copyright (c) 2022 Omini
 * -----
 */

import { ApiAsync } from "./api.js";
import { User } from "./userTracker.js";
import { LocationInfo } from "./locationInfoTracker.js";
import { deviceInfo } from "./deviceInfoTracker.js";

class OminiTracker {
    constructor(url) {
        this.api = new ApiAsync(url);
        this.locationInfo = new LocationInfo();
        this.deviceInfo = new deviceInfo();
        this.user = new User(this.api);
    }

    async initTracker() {
        this.locationInfo.initTracker();
        await this.deviceInfo.initTracker();
        this.user.populate({
            browser: this.deviceInfo.browser,
            os: this.deviceInfo.os,
            type: this.deviceInfo.type,
            country: this.locationInfo.getCountry(),
        });
        await this.user.init();
    }
}
export default OminiTracker;
/*
<script src="https://host/omini-tracker.min.js" async defer>
const ominitracker = new OminiTracker("http://host/");
ominitracker.initTracker();
</script>
*/
