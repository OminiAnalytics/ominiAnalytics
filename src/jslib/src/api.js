/*
 * File: api.js
 * Created Date: 28 Aug 2022
 * Author: realbacon
 * -----
 * Last Modified: 29/08/2022 11:05:53
 * Modified By: realbacon
 * -----
 * Copyright (c) 2022 Omini
 * -----
 */

export class ApiAsync {
    constructor(url) {
        // Delete the slash at the end of the url if it exists
        this.url = url;
        if (this.url.slice(-1) === "/") {
            this.url = this.url.slice(0, -1);
        }
        this.endpoints = {
            uid: "/api/uid",
            parkour: "/api/parkour",
            alive: "/api/alive",
        };
    }
    /*
    endpoint : api endpoint @string
    data : data to send @object
    return : result @object
    */
    async getData({ endpoint, params }) {
        var paramString = "";
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
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                },
                // allow cors
                mode: "cors",
            }
        )
            .then((response) => ["success", response.json()])
            .catch((err) => {
                console.error("Debug : " + err);
                return ["error", err];
            });
        return response;
    }

    async postDataJSON({ endpoint, data }) {
        var Response = new Result();
        await fetch(this.url + this.endpoints[endpoint], {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
            // allow cors
            mode: "cors",
        })
            .then((resp) => {
                if (resp.ok) {
                    Response.new({
                        result: "success",
                        response: resp,
                        error: resp.statusText,
                    });
                } else {
                    Response.new({
                        result: "error",
                        response: resp,
                        error: resp.statusText,
                    });
                    throw new Error(resp.statusText);
                }
            })
            .catch((err) => {
                console.error("Debug : " + err);
                Response.new({
                    result: "error",
                    response: undefined,
                    error: err,
                });
            });
        return Response;
    }
}

class Result {
    constructor() {
        this.result;
        this.statusCode;
        this.data;
        this.headers;
        this.error;
    }

    new({ result, response, error }) {
        this.result = result;
        this.statusCode = response.status;
        this.headers = response.headers;
        this.error = error;
        this.data = response;
    }

    isErr() {
        return this.result === "error";
    }
    isOk() {
        return this.result === "success";
    }

    async toJson() {
        this.data = await this.data.json().then((data) => data);
    }

    async clone() {
        this.data = await this.data.clone().then((data) => data);
    }
}
