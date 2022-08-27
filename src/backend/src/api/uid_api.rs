/*
 * @Author: realbacon
 * @Date: 2022-08-27 21:07:20
 * @Last Modified by:   realbacon
 * @Last Modified time: 2022-08-27 21:07:20
 */

use actix_web::{post, web::Data, web::Json, HttpResponse};

// Import data structure
use super::structs::{UserResult, ValidUidMessage};
use crate::DBPool;
use uuid::Uuid;

// Db
use super::uid_db::{insert_user, is_uid_valid};

// Chrono
use chrono::Utc;

#[post("/uid")]
pub async fn check_user_or_create(pool: Data<DBPool>, user: Json<ValidUidMessage>) -> HttpResponse {
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let device = serde_json::to_string(&user.device).unwrap();
    let device: serde_json::Value = serde_json::from_str(&device[..]).unwrap();
    let user_validity = is_uid_valid(&conn, user.uid, &device);
    let uid: Uuid;
    let at = Utc::now().timestamp();
    if user_validity {
        uid = user.uid;
    } else {
        let new_user = insert_user(&conn, device);
        match new_user {
            Ok(user) => uid = user.0,
            Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
        }
    }
    HttpResponse::Ok().json(UserResult {
        uid: uid.to_string(),
        at,
    })
}
