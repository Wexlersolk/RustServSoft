use crate::routes::*;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/new_user", web::post().to(new_user))
            .route("/update_password/{user_id}", web::put().to(update_password))
            .route(
                "/elevate_privileges/{user_id}",
                web::put().to(elevate_priviliges),
            )
            .route("/get_user/{user_id}", web::get().to(get_user))
            .route("/get_user_id", web::get().to(get_user_id))
            .route("/get_all_users", web::get().to(get_all_users))
            .route("/delete_user/{user_id}", web::delete().to(delete_user))
            .route("/health_check", web::get().to(health_check))
            
           //Books 
            .route("/new_book", web::post().to(new_book))
            .route("/get_all_books", web::get().to(get_all_books))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
