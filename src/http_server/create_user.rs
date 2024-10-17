use super::val::DbPool;
use crate::{db, models};
use actix_web::post;
use actix_web::{web, Error, HttpResponse};

#[post("users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}
