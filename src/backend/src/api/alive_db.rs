/*
 * @Author: realbacon
 * @Date: 2022-08-26 18:09:09
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-27 00:21:03
 */

use crate::models::NewAliveMessage;
use crate::schema::omini_alive_messages;
use crate::schema::omini_alive_messages::dsl::*;
use crate::DBPooledConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn insert_alive_message(
    conn: &DBPooledConnection,
    new_alive_message: &NewAliveMessage,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(omini_alive_messages::table)
        .values(new_alive_message)
        .execute(conn)
}

pub fn get_last_signal(
    conn: &DBPooledConnection,
    selected_u_id: Uuid,
) -> Result<(i64, Uuid, i32), diesel::result::Error> {
    omini_alive_messages
        .filter(u_id.eq(selected_u_id))
        .order(created_at.desc())
        .select((created_at, s_id, s_duration))
        .first(conn)
}
