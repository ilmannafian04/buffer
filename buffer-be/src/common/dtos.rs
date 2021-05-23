use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct SimpleError<T: Serialize> {
    pub error: T,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: String,
}

#[derive(Deserialize, Validate)]
pub struct IndexRequestDto {
    pub id: String,
    #[validate(range(min = 0))]
    #[serde(default)]
    pub skip: i64,
}
