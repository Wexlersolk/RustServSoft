use crate::routes::*;


use actix_web::{
    web::{self, Bytes},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;

pub async fn get_books_from_db(pool: web::Data<PgPool>) -> Result<Vec<BookData>, sqlx::Error> {
    sqlx::query_as!(BookData, "SELECT book_view.*, '' as img FROM book_view")
        .fetch_all(pool.as_ref())
        .await
}

pub async fn get_all_books(pool: web::Data<PgPool>) -> HttpResponse {
    match get_books_from_db(pool).await {
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
    sorting_params: web::Query<(String,)>,
) -> HttpResponse {
    let parameter = &sorting_params.0 .0; // Access the inner value of the tuple

    let safe_parameter = match parameter.as_str() {
        "cost" | "score" | "downloads" => parameter,
        _ => {
            log::error!("Invalid sorting parameter: {:?}", sorting_params);
            return HttpResponse::BadRequest().finish();
        }
    };

    let query = format!(
        "SELECT book_view.*, '' as img FROM book_view ORDER BY {} DESC",
        safe_parameter
    );

    match get_books_from_db(pool).await {
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

