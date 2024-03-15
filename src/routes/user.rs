use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
 
pub struct UserData {
    login: String,
    password: String,
    access_id: i32,
}
pub async fn new_user(form: web::Form<UserData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new subscriber details in the database");
    match sqlx::query!(
        "
                    INSERT INTO user_table (user_id, login, password, access_id)
                    VALUES ($1, $2, $3, $4)
                    ",
        Uuid::new_v4(),
        form.login,
        form.password,
        form.access_id
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("New user has been created");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all_users(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        "SELECT user_id, login, access_id, created_at, updated_at FROM user_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(users) => {
            log::info!("All users have been fetched");
            for user in &users {
                println!("User ID: {}, Login: {}, Access ID: {}", 
                    user.user_id, user.login, user.access_id);
            }
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
