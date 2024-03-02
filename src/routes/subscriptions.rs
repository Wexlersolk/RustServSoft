use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserData {
    user_id: String,
    login: String,
    password: String,
    access_id: String,
}
pub async fn subscribe(
    form: web::Form<UserData>,
    pool: web::Data<PgPool>, 
) -> HttpResponse {
    match sqlx::query!(
                    r#"
                    INSERT INTO user_table (user_id, login, password, access_id)
                    VALUES ($1, $2, $3, $4)
                    "#
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
