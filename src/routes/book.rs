use actix_web::{http::header::ContentType, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BookData {
    name: String,
    author: String,
    scores: i32,
    cost: i32,
    file_name: String,
}
pub async fn new_book(form: web::Form<BookData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new book details in the database");
    match sqlx::query!(
        "
                    INSERT INTO book_table (book_id, name, author, scores, cost, file_name)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ",
        Uuid::new_v4(),
        form.name,
        form.author,
        form.scores,
        form.cost,
        form.file_name
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
        "SELECT name, author, scores, cost, file_name FROM book_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => {
            log::info!("All book have been fetched");
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
