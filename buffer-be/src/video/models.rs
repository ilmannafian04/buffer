use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    result::DatabaseErrorKind,
    result::Error,
    sql_types::{Float, Text},
    PgConnection, QueryResult, RunQueryDsl,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    common::models::ResolveMediaUrl,
    schema::collection_videos::{self, dsl::collection_videos as all_collection_videos},
    schema::collections::{self, dsl::collections as all_collections},
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

sql_function!(fn similarity(x: Text, y: Text) -> Float);

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

    pub fn find_many_by_title_fuzzy(
        conn: &PgConnection,
        term: &str,
    ) -> QueryResult<Vec<(Video, User)>> {
        use crate::schema::videos::title;
        all_videos
            .inner_join(users::table)
            .filter(similarity(title, term).gt(0.0))
            .order_by(similarity(title, term).desc())
            .get_results(conn)
    }
}

impl ResolveMediaUrl for Video {
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
    #[serde(rename = "isAnonymous")]
    pub is_anonymous: bool,
}

impl Comment {
    pub fn find_many_by_video_join_user(
        conn: &PgConnection,
        v_id: &str,
        skip: i64,
    ) -> QueryResult<Vec<(Comment, Option<User>)>> {
        use crate::schema::comments::dsl::{created_at, video_id};
        all_comments
            .filter(video_id.eq(v_id))
            .left_outer_join(users::table)
            .order_by(created_at.desc())
            .offset(skip)
            .limit(5)
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
    pub is_anonymous: bool,
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
            is_anonymous: false,
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

#[derive(Associations, Identifiable, Queryable, Serialize)]
#[belongs_to(User)]
pub struct Collection {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl Collection {
    pub fn find_by_id(conn: &PgConnection, c_id: &str) -> QueryResult<Collection> {
        all_collections.find(c_id).get_result(conn)
    }

    pub fn find_by_user_join_video(
        conn: &PgConnection,
        u_id: &str,
    ) -> QueryResult<Vec<(Collection, Option<(CollectionVideo, Video)>)>> {
        use crate::schema::collections::dsl::user_id;
        all_collections
            .filter(user_id.eq(u_id))
            .left_join(all_collection_videos.inner_join(videos::table))
            .get_results(conn)
    }

    pub fn find_by_id_and_user(
        conn: &PgConnection,
        c_id: &str,
        u_id: &str,
    ) -> QueryResult<Collection> {
        use crate::schema::collections::dsl::{id, user_id};
        all_collections
            .filter(id.eq(c_id))
            .filter(user_id.eq(u_id))
            .get_result(conn)
    }
}

#[derive(Insertable)]
#[table_name = "collections"]
pub struct NewCollection {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
}

impl NewCollection {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Collection> {
        self.insert_into(collections::table).get_result(conn)
    }
}

impl Default for NewCollection {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_simple().to_string(),
            user_id: "".to_owned(),
            name: "".to_owned(),
            description: "".to_owned(),
        }
    }
}

#[derive(Associations, Deserialize, Identifiable, Insertable, Queryable)]
#[belongs_to(Collection)]
#[belongs_to(Video)]
#[primary_key(collection_id, video_id)]
pub struct CollectionVideo {
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "videoId")]
    pub video_id: String,
}

impl CollectionVideo {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<CollectionVideo> {
        self.insert_into(collection_videos::table).get_result(conn)
    }

    pub fn find_many_by_collection_join_video_join_user(
        conn: &PgConnection,
        c_id: &str,
        skip: i64,
    ) -> QueryResult<Vec<(CollectionVideo, (Video, User))>> {
        use crate::schema::collection_videos::dsl::collection_id;
        all_collection_videos
            .inner_join(all_videos.inner_join(users::table))
            .filter(collection_id.eq(c_id))
            .limit(5)
            .offset(skip)
            .get_results(conn)
    }
}
