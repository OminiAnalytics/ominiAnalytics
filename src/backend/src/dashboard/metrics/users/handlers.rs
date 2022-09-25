/*
 File: handlers.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 25/09/2022 03:00:25
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::sql::GET_CONNECTED_USERS;
use crate::db::get_connection;
use chrono::Utc;
use deadpool_postgres::Pool;

pub async fn get_connected_users(pool: &Pool) -> Result<i64, String> {
    let client = get_connection(pool).await?;
    let now = Utc::now().timestamp() - 30;
    let rows = client
        .query(GET_CONNECTED_USERS, &[&now])
        .await
        .map_err(|err| err.to_string())?;
    Ok(rows[0].get(0))
}
