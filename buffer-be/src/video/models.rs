use chrono::NaiveDateTime;
use diesel::{
    prelude::*, result::DatabaseErrorKind, result::Error, PgConnection, QueryResult, RunQueryDsl,
};
use rand::Rng;
use serde::Serialize;

use crate::{
    common::models::ResolveMediaURL,
    schema::comments::{self, dsl::comments as all_comments},
    schema::ratings::{self, dsl::ratings as all_ratings},
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

    pub fn find_many_by_id_join_user(
        conn: &PgConnection,
        u_id: &str,
    ) -> QueryResult<Vec<(Video, User)>> {
        use crate::schema::videos::dsl::{created_at, uploader};
        all_videos
            .filter(uploader.eq(u_id))
            .inner_join(users::table)
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

#[derive(Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(Video)]
#[belongs_to(User)]
#[primary_key(video_id, user_id)]
pub struct Rating {
    pub video_id: String,
    pub user_id: String,
    pub is_dislike: bool,
}

impl Rating {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Rating> {
        self.insert_into(ratings::table).get_result(conn)
    }

    pub fn update(self, conn: &PgConnection, is_d: bool) -> QueryResult<Rating> {
        use crate::schema::ratings::dsl::{is_dislike, user_id, video_id};
        diesel::update(
            all_ratings
                .filter(video_id.eq(&self.video_id))
                .filter(user_id.eq(&self.user_id)),
        )
        .set(is_dislike.eq(is_d))
        .get_result(conn)
    }

    pub fn find_by_video_and_user(
        conn: &PgConnection,
        v_id: &str,
        u_id: &str,
    ) -> QueryResult<Rating> {
        use crate::schema::ratings::dsl::{user_id, video_id};
        all_ratings
            .filter(video_id.eq(v_id))
            .filter(user_id.eq(u_id))
            .get_result(conn)
    }

    pub fn count(conn: &PgConnection, v_id: &str, is_d: bool) -> QueryResult<i64> {
        use crate::schema::ratings::dsl::{is_dislike, video_id};
        all_ratings
            .count()
            .filter(video_id.eq(v_id))
            .filter(is_dislike.eq(is_d))
            .get_result(conn)
    }

    pub fn delete(conn: &PgConnection, v_id: &str, u_id: &str) -> QueryResult<usize> {
        use crate::schema::ratings::dsl::{user_id, video_id};
        diesel::delete(
            all_ratings
                .filter(video_id.eq(v_id))
                .filter(user_id.eq(u_id)),
        )
        .execute(conn)
    }
}
