use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewVideoDTO {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct NewCommentDTO {
    pub video_id: String,
    pub content: String,
}
