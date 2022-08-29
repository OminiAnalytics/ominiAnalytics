/*
 File: main.rs
 Created Date: 28 Aug 2022
 Author: realbacon
 -----
 Last Modified: 29/08/2022 03:00:53
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/

mod api;
use actix_cors::Cors;
use actix_web::{
    error, guard, http::header, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use api::alive_api::is_alive;
use api::uid_api::check_user_or_create;
mod config;
mod models;
mod schema;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(header::CONTENT_TYPE)
            .allow_any_origin();
        App::new()
            .service(
                web::scope("/api")
                    .guard(guard::Header("content-type", "application/json"))
                    .service(is_alive)
                    .service(check_user_or_create),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::InternalServerError().body("Something went wrong"),
                )
                .into()
            }))
            .wrap(logger)
            .wrap(cors)
    })
    .bind(env::var("SERVER_HOST").expect("SERVER_HOST not present in env"))?
    .run()
    .await
}
