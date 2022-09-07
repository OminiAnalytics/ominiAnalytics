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
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub mod db {
    use deadpool_postgres::{Client, ManagerConfig, Pool, RecyclingMethod};
    // Helper function to read environment variable

    pub fn get_db_config() -> deadpool_postgres::Config {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let mut config = deadpool_postgres::Config::new();
        config.user = Some(env::var("DB_USER").expect("DB_USER not set"));
        config.password = Some(env::var("DB_PASSWORD").expect("DB_PASSWORD set"));
        config.dbname = Some(env::var("DB_NAME").expect("DB_NAME not set"));
        config.host = Some(env::var("DB_HOST").expect("DB_USER net set"));

        config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        config
    }

    pub async fn get_connection(pool: &Pool) -> Result<Client, String> {
        pool.get().await.map_err(|err| err.to_string())
    }
}

extern crate tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();

    let pool = db::get_db_config().create_pool(None, NoTls).unwrap();

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
