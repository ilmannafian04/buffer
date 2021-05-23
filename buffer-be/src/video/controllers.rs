use std::{io::Write, path::Path};

use actix_multipart::Multipart;
use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error};
use futures::TryStreamExt;
use validator::Validate;

use super::{
    dtos::{
        CommentDto, HasRatedDto, NewCommentDto, NewVideoDto, RateVideoRequest, SearchVideoDto,
        VideoDetailDto, VideoListDto, VideoListResponseDto, VideoRatingDto,
    },
    models::{Comment, NewComment, NewVideo, Rating, Video},
};
use crate::video::dtos::CollectionVideoUserDTO;
use crate::{
    common::{
        dtos::{IdQuery, IndexRequestDto},
        models::ResolveMediaUrl,
        types::DbPool,
    },
    config::Config,
    user::{dtos::CreatorLookUpDto, models::User},
    video::dtos::VideoUserDTO,
    video::{
        dtos::{CollectionDetailDto, CollectionDto, NewCollectionDto},
        models::{Collection, CollectionVideo, NewCollection},
    },
};

pub async fn upload_video(
    mut payload: Multipart,
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> HttpResponse {
    let user = req.head().extensions().get::<User>().unwrap().clone();
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
            let path_closure = fs_path.clone();
            let mut thumbnail = match web::block(move || std::fs::File::create(path_closure)).await
            {
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
            if web::block(move || NewVideo::crop_thumbnail(&fs_path))
                .await
                .is_err()
            {
                return HttpResponse::InternalServerError().finish();
            }
            thumbnail_is_saved = true;
        }
    }
    if metadata_is_parsed && video_is_saved && thumbnail_is_saved {
        let conn = pool.get().unwrap();
        match web::block(move || new_video.insert(&conn)).await {
            Ok(mut video) => {
                video.resolve(&config.media_base_url);
                HttpResponse::Ok().json(video)
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
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
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let u_id = user.id.clone();
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
        is_anonymous: payload.is_anonymous,
        ..Default::default()
    };
    let user_closure = user.clone();
    match web::block(move || new_comment.insert(&conn)).await {
        Ok(c) => HttpResponse::Ok().json(CommentDto::from((c, Some(user_closure)))),
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
    if query.validate().is_err() {
        return HttpResponse::BadRequest().finish();
    };
    let conn = pool.get().unwrap();
    let id_closure = query.id.clone();
    let video = match web::block(move || Video::find_by_id(&conn, &id_closure)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    match web::block(move || Comment::find_many_by_video_join_user(&conn, &video.id, query.skip))
        .await
    {
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
    match web::block(move || Video::find_many_by_title_fuzzy(&conn, &query.term)).await {
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

pub async fn collection_info(
    pool: web::Data<DbPool>,
    query: web::Query<IndexRequestDto>,
    config: web::Data<Config>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id_closure = query.id.clone();
    let collection = match web::block(move || Collection::find_by_id(&conn, &id_closure)).await {
        Ok(c) => c,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    let conn = pool.get().unwrap();
    let id_closure = collection.id.clone();
    match web::block(move || {
        CollectionVideo::find_many_by_collection_join_video_join_user(
            &conn,
            &id_closure,
            query.skip,
        )
    })
    .await
    {
        Ok(t) => HttpResponse::Ok().json(CollectionVideoUserDTO::from((
            collection,
            t.into_iter()
                .map(|mut tuple| {
                    tuple.1 .0.resolve(&config.media_base_url);
                    tuple.1
                })
                .collect::<Vec<(Video, User)>>(),
        ))),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn users_collection(pool: web::Data<DbPool>, req: HttpRequest) -> HttpResponse {
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let id_closure = user.id.clone();
    let tuples =
        match web::block(move || Collection::find_by_user_join_video(&conn, &id_closure)).await {
            Ok(rows) => rows
                .into_iter()
                .map(|row| {
                    (
                        row.0,
                        match row.1 {
                            Some(t) => Some(t.1),
                            None => None,
                        },
                    )
                })
                .collect::<Vec<(Collection, Option<Video>)>>(),
            _ => return HttpResponse::InternalServerError().finish(),
        };
    let mut result: Vec<CollectionDto> = Vec::new();
    let mut temp_dto = CollectionDto::default();
    for tuple in tuples {
        let (c, v) = tuple;
        if c.id != temp_dto.id {
            if !temp_dto.id.is_empty() {
                result.push(temp_dto);
            }
            temp_dto = CollectionDto::from(c);
        }
        if v.is_some() {
            temp_dto.videos.push(v.unwrap());
        }
    }
    if !temp_dto.id.is_empty() {
        result.push(temp_dto);
    }
    HttpResponse::Ok().json(result)
}

pub async fn add_video_to_collection(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    payload: web::Json<CollectionVideo>,
) -> HttpResponse {
    let ext = req.head().extensions();
    let user = ext.get::<User>().unwrap();
    let conn = pool.get().unwrap();
    let tuple_closure = (payload.collection_id.clone(), user.id.clone());
    match web::block(move || {
        Collection::find_by_id_and_user(&conn, &tuple_closure.0, &tuple_closure.1)
    })
    .await
    {
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::Forbidden().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
        _ => (),
    };
    let conn = pool.get().unwrap();
    let payload_closure = CollectionVideo {
        collection_id: payload.collection_id.clone(),
        video_id: payload.video_id.clone(),
    };
    match web::block(move || payload_closure.insert(&conn)).await {
        Ok(_)
        | Err(BlockingError::Error(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _))) => {
            HttpResponse::Ok().finish()
        }
        Err(BlockingError::Error(Error::DatabaseError(
            DatabaseErrorKind::ForeignKeyViolation,
            _,
        ))) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_liked_videos(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    config: web::Data<Config>,
) -> HttpResponse {
    let user = req.head().extensions().get::<User>().unwrap().clone();
    match web::block(move || Rating::find_liked_join_video_and_user(&pool.get().unwrap(), &user.id))
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(
            rows.into_iter()
                .map(|row| {
                    let (_, mut v, u) = row;
                    v.resolve(&config.media_base_url);
                    VideoUserDTO::from((v, u))
                })
                .collect::<Vec<VideoUserDTO>>(),
        ),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_video(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    payload: web::Json<IdQuery>,
    config: web::Data<Config>,
) -> HttpResponse {
    let user = req.head().extensions().get::<User>().unwrap().clone();
    let conn = pool.get().unwrap();
    let video = match web::block(move || Video::find_by_id(&conn, &payload.id)).await {
        Ok(v) => v,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    if video.uploader != user.id {
        return HttpResponse::Forbidden().finish();
    }
    let video_folder_dir = Path::new(&config.media_base_dir)
        .join(&user.id)
        .join(&video.id);
    if web::block(|| std::fs::remove_dir_all(video_folder_dir))
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    let conn = pool.get().unwrap();
    match web::block(move || video.delete(&conn)).await {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_collection(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    payload: web::Json<IdQuery>,
) -> HttpResponse {
    let user = req.head().extensions().get::<User>().unwrap().clone();
    let conn = pool.get().unwrap();
    let collection = match web::block(move || Collection::find_by_id(&conn, &payload.id)).await {
        Ok(c) => c,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    if collection.user_id != user.id {
        return HttpResponse::Forbidden().finish();
    }
    let conn = pool.get().unwrap();
    match web::block(move || collection.delete(&conn)).await {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_comment(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    payload: web::Json<IdQuery>,
) -> HttpResponse {
    let user = req.head().extensions().get::<User>().unwrap().clone();
    let conn = pool.get().unwrap();
    let comment = match web::block(move || Comment::find_by_id(&conn, &payload.id)).await {
        Ok(c) => c,
        Err(BlockingError::Error(Error::NotFound)) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::InternalServerError().finish(),
    };
    if comment.user_id != user.id {
        return HttpResponse::Forbidden().finish();
    }
    let conn = pool.get().unwrap();
    match web::block(move || comment.delete(&conn)).await {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
