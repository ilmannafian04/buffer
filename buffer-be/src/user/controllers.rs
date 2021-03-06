use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse, ResponseError};
use diesel::{result::DatabaseErrorKind, result::Error};
use validator::Validate;

use crate::{
    common::{dtos::SimpleError, errors::DatabaseError, types::DbPool},
    config::Config,
};

use super::{
    dtos::{
        CreatorLookUpDto, CreatorProfileResponseDto, IsFollowingResponseDto, JwtResponse,
        SignInDto, UpdateProfileDto,
    },
    models::{Creator, Follower, NewFollower, NewUser, UniqueViolationKind, User},
};

pub async fn signup(mut new_user: web::Json<NewUser>, pool: web::Data<DbPool>) -> HttpResponse {
    if new_user.validate().is_err() {
        return HttpResponse::BadRequest().json(SimpleError {
            error: "Field validation failed",
        });
    }
    match pool.get() {
        Ok(conn) => {
            if let Err(kind) =
                User::check_unique_integrity(&conn, &new_user.username, &new_user.email)
            {
                return match kind {
                    UniqueViolationKind::Email => {
                        HttpResponse::BadRequest().json(SimpleError { error: "email" })
                    }
                    UniqueViolationKind::Username => {
                        HttpResponse::BadRequest().json(SimpleError { error: "username" })
                    }
                };
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
    if new_user.hash_password().is_err() {
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

pub async fn sign_in(
    pool: web::Data<DbPool>,
    credential: web::Json<SignInDto>,
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
    match user.authenticate(&credential.password, &config.secret_key) {
        Ok(jwt) => HttpResponse::Ok().json(JwtResponse { jwt }),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

pub async fn user_info(request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(request.head().extensions().get::<User>().unwrap())
}

pub async fn follow(
    request: HttpRequest,
    payload: web::Json<CreatorLookUpDto>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let extension = request.head().extensions();
    let user = extension.get::<User>().unwrap();
    let creator = match pool.get() {
        Ok(conn) => {
            let query =
                web::block(move || User::find_by_username(&conn, &payload.username.clone())).await;
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
            // following already followed person won't do anything
            Ok(_)
            | Err(BlockingError::Error(Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            ))) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
}

pub async fn unfollow(
    request: HttpRequest,
    payload: web::Json<CreatorLookUpDto>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let extension = request.head().extensions();
    let user = extension.get::<User>().unwrap();
    let creator = match pool.get() {
        Ok(conn) => {
            let query =
                web::block(move || User::find_by_username(&conn, &payload.username.clone())).await;
            match query {
                Ok(creator) => creator,
                Err(_) => return HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    let user_id_closure = user.id.clone();
    match pool.get() {
        Ok(conn) => {
            match web::block(move || Follower::delete(&conn, &creator.id, &user_id_closure)).await {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    }
}

pub async fn creator_profile(
    pool: web::Data<DbPool>,
    query: web::Query<CreatorLookUpDto>,
) -> HttpResponse {
    let creator = match pool.get() {
        Ok(conn) => {
            match web::block(move || User::find_by_username(&conn, &query.username)).await {
                Ok(creator) => creator,
                Err(_) => return HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    let c_id_closure = creator.id.clone();
    let follower_count = match pool.get() {
        Ok(conn) => {
            match web::block(move || Follower::get_follower_count(&conn, &c_id_closure)).await {
                Ok(count) => count,
                Err(_) => return HttpResponse::InternalServerError().body("count"),
            }
        }
        Err(_) => return DatabaseError::PoolLockError.error_response(),
    };
    HttpResponse::Ok().json(CreatorProfileResponseDto {
        creator,
        follower_count,
    })
}

pub async fn is_following(
    request: HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<CreatorLookUpDto>,
) -> HttpResponse {
    let extension = request.head().extensions();
    let user = extension.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let creator = match web::block(move || User::find_by_username(&conn, &query.username)).await {
        Ok(c) => c,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let u_id_closure = user.id.clone();
    match web::block(move || Follower::get_by_creator_and_viewer(&conn, &creator.id, &u_id_closure))
        .await
    {
        Ok(_) => HttpResponse::Ok().json(IsFollowingResponseDto { is_following: true }),
        Err(BlockingError::Error(Error::NotFound)) => {
            HttpResponse::Ok().json(IsFollowingResponseDto {
                is_following: false,
            })
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_profile(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    payload: web::Json<UpdateProfileDto>,
) -> HttpResponse {
    if payload.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    };
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let id_closure = user.id.clone();
    match web::block(move || {
        User::update(&conn, &id_closure, &payload.email, &payload.display_name)
    })
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(BlockingError::Error(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => {
            HttpResponse::BadRequest().finish()
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
