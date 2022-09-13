/*
 File: sql.rs
 Created Date: 13 Sep 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 09:02:14
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

pub const GET_LAST_SIGNAL_QUERY: &str =
    "SELECT * FROM omini_alive_messages WHERE u_id = $1 ORDER BY created_at DESC LIMIT 1";

pub const INSERT_ALIVE_MESSAGE_QUERY: &str =
    "INSERT INTO omini_alive_messages (id, u_id, s_id, mtype, s_duration) VALUES ($1, $2, $3, $4, $5)";
