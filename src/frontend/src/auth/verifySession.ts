/**
 * File: verifyCookie.ts
 * Created Date: 24 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 27/09/2022 01:00:10
 * Modified By: realbacon
 * -----
 * License  : MIT
 * -----
 * */

/**
 * Verify is a cookie is present and valid
 * if not, redirect to login page
 */
export async function verifySession(): Promise<boolean> {
    if (!process.env.NODE_ENV || process.env.NODE_ENV === "development") {
        return true;
    }
    const result = await fetch(process.env.REACT_APP_API + "/dsh/verify", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
    })
        .then((res) => res.json())
        .then((data) => {
            if (data.status === "failed") {
                return false;
            }
            return true;
        })
        .catch(() => {
            return false;
        });
    return result;
}
