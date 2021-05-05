use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    common::controllers as c, middleware::auth_validator, user::controllers as u,
    video::controllers as v,
};

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/a/video")
            .route("/", web::post().to(v::upload_video))
            .route("/comment", web::post().to(v::new_comment))
            .route("/rate", web::post().to(v::rate_video))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(
        web::scope("/api/video")
            .route("", web::get().to(v::list_videos))
            .route("/detail", web::get().to(v::video_detail))
            .route("/comments", web::get().to(v::video_comments))
            .route("/rating", web::get().to(v::get_rating)),
    )
    .service(
        web::scope("/api/a/creator")
            .route("/follow", web::get().to(u::is_following))
            .route("/follow", web::post().to(u::follow))
            .route("/unfollow", web::post().to(u::unfollow))
            .route("/profile", web::post().to(u::update_profile))
            .wrap(HttpAuthentication::bearer(auth_validator)),
    )
    .service(web::scope("/api/creator").route("", web::get().to(u::creator_profile)))
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
