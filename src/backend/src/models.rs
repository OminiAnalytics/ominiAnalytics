/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:42
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 11:57:26
 */
use crate::schema::omini_alive_messages;
use diesel::{Insertable, Queryable};
use uuid::Uuid;
#[derive(Insertable)]
#[table_name = "omini_alive_messages"]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub mtype: String,
}
