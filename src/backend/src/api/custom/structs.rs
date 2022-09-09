/*
 File: structs.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 9/09/2022 11:10:1
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct PageCall {
    pub nam: String, // page name
    pub url: String, // url
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct NewInst {
    pub bp: BPayload,  // base64 payload and hash
    pub pag: PageCall, // page
}

#[derive(Serialize, Deserialize)]
pub struct ValidAliveMessage {
    pub uid: String,
    pub date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BPayload {
    pub dt: String,   // Base64 payload
    pub hash: String, // hash
}
