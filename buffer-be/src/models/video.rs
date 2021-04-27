use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use rand::Rng;
use serde::Serialize;

use crate::models::user::User;
use crate::schema::videos::{self, dsl::videos as all_videos};

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(User, foreign_key = "uploader")]
pub struct Video {
    pub id: String,
    pub uploader: i32,
    pub title: String,
    pub description: String,
    pub video_path: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl Video {
    pub fn find_all(conn: &PgConnection) -> QueryResult<Vec<Video>> {
        all_videos.get_results(conn)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "videos"]
pub struct NewVideo {
    pub id: String,
    pub uploader: i32,
    pub title: String,
    pub description: String,
    pub video_path: String,
}

impl NewVideo {}

impl Default for NewVideo {
    fn default() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect(),
            uploader: 0,
            title: "My New Video".to_owned(),
            description: "".to_owned(),
            video_path: "".to_owned(),
        }
    }
}
