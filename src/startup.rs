use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
 
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/get_all_users", web::get().to(get_all_users))
            .route("/get_user/{user_id}", web::get().to(get_user))
            .route("/new_user", web::post().to(new_user))
            .route("/new_book", web::post().to(new_book))
            .route("/get_all_books", web::get().to(get_all_books))
            .route("/new_comment", web::post().to(new_comment))
            .route("get_all_comments", web::get().to(get_all_comments))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
