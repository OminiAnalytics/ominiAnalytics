/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:25
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 23:13:26
 */

// Actix web & co
use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use chrono::{NaiveDateTime, Utc};
use derive_more::Display;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

// Diesel stuff
extern crate diesel;
use crate::{DBPool, DBPooledConnection};
use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::result::Error;
use diesel::{ExpressionMethods, RunQueryDsl};

// import data structure
use super::structs::{SignalResult, ValidAliveMessage};
use crate::models::NewAliveMessage;

// DB
use super::alive_db::insert_alive_message;

#[post("/alive")]
pub async fn is_alive(pool: Data<DBPool>, alive_message: Json<ValidAliveMessage>) -> HttpResponse {
    let id = Uuid::new_v4();
    let _u_id = Uuid::parse_str(&alive_message.u_id[..]);
    let u_id: Uuid;
    match _u_id {
        Ok(o) => u_id = o,
        Err(_) => {
            return HttpResponse::BadRequest().json(SignalResult {
                success: false,
                at: Utc::now().to_rfc3339(),
                message: "Invalid UID format".to_string(),
            });
        }
    }
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let new_alive_message = NewAliveMessage {
        id: id.clone(),
        u_id,
        mtype: "isalive".to_string(),
    };

    let result = insert_alive_message(&conn, &new_alive_message);
    match result {
        Ok(_) => HttpResponse::Ok().json(SignalResult {
            success: true,
            message: "Alive message sent".to_string(),
            at: Utc::now().naive_utc().to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(SignalResult {
            success: false,
            message: "Error while handling alive message".to_string(),
            at: Utc::now().naive_utc().to_string(),
        }),
    }
}
