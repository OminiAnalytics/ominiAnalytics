/*
 File: endpoint.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 24/09/2022 11:36:39
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use std::net::IpAddr;

use actix_web::web;
use actix_web::{post, web::Data, web::Json, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
// Import data structure
use super::crypto;
use super::structs::Device;
use super::structs::NewInst;
use crate::api::security;
use crate::errors::HandlerError;
use serde::Serialize;

// Db
use super::handlers::{insert_user, user_exists};

#[post("/main")]
pub async fn main_procedure_handler(
    req: HttpRequest,
    pool: web::Data<Pool>,
    payload: Json<NewInst>,
) -> Result<HttpResponse, HandlerError> {
    let valid_resp_or_not = main_procedure(req, pool, payload).await;
    match valid_resp_or_not {
        Ok(resp) => return Ok(resp),
        Err(err) => return Err(err),
    }
}

#[derive(Serialize)]
struct RespUid {
    uid: String,
}

async fn main_procedure(
    req: HttpRequest,
    pool: Data<Pool>,
    payload: Json<NewInst>,
) -> Result<HttpResponse, HandlerError> {
    let ip: IpAddr;
    let response: RespUid;
    // First filter the request.
    // Then verify that the request is coming from a trusted source
    // and that maximum request per minute is not exceeded.
    // If not it gets the ip.
    if let Some(_ip) = security::request_filter(&req, &pool).await {
        ip = _ip;
    } else {
        return Err(HandlerError::Unauthorized);
    };
    // Prevent DDOS attack by blocking ip.
    // prevent_flood also prevent data corruption
    // by preventing too many request per minute.
    let sec = security::prevent_flood(ip, &pool).await;
    if sec.is_err() {
        return Err(HandlerError::DBError);
    } else {
        if !sec.unwrap() {
            return Err(HandlerError::ToomManyRequest);
        }
    }
    // Stores the user agent after its been verified.
    let user_agent = get_ua(&req).unwrap();
    // Verify signature.
    // This is a heavy operation
    // so it uses tokio::task::spawn_blocking.
    let (sign_is_valid, json_obj) = tokio::task::spawn_blocking(move || {
        // Get the encoded json.
        // Decode from base64.
        // Then verify signature.
        let encoded_json = &payload.bp.dt;
        let hash = &payload.bp.hash;
        // Decode from base64.
        let decoded_json = crypto::decodeb64(encoded_json);
        crypto::verify_signature(hash, decoded_json, user_agent).unwrap()
    })
    .await
    .unwrap();
    // If signature isn't valid it returns error.
    if !sign_is_valid {
        return Err(HandlerError::Unauthorized);
    }
    // Else json_obj is serialized into a super::device::Device struct
    let user_device = serde_json::from_str(&json_obj);
    // If the format is not respected => error.
    let user_device: Device = match user_device {
        Ok(device) => device,
        Err(_) => return Err(HandlerError::InvalidRequest),
    };
    // Check if user exists with the uid given by the user
    // If it does not exist, we create a new user
    // Else we return the uid
    match user_exists(&user_device, ip, &pool).await {
        Some(id) => {
            response = RespUid {
                uid: id.to_string(),
            };
        }
        None => {
            // We didn't find the user in the database
            // So we create a new user
            let new_user = tokio::task::spawn_blocking(move || {
                let u = insert_user(pool, user_device, ip.to_string());
                return u;
            })
            .await
            .unwrap()
            .await;
            match new_user {
                Ok(id) => match id {
                    Some(id) => {
                        response = RespUid {
                            uid: id.to_string(),
                        };
                    }
                    None => return Err(HandlerError::DBError),
                },
                Err(_) => return Err(HandlerError::DBError),
            };
        }
    };
    Ok(HttpResponse::Ok().json(response))
}

fn get_ua(req: &HttpRequest) -> Option<String> {
    let header = req.headers().get("user-agent")?;
    let header = header.to_str().ok()?;
    Some(header.to_string())
}
