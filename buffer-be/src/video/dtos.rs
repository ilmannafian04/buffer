use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::user::models::User;

use super::models::Video;

#[derive(Deserialize)]
pub struct NewVideoDTO {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NewCommentDTO {
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
pub struct VideoListDTO {
    #[serde(default)]
    #[validate(range(min = 0))]
    pub skip: i64,
    #[serde(rename = "listType")]
    pub list_type: Option<VideoListType>,
}

#[derive(Serialize)]
pub struct VideoListResponseDTO {
    pub id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    pub uploader: String,
    #[serde(rename = "uploaderId")]
    pub uploader_id: String,
}

impl From<(Video, Option<User>)> for VideoListResponseDTO {
    fn from(tuple: (Video, Option<User>)) -> Self {
        let user = tuple.1.unwrap();
        Self {
            id: tuple.0.id,
            title: tuple.0.title,
            created_at: tuple.0.created_at,
            uploader: user.display_name,
            uploader_id: user.id,
        }
    }
}

#[derive(Serialize)]
pub struct VideoDetailDTO {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    pub uploader: String,
    #[serde(rename = "uploaderId")]
    pub uploader_id: String,
}

impl From<(Video, User)> for VideoDetailDTO {
    fn from(t: (Video, User)) -> Self {
        Self {
            id: t.0.id,
            title: t.0.title,
            description: t.0.description,
            path: t.0.video_path,
            created_at: t.0.created_at,
            uploader: t.1.display_name,
            uploader_id: t.1.id,
        }
    }
}
