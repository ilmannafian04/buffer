use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::debug;
use validator::Validate;

use crate::models::NewUser;
use crate::{dto::SimpleError, error::DatabaseError};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

pub async fn signup(mut new_user: web::Json<NewUser>, pool: web::Data<DbPool>) -> HttpResponse {
    if let Err(_) = new_user.validate() {
        return HttpResponse::BadRequest().json(SimpleError {
            error: "Field validation failed",
        });
    }
    if let Err(_) = new_user.hash_password() {
        return HttpResponse::InternalServerError().finish();
    }
    let user = match pool.get() {
        Ok(conn) => {
            let query = web::block(move || new_user.into_inner().insert(&conn)).await;
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    HttpResponse::Ok().finish()
}
