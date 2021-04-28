use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use validator::Validate;

use crate::{
    common::{dtos::SimpleError, errors::DatabaseError, types::DbPool},
    config::Config,
};

use super::{
    dtos::{FollowDTO, JWTResponse, SignInDTO},
    models::{Creator, NewFollower, NewUser, UniqueViolationKind, User},
};

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
    let user = match pool.get() {
        Ok(conn) => {
            new_user.generate_id();
            let query = web::block(move || new_user.into_inner().insert(&conn)).await;
            match query {
                Ok(user) => user,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    match pool.get() {
        Ok(conn) => {
            let query = web::block(move || Creator { user_id: user.id }.insert(&conn)).await;
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
    credential: web::Json<SignInDTO>,
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
        Ok(jwt) => HttpResponse::Ok().json(JWTResponse { jwt }),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn user_info(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(request.head().extensions().get::<User>().unwrap())
}

pub async fn follow(
    request: HttpRequest,
    payload: web::Json<FollowDTO>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let extension = request.head().extensions();
    let user = extension.get::<User>().unwrap();
    let creator = match pool.get() {
        Ok(conn) => {
            let query =
                web::block(move || User::find_by_id(&conn, payload.creator_id.clone())).await;
            match query {
                Ok(creator) => creator,
                Err(_) => return HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    let follower = NewFollower {
        creator_id: creator.id,
        viewer_id: user.id.clone(),
    };
    match pool.get() {
        Ok(conn) => match web::block(move || follower.insert(&conn)).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
}
