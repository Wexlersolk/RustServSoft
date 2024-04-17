use std::future::{ Ready, ready };
use uuid::Uuid;
use actix_web::{
    web,
    FromRequest,
    HttpRequest,
    http::header::HeaderValue,
    dev::Payload,
    Error as ActixWebError,
    error::ErrorUnauthorized,
};
use serde::{ Serialize, Deserialize };
use jsonwebtoken::{
    TokenData,
    Algorithm,
    Validation,
    DecodingKey,
    errors::Error as JwtError,
    decode,
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: Uuid,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
	let req = req.clone();

	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);

	// No Header was sent
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }

	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();

	// Couldn't convert Header::Authorization to String
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has foreign chars!"))) }

	// TODO put secret in app_state
        let secret: &str = &req.app_data::<web::Data<String>>().unwrap();
	// let secret: &str = "secret";

	let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
	    &authentication_token,
	    &DecodingKey::from_secret(secret.as_ref()),
	    &Validation::new(Algorithm::HS256),
	);

	match token_result {
	    Ok(token) => ready(Ok(AuthenticationToken { id: token.claims.id })),
	    Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
	}
    }
}

