use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use futures::TryStreamExt;

use crate::{
    dtos::video::NewVideoDTO,
    error::DatabaseError,
    models::{user::User, video::NewVideo},
};

use super::DbPool;

pub async fn upload_video(
    mut payload: Multipart,
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> HttpResponse {
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
            let parent_path = std::path::Path::new("/temp")
                .join(user.id.to_string())
                .join(new_video.id.clone());
            if let Err(_) = std::fs::create_dir_all(&parent_path) {
                return HttpResponse::InternalServerError().finish();
            };
            let video_path = parent_path.join("raw.mp4");
            new_video.video_path = video_path.to_str().unwrap().to_owned();
            let mut video = match web::block(move || std::fs::File::create(video_path)).await {
                Ok(file) => file,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
            while let Ok(Some(chunk)) = field.try_next().await {
                video = match web::block(move || video.write(&chunk).map(|_| video)).await {
                    Ok(f) => f,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };
            }
            video_is_saved = true;
        }
    }
    if metadata_is_parsed && video_is_saved {
        match pool.get() {
            Ok(conn) => {
                let query = web::block(move || new_video.insert(&conn)).await;
                match query {
                    Ok(video) => HttpResponse::Ok().json(video),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            Err(_) => return DatabaseError::PoolLockError.error_response(),
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}
