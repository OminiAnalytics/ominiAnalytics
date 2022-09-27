/*
 File: handler.rs
 Created Date: 24 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 05:45:32
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::sql;
use super::structs::Creds;
use crate::db::get_connection;
use actix_session::Session;
use deadpool_postgres::Pool;
use uuid::Uuid;

pub async fn login_handler(pool: &Pool, creds: &Creds) -> Result<Option<String>, String> {
    let connection = get_connection(&pool).await?;
    let exists = connection
        .query_opt(sql::GET_USER, &[&creds.username, &creds.password])
        .await
        .map_err(|e| e.to_string())?;
    match exists {
        Some(row) => Ok(Some(row.try_get::<_, Uuid>(0).unwrap().to_string())),
        None => Ok(None),
    }
}

pub fn verify_session(session: Session) -> Result<String, ()> {
    // TO-DO : Verify the uid
    match session.get::<String>("uid") {
        Ok(opt) => match opt {
            Some(id) => Ok(id),
            None => Err(()),
        },
        Err(_) => Err(()),
    }
}
