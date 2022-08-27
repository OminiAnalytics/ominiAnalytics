/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:42
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-27 00:10:36
 */
use crate::schema::*;

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "omini_alive_messages"]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub s_id: Uuid,
    pub mtype: String,
    pub s_duration: i32,
}

// diesel::sql_types::Uuid, Double, Double, diesel::sql_types::Jsonb
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct GetUser {
    pub id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub device_info: serde_json::Value,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "omini_users"]
pub struct NewUser {
    pub id: Uuid,
    pub device_info: serde_json::Value,
}
