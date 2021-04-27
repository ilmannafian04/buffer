use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse};
use futures::TryStreamExt;

use crate::dtos::video::NewVideoDTO;
use crate::{models::user::User, models::video::NewVideo};

pub async fn upload_video(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let extension = req.head().extensions();
    let user = extension.get::<User>().unwrap();
    let mut new_video = NewVideo::default();
    let mut metadata_is_parsed = false;
    let mut video_is_saved = false;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let disposition = field.content_disposition().unwrap();
        let name = disposition.get_name().unwrap();
        if name == "metadata" {
            let mut json_string = String::new();
            while let Ok(Some(chunk)) = field.try_next().await {
                match std::str::from_utf8(&chunk) {
                    Ok(string_chunk) => json_string += string_chunk,
                    Err(_) => return HttpResponse::BadRequest().finish(),
                };
            }
            match serde_json::from_str::<NewVideoDTO>(json_string.as_str()) {
                Ok(dto) => {
                    new_video.title = dto.title;
                    new_video.description = dto.description;
                    new_video.uploader = user.id;
                }
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            metadata_is_parsed = true;
        } else if name == "video" {
            // debug!("video");
            video_is_saved = true;
        }
    }
    if metadata_is_parsed && video_is_saved {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}
