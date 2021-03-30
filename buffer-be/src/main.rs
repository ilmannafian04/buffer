use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;

mod config;
mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    info!("Reading config");
    let config = config::Config::new();
    info!("Binding server to {}:{}", &config.host, &config.port);

    HttpServer::new(|| App::new().configure(routes::configuration))
        .bind(format!("{}:{}", &config.host, &config.port))?
        .run()
        .await
}
