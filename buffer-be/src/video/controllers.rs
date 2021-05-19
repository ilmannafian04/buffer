use std::{io::Write, path::Path};

use actix_multipart::Multipart;
use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse, ResponseError};
use diesel::result::Error;
use futures::TryStreamExt;
use validator::Validate;

use crate::{
    common::{
        dtos::{IdQuery, IndexRequestDto},
        errors::DatabaseError,
        models::ResolveMediaUrl,
        types::DbPool,
    },
    config::Config,
    user::{dtos::CreatorLookUpDto, models::User},
};

use super::{
    dtos::{
        CommentDto, HasRatedDto, NewCommentDto, NewVideoDto, RateVideoRequest, SearchVideoDto,
        VideoDetailDto, VideoListDto, VideoListResponseDto, VideoRatingDto,
    },
    models::{Comment, NewComment, NewVideo, Rating, Video},
};
use crate::video::dtos::{CollectionDetailDto, NewCollectionDto};
use crate::video::models::{Collection, CollectionVideo, NewCollection};

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
    if std::fs::create_dir_all(&base_path.join(&path_to_video_folder)).is_err() {
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
            match serde_json::from_str::<NewVideoDto>(json_string.as_str()) {
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
    payload: web::Json<NewCommentDto>,
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
    let new_comment = NewComment {
        user_id: u_id,
        video_id: video.id.clone(),
        content: payload.content.clone(),
        ..Default::default()
    };
    match web::block(move || new_comment.insert(&conn)).await {
        Ok(c) => HttpResponse::Ok().json(c),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_videos(
    pool: web::Data<DbPool>,
    query: web::Query<VideoListDto>,
    config: web::Data<Config>,
) -> HttpResponse {
    if query.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let conn = pool.get().unwrap();
    match web::block(move || Video::find_many_sort_by_new(&conn, query.skip)).await {
        Ok(v) => HttpResponse::Ok().json(
            v.into_iter()
                .map(|mut t| {
                    t.0.resolve(&config.media_base_url);
                    VideoListResponseDto::from(t)
                })
                .collect::<Vec<VideoListResponseDto>>(),
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
            HttpResponse::Ok().json(VideoDetailDto::from(t))
        }
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn video_comments(
    pool: web::Data<DbPool>,
    query: web::Query<IndexRequestDto>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id_closure = query.id.clone();
    let video = match web::block(move || Video::find_by_id(&conn, &id_closure)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    match web::block(move || Comment::find_many_by_video_join_user(&conn, &video.id)).await {
        Ok(tuples) => HttpResponse::Ok().json(
            tuples
                .into_iter()
                .map(CommentDto::from)
                .collect::<Vec<CommentDto>>(),
        ),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn rate_video(
    pool: web::Data<DbPool>,
    payload: web::Json<RateVideoRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let extension = req.head().extensions();
    let user = extension.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let id_closure = payload.id.clone();
    let video = match web::block(move || Video::find_by_id(&conn, &id_closure)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let param = (video.id.clone(), user.id.clone());
    match web::block(move || Rating::find_by_video_and_user(&conn, &param.0, &param.1)).await {
        Ok(rating) => {
            let conn = pool.get().unwrap();
            if rating.is_dislike != payload.is_dislike {
                if web::block(move || rating.update(&conn, payload.is_dislike))
                    .await
                    .is_err()
                {
                    return HttpResponse::InternalServerError().finish();
                }
            } else if web::block(move || Rating::delete(&conn, &rating.video_id, &rating.user_id))
                .await
                .is_err()
            {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
        Err(BlockingError::Error(Error::NotFound)) => {
            let rating = Rating {
                video_id: video.id.clone(),
                user_id: user.id.clone(),
                is_dislike: payload.is_dislike,
            };
            let conn = pool.get().unwrap();
            match web::block(move || rating.insert(&conn)).await {
                Ok(_) => HttpResponse::Ok().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_rating(pool: web::Data<DbPool>, query: web::Query<IdQuery>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let video = match web::block(move || Video::find_by_id(&conn, &query.id)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let id_closure = video.id.clone();
    let like = match web::block(move || Rating::count(&conn, &id_closure, false)).await {
        Ok(c) => c,
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let id_closure = video.id.clone();
    let dislike = match web::block(move || Rating::count(&conn, &id_closure, true)).await {
        Ok(c) => c,
        _ => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().json(VideoRatingDto { like, dislike })
}

pub async fn has_rated(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    query: web::Query<IdQuery>,
) -> HttpResponse {
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let id_closure = user.id.clone();
    match web::block(move || Rating::find_by_video_and_user(&conn, &query.id, &id_closure)).await {
        Ok(rating) => HttpResponse::Ok().json(HasRatedDto {
            has_rated: true,
            is_dislike: rating.is_dislike,
        }),
        Err(BlockingError::Error(Error::NotFound)) => HttpResponse::Ok().json(HasRatedDto {
            has_rated: false,
            is_dislike: false,
        }),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn creator_videos(
    pool: web::Data<DbPool>,
    query: web::Query<CreatorLookUpDto>,
    config: web::Data<Config>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let user = match web::block(move || User::find_by_username(&conn, &query.username)).await {
        Ok(u) => u,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    match web::block(move || Video::find_many_by_id_join_user(&conn, &user.id)).await {
        Ok(videos) => HttpResponse::Ok().json(
            videos
                .into_iter()
                .map(|tuple| {
                    let (mut v, u) = tuple;
                    v.resolve(&config.media_base_url);
                    VideoDetailDto::from((v, u))
                })
                .collect::<Vec<VideoDetailDto>>(),
        ),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn search_videos(
    pool: web::Data<DbPool>,
    query: web::Query<SearchVideoDto>,
    config: web::Data<Config>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    match web::block(move || Video::find_many_by_title_or_description_join_user(&conn, &query.term))
        .await
    {
        Ok(videos) => HttpResponse::Ok().json(
            videos
                .into_iter()
                .map(|tuple| {
                    let (mut v, u) = tuple;
                    v.resolve(&config.media_base_url);
                    VideoDetailDto::from((v, u))
                })
                .collect::<Vec<VideoDetailDto>>(),
        ),
        _ => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn new_collection(
    pool: web::Data<DbPool>,
    payload: web::Json<NewCollectionDto>,
    req: HttpRequest,
) -> HttpResponse {
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let new_collection = NewCollection {
        name: payload.name.clone(),
        description: payload.description.clone(),
        user_id: user.id.clone(),
        ..NewCollection::default()
    };
    let conn = pool.get().unwrap();
    match web::block(move || new_collection.insert(&conn)).await {
        Ok(c) => HttpResponse::Ok().json(c),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn collection_info(pool: web::Data<DbPool>, query: web::Query<IdQuery>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let collection = match web::block(move || Collection::find_by_id(&conn, &query.id)).await {
        Ok(c) => c,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let id_closure = collection.id.clone();
    match web::block(move || {
        CollectionVideo::find_many_by_collection_join_video_join_user(&conn, &id_closure)
    })
    .await
    {
        Ok(t) => HttpResponse::Ok().json(CollectionDetailDto::from((
            collection,
            t.into_iter()
                .map(|tuple| tuple.1)
                .collect::<Vec<(Video, User)>>(),
        ))),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
