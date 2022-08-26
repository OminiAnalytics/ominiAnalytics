/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:25
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 11:57:37
 */
use crate::models::NewAliveMessage;
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
// Import models
#[derive(Serialize, Deserialize)]
pub struct ValidAliveMessage {
    pub u_id: String,
    pub date: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RequestResult {
    pub success: bool,
    pub message: String,
    pub at: String,
}

#[post("/alive")]
pub async fn is_alive(pool: Data<DBPool>, _alive_message: Json<ValidAliveMessage>) -> HttpResponse {
    let id = Uuid::new_v4();
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    use crate::schema::omini_alive_messages;
    let new_alive_message = NewAliveMessage {
        id: id.clone(),
        u_id: id.clone(),
        mtype: "isalive".to_string(),
    };

    let result = diesel::insert_into(omini_alive_messages::table)
        .values(&new_alive_message)
        .execute(&conn);

    match result {
        Ok(_) => HttpResponse::Ok().json(RequestResult {
            success: true,
            message: "Alive message sent".to_string(),
            at: Utc::now().naive_utc().to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(RequestResult {
            success: false,
            message: "Error while handling alive message".to_string(),
            at: Utc::now().naive_utc().to_string(),
        }),
    }
}
