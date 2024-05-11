use letsgetrusty::routes::book::{new_book, BookData};
use actix_web::{ web};
use sqlx::{PgPool};
use chrono::Utc;
use std::fs::File;
use std::io::Read;
use base64::encode;

#[tokio::test]
async fn test_new_book() {
    let image_path = "images/test_image.jpg";

    let mut file = File::open(image_path).expect("Failed to open image file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read image file");

    let encoded_image = encode(&buffer);

    let book_data = BookData {
        name: Some("Test Book".to_string()),
        genre_name: Some("Classic".to_string()),
        author: Some("user1".to_string()),
        cost: Some(10.99),
        score: Some(4.5),
        downloads: Some(100),
        img_name: Some("test_image.jpg".to_string()),
        img: Some(format!("data:image/jpeg;base64,{}", encoded_image)),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    dotenv::dotenv().ok(); 


    let db_user = std::env::var("DB_USER").expect("DB_USER not found");
    let password = std::env::var("PASSWORD").expect("PASSWORD not found");
    let host = std::env::var("HOST").expect("HOST not found");
    let port = std::env::var("DB_PORT").expect("DB_PORT not found");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME not found");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable",
        db_user, password, host, port, db_name
    );
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    let request_payload = web::Json(book_data);

    let response = new_book(request_payload, web::Data::new(pool.clone())).await;

    assert_eq!(response.status(), actix_web::http::StatusCode::OK);

    let book_name = "Test Book";
    let _ = sqlx::query!("DELETE FROM book_table WHERE name = $1", book_name)
        .execute(&pool)
        .await
        .expect("Failed to delete book");
}

