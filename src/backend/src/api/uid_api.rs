/*
 File: uid_api.rs
 Created Date: 28 Aug 2022
 Author: realbacon
 -----
 Last Modified: 29/08/2022 02:43:14
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/

use actix_web::{post, web::Data, web::Json, HttpRequest, HttpResponse};

// Import data structure
use super::structs::{UserResult, ValidUidMessage};
use crate::DBPool;
use uuid::Uuid;

// Db
use super::uid_db::{insert_user, is_uid_valid};

// Chrono
use chrono::Utc;

// Config
use crate::config::valid_origin;

#[post("/uid")]
pub async fn check_user_or_create(
    req: HttpRequest,
    pool: Data<DBPool>,
    user: Json<ValidUidMessage>,
) -> HttpResponse {
    // Check origin validity
    let is_auth = valid_origin(req);
    println!("{:?}", is_auth);
    if is_auth == false {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    // Initiate connection
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    // Serialize device
    let device = serde_json::to_string(&user.device).unwrap();
    let device: serde_json::Value = serde_json::from_str(&device[..]).unwrap();
    // Serialize country
    let country = &user.country;
    // try to parse uid
    let try_des_uid = Uuid::parse_str(&user.uid[..]);
    let mut uid: Uuid;
    if try_des_uid.is_ok() {
        uid = try_des_uid.unwrap();
        let user_validity = is_uid_valid(&conn, uid, &device);
        if user_validity {
            // We keep the same uid because it is a valid one
        } else {
            let new_user = insert_user(&conn, device, country);
            match new_user {
                Ok(user) => uid = user.0,
                Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
            }
        }
    } else {
        let new_user = insert_user(&conn, device, country);
        match new_user {
            Ok(user) => uid = user.0,
            Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
        }
    }
    let at = Utc::now().timestamp();
    HttpResponse::Ok().json(UserResult {
        uid: uid.to_string(),
        at,
    })
}
