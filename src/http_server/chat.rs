use super::val::DbPool;
use crate::server::{chat::ChatServer, session::WsChatSession};
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn chat_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db_pool: pool,
        },
        &req,
        stream,
    )
}
