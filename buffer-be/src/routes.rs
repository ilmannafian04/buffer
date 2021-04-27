use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::controllers::{self as c, user as u, video as v};
use crate::middleware::auth_validator;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth/video")
            .route("/", web::post().to(v::upload_video))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api/auth")
            .route("/ping", web::get().to(c::ping))
            .route("/account", web::get().to(u::user_info))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .route("/signup", web::post().to(u::signup))
            .route("/signin", web::post().to(u::singin)),
    );
}
