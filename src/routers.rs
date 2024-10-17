use crate::http_server::{
    chat::chat_server,
    create_user::create_user,
    get_conversation::get_conversation_by_uid,
    get_room::get_room,
    get_user::{get_user_by_id, get_user_by_phone},
    html::index,
};
use actix_files::Files;
use actix_web::web::{self, service};

pub fn init_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
        .route("/ws", web::get().to(chat_server))
        .service(create_user)
        .service(get_conversation_by_uid)
        .service(get_room)
        .service(get_user_by_id)
        .service(get_user_by_id)
        .service(Files::new("/", "./static"));
}
