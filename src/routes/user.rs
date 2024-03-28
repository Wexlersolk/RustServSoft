use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use chrono::Utc;
use serde_json::json;
use sha256::digest;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]

pub struct UserData {
    login: String,
    password: String,
    access_id: i32,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn new_user(form: web::Form<UserData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new subscriber details in the database");
    let user_id = Uuid::new_v4();
    match sqlx::query!(
        "
        INSERT INTO user_table (user_id, login, password)
        VALUES ($1, $2, $3)
        ",
        user_id,
        &form.login,
        digest(form.password.trim()),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("New user has been created");
            HttpResponse::Ok().body(format!("{}", user_id))
        }
        Err(e) => {
            log::error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to create user: {}", e))
        }
    }
}

pub async fn update_user(
    form: web::Form<UserData>,
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user_id = req.match_info().get("user_id").unwrap();
    let user_id = Uuid::parse_str(user_id).unwrap();
    match sqlx::query!(
        "
        UPDATE user_table
        SET login = $2, password = $3
        WHERE user_id = $1
        ",
        user_id,
        form.login,
        digest(&form.password),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("User has been updated");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to update user: {}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}

pub async fn get_all_users(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        UserData,
        "SELECT login, password, access_id, created_at, updated_at FROM user_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(users) => {
            log::info!("All users have been fetched");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&users).unwrap())
        }
        Err(e) => {
            log::error!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

pub async fn get_user(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = req.match_info().get("user_id").unwrap();
    let user_id = Uuid::parse_str(user_id).unwrap();
    match sqlx::query!(
        "SELECT login, access_id FROM user_table WHERE user_id = $1",
        user_id
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(user) => {
            log::info!("One user has been fetched");
            let user_data = json!({
                "login": user.login,
                "access_id": user.access_id
            });
            HttpResponse::Ok().json(user_data)
        }
        Err(e) => {
            log::error!("Failed to fetch user: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

pub async fn get_user_id(form: web::Form<UserData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        "SELECT user_id, access_id FROM user_table WHERE login = $1 AND password = $2",
        form.login,
        digest(form.password.trim())
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(user) => {
            log::info!("User has been fetched");
            let user_data = json!({
                "user_id": user.user_id,
                "access_id": user.access_id
            });
            HttpResponse::Ok().json(user_data)
        }
        Err(e) => {
            log::error!("Failed to fetch user: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

pub async fn delete_user(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = req.match_info().get("user_id").unwrap();
    let user_id = Uuid::parse_str(user_id).unwrap();
    match sqlx::query!("DELETE FROM user_table WHERE user_id = $1", user_id)
        .execute(pool.as_ref())
        .await
    {
        Ok(_) => {
            log::info!("User has been deleted");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to delete user: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}
