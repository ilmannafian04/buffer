use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SimpleError<T: Serialize> {
    pub error: T,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: String,
}
