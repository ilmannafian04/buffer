use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{middleware::auth_validator, user::controllers as u, video::controllers as v};

async fn ping() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body("pong")
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth/video")
            .route("/", web::post().to(v::upload_video))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api/auth")
            .route("/ping", web::get().to(ping))
            .route("/account", web::get().to(u::user_info))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api")
            .route("/ping", web::get().to(ping))
            .route("/signup", web::post().to(u::signup))
            .route("/signin", web::post().to(u::singin)),
    );
}
