use actix_web::{
    dev::ServiceRequest,
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, Error, HttpMessage,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use jsonwebtoken::{DecodingKey, Validation};

use crate::{config::Config, models::User, service::Claims};

pub async fn auth_validator(
    request: ServiceRequest,
    token: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = request.app_data::<web::Data<Config>>().unwrap();
    let decode_attempt = jsonwebtoken::decode::<Claims>(
        &token.token(),
        &DecodingKey::from_secret(&config.secret_key.as_bytes()),
        &Validation::default(),
    );
    let claim = match decode_attempt {
        Ok(claim) => claim,
        Err(e) => return Err(ErrorUnauthorized(e)),
    };
    let pool = request
        .app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>()
        .unwrap();
    match pool.get() {
        Ok(conn) => {
            let query = web::block(move || {
                User::find_by_id(&conn, claim.claims.sub.parse::<i32>().unwrap())
            })
            .await;
            match query {
                Ok(user) => {
                    request.extensions_mut().insert(user);
                    Ok(request)
                }
                Err(e) => return Err(ErrorNotFound(e)),
            }
        }
        Err(e) => return Err(ErrorInternalServerError(e)),
    }
}
