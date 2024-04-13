use actix_web::{ HttpResponse, Scope, web };
use jsonwebtoken::{
    encode,
    decode,
    Algorithm,
    Validation,
    Header,
    EncodingKey,
    DecodingKey,
    TokenData,
    errors::Error as JwtError,
};
use serde::{ Serialize, Deserialize };
use chrono::{ Utc, Duration };
use crate::extractors::authtoken::{ Claims, AuthenticationToken };


#[derive(Serialize, Deserialize)]
pub struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct EncodeResponse {
    message: String,
    token: String,
}

pub async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
	&Header::default(),
	&claims,
	&EncodingKey::from_secret(secret.as_str().as_ref()),
    ).unwrap();
    HttpResponse::Ok().json(EncodeResponse {
	message: "Successfully created account.".to_owned(),
	token,
    })
}

#[derive(Serialize, Deserialize)]
pub struct DecodeResponse {
    message: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    token: String
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
	Err(e) => HttpResponse::Unauthorized().json(Response { message: e.to_string() }),
    }
}

pub async fn protected_route(auth_token: AuthenticationToken) -> HttpResponse {
    println!("{:#?}", auth_token);
    HttpResponse::Ok().json(Response { message: "Authorized".to_owned() })
}
