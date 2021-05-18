use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::user::models::User;

use super::models::{Comment, Video};

#[derive(Deserialize)]
pub struct NewVideoDto {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NewCommentDto {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub content: String,
}

#[non_exhaustive]
pub enum VideoListType {
    New,
    Unknown,
}

impl<'de> Deserialize<'de> for VideoListType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "new" => VideoListType::New,
            _ => VideoListType::Unknown,
        })
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Validate)]
pub struct VideoListDto {
    #[serde(default)]
    #[validate(range(min = 0))]
    pub skip: i64,
    #[serde(rename = "listType")]
    pub list_type: Option<VideoListType>,
}

#[derive(Serialize)]
pub struct VideoListResponseDto {
    pub id: String,
    pub title: String,
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    pub uploader: String,
    #[serde(rename = "uploaderId")]
    pub uploader_id: String,
}

impl From<(Video, Option<User>)> for VideoListResponseDto {
    fn from(tuple: (Video, Option<User>)) -> Self {
        let (v, u) = tuple;
        let u = u.unwrap();
        Self {
            id: v.id,
            title: v.title,
            thumbnail_path: v.thumbnail_path,
            created_at: v.created_at,
            uploader: u.display_name,
            uploader_id: u.id,
        }
    }
}

#[derive(Serialize)]
pub struct VideoDetailDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    #[serde(rename = "thumbnailPath")]
    pub thumbnaail_path: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    pub uploader: String,
    #[serde(rename = "uploaderUsername")]
    pub uploader_username: String,
    #[serde(rename = "uploaderId")]
    pub uploader_id: String,
}

impl From<(Video, User)> for VideoDetailDto {
    fn from(t: (Video, User)) -> Self {
        let (v, u) = t;
        Self {
            id: v.id,
            title: v.title,
            description: v.description,
            path: v.video_path,
            thumbnaail_path: v.thumbnail_path,
            created_at: v.created_at,
            uploader: u.display_name,
            uploader_username: u.username,
            uploader_id: u.id,
        }
    }
}

#[derive(Serialize)]
pub struct CommentDto {
    pub id: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl From<(Comment, Option<User>)> for CommentDto {
    fn from(tuple: (Comment, Option<User>)) -> Self {
        let (c, u) = tuple;
        let u = u.unwrap();
        Self {
            id: c.id,
            content: c.content,
            created_at: c.created_at,
            user_id: u.id,
            user_display_name: u.display_name,
            username: u.username,
        }
    }
}

#[derive(Deserialize)]
pub struct RateVideoRequest {
    pub id: String,
    #[serde(rename = "isDislike")]
    pub is_dislike: bool,
}

#[derive(Serialize)]
pub struct VideoRatingDto {
    pub like: i64,
    pub dislike: i64,
}

#[derive(Serialize)]
pub struct HasRatedDto {
    #[serde(rename = "hasRated")]
    pub has_rated: bool,
    #[serde(rename = "isDislike")]
    pub is_dislike: bool,
}

#[derive(Deserialize)]
pub struct SearchVideoDto {
    pub term: String,
}
