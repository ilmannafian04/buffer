use actix_web::web;

use crate::controllers as c;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/ping", web::get().to(c::ping)));
}
