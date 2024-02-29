use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
 
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

