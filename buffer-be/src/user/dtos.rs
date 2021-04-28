use serde::{Deserialize, Serialize};

use super::models::User;

#[derive(Deserialize)]
pub struct SignInDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct JWTResponse {
    pub jwt: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatorLookUpDTO {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Serialize)]
pub struct CreatorProfileResponseDTO {
    pub creator: User,
    #[serde(rename = "followerCount")]
    pub follower_count: i64,
}

#[derive(Serialize)]
pub struct IsFollowingResponseDTO {
    #[serde(rename = "isFollowing")]
    pub is_following: bool,
}
