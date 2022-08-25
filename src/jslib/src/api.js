/*
 * @Author: realbacon
 * @Date: 2022-08-25 11:31:58
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-25 12:03:18
 */

export class ApiAsync {
    constructor(url, apiKey) {
        this.url = url;
        this.key = apiKey;
        this.endpoints = {
            uid: "/uid",
            parkour: "/parkour",
            alive: "/alive",
        };
    }
    async getData({ endpoint, params }) {
        var paramString = "";
        params["key"] = this.key;
        paramString =
            "?" +
            Object.keys(params)
                .map(function (key) {
                    return key + "=" + params[key];
                })
                .join("&");

        const response = await fetch(
            this.url + this.endpoints[endpoint] + paramString,
            {
                method: method || "GET",
                headers: {
                    "Content-Type": "application/json",
                },
                // allow cors
                mode: "cors",
            }
        );
        const data = await response.json();
        return data;
    }

    async postDataJSON({ endpoint, data }) {
        data["key"] = this.key;
        const response = await fetch(this.url + this.endpoints[endpoint], {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
            // allow cors
            mode: "cors",
        });
        const rdata = await response.json();
        return rdata;
    }
}
