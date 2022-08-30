/*
 File: endpoint.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:55:26
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

// Actix web & co
use actix_web::{post, web::Data, web::Json, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

// Diesel stuff
extern crate diesel;
use crate::DBPool;

// import data structure
use crate::api::structs::ValidAliveMessage;
use crate::models::NewAliveMessage;

// DB
use super::db::{get_last_signal, insert_alive_message};

// Constants
use crate::api::constants::{KILL_SESSION_DELAY, SIGNAL_TYPE_ALIVE, SIGNAL_TYPE_SESSION};

#[post("/alive")]
pub async fn is_alive(pool: Data<DBPool>, alive_message: Json<ValidAliveMessage>) -> HttpResponse {
    // Generate new signal id
    let id = Uuid::new_v4();
    // Parse user id
    // Return error if "uid" is invalid
    let _u_id = Uuid::parse_str(&alive_message.u_id[..]);
    let u_id: Uuid;
    match _u_id {
        Ok(o) => u_id = o,
        Err(_) => {
            return HttpResponse::BadRequest().body("");
        }
    }
    // Create connection
    let conn = pool.get().expect("CONNECTION_POOL_ERROR");
    // Get last signal message
    // If no signal message or last signal was more than KILL_SESSION_DELAY seconds ago ->
    // new session
    let last_signal = get_last_signal(&conn, u_id);
    let signal_type: String;
    let signal_id;
    let s_duration;
    if last_signal.is_err() {
        signal_type = SIGNAL_TYPE_SESSION.to_string();
        signal_id = Uuid::new_v4();
        s_duration = 0;
    } else {
        let last_signal = last_signal.unwrap();
        let last_signal_date = last_signal.0;
        let now = Utc::now().timestamp();
        let diff = now - last_signal_date;
        if diff > KILL_SESSION_DELAY as i64 {
            signal_type = SIGNAL_TYPE_SESSION.to_string();
            signal_id = Uuid::new_v4();
            s_duration = 0;
        } else {
            signal_type = SIGNAL_TYPE_ALIVE.to_string();
            signal_id = last_signal.1;
            s_duration = last_signal.2 + diff as i32;
        }
    }
    // Create new signal message
    let new_alive_message = NewAliveMessage {
        id,
        u_id,
        s_id: signal_id,
        mtype: signal_type,
        s_duration,
    };

    let result = insert_alive_message(&conn, &new_alive_message);
    match result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}
