use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
 
pub struct CommentData {
    comment_text: String,
    comment_author: String, 
    commented_book: String,
}
pub async fn new_comment(form: web::Form<CommentData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new Comment details in the database");
    match sqlx::query!(
        "
                    INSERT INTO comments_table (comment_id, comment_text, comment_author, commented_book)
                    VALUES ($1, $2, $3, $4)
                    ",
        Uuid::new_v4(),
        form.comment_text,
        form.comment_author,
        form.commented_book,
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("New comment has been created");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all_comments(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        "SELECT comment_id, comment_text, comment_author, commented_book FROM comments_table"
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(comments) => {
            log::info!("All comments have been fetched");
            for comment in &comments {
                println!("Comment ID: {}, Text: {}, Author: {}, Book: {}", 
                    comment.comment_id, comment.comment_text, comment.comment_author, comment.commented_book);
            }
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to fetch comments: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}