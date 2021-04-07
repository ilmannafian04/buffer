use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use validator::Validate;

use crate::config::Config;
use crate::dtos::{user_dto, SimpleError};
use crate::error::DatabaseError;
use crate::models::{NewUser, UniqueViolationKind, User};

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

pub async fn singin(
    pool: web::Data<DbPool>,
    credential: web::Json<user_dto::SignInDTO>,
    config: web::Data<Config>,
) -> HttpResponse {
    let user = match pool.get() {
        Ok(conn) => {
            let username = credential.username.clone();
            let query = web::block(move || User::find_by_username(&conn, &username)).await;
            match query {
                Ok(user) => user,
                Err(_) => return HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    match user.authennticate(&credential.password, &config.secret_key) {
        Ok(jwt) => HttpResponse::Ok().json(user_dto::JWTResponse { jwt }),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
