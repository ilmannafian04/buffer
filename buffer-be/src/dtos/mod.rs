use serde::Serialize;

pub(crate) mod user_dto;

#[derive(Serialize)]
pub struct SimpleError<T: Serialize> {
    pub error: T,
}
