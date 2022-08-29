/*
 * File: locationInfoTracker.js
 * Created Date: 28 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 28/08/2022 11:38:31
 * Modified By: realbacon
 * -----
 * Copyright (c) 2022 Omini
 * -----
 */

const { countries, zones } = require("moment-timezone/data/meta/latest.json");
export class LocationInfo {
    constructor() {
        this.country;
    }
    initTracker() {
        // Get timezone from browser and set it to moment
        const timeZoneToCountry = {};
        const userTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;

        Object.keys(zones).forEach((z) => {
            timeZoneToCountry[z] = countries[zones[z].countries[0]].name;
        });

        this.country = timeZoneToCountry[userTimeZone];
    }

    getCountry() {
        return this.country;
    }
}
