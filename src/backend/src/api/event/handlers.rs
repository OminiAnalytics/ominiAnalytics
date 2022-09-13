/*
 File: db.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 09:14:46
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::sql::*;
use crate::db::get_connection;
use crate::models::NewAliveMessage;
use actix_web::web::Data;
use deadpool_postgres::Pool;
use uuid::Uuid;

pub async fn insert_alive_message(
    pool: &Data<Pool>,
    new_alive_message: &NewAliveMessage,
) -> Result<u8, tokio_postgres::Error> {
    /*diesel::insert_into(omini_alive_messages::table)
    .values(new_alive_message)
    .execute(conn)*/
    let conn = get_connection(pool)
        .await
        .expect("Failed to get connection");
    let _result = conn
        .query_opt(
            INSERT_ALIVE_MESSAGE_QUERY,
            &[
                &new_alive_message.id,
                &new_alive_message.u_id,
                &new_alive_message.s_id,
                &new_alive_message.mtype,
                &new_alive_message.s_duration,
            ],
        )
        .await?;
    Ok(1)
}
/// Get the last signal of a user
/// # Arguments
/// * `pool` - The database pool
/// * `uid` - The user id
/// # Returns
/// * Result<(i64, Uuid, i32), tokio_postgres::Error>
pub async fn get_last_signal(
    pool: &Data<Pool>,
    u_id: Uuid,
) -> Result<Option<(i64, Uuid, i32)>, tokio_postgres::Error> {
    let conn = get_connection(pool)
        .await
        .expect("Error getting connection");
    let result = conn.query_opt(GET_LAST_SIGNAL_QUERY, &[&u_id]).await?;
    if let Some(row) = result {
        let created_at: i64 = row.get("created_at");
        let s_id: Uuid = row.get("s_id");
        let s_duration: i32 = row.get("s_duration");
        return Ok(Some((created_at, s_id, s_duration)));
    } else {
        return Ok(None);
    }
}
