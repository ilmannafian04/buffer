use argon2::hash_encoded;
use chrono::NaiveDateTime;
use diesel::{prelude::*, PgConnection};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::{
    creators, followers,
    users::{self, dsl::users as all_users, dsl::*},
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
        all_users.filter(id.eq(user_id)).first(conn)
    }

    pub fn find_by_email(conn: &PgConnection, mail: &String) -> QueryResult<User> {
        all_users.filter(email.eq(mail)).first(conn)
    }

    pub fn find_by_username(conn: &PgConnection, uname: &String) -> QueryResult<User> {
        all_users.filter(username.eq(uname)).first(conn)
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

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(User)]
#[primary_key(user_id)]
pub struct Creator {
    pub user_id: String,
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
