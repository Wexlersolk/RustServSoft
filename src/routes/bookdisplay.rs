use crate::routes::*;

use actix_web::{
    web::{self, Bytes},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SortedInfo {
    genre: String,
    parameter: String
}

pub async fn get_books_from_db(pool: web::Data<PgPool>, query: String) -> Result<Vec<BookData>, sqlx::Error> {
    sqlx::query_as!(BookData, query)       
        .fetch_all(pool.as_ref())
        .await
}

pub async fn get_all_books(pool: web::Data<PgPool>) -> HttpResponse {
    let query = "SELECT book_view.*, '' as img FROM book_view";
    match get_books_from_db(pool, query.to_string()).await {
        Ok(books) => {
            log::info!("All books have been fetched");
            HttpResponse::Ok().json(create_reduced_info_json(books))
        }
        Err(e) => {
            log::error!("Failed to fetch books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_sorted_books(
    pool: web::Data<PgPool>,
    sorting_params: web::Query<SortedInfo>
) -> HttpResponse {

    let query = format!(
        "SELECT book_view.*, '' as img FROM book_view WHERE genre_name = {} ORDER BY {} DESC",
        sorting_params.genre,
        sorting_params.parameter
    );

    match get_books_from_db(pool, query).await {
        Ok(books) => {
            log::info!("All books have been fetched");
            HttpResponse::Ok().json(books)
        }
        Err(e) => {
            log::error!("Failed to fetch books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
