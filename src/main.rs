extern crate diesel;
use crate::constant::server::{SERVER_ADDR, SERVER_PORT};
use actix::*;
use actix_cors::Cors;
use actix_web::{
    http,
    web, App, HttpServer,
};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

mod constant;
mod db;
mod http_server;
mod models;
mod routers;
mod schema;
mod server;

const CONN_SPEC: &'static str = "chat.db";
const ALLOWED_ORIGIN1: &'static str = "127.0.0.1:3000";
const ALLOWED_ORIGIN2: &'static str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::chat::ChatServer::new().start();
    let manager = ConnectionManager::<SqliteConnection>::new(CONN_SPEC);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = HttpServer::new(move || {
        let cores = Cors::default()
            .allowed_origin(ALLOWED_ORIGIN1)
            .allowed_origin(ALLOWED_ORIGIN2)
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(cores)
            .configure(routers::init_router)
    })
    .workers(2)
    .bind((SERVER_ADDR, SERVER_PORT))?
    .run();
    println!("Server running at http://{SERVER_ADDR}:{SERVER_PORT}/");
    app.await
}
