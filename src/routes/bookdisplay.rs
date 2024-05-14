use crate::routes::*;

use actix_web::{
    web::{self},
    HttpResponse,
};
use serde_json::json;
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

pub async fn get_popular_books(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        BookData,
        "SELECT book_view.*, '' as img FROM book_view ORDER BY downloads DESC LIMIT 10;",
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => {
            log::info!("All books have been fetched");
            let mut book_json = vec![];
            for book in books {
                let image_path = format!("{}{}", IMAGE_DIRECTORY, book.img_name.unwrap());
                let json_book = json!({
                    "id": book.book_id,
                    "name": book.name,
                    "img": image_base64::to_base64(&image_path)
                });
                book_json.push(json_book);
            }
            HttpResponse::Ok().json(book_json)
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
    let result = match sorting_params.parameter.as_str() {
        "cost" => {
        sqlx::query_as!(
            BookData,
            "SELECT book_view.*, '' as img FROM book_view WHERE genre_name = $1 ORDER BY cost DESC",
            sorting_params.genre,
        )
        .fetch_all(pool.as_ref()).await
        }
        "downloads" => {
            sqlx::query_as!(
            BookData,
            "SELECT book_view.*, '' as img FROM book_view WHERE genre_name = $1 ORDER BY downloads DESC",
            sorting_params.genre,
        ).fetch_all(pool.as_ref()).await
    },
        "score" => {
        sqlx::query_as!(
            BookData,
            "SELECT book_view.*, '' as img FROM book_view WHERE genre_name = $1 ORDER BY score DESC",
            sorting_params.genre,
        ).fetch_all(pool.as_ref()).await},
        _ => {return HttpResponse::BadRequest().finish();}
    };
    match result {
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
