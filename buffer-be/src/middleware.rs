use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn auth_validator(
    request: ServiceRequest,
    _credential: BearerAuth,
) -> Result<ServiceRequest, Error> {
    Ok(request)
}
