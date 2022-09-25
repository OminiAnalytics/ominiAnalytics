/*
 File: login.rs
 Created Date: 17 Sep 2022
 Author: realbacon
 -----
 Last Modified: 25/09/2022 02:17:44
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::handlers::login_handler;
use super::structs::{Creds, LoginResponse};
use crate::errors::{self, HandlerError};
use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use deadpool_postgres::Pool;

#[post("/login")]
pub async fn login(
    pool: Data<Pool>,
    creds: Json<Creds>,
    session: Session,
) -> Result<HttpResponse, HandlerError> {
    let valid_cred = login_handler(&pool, &creds).await;
    match valid_cred {
        Ok(valid) => match valid {
            Some(uid) => {
                session.insert("uid", uid).unwrap();
                return Ok(HttpResponse::Ok().json(LoginResponse {
                    status: "success".to_string(),
                }));
            }
            None => Ok(HttpResponse::Ok().json(LoginResponse {
                status: "failed".to_string(),
            })),
        },
        Err(_) => Err(errors::HandlerError::DBError),
    }
}

#[post("/verify")]
pub async fn verify_session(session: Session) -> Result<HttpResponse, HandlerError> {
    // TO-DO : Verify the uid
    match session.get::<String>("uid") {
        Ok(_) => Ok(HttpResponse::Ok().json(LoginResponse {
            status: "success".to_string(),
        })),
        Err(_) => Ok(HttpResponse::Ok().json(LoginResponse {
            status: "failed".to_string(),
        })),
    }
}
