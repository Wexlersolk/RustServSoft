use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/new_user", web::post().to(new_user))
            .route("/update_user", web::put().to(update_user))
            .route("/get_user/{user_id}", web::get().to(get_user))
            .route(
                "/get_user_id/{login}/{password}",
                web::get().to(get_user_id),
            )
            .route("/get_all_users", web::get().to(get_all_users))
            .route("/delete_user/{user_id}", web::delete().to(delete_user))
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
