use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{controllers as c, middleware::auth_validator};

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/ping", web::get().to(c::ping))
            .route("/account", web::get().to(c::user_info))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api")
            .route("/ping", web::get().to(c::ping))
            .route("/signup", web::post().to(c::signup))
            .route("/signin", web::post().to(c::singin)),
    );
}
