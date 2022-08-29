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

export class deviceInfo {
    constructor() {
        this.browser = {
            name: undefined,
            version: undefined,
        };
        this.os = {
            name: undefined,
            version: undefined,
        };
        this.type = undefined;
    }

    async initTracker() {
        this.browser = this.getBrowser();
        [this.os, this.type] = await this.getOsAndTypeExp();
    }

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

        // Get os
        // @deprecated : navigator.platform
        var version = "unknown";
        var name = navigator.platform;
        if (name.indexOf("Win") > -1) {
            name = "Windows";
        } else if (name.indexOf("Mac") > -1 || name.indexOf("iP") > -1) {
            name = navigator.platform;
        } else if (
            name.indexOf("Android") > -1 ||
            name.indexOf("Linux armv7l") > -1
        ) {
            name = "Android";
        } else if (name.indexOf("Linux") > -1) {
            name = "Linux";
        } else if (name.indexOf("Nintendo") > -1) {
            name = "Nintendo";
        } else if (name.indexOf("PlayStation") > -1) {
            name = "PlayStation";
        } else if (name.indexOf("BlackBerry") > -1) {
            name = "BlackBerry";
        } else if (name.indexOf("BlackBerry") > -1) {
            name = "BlackBerry";
        } else {
            name = "unknown";
        }

        // Try to get os version
        if (ua && type === "desktop") {
            // Try to get os version from oscpu for windows
            if (ua.indexOf("Windows") > -1) {
                if (ua.indexOf("NT 10.0") > -1) {
                    version = "10";
                } else if (ua.indexOf("NT 13") > -1) {
                    version = "11";
                } else {
                    version = "8.1 or lower";
                }
            } else if (ua.indexOf("Mac")) {
                if (ua.match("(OS X 10_10)|(OS X 10.10)")) {
                    version = "10.10";
                } else if (ua.match("(OS X 10_9)|(OS X 10.9)")) {
                    version = "10.9";
                } else if (ua.match("(OS X 10_8)|(OS X 10.8)")) {
                    version = "10.8";
                } else if (ua.match("(OS X 10_7)|(OS X 10.7)")) {
                    version = "10.7";
                }
            }
        } else if (
            ua &&
            type === "mobile" &&
            ["iPhone", "iPad", "iPod"].includes(name)
        ) {
            version = String(
                ua
                    .match(/OS (\d)?\d_\d(_\d)?/i)[0]
                    .replace(/_/g, ".")
                    .replace("OS ", "")
            );
        }
        var os = {
            name,
            version,
        };
        return [os, type];
    }
}
