use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token(
    user_id: &str,
)-> Result<String, jsonwebtoken::errors::Error> {

    let key_var = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let key = EncodingKey::from_secret(key_var.as_ref());

    let now = Utc::now();
    let exp = now + Duration::hours(1);
    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &key)?;

    Ok(token)
}