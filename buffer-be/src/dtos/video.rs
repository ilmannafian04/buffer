use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewVideoDTO {
    pub title: String,
    pub description: String,
}
