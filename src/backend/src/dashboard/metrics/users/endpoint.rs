/*
 File: endpoint.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 25/09/2022 02:22:51
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::handlers::get_connected_users;
use super::structs::ConnectedUsers;
use crate::errors::HandlerError::DBError;
use crate::{dashboard::auth::handlers::verify_session, errors::HandlerError};
use actix_session::Session;
use actix_web::{get, web::Data, HttpResponse};
use deadpool_postgres::Pool;

#[get("/connected")]
pub async fn connected(pool: Data<Pool>, session: Session) -> Result<HttpResponse, HandlerError> {
    match verify_session(session) {
        false => return Err(HandlerError::Unauthorized),
        true => {}
    };
    let request = get_connected_users(&pool);
    match request.await {
        Ok(res) => Ok(HttpResponse::Ok().json(ConnectedUsers { count: res })),
        Err(_) => Err(DBError),
    }
}
