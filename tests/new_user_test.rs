use actix_web::{web};
use chrono::Utc;
use sqlx::{PgPool};
use letsgetrusty::routes::user::{new_user, UserData};

#[tokio::test]
async fn test_new_user() {

    let test_data = UserData {
        login: Some("karina7".to_string()),
        password: Some("who im".to_string()),
        email: Some("email7@gmail.com".to_string()),
        access_id: Some(1),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    let request_payload = web::Json(test_data);

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
    let pool = PgPool::connect(&database_url).await.expect("error");
    let secret = web::Data::new("test_secret".to_string());

    let response = new_user(request_payload, web::Data::new(pool.clone()), secret).await;

    assert_eq!(response.status(), actix_web::http::StatusCode::OK);

    let login = "karina7";
    let _ = sqlx::query!("DELETE FROM user_table WHERE login = $1", login)
    .execute(&pool) 
    .await;       

}
