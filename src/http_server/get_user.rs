use super::val::DbPool;
use crate::db;
use actix::*;
use actix_web::web::Json;
use actix_web::{get, Error};
use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

// 注意这里的坑,uuid反序列化问题,解决方法在uuid的creat中添加features = ["serde"]
#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error":404,
                "message":format!("No user found with phone:{id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("users/phone/{user_phone}")]
pub async fn get_user_by_phone(
    pool: web::Data<DbPool>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_phone(&mut conn, user_phone)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "erroe":404,
                "message":format!("not found user by phone:{}",phone.to_string())
            })
            .to_string(),
        );
        Ok(res)
    }
}
