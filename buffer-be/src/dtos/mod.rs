use serde::Serialize;

pub(crate) mod user_dto;
pub(crate) mod video;
#[derive(Serialize)]
pub struct SimpleError<T: Serialize> {
    pub error: T,
}
