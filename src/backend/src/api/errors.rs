/*
 File: errors.rs
 Created Date: 31 Aug 2022
 Author: realbacon
 -----
 Last Modified: 1/09/2022 11:25:57
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
#[derive(Display, Debug)]
pub enum HandlerError {
    DBError,
    InvalidRequest,
    InternalError,
    Unauthorized,
}

impl ResponseError for HandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HandlerError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::DBError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HandlerError::InvalidRequest => actix_web::http::StatusCode::BAD_REQUEST,
            HandlerError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(match self {
            HandlerError::InternalError => "Internal server error",
            HandlerError::DBError => "DB error",
            HandlerError::InvalidRequest => "Invalid request",
            HandlerError::Unauthorized => "Unauthorized",
        })
    }
}
