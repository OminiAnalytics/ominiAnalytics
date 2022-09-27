/*
 File: handlers.rs
 Created Date: 27 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 05:53:41
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use super::sql::CREATE_EVENT;
use super::structs::Event;
use crate::db::get_connection;
use deadpool_postgres::Pool;

pub async fn create_event(pool: &Pool, event: &Event) -> Result<Result<String, String>, String> {
    let client = get_connection(pool).await?;
    let result = client
        .query_one(
            CREATE_EVENT,
            &[
                &event.creator_id,
                &event.compaign_id,
                &event.event_id,
                &event.slug,
                &event.descr,
                &event.active,
            ],
        )
        .await
        .map_err(|err| err.to_string())?;
    Ok(result.try_get(0).map_err(|err| err.to_string()))
}
