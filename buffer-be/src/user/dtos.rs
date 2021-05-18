use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::User;

#[derive(Deserialize)]
pub struct SignInDto {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct JwtResponse {
    pub jwt: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatorLookUpDto {
    #[serde(rename = "username")]
    pub username: String,
}

#[derive(Serialize)]
pub struct CreatorProfileResponseDto {
    pub creator: User,
    #[serde(rename = "followerCount")]
    pub follower_count: i64,
}

#[derive(Serialize)]
pub struct IsFollowingResponseDto {
    #[serde(rename = "isFollowing")]
    pub is_following: bool,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProfileDto {
    #[validate(email)]
    pub email: String,
    #[serde(rename = "displayName")]
    #[validate(length(min = 1))]
    pub display_name: String,
}
