use argon2::hash_encoded;
use chrono::NaiveDateTime;
use diesel::{prelude::*, PgConnection};
use rand::Rng;
use serde::Deserialize;
use validator::Validate;

use crate::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
    pub display_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[table_name = "users"]
pub struct NewUser {
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
