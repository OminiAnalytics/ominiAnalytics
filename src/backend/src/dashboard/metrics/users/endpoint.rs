/*
 File: endpoint.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 03:55:9
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::handlers::{get_active_users_since, get_connected_users, get_total_users};
use super::structs::CountStruct;
use crate::errors::HandlerError::DBError;
use crate::{dashboard::auth::handlers::verify_session, errors::HandlerError};
use actix_session::Session;
use actix_web::{get, web::Data, web::Path, HttpResponse};
use deadpool_postgres::Pool;

#[get("/connected")]
pub async fn connected(pool: Data<Pool>, session: Session) -> Result<HttpResponse, HandlerError> {
    match verify_session(session) {
        false => return Err(HandlerError::Unauthorized),
        true => {}
    };
    let request = get_connected_users(&pool);
    match request.await {
        Ok(res) => Ok(HttpResponse::Ok().json(CountStruct { count: res })),
        Err(_) => Err(DBError),
    }
}

#[get("/total")]
pub async fn total_users(pool: Data<Pool>, session: Session) -> Result<HttpResponse, HandlerError> {
    match verify_session(session) {
        false => return Err(HandlerError::Unauthorized),
        true => {}
    };
    let request = get_total_users(&pool);
    match request.await {
        Ok(res) => Ok(HttpResponse::Ok().json(CountStruct { count: res })),
        Err(_) => Err(DBError),
    }
}

#[get("/active/{time}/{to}")]
pub async fn active_users(
    pool: Data<Pool>,
    session: Session,
    from: Path<i64>,
    to: Path<i64>,
) -> Result<HttpResponse, HandlerError> {
    match verify_session(session) {
        false => return Err(HandlerError::Unauthorized),
        true => {}
    };
    let request = get_active_users_since(&pool, from.into_inner(), to.into_inner());
    match request.await {
        Ok(res) => Ok(HttpResponse::Ok().json(CountStruct { count: res })),
        Err(_) => Err(DBError),
    }
}
