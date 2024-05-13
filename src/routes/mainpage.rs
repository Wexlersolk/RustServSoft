use actix_web::{
    web::{self, Bytes},
    HttpResponse,
};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::PgPool;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct GenresData{
    pub genre_name: Option<String>,
    
}

pub async fn get_all_genres(pool: web::Data<PgPool>) -> HttpResponse {
    let query = sqlx::query_as!(
    GenresData,
    "Select genre_name FROM genre_table ")
        .fetch_all(pool.as_ref())
        .await;
    match query {
        Ok(genres) => {
            log::info!("Genres have been fetched");
        HttpResponse::Ok().json(serde_json::to_value(&genres).unwrap())
}
        Err(e) => {
            log::info!("Failed to fetch genres {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}



