// use std::time::Instant;
//
// use actix::Addr;
// use actix_web::{error::BlockingError, web, Error, HttpRequest, HttpResponse};
// use actix_web_actors::ws;
// use diesel::result::Error::NotFound;
//
// use crate::{
//     chat::{server::ChatServer, session::WsChatSession},
//     common::types::DbPool,
//     user::{dtos::CreatorLookUpDto, models::User},
// };
//
// pub async fn entrypoint(
//     pool: web::Data<DbPool>,
//     query: web::Query<CreatorLookUpDto>,
//     stream: web::Payload,
//     srv: web::Data<Addr<ChatServer>>,
//     req: HttpRequest,
// ) -> Result<HttpResponse, Error> {
//     let conn = pool.get().unwrap();
//     let user = match web::block(move || User::find_by_username(&conn, &query.username)).await {
//         Ok(u) => u,
//         Err(BlockingError::Error(NotFound)) => return Err(Error::from(HttpResponse::NotFound())),
//         _ => return Err(Error::from(HttpResponse::InternalServerError())),
//     };
//     ws::start(
//         WsChatSession {
//             id: 0,
//             heartbeat: Instant::now(),
//             room: "".to_string(),
//             user_id: "".to_string(),
//         },
//         &req,
//         stream,
//     )
// }
