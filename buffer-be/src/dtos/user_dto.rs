use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct JWTResponse {
    pub jwt: String,
}
