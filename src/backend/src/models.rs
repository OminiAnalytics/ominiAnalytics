/*
 File: models.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 31/08/2022 07:42:25
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

#[derive(PostgresMapper)]
#[pg_mapper(table = "omini_alive_messages")]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub s_id: Uuid,
    pub mtype: String,
    pub s_duration: i32,
}

#[derive(PostgresMapper, Debug, Serialize, Deserialize)]
#[pg_mapper(table = "omini_users")]
pub struct GetUser {
    pub id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub device_info: serde_json::Value,
    pub country: String,
    pub ip: String,
}

#[derive(PostgresMapper, Debug, Serialize, Deserialize)]
#[pg_mapper(table = "omini_users")]
pub struct NewUser {
    pub id: Uuid,
    pub device_info: serde_json::Value,
    pub country: String,
    pub ip: String,
}
