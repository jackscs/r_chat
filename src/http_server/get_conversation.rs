use crate::db;
use actix_web::{get, web, Error, HttpResponse};
use serde_json::json;
use uuid::Uuid;

use super::val::DbPool;

#[get("/conversation/{uid}")]
pub async fn get_conversation_by_uid(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let conversation = web::block(move || {
        let mut conn = pool.get()?;
        db::get_conversation_by_uid(&mut conn, room_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(conversation) = conversation {
        Ok(HttpResponse::Ok().json(conversation))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":format!("No user found with roomid::{room_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}
