/*
 * @Author: realbacon
 * @Date: 2022-08-26 18:02:38
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 18:03:38
 */
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub browser: Browser,
    pub os: String,
    pub lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Browser {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub uid: String,
    pub at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidUidMessage {
    pub uid: Uuid,
    pub device: DeviceInfo,
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
    pub at: String,
}
