use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BookData {
    name: Option<String>,
    genre_id: i32,
    author: Uuid,
    cost: Option<f64>,
    score: Option<f64>,
    downloads: Option<i32>,
    file_name: Option<String>,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn new_book(form: web::Form<BookData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new book details in the database");
    match sqlx::query!(
        "
        INSERT INTO book_table (name, genre_id, author, cost, score, downloads, file_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ",
        form.name,
        form.genre_id,
        &form.author,
        form.cost,
        form.score,
        form.downloads,
        form.file_name,
        Utc::now(),
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("New book has been created");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all_books(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        BookData,
        "SELECT name, genre_id, author, cost, score, downloads, file_name, created_at, updated_at FROM book_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => {
            log::info!("All books have been fetched");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&books).unwrap())
        }
        Err(e) => {
            log::error!("Failed to fetch books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_sorted_books(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        BookData,
        "SELECT name, genre_id, author, cost, score, downloads, file_name, created_at, updated_at FROM book_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => {
            log::info!("All books have been fetched");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&books).unwrap())
        }
        Err(e) => {
            log::error!("Failed to fetch books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
