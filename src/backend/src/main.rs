/*
 File: main.rs
 Created Date: 25 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:57:55
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/



mod api;
use actix_cors::Cors;
use actix_web::{
    error, guard, http::header, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use api::alive::endpoint::is_alive;
use api::main::endpoint::main_procedure_handler;
mod config;
mod models;
mod schema;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

extern crate tokio;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[tokio::main(flavor = "multi_thread")]
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

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(700)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(header::CONTENT_TYPE)
            .allow_any_origin();
        App::new()
            .service(
                web::scope("/api")
                    .app_data(json_config)
                    .guard(guard::Header("content-type", "application/json"))
                    .service(is_alive)
                    .service(main_procedure_handler),
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
    .await?;
    Ok(server)
}
