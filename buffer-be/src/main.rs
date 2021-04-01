extern crate openssl;
#[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use log::info;

mod config;
mod controllers;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    info!("Reading config");
    let config = config::Config::new();

    info!("Connecting to database");
    let db_manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let db_pool = Pool::builder()
        .build(db_manager)
        .expect("Cannot connect to database");
    info!("Server connected to database");

    info!("Binding server to {}:{}", &config.host, &config.port);
    HttpServer::new(move || {
        App::new()
            .configure(routes::configuration)
            .data(db_pool.clone())
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", &config.host, &config.port))?
    .run()
    .await
}
