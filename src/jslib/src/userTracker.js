/*
 * File: userTracker.js
 * Created Date: 28 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 29/08/2022 11:06:56
 * Modified By: realbacon
 * -----
 * Copyright (c) 2022 Omini
 * -----
 */

// User class
export class User {
    constructor(api) {
        this.uid; // User id
        this.api = api; // Api instance
        this.userIsValid = false; // User is valid
    }

    // Collect data to send to server
    populate({ browser, os, type, country }) {
        this.browser = browser;
        this.os = os;
        this.type = type;
        this.country = country;
    }

    // Try to get uid cookie or get an empty one
    // Then validate it to the server and get a new uid if not valid
    /*
    return : Result => [success?,data] @array
    */
    async getUid() {
        // Check if we can access cookie
        if (navigator.cookieEnabled) {
            // Check if user has cookie "Om_uid"
            if (document.cookie.indexOf("Om_uid") > -1) {
                // If yes, check if it is valid
                const guidcookie = document.cookie
                    .split("Om_uid=")[1]
                    .split(";")[0];
                var [result, uidcookie] = await this.validateUidFetch(
                    guidcookie
                ).then();
            } else {
                // If no, create new uid cookie
                var [result, uidcookie] = this.validateUidFetch("");
            }
            console.log(uidcookie);
            if (result === "success") {
                this.setCookie(uidcookie);
                return ["success", uidcookie];
            } else {
                return ["error", false];
            }
        } else {
            return ["error", false];
        }
    }

    // Create new uid cookie
    /*
    uid : uid @string
    */
    setCookie(uid) {
        document.cookie =
            "Om_uid=" + uid + "; max-age=31536000; samesite=strict; Path=/";
    }

    // Init
    // If we succeed to get a uid and create a cookie, we set the user as valid
    async init() {
        // Set uid cookie
        [result, d] = await this.getUid();
        if (result === "success") {
            this.uid = d;
            this.userIsValid = true;
        }
    }

    // Fetch uid endpoint to validate uid
    // If not valid, we get a new uid from the server
    // Then we try to deserialize the uid
    /*
    uidcookie : uid to validate @string
    return : Result => [success?,data] @array
    */
    async validateUidFetch(uidcookie) {
        const response = await this.api.postDataJSON({
            endpoint: "uid",
            data: {
                uid: uidcookie,
                country: this.country,
                device: {
                    browser: this.browser,
                    os: this.os,
                    dtype: this.type,
                },
            },
        });
        if (response.isOk()) {
            try {
                await response.toJson();
                const { uid, _at } = response.data;
                return ["success", uid];
            } catch (_) {
                return ["error", false];
            }
        }
        return ["error", false];
    }
}
