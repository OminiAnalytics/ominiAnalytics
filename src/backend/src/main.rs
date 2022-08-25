mod api;
mod db;
use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};
use api::task::get_task;
use db::postgres;
use dotenv::dotenv;
use std::env;
mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct EnvConfig {
        pub host: String,
        pub port: u16,
        pub pg: deadpool_postgres::Config,
    }
}
use crate::config::EnvConfig;
use ::config::Config;
use tokio_postgres::NoTls;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();
    let config: EnvConfig = config_.try_deserialize().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    env_logger::init();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .service(get_task)
    })
    .bind((config.host.clone(), config.port.clone()))?
    .run()
    .await
}
