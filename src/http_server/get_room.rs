use actix_web::{get, web, Error, HttpResponse};
use serde_json::json;

use crate::db;

use super::val::DbPool;

#[get("/room")]
pub async fn get_room(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_rooms(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !rooms.is_empty() {
        Ok(HttpResponse::Ok().json(rooms))
    } else {
        Ok(HttpResponse::NotFound().body(
            json!({
                "error":"404",
                "message":format!("not found room")
            })
            .to_string(),
        ))
    }
}
