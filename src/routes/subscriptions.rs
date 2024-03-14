use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserData {
    login: String,
    password: String,
    access_id: i32,
}
pub async fn subscribe(form: web::Form<UserData>, pool: web::Data<PgPool>) -> HttpResponse {
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
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
