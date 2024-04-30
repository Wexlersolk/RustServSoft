use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;

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
    name: String,
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
            let mut json_vec = vec![];
            for book in books {
                let json_book = json!({
                    "name": book.name,
                    "genre_name":book.genre_name,
                    "author": book.author,
                    "cost": book.cost,
                    "score": book.score,
                    "downloads": book.downloads,
                });
                json_vec.push(json_book)
            }
            HttpResponse::Ok().json(json_vec)
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
            let mut json_vec = vec![];
            for book in books {
                let json_book = json!({
                    "name": book.name,
                    "genre_name":book.genre_name,
                    "author": book.author,
                    "cost": book.cost,
                    "score": book.score,
                    "downloads": book.downloads,
                });
                json_vec.push(json_book)
            }
            HttpResponse::Ok().json(json_vec)
        }
        Err(e) => {
            log::error!("Failed to fetch books: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_books_from_db(pool: web::Data<PgPool>) -> Result<Vec<BookData>, sqlx::Error> {
    sqlx::query_as!(
        BookData,
        "SELECT name, book_table.genre_id, genre_name, author, cost, score, downloads, file_name, img_name, created_at, updated_at FROM book_table JOIN genre_table ON book_table.genre_id = genre_table.genre_id"
    )
    .fetch_all(pool.as_ref())
    .await
}

pub async fn get_book_image(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    data: web::Json<Info>,
) -> HttpResponse {
    let file_path = match sqlx::query!("SELECT img_name FROM book_table WHERE name = $1", data.name)
        .fetch_one(pool.as_ref())
        .await
    {
        Ok(path) => {
            log::info!("image path has been fetched");
            let path = path.img_name.clone().unwrap();
            format!("{}{}", IMAGE_DIRECTORY, path)
        }
        Err(e) => {
            log::error!("Failed to fetch path: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let file = match actix_files::NamedFile::open_async(file_path).await {
        Ok(file) => {
            log::info!("image has been fetched");
            file
        }
        Err(e) => {
            log::error!("Failed to fetch image: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    file.into_response(&req)
}
