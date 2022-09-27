/*
 File: endpoint.rs
 Created Date: 27 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 06:00:54
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use super::handlers::create_event as create_event_handler;
use super::structs::{Event, EventResponse};
use crate::dashboard::auth::handlers::verify_session;
use actix_session::Session;
use actix_web::{post, web, web::Data, HttpResponse};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::errors::HandlerError;

#[post("/create")]
pub async fn create_event(
    pool: Data<Pool>,
    session: Session,
    event: web::Json<Event>,
) -> Result<HttpResponse, HandlerError> {
    let id: String;
    match verify_session(session) {
        Err(_) => return Err(HandlerError::Unauthorized),
        Ok(_id) => id = _id,
    };
    let mut event = event;
    event.creator_id = Uuid::parse_str(&id).unwrap();
    let request = create_event_handler(&pool, &event);
    match request.await {
        Ok(res) => match res {
            Ok(id) => Ok(HttpResponse::Ok().json(EventResponse {
                status: "success".to_string(),
                msg: id,
            })),
            Err(err) => {
                return Ok(HttpResponse::Ok().json(EventResponse {
                    status: "error".to_string(),
                    msg: err.to_string(),
                }))
            }
        },
        Err(_) => {
            return Err(HandlerError::DBError);
        }
    }
}
