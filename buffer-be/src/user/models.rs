use std::usize;

use argon2::hash_encoded;
use chrono::NaiveDateTime;
use diesel::{dsl::count_star, prelude::*, PgConnection};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::{
    creators,
    followers::{self, dsl::followers as all_followers},
    users::{self, dsl::users as all_users},
};

pub enum UniqueViolationKind {
    Email,
    Username,
}

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn check_unique_integrity(
        conn: &PgConnection,
        uname: &String,
        mail: &String,
    ) -> Result<(), UniqueViolationKind> {
        if let Ok(_) = Self::find_by_email(conn, mail) {
            Err(UniqueViolationKind::Email)
        } else if let Ok(_) = Self::find_by_username(conn, uname) {
            Err(UniqueViolationKind::Username)
        } else {
            Ok(())
        }
    }

    pub fn find_by_id(conn: &PgConnection, user_id: String) -> QueryResult<User> {
        use crate::schema::users::dsl::id;
        all_users.filter(id.eq(user_id)).first(conn)
    }

    pub fn find_by_email(conn: &PgConnection, mail: &String) -> QueryResult<User> {
        use crate::schema::users::dsl::email;
        all_users.filter(email.eq(mail)).first(conn)
    }

    pub fn find_by_username(conn: &PgConnection, uname: &String) -> QueryResult<User> {
        use crate::schema::users::dsl::username;
        all_users.filter(username.eq(uname)).first(conn)
    }

    pub fn find_by_display_name(conn: &PgConnection, d_name: &String) -> QueryResult<User> {
        use crate::schema::users::dsl::display_name;
        all_users.filter(display_name.eq(d_name)).first(conn)
    }
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[serde(default)]
    pub id: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1))]
    pub username: String,
    #[serde(alias = "displayName")]
    #[validate(length(min = 1))]
    pub display_name: String,
}

impl NewUser {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<User> {
        self.insert_into(users::table).get_result(conn)
    }

    pub fn generate_id(&mut self) {
        self.id = uuid::Uuid::new_v4().to_simple().to_string();
    }

    pub fn hash_password(&mut self) -> Result<(), ()> {
        match hash_encoded(
            self.password.as_bytes(),
            &rand::thread_rng().gen::<[u8; 16]>(),
            &argon2::Config::default(),
        ) {
            Ok(hash) => {
                self.password = hash;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable)]
#[belongs_to(User)]
#[primary_key(user_id)]
pub struct Creator {
    pub user_id: String,
}

impl Creator {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Creator> {
        self.insert_into(creators::table).get_result(conn)
    }
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Creator)]
#[belongs_to(User, foreign_key = "viewer_id")]
#[primary_key(creator_id, viewer_id)]
pub struct Follower {
    pub creator_id: String,
    pub viewer_id: String,
    pub created_at: NaiveDateTime,
}

impl Follower {
    pub fn delete(
        conn: &PgConnection,
        creator_id: &String,
        user_id: &String,
    ) -> QueryResult<usize> {
        diesel::delete(
            all_followers
                .filter(followers::dsl::creator_id.eq(creator_id))
                .filter(followers::dsl::viewer_id.eq(user_id)),
        )
        .execute(conn)
    }

    pub fn get_follower_count(conn: &PgConnection, c_id: &String) -> QueryResult<i64> {
        use crate::schema::followers::dsl::creator_id;
        all_followers
            .filter(creator_id.eq(c_id))
            .select(count_star())
            .first(conn)
    }

    pub fn get_by_creator_and_viewer(
        conn: &PgConnection,
        c_id: &str,
        v_id: &str,
    ) -> QueryResult<Follower> {
        use crate::schema::followers::dsl::{creator_id, viewer_id};
        all_followers
            .filter(creator_id.eq(c_id))
            .filter(viewer_id.eq(v_id))
            .first(conn)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    pub creator_id: String,
    pub viewer_id: String,
}

impl NewFollower {
    pub fn insert(self, conn: &PgConnection) -> QueryResult<Follower> {
        self.insert_into(followers::table).get_result(conn)
    }
}
