/*
 * @Author: realbacon
 * @Date: 2022-08-25 11:30:12
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-25 14:42:31
 */

/*
Endpoints:
@uid
@parkour
@alive
*/

// User class
export class User {
    constructor(api) {
        this.uid; // user id
        this.device = {
            browser: {
                name: undefined,
                version: undefined,
            },
            os: undefined,
            type: undefined,
        }; // device information
        this.lang; // language
        this.camefrom = undefined; // Came from (referer)
        this.page = {
            title: undefined,
            urlpath: undefined,
            search: undefined,
            performance: undefined,
        }; // page info
        this.host = document.location.origin; // hostname
        this.api = api; // api object
    }

    // Getters
    getUid() {
        // Check if we can access cookie
        if (document) {
            // Check if user has cookie "Om_uid"
            if (document.cookie.indexOf("Om_uid") > -1) {
                // If yes, check if it is valid
                var uidcookie = document.cookie
                    .split("Om_uid=")[1]
                    .split(";")[0];
                // Create new uid coookie with max exp date, https
                uidcookie = this.validateUid_fetch(uidcookie);
                document.cookie =
                    "Om_uid=" +
                    uidcookie +
                    "; max-age=31536000; samesite=strict";
            } else {
                // If no, create new uid cookie
                var uidcookie = this.validateUid_fetch("");
                document.cookie =
                    "Om_uid=" +
                    uidcookie +
                    "; max-age=31536000; samesite=strict";
            }
            return uidcookie;
        } else {
            return false;
        }
    }

    getPageInfo() {
        // Get page info
        this.page.title = document.title;
        this.page.urlpath = window.location.origin + window.location.pathname;
        this.page.search = window.location.search;
        // Use deprecated performance API
        // @deprecated
        this.page.performance =
            window.performance.timing.domContentLoadedEventEnd -
            window.performance.timing.navigationStart;
    }

    getDeviceInfo() {
        // Get browser name from user agent string
        let uabrand = navigator.userAgentData.brands;
        uabrand = uabrand.filter(
            (brand) =>
                brand.brand.indexOf("Brand") === -1 &&
                brand.brand.indexOf("Chromium") === -1
        )[0];
        const browser = {
            name: uabrand.brand,
            version: uabrand.version,
        };

        // Get os
        // @deprecated : navigator.platform
        var os = navigator.platform;
        if (os.indexOf("Win32") > -1) {
            os = "Windows 64-bit";
        } else if (os.indexOf("Win16") > -1) {
            os = "Windows 32-bit";
        } else if (os.indexOf("Mac") > -1) {
            os = "Mac";
        } else if (
            os.indexOf("Android") > -1 ||
            os.indexOf("Linux armv7l") > -1
        ) {
            os = "Android";
        } else if (os.indexOf("Linux") > -1) {
            os = "Linux";
        } else if (os.indexOf("iPhone") > -1) {
            os = "iPhone";
        } else if (os.indexOf("iPad") > -1) {
            os = "iPad";
        } else if (os.indexOf("Nintendo") > -1) {
            os = "Nintendo";
        } else if (os.indexOf("PlayStation") > -1) {
            os = "PlayStation";
        } else if (os.indexOf("BlackBerry") > -1) {
            os = "BlackBerry";
        } else {
            os = "unknown";
        }

        // Get device type
        const mobile = ["iPhone", "iPad", "Android", "BlackBerry"];
        const tablet = ["iPad"];
        const desktop = ["Windows 64-bit", "Mac", "Linux", "Windows 32-bit"];
        const consoles = ["Nintendo", "PlayStation"];
        var type;
        if (mobile.includes(os)) {
            type = "mobile";
        }
        if (tablet.includes(os)) {
            type = "tablet";
        }
        if (desktop.includes(os)) {
            type = "desktop";
        }
        if (consoles.includes(os)) {
            type = "console";
        }
        this.device = {
            browser: browser,
            os: os,
            type: type,
        };
    }

    getLanguage() {
        // Get language from browser
        this.lang = navigator.language;
    }

    getPrevPage() {
        // Get previous page
        const prevPage = document.referrer;
        // Check if previous page is not empty
        if (prevPage !== "") {
            this.camefrom = prevPage;
            // Send previous page to backend
        }
    }

    // Init
    init() {
        // Initialize user class

        // Get device info
        this.getDeviceInfo();

        // Get language
        this.getLanguage();

        // Set uid cookie
        this.uid = this.getUid();

        // Get page info
        this.getPageInfo();

        // Init tracker
        this.initTracker();

        // Get previous page
        this.getPrevPage();

        // Send page info to backend
        this.PageInfo_fetch();
    }

    initTracker() {
        // Add event on page unload

        // Get date in seconds when page is loaded
        var dateTimeAtPageLoad = new Date();
        var timeAtPageLoad = Math.floor(dateTimeAtPageLoad.getTime() / 1000);
        window.addEventListener("beforeunload", (event) => {
            let dateTimeAtPageLeave = new Date();
            let timeAtPageLeave = Math.floor(
                dateTimeAtPageLeave.getTime() / 1000
            );
            let timeSpent = timeAtPageLeave - timeAtPageLoad;

            // Send time spent to backend
            this.leavePage_fetch(timeSpent);
        });

        // Add event on click
        document.addEventListener("click", (event) => {
            // TO-DO : Send event to backend
            // @parkour
        });

        // Send alive message
        this.isAlive_fetch();
        setInterval(() => {
            this.isAlive_fetch();
        }, 1000 * 20); // 20 minutes
    }

    // Fetch

    validateUid_fetch(uidcookie) {
        return "12345abcdef6789ghij";
        // TO-DO : validate uid cookie with backend
        // TO-DO : Register lang and device info in backend
        // @uid
    }

    leavePage_fetch(timeSpent) {
        // TO-DO : Send timeSpent to backend
        // @parkour
        console.log(timeSpent);
    }

    isAlive_fetch() {
        // Send that user is alive
        // TO-DO : Send event to backend
        // @alive

        // Get date in seconds
        let dateTimeCurrent = new Date();
        let timeCurrent = Math.floor(dateTimeCurrent.getTime() / 1000);
    }

    PageInfo_fetch() {
        // TO-DO : Send previous page to backend
        // @parkour
        // Send current page to backend
        /*
        send(
            {
                page : this.page,
                lang : this.lang,
                camfrom : this.camefrom || false,
            }
        )
        */
    }
}
