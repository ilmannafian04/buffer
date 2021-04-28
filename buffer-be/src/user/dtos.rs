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

#[derive(Deserialize)]
pub struct FollowDTO {
    #[serde(rename = "creatorId")]
    pub creator_id: String,
}
