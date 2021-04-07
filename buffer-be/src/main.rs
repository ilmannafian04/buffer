extern crate openssl;
#[macro_use]
extern crate diesel;

use actix_redis::RedisSession;
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use log::info;
use validator::Validate;

mod config;
mod controllers;
mod dtos;
mod error;
mod middleware;
mod models;
mod routes;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    info!("Reading config");
    let config = config::Config::new();
    if let Err(e) = config.validate() {
        panic!("{}", e);
    };

    info!("Connecting to database");
    let db_manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let db_pool = Pool::builder()
        .build(db_manager)
        .expect("Cannot connect to database");
    info!("Server connected to database");

    info!("Binding server to {}:{}", &config.host, &config.port);
    let config_clone = config.clone();
    HttpServer::new(move || {
        App::new()
            .configure(routes::configuration)
            .data(db_pool.clone())
            .data(config_clone.clone())
            .wrap(RedisSession::new(
                &config_clone.redis_url,
                &config_clone.secret_key.as_bytes(),
            ))
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", &config.host, &config.port))?
    .run()
    .await
}
