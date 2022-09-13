/*
 File: models.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 10:36:56
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

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
