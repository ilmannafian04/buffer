use chrono::NaiveDateTime;

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

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
    pub display_name: String,
}
