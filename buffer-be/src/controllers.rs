use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use validator::Validate;

use crate::models::{NewUser, UniqueViolationKind, User};
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
    match pool.get() {
        Ok(conn) => {
            if let Err(kind) =
                User::check_unique_integrity(&conn, &new_user.username, &new_user.email)
            {
                match kind {
                    UniqueViolationKind::Email => {
                        return HttpResponse::BadRequest().json(SimpleError { error: "email" })
                    }
                    UniqueViolationKind::Username => {
                        return HttpResponse::BadRequest().json(SimpleError { error: "username" })
                    }
                }
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
    if let Err(_) = new_user.hash_password() {
        return HttpResponse::InternalServerError().finish();
    }
    match pool.get() {
        Ok(conn) => {
            let query = web::block(move || new_user.into_inner().insert(&conn)).await;
            match query {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => return HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
}
