use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum DatabaseError {
    #[display(fmt = "Error aquiring database lock")]
    PoolLockError,
}

impl error::ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DatabaseError::PoolLockError => StatusCode::INTERNAL_SERVER_ERROR,
            // _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
