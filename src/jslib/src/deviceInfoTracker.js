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

    // @Experimental
    // This function uses an experimental feature
    // that is not yet supported in all browsers
    // It fall back to getOsAndType() if not working
    async getOsAndTypeExp() {
        if (navigator.userAgentData === undefined) {
            var os,
                type = this.getOsAndType();
            return [os, type];
        }
        var device_info = navigator.userAgentData.getHighEntropyValues([
            "model",
            "platform",
            "platformVersion",
            "fullVersionList",
        ]);
        var [os, type] = [{}, "unknown"];
        await device_info.then((ua) => {
            if (ua.platform === "Windows" && !ua.mobile) {
                if (ua.platformVersion.split(".")[0] === "10") {
                    os = {
                        name: "Windows",
                        version: "10",
                    };
                } else if (ua.platformVersion.split(".")[0] === "6") {
                    os = {
                        name: "Windows",
                        version: "8.1 or lower",
                    };
                } else if (parseint(ua.platformVersion.split(".")[0]) >= 13) {
                    os = {
                        name: "Windows",
                        version: "11",
                    };
                }
                type = "desktop";
            } else {
                os = {
                    name: ua.platform,
                    version: ua.platformVersion,
                };
                type = ua.mobile ? "mobile" : "desktop";
            }
        });
        return [os, type];
    }

    getOsAndType() {
        // init ua
        const ua = navigator.userAgent;
        // Get device type
        var type = new Boolean(false);
        if ("maxTouchPoints" in navigator) {
            type = navigator.maxTouchPoints > 0;
        } else if ("msMaxTouchPoints" in navigator) {
            type = navigator.msMaxTouchPoints > 0;
        } else {
            var mQ = window.matchMedia && matchMedia("(pointer:coarse)");
            if (mQ && mQ.media === "(pointer:coarse)") {
                type = !!mQ.matches;
            } else if ("orientation" in window) {
                type = true; // dépréciée mais utile au cas où
            } else {
                // en dernier recours, on regarde userAgent
                var UA = navigator.userAgent;
                type =
                    /\b(BlackBerry|webOS|iPhone|IEMobile)\b/i.test(UA) ||
                    /\b(Android|Windows Phone|iPad|iPod)\b/i.test(UA);
            }
        }
