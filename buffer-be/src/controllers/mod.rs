use actix_web::{HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub(crate) mod user;
pub(crate) mod video;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
