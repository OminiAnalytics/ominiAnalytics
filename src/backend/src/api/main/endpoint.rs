/*
 File: endpoint.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:55:10
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/







use actix_web::{post, web::Data, web::Json, HttpRequest, HttpResponse, ResponseError};
use derive_more::Display;
// Import data structure
use crate::api::structs::NewInst;
use crate::DBPool;
use uuid::Uuid;

// Db
use super::db::{insert_user, user_exists};

#[post("/main")]
pub async fn main_procedure_handler(
    req: HttpRequest,
    pool: Data<DBPool>,
    payload: Json<NewInst>,
) -> Result<HttpResponse, HandlerError> {
    actix_rt::spawn_blocking(async { main_procedure(req, pool, payload) })
        .await
        .unwrap()
}

fn main_procedure(
    req: HttpRequest,
    pool: Data<DBPool>,
    payload: Json<NewInst>,
) -> Result<HttpResponse, HandlerError> {
    // We first filter the request
    let ip: String;
    if let Some(_ip) = crate::api::security::request_filter(&req) {
        ip = _ip.to_string();
    } else {
        return Err(HandlerError::InvalidRequest);
    };
    let user = payload.into_inner().usr;
    // Initiate connection
    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");
    // Serialize device
    let device = serde_json::to_string(&user.dev).unwrap();
    let device: serde_json::Value = serde_json::from_str(&device[..]).unwrap();
    // Serialize country
    let country = &user.cou;
    // Init uid
    let _uid: Uuid;
    // Check if user exists
    let user_with_given_ip = user_exists(&mut conn, &ip);
    if user_with_given_ip.is_ok() {
        _uid = user_with_given_ip.unwrap();
    } else {
        let new_user = insert_user(&mut conn, device, country, &ip);
        if new_user.is_err() {
            return Err(HandlerError::DBError);
        }
        _uid = new_user.unwrap().0;
    }
    Ok(HttpResponse::Ok().body(format!("{{ \"uid\": \"{}\" }} ", _uid)))
}

#[derive(Display, Debug)]
pub enum HandlerError {
    DBError,
    InvalidRequest,
    InternalError,
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HandlerError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::DBError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::InvalidRequest => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(match self {
            HandlerError::InternalError => "Internal server error",
            HandlerError::DBError => "DB error",
            HandlerError::InvalidRequest => "Invalid request",
        })
    }
}
