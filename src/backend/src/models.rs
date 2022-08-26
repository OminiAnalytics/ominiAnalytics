/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:42
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 23:35:21
 */
use crate::schema::*;

use diesel::pg::types::sql_types::Jsonb;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "omini_alive_messages"]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub mtype: String,
}

// diesel::sql_types::Uuid, Double, Double, diesel::sql_types::Jsonb
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct GetUser {
    pub id: Uuid,
    pub created_at: f64,
    pub updated_at: f64,
    pub device_info: serde_json::Value,
}
