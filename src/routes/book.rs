use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::fs;

const IMAGE_DIRECTORY: &str = "images/";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BookData {
    name: Option<String>,
    genre_id: Option<i32>,
    genre_name: Option<String>,
    author: Option<String>,
    cost: Option<f64>,
    score: Option<f64>,
    downloads: Option<i32>,
    file_name: Option<String>,
    img_name: Option<String>,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Info {
    file_name: String,
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
        form.author,
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

pub async fn get_sorted_books(pool: web::Data<PgPool>) -> HttpResponse {
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

async fn get_books_from_db(pool: web::Data<PgPool>) -> Result<Vec<BookData>, sqlx::Error> {
    sqlx::query_as!(BookData, "SELECT * FROM book_view")
        .fetch_all(pool.as_ref())
        .await
}

fn create_reduced_info_json(books: Vec<BookData>) -> Vec<Value> {
    let mut json_vec = vec![];
    for book in books {
        let image_path = format!("{}{}", IMAGE_DIRECTORY, book.img_name.unwrap());
        let json_book = json!({
            "name": book.name,
            "genre_name": book.genre_name,
            "author": book.author,
            "cost": book.cost,
            "score": book.score,
            "downloads": book.downloads,
            "img": image_base64::to_base64(&image_path)
        });
        json_vec.push(json_book);
    }
    json_vec
}

pub async fn get_book_file(
    pool: web::Data<PgPool>,
    data: web::Query<Info>,
) -> HttpResponse {
    let query = sqlx::query!(
        "Select file FROM book_files WHERE file_name = $1",
        data.file_name
    )
    .fetch_one(pool.as_ref())
    .await;
    match query {
        Ok(result) => {
            log::info!("File has been fetched");
            HttpResponse::Ok().body(result.file.unwrap())
        }
        Err(e) => {
            log::info!("Failed to fetch file {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}