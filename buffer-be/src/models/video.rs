use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
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
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl Video {
    pub fn find_all(conn: &PgConnection) -> QueryResult<Vec<Video>> {
        all_videos.get_results(conn)
    }
}
