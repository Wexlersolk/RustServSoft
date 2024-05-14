use crate::jwtauth::jwtauth::*;
use crate::routes::*;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
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
            .route("/update_password", web::put().to(update_password))
            .route("/elevate_privileges", web::put().to(elevate_priviliges))
            .route("/authorize", web::put().to(authorize))
            .route("/get_all_users", web::get().to(get_all_users))
            .route("/delete_user", web::delete().to(delete_user))
            .route("/health_check", web::get().to(health_check))
            //Books
            .route("/new_book", web::post().to(new_book))
            .route("/upload_book", web::post().to(upload_file))
            .route("/get_all_books", web::get().to(get_all_books))
            .route("/get_popular_books", web::get().to(get_popular_books))
            .route("/get_sorted_books", web::get().to(get_sorted_books))
            .route("/get_book_file", web::get().to(get_book_file))
            .route("/get_book_by_id", web::get().to(get_book_by_id))
            //Main page
            .route("/get_all_genres", web::get().to(get_all_genres))
            //JWT
            .route("/decode_token", web::post().to(decode_token))
            .app_data(db_pool.clone())
            .app_data(web::Data::<String>::new("Padishah Emperor".to_owned()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
