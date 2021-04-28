use chrono::NaiveDateTime;
use diesel::{result::DatabaseErrorKind, result::Error, PgConnection, QueryResult, RunQueryDsl};
use rand::Rng;
use serde::Serialize;

use crate::{
    schema::videos::{self, dsl::videos as all_videos},
    user::models::User,
};

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(User, foreign_key = "uploader")]
pub struct Video {
    pub id: String,
    pub uploader: String,
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
    pub uploader: String,
    pub title: String,
    pub description: String,
    pub video_path: String,
}

impl NewVideo {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Video> {
        loop {
            let query = diesel::insert_into(videos::table)
                .values(&self)
                .get_result::<Video>(conn);
            match query {
                Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => continue,
                _ => return query,
            }
        }
    }

    pub fn generate_id() -> String {
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}

impl Default for NewVideo {
    fn default() -> Self {
        Self {
            id: Self::generate_id(),
            uploader: "".to_owned(),
            title: "My New Video".to_owned(),
            description: "".to_owned(),
            video_path: "".to_owned(),
        }
    }
}
