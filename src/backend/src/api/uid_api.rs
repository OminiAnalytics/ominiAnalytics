/*
 * @Author: realbacon
 * @Date: 2022-08-26 11:34:18
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 18:34:43
 */
use actix_web::{post, web::Data, web::Json, HttpResponse};

// Import data structure
use super::structs::{UserResult, ValidUidMessage};
use crate::DBPool;

// Db
use super::uid_db::is_uid_valid;

// Chrono
use chrono::Utc;

#[post("/uid")]
pub async fn check_user(pool: Data<DBPool>, user: Json<ValidUidMessage>) -> HttpResponse {
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    let device = serde_json::to_string(&user.device).unwrap();
    let device: serde_json::Value = serde_json::from_str(&device[..]).unwrap();
    let user_validity = is_uid_valid(&conn, user.uid, device);
    if user_validity {
        return HttpResponse::Ok().json(UserResult {
            uid: user.uid.to_string(),
            at: Utc::now().to_rfc3339(),
        });
    }
    HttpResponse::Ok().json("test")
}
