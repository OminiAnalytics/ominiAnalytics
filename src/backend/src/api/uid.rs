/*
 * @Author: realbacon 
 * @Date: 2022-08-26 11:34:18 
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 11:35:20
 */
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
pub struct ValidateUidMessage {
    pub u_id: String,
    pub device
}

#[derive(Serialize, Deserialize)]
pub struct RequestResult {
    pub success: bool,
    pub message: String,
    pub at: String,
}

#[post("/uid")]
pub async fn is_alive(pool: Data<DBPool>, _alive_message: Json<ValidAliveMessage>) -> HttpResponse {
    let id = Uuid::new_v4();
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    use crate::schema::alive_messages;
    let new_alive_message = NewAliveMessage {
        id: id.clone(),
        u_id: id.clone(),
        mtype: "isalive".to_string(),
    };

    let result = diesel::insert_into(alive_messages::table)
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
