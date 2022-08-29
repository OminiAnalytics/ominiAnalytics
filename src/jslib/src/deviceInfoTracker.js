/*
 * File: deviceInfoTracker.js
 * Created Date: 28 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 29/08/2022 02:30:59
 * Modified By: realbacon
 * -----
 * Copyright (c) 2022 Omini
 * -----
 */

    getBrowser() {
        if (navigator.userAgentData != undefined) {
            let fbrands = navigator.userAgentData.brands.filter(
                (brand) =>
                    brand.brand.indexOf("Brand") === -1 &&
                    brand.brand.indexOf("Chromium") === -1
            )[0];
            return {
                name: fbrands.brand,
                version: fbrands.version,
            };
        }
        let browser = navigator.userAgent.match(
            /(Firefox|Safari|Edge|Chrome)\/([\S]+)/i
        );
        if (navigator.brave && navigator.brave.isBrave()) {
            var name = "Brave";
        } else {
            var name = browser[0];
        }
        let version = browser[1].split(".")[0];
        return {
            name,
            version,
        };
    }
