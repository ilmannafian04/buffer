use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    common::controllers as c, middleware::auth_validator, user::controllers as u,
    video::controllers as v,
};

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
            .route("/follow", web::post().to(u::follow))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .route("/signup", web::post().to(u::signup))
            .route("/signin", web::post().to(u::singin)),
    );
}
