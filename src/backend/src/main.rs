/*
 File: main.rs
 Created Date: 25 Aug 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 03:18:15
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod api;
mod dashboard;
use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time, Key};
use actix_web::{
    error, guard, http::header, middleware::Logger, web, App, HttpResponse, HttpServer,
};
use api::event::alive::endpoint::is_alive;
use api::event::custom::endpoint::log_event;
use api::main::endpoint::main_procedure_handler;
pub mod errors;
use dashboard::{
    auth::login::{login, verify_session},
    metrics::users::endpoint::connected,
    metrics::users::endpoint::total_users,
};
use diesel::{pg::PgConnection, sql_query, sql_types::VarChar, Connection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;
use uuid::Uuid;
mod models;
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
    dotenv().ok();
    let config = db::get_db_config();
    // Init sessions
    let secret_key = Key::generate();
    // Run migrations and create default user
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url);
    match conn {
        Ok(conn) => {
            let mut conn = conn;
            println!("Running migrations...");
            conn.run_pending_migrations(MIGRATIONS).unwrap();
            let create_default_user: Result<usize, diesel::result::Error> = sql_query(
                "INSERT INTO omini_managers (id, name, password, status) VALUES ($1,'Administrator', encode(digest($2, 'sha256'),'hex'), 'admin') ON CONFLICT DO NOTHING;",
            )
            .bind::<diesel::sql_types::Uuid, _>(Uuid::new_v4())
            .bind::<VarChar, _>(config.password.as_ref().unwrap())
            .execute(&mut conn);
            if create_default_user.is_err() {
                println!("Failed to create default user...");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    let pool = config.create_pool(None, NoTls).unwrap();

    env_logger::init();

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(1000)
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
                web::scope("")
                    .app_data(json_config)
                    .guard(guard::Header("content-type", "application/json"))
                    .service(main_procedure_handler)
                    .service(web::scope("/event").service(log_event).service(is_alive))
                    .service(web::scope("/dsh").service(login).service(verify_session)),
            )
            .service(
                web::scope("").service(
                    web::scope("/dsh/metrics")
                        .service(connected)
                        .service(total_users),
                ),
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
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(time::Duration::days(5)),
                    )
                    .cookie_name("sid".to_string())
                    .build(),
            )
    })
    .bind(env::var("SERVER_HOST").expect("SERVER_HOST not present in env"))?
    .run()
    .await?;
    Ok(server)
}
