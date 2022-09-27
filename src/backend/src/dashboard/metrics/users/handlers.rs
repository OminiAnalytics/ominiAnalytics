/*
 File: handlers.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 03:55:34
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::sql::{GET_ACTIVE_USERS_SINCE, GET_CONNECTED_USERS, GET_TOTAL_USERS};
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

pub async fn get_total_users(pool: &Pool) -> Result<i64, String> {
    let client = get_connection(pool).await?;
    let rows = client
        .query(GET_TOTAL_USERS, &[])
        .await
        .map_err(|err| err.to_string())?;
    Ok(rows[0].get(0))
}

pub async fn get_active_users_since(pool: &Pool, from: i64, to: i64) -> Result<i64, String> {
    let client = get_connection(pool).await?;
    let rows = client
        .query(GET_ACTIVE_USERS_SINCE, &[&from, &to])
        .await
        .map_err(|err| err.to_string())?;
    Ok(rows[0].get(0))
}
