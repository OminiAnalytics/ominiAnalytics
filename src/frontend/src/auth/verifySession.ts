/**
 * File: verifyCookie.ts
 * Created Date: 24 Sep 2022
 * Author: realbacon
 * -----
 * Last Modified: 25/09/2022 11:52:54
 * Modified By: realbacon
 * -----
 * License  : MIT
 * -----
 * */

/**
 * Verify is a cookie is present and valid
 * if not, redirect to login page
 */
export async function verifySession() {
  const result = await fetch(process.env.REACT_APP_API + '/dsh/verify', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    }
  })
    .then((res) => res.json())
    .then((data) => {
      if (data.status === 'failed') {
        return false
      }
      return true
    })
    .catch(() => {
      return false
    })
  return result
}
