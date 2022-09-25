/*
 File: endpoint.rs
 Created Date: 13 Sep 2022
 Author: realbacon
 -----
 Last Modified: 24/09/2022 11:36:48
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use crate::errors::HandlerError;
use actix_web::{post, web::Data, web::Path, HttpResponse};
use deadpool_postgres::Pool;

#[post("/c/{id}")]
pub async fn log_event(id: Path<String>, pool: Data<Pool>) -> Result<HttpResponse, HandlerError> {
    Ok(HttpResponse::Ok().body(format!("Hello {}", id)))
}
