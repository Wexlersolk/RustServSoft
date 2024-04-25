use crate::extractors::authtoken::Claims;
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

pub async fn encode_token(ida: Uuid, secret: web::Data<String>) -> String {
    let id: Uuid = ida;
    let exp: usize = (Utc::now() + Duration::minutes(1)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    token
}

#[derive(Serialize, Deserialize)]
pub struct DecodeResponse {
    message: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    token: String,
}

pub async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
    let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_result {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Successfully logged in.".to_owned(),
            id: token.claims.id,
        }),
        Err(e) => HttpResponse::Unauthorized().json(Response {
            message: e.to_string(),
        }),
    }
}
