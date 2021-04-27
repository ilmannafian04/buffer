use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleError<T: Serialize> {
    pub error: T,
}
