use actix_web::{web, App, HttpResponse, HttpServer};
 
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
