/*
 File: endpoint.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 24/09/2022 11:36:55
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

// Actix web & co
use crate::errors::HandlerError;
use actix_web::{post, web::Data, web::Json, HttpResponse};
use chrono::Utc;
use deadpool_postgres::Pool;
use uuid::Uuid;

// import data structure
use crate::api::event::structs::ValidAliveMessage;
use crate::models::NewAliveMessage;

// DB
use super::handlers::{get_last_signal, insert_alive_message};
// Constants
use crate::api::custom::constants::{KILL_SESSION_DELAY, SIGNAL_TYPE_ALIVE, SIGNAL_TYPE_SESSION};

/// Generate a new alive signal
/// # Arguments
/// * `pool` - The database pool
/// * `alive_message` - ValidAliveMessage struct
#[post("/alive")]
pub async fn is_alive(
    pool: Data<Pool>,
    alive_message: Json<ValidAliveMessage>,
) -> Result<HttpResponse, HandlerError> {
    // Generate new signal id
    let signal_id = Uuid::new_v4();
    // Parse user id
    // Return error if "uid" is invalid
    let _u_id = Uuid::parse_str(&alive_message.uid[..]);
    let uid: Uuid;
    match _u_id {
        Ok(o) => uid = o,
        Err(_) => {
            return Err(HandlerError::InvalidRequest);
        }
    }
    // Get last signal message
    // If no signal message or last signal was more than KILL_SESSION_DELAY seconds ago ->
    // new session

    // Get last signal
    let last_signal = get_last_signal(&pool, uid);
    // Session id
    let session_id: Uuid;
    // Signal type (alive or session)
    let signal_type: String;
    // Duration since last signal
    let s_duration;
    match last_signal.await {
        Ok(last_signal) => {
            match last_signal {
                Some(last_signal) => {
                    let last_signal_date = last_signal.0;
                    let now = Utc::now().timestamp();
                    let diff = now - last_signal_date;
                    if diff > KILL_SESSION_DELAY as i64 {
                        signal_type = SIGNAL_TYPE_SESSION.to_string();
                        s_duration = 0;
                        session_id = Uuid::new_v4();
                    } else {
                        signal_type = SIGNAL_TYPE_ALIVE.to_string();
                        session_id = last_signal.1;
                        s_duration = last_signal.2 + diff as i32;
                    }
                }
                None => {
                    // No signal found
                    // New session
                    session_id = Uuid::new_v4();
                    signal_type = SIGNAL_TYPE_SESSION.to_string();
                    s_duration = 0;
                }
            }
        }
        Err(_) => {
            signal_type = SIGNAL_TYPE_SESSION.to_string();
            s_duration = 0;
            session_id = Uuid::new_v4();
        }
    };
    // Create new signal message
    let new_alive_message = NewAliveMessage {
        id: signal_id,
        u_id: uid,
        s_id: session_id,
        mtype: signal_type,
        s_duration,
    };

    let result = insert_alive_message(&pool, &new_alive_message).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("")),
        Err(_) => Err(HandlerError::DBError),
    }
}
