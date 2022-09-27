/*
 File: sql.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 03:09:58
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

pub const GET_CONNECTED_USERS: &str =
    r#"SELECT COUNT(DISTINCT u_id) FROM omini_alive_messages WHERE created_at > $1;"#;

pub const GET_TOTAL_USERS: &str = r#"SELECT COUNT(*) FROM users"#;
