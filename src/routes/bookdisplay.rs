use crate::routes::*;

use actix_web::{
    web::{self},
    HttpResponse,
};
use sqlx::PgPool;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SortedInfo {
    genre: String,
    parameter: String,
}

pub async fn get_all_books(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(BookData, "SELECT book_view.*, '' as img FROM book_view",)
        .fetch_all(pool.as_ref())
        .await
    {
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
    sorting_params: web::Query<SortedInfo>,
) -> HttpResponse {
    match sqlx::query_as!(
        BookData,
        "SELECT book_view.*, '' as img FROM book_view WHERE genre_name = $1 ORDER BY $2 DESC",
        sorting_params.genre,
        sorting_params.parameter
    )
    .fetch_all(pool.as_ref())
    .await
    {
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
