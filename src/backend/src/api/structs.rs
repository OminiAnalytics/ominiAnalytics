/*
 File: structs.rs
 Created Date: 28 Aug 2022
 Author: realbacon
 -----
 Last Modified: 29/08/2022 02:32:6
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub browser: Browser,
    pub os: Os,
    pub dtype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Browser {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub uid: String,
    pub at: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidUidMessage {
    pub uid: String,
    pub device: DeviceInfo,
    pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidAliveMessage {
    pub u_id: String,
    pub date: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SignalResult {
    pub success: bool,
    pub message: String,
    pub at: i64,
}
