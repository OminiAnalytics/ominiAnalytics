/*
 File: structs.rs
 Created Date: 24 Sep 2022
 Author: realbacon
 -----
 Last Modified: 24/09/2022 12:06:37
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub status: String,
}
