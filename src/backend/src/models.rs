use crate::schema::alive_messages;
use diesel::{Insertable, Queryable};
use uuid::Uuid;
#[derive(Insertable)]
#[table_name = "alive_messages"]
pub struct NewAliveMessage {
    pub id: Uuid,
    pub u_id: Uuid,
    pub mtype: String,
}
