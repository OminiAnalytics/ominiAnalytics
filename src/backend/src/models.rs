/*
 File: models.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:58:14
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/


use crate::schema::*;

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
extern crate diesel;
#[macro_use]
mod diesel;

#[derive(Insertable)]
#[table_name = "omini_alive_messages"]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub s_id: Uuid,
    pub mtype: String,
    pub s_duration: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct GetUser {
    pub id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub device_info: serde_json::Value,
    pub country: String,
    pub ip: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "omini_users"]
pub struct NewUser {
    #[sql_type = "SomeType"]
    pub id: Uuid,
    pub device_info: serde_json::Value,
    pub country: String,
    pub ip: String,
}
