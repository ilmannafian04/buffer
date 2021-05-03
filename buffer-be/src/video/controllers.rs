use std::{io::Write, path::Path};

use actix_multipart::Multipart;
use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse, ResponseError};
use diesel::result::Error;
use futures::TryStreamExt;
use validator::Validate;

use crate::{
    common::{dtos::IdQuery, errors::DatabaseError, models::ResolveMediaURL, types::DbPool},
    config::Config,
    user::models::User,
};

use super::{
    dtos::{
        CommentDTO, NewCommentDTO, NewVideoDTO, VideoDetailDTO, VideoListDTO, VideoListResponseDTO,
    },
    models::{Comment, NewComment, NewVideo, Video},
};

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
    let mut thumbnail_is_saved = false;
    let base_path = Path::new(&config.media_base_dir);
    let path_to_video_folder = Path::new(&user.id.to_string()).join(new_video.id.clone());
    if let Err(_) = std::fs::create_dir_all(&base_path.join(&path_to_video_folder)) {
        return HttpResponse::InternalServerError().finish();
    };
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
            let extension = match Path::new(disposition.get_filename().unwrap()).extension() {
                Some(ext) => format!("raw.{}", ext.to_str().unwrap()),
                None => return HttpResponse::BadRequest().finish(),
            };
            let video_file_name = Path::new(&extension);
            let fs_path = base_path.join(&path_to_video_folder).join(&video_file_name);
            let db_path = Path::new("/")
                .join(&path_to_video_folder)
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
        } else if name == "thumbnail" {
            let file_name = match Path::new(disposition.get_filename().unwrap()).extension() {
                Some(ext) => format!("thumbnail.{}", ext.to_str().unwrap()),
                None => return HttpResponse::BadRequest().finish(),
            };
            let thumbnail_name = Path::new(&file_name);
            let fs_path = base_path.join(&path_to_video_folder).join(&thumbnail_name);
            let db_path = Path::new("/")
                .join(&path_to_video_folder)
                .join(&thumbnail_name);
            new_video.thumbnail_path = db_path.to_str().unwrap().to_owned();
            let mut thumbnail = match web::block(move || std::fs::File::create(fs_path)).await {
                Ok(f) => f,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
            while let Ok(Some(chunk)) = field.try_next().await {
                thumbnail =
                    match web::block(move || thumbnail.write(&chunk).map(|_| thumbnail)).await {
                        Ok(f) => f,
                        Err(_) => return HttpResponse::InternalServerError().finish(),
                    }
            }
            thumbnail_is_saved = true;
        }
    }
    if metadata_is_parsed && video_is_saved && thumbnail_is_saved {
        match pool.get() {
            Ok(conn) => {
                let query = web::block(move || new_video.insert(&conn)).await;
                match query {
                    Ok(mut video) => {
                        video.resolve(&config.media_base_url);
                        HttpResponse::Ok().json(video)
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            Err(_) => return DatabaseError::PoolLockError.error_response(),
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}

pub async fn new_comment(
    pool: web::Data<DbPool>,
    payload: web::Json<NewCommentDTO>,
    req: HttpRequest,
) -> HttpResponse {
    let u_id = req.head().extensions().get::<User>().unwrap().id.clone();
    let conn = pool.get().unwrap();
    let v_id_closure = payload.video_id.clone();
    let video = match web::block(move || Video::find_by_id(&conn, &v_id_closure)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let mut new_comment = NewComment::default();
    new_comment.user_id = u_id;
    new_comment.video_id = video.id.clone();
    new_comment.content = payload.content.clone();
    match web::block(move || new_comment.insert(&conn)).await {
        Ok(c) => HttpResponse::Ok().json(c),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_videos(
    pool: web::Data<DbPool>,
    query: web::Query<VideoListDTO>,
    config: web::Data<Config>,
) -> HttpResponse {
    if let Err(_) = query.validate() {
        return HttpResponse::BadRequest().finish();
    }
    let conn = pool.get().unwrap();
    match web::block(move || Video::find_many_sort_by_new(&conn, query.skip)).await {
        Ok(v) => HttpResponse::Ok().json(
            v.into_iter()
                .map(|mut t| {
                    t.0.resolve(&config.media_base_url);
                    VideoListResponseDTO::from(t)
                })
                .collect::<Vec<VideoListResponseDTO>>(),
        ),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn video_detail(
    pool: web::Data<DbPool>,
    query: web::Query<IdQuery>,
    config: web::Data<Config>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    match web::block(move || Video::find_by_id_join_user(&conn, &query.id)).await {
        Ok(mut t) => {
            t.0.resolve(&config.media_base_url);
            HttpResponse::Ok().json(VideoDetailDTO::from(t))
        }
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn video_comments(pool: web::Data<DbPool>, query: web::Query<IdQuery>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let video = match web::block(move || Video::find_by_id(&conn, &query.id)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    match web::block(move || Comment::find_many_by_video_join_user(&conn, &video.id)).await {
        Ok(tuples) => HttpResponse::Ok().json(
            tuples
                .into_iter()
                .map(CommentDTO::from)
                .collect::<Vec<CommentDTO>>(),
        ),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}
