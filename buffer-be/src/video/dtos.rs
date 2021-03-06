use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::user::models::User;

use super::models::{Comment, Video};
use crate::video::models::Collection;

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
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
}

#[derive(Serialize)]
pub struct VideoDetailDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: String,
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
            thumbnail_path: v.thumbnail_path,
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
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
}

impl From<(Comment, Option<User>)> for CommentDto {
    fn from(tuple: (Comment, Option<User>)) -> Self {
        let (c, u) = tuple;
        let mut u = u.unwrap();
        if c.is_anonymous {
            u.id = "".to_owned();
            u.display_name = "Anonymous".to_owned();
            u.username = "".to_owned();
        }
        Self {
            id: c.id,
            content: c.content,
            created_at: c.created_at,
            user_id: u.id,
            user_display_name: u.display_name,
            username: u.username,
            is_anonymous: c.is_anonymous,
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

#[derive(Deserialize)]
pub struct NewCollectionDto {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct CollectionDetailDto {
    pub name: String,
    pub description: String,
    pub videos: Vec<VideoDetailDto>,
}

impl From<(Collection, Vec<(Video, User)>)> for CollectionDetailDto {
    fn from(tuple: (Collection, Vec<(Video, User)>)) -> Self {
        let (c, vs) = tuple;
        Self {
            name: c.name,
            description: c.description,
            videos: vs.into_iter().map(VideoDetailDto::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct CollectionDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub videos: Vec<Video>,
}

impl From<Collection> for CollectionDto {
    fn from(c: Collection) -> Self {
        Self {
            id: c.id,
            name: c.name,
            description: c.description,
            videos: vec![],
        }
    }
}

impl Default for CollectionDto {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            videos: vec![],
        }
    }
}

#[derive(Serialize)]
pub struct VideoUserDto {
    pub user: User,
    pub video: Video,
}

impl From<(Video, User)> for VideoUserDto {
    fn from(tuple: (Video, User)) -> Self {
        Self {
            user: tuple.1,
            video: tuple.0,
        }
    }
}

#[derive(Serialize)]
pub struct CollectionVideoUserDto {
    pub collection: Collection,
    #[serde(rename = "videoUsers")]
    pub video_users: Vec<VideoUserDto>,
}

impl From<(Collection, Vec<(Video, User)>)> for CollectionVideoUserDto {
    fn from(tuple: (Collection, Vec<(Video, User)>)) -> Self {
        let (c, vs) = tuple;
        Self {
            collection: c,
            video_users: vs.into_iter().map(VideoUserDto::from).collect(),
        }
    }
}
