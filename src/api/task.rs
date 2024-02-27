use actix_web::{
    error::ResponceError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponce,
};

use serde::{Deserialize, Serialize};
use derive_more::Display;
 
#[derive (Deserialize, Serialize)] 
pub struct BookIdentifier {
    book_id: String,
    
}
 
 
#[get("/book/{book_id}")] 
pub async fn get_book(book_identifier: Path<BookIdentifier>) -> Json<String> {
    return Json(book_identifier.into_inner().book_id);
} 
