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
pub async fn subscribe(form: web::Form<UserData>, pool: web::Data<PgPool>) -> HttpResponse {
            HttpResponse::Ok().finish()
}
