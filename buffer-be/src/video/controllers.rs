use std::{io::Write, path::Path};

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use futures::TryStreamExt;

use crate::{
    common::{errors::DatabaseError, types::DbPool},
    config::Config,
    user::models::User,
};

use super::{dtos::NewVideoDTO, models::NewVideo};

pub async fn upload_video(
    mut payload: Multipart,
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
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
                    new_video.uploader = user.id.clone();
                }
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            metadata_is_parsed = true;
        } else if name == "video" {
            let base_path = Path::new(&config.static_files_dir);
            let path_to_video_folder = Path::new(&user.id.to_string()).join(new_video.id.clone());
            let extension = match Path::new(disposition.get_filename().unwrap()).extension() {
                Some(ext) => format!("raw.{}", ext.to_str().unwrap()),
                None => return HttpResponse::BadRequest().finish(),
            };
            let video_file_name = Path::new(&extension);
            if let Err(_) = std::fs::create_dir_all(&base_path.join(&path_to_video_folder)) {
                return HttpResponse::InternalServerError().finish();
            };
            let fs_path = base_path.join(&path_to_video_folder).join(&video_file_name);
            let db_path = Path::new("/")
                .join(path_to_video_folder)
                .join(&video_file_name);
            new_video.video_path = db_path.to_str().unwrap().to_owned();
            let mut video = match web::block(move || std::fs::File::create(fs_path)).await {
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
