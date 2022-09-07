/*
 File: structs.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 31/08/2022 02:11:6
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub bro: Browser, // browser
    pub os: Os,       // os
    pub dty: String,  // device type
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub nam: String, // name
    pub ver: String, // version
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct Browser {
    pub nam: String, // name
    pub ver: String, // version
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uid: String,
    pub dev: DeviceInfo, // device
    pub cou: String,     // country
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct PageCall {
    pub nam: String, // page name
    pub url: String, // url
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct NewInst {
    pub usr: User,     // user
    pub pag: PageCall, // page
}

#[derive(Serialize, Deserialize)]
pub struct ValidAliveMessage {
    pub uid: String,
    pub date: i64,
}
