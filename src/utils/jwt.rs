use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Uuid,
    pub exp: usize,
}

pub fn generate_token(user_id: Uuid) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");

    let claims = Claims {
        sub: issuer,
        user_id,
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    // Middleware untuk mendapat Claims
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    return match decode_token(token) {
                        Ok(claims) => ready(Ok(claims)),
                        Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
                    };
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header")))
    }
}
