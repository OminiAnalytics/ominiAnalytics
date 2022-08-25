mod api;
use actix_web::{error, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer};
use api::task::{get_task, post_task};
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
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::InternalServerError().body("Something went wrong"),
                )
                .into()
            }))
            .wrap(logger)
            .service(get_task)
            .service(post_task)
    })
    .bind(env::var("SERVER_HOST").expect("SERVER_HOST"))?
    .run()
    .await
}
