/*
 * @Author: realbacon
 * @Date: 2022-08-26 18:09:09
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 18:21:50
 */

use crate::models::NewAliveMessage;
use crate::schema::omini_alive_messages;
use crate::DBPooledConnection;
use diesel::RunQueryDsl;

pub fn insert_alive_message(
    conn: &DBPooledConnection,
    new_alive_message: &NewAliveMessage,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(omini_alive_messages::table)
        .values(new_alive_message)
        .execute(conn)
}
