use actix_web::{
    web::{self, Bytes},
    HttpResponse,
};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{fs::File, io::Write};

const IMAGE_DIRECTORY: &str = "images/";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Info {
    file_name: String,
}
#[derive(serde::Deserialize, serde::Serialize)]

pub struct BookData {
    pub name: Option<String>,
    pub genre_name: Option<String>,
    pub author: Option<String>,
    pub cost: Option<f64>,
    pub score: Option<f64>,
    pub downloads: Option<i32>,
    pub img_name: Option<String>,
    pub img: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn new_book(data: web::Json<BookData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new book details in the database");
    let genre_id = match sqlx::query!(
        "Select * FROM genre_table WHERE genre_name = $1",
        data.genre_name
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(result) => {
            log::info!("genre_id has been fetched");
            result.genre_id
        }
        Err(e) => {
            log::info!("Failed to fetch genre_id: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let image_bytes = image_base64::from_base64(data.img.clone().unwrap());
    let image_path = format!("{}{}", IMAGE_DIRECTORY, data.img_name.as_ref().unwrap());
    let mut image = File::create(image_path).unwrap();
    image.write_all(&image_bytes).unwrap();
    match sqlx::query!(
        "
        INSERT INTO book_table (name, genre_id, author, cost, score, downloads, img_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8,$9)
        ",
        data.name,
        genre_id,
        data.author,
        data.cost,
        data.score,
        data.downloads,
        data.img_name,
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
    sqlx::query_as!(BookData, "SELECT book_view.*, '' as img FROM book_view")
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

pub async fn get_book_file(pool: web::Data<PgPool>, data: web::Query<Info>) -> HttpResponse {
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

pub async fn upload_file(
    file: Bytes,
    pool: web::Data<PgPool>,
    data: web::Query<Info>,
) -> HttpResponse {
    let query = sqlx::query!(
        "INSERT INTO book_table (file_name, file) VALUES ($1, $2)",
        &data.file_name,
        &file[..]
    )
    .execute(pool.as_ref())
    .await;
    match query {
        Ok(_) => {
            log::info!("File has been uploaded");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::info!("Upload failed due to {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
