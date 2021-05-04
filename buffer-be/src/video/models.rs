use chrono::NaiveDateTime;
use diesel::{
    prelude::*, result::DatabaseErrorKind, result::Error, PgConnection, QueryResult, RunQueryDsl,
};
use rand::Rng;
use serde::Serialize;

use crate::{
    common::models::ResolveMediaURL,
    schema::comments::{self, dsl::comments as all_comments},
    schema::users,
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
    pub thumbnail_path: String,
    pub created_at: NaiveDateTime,
}

impl Video {
    pub fn find_all(conn: &PgConnection) -> QueryResult<Vec<Video>> {
        all_videos.get_results(conn)
    }

    pub fn find_by_id(conn: &PgConnection, v_id: &str) -> QueryResult<Video> {
        all_videos.find(v_id).get_result(conn)
    }

    pub fn find_by_id_join_user(conn: &PgConnection, v_id: &str) -> QueryResult<(Video, User)> {
        use crate::schema::videos::dsl::id;
        all_videos
            .filter(id.eq(v_id))
            .inner_join(users::table)
            .get_result(conn)
    }

    pub fn find_many_sort_by_new(
        conn: &PgConnection,
        skip: i64,
    ) -> QueryResult<Vec<(Video, Option<User>)>> {
        use crate::schema::videos::dsl::created_at;
        all_videos
            .left_join(users::table)
            .limit(10)
            .offset(skip)
            .order_by(created_at.desc())
            .get_results(conn)
    }
}

impl ResolveMediaURL for Video {
    fn resolve(&mut self, base_url: &str) {
        let base = url::Url::parse(base_url).unwrap();
        self.video_path.remove(0);
        self.video_path = base.join(&self.video_path).unwrap().to_string();
        self.thumbnail_path.remove(0);
        self.thumbnail_path = base.join(&self.thumbnail_path).unwrap().to_string();
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
    pub thumbnail_path: String,
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
            thumbnail_path: "".to_owned(),
        }
    }
}

#[derive(Associations, Debug, Identifiable, Queryable, Serialize)]
#[belongs_to(Video)]
#[belongs_to(User)]
pub struct Comment {
    pub id: String,
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Comment {
    pub fn find_many_by_video_join_user(
        conn: &PgConnection,
        v_id: &str,
    ) -> QueryResult<Vec<(Comment, Option<User>)>> {
        use crate::schema::comments::dsl::{created_at, video_id};
        all_comments
            .filter(video_id.eq(v_id))
            .left_outer_join(users::table)
            .order_by(created_at.desc())
            .get_results(conn)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub id: String,
    pub video_id: String,
    pub user_id: String,
    pub content: String,
}

impl NewComment {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Comment> {
        self.insert_into(comments::table).get_result(conn)
    }
}

impl Default for NewComment {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_simple().to_string(),
            video_id: "".to_owned(),
            user_id: "".to_owned(),
            content: "".to_owned(),
        }
    }
}
