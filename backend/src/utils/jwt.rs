use actix_web::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::Error as JwtError,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

static SECRET: Lazy<String> =
    Lazy::new(|| env::var("SECRET").expect("SECRET must be set in environment"));

pub fn encode_jwt(claims: &Claims) -> Result<String, JwtError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    )
}

/// Mengecek apakah token sudah expired berdasarkan `exp` dalam UNIX timestamp
pub fn is_jwt_expired(exp: usize) -> bool {
    let now = Utc::now().timestamp() as usize;
    exp < now
}

pub fn create_auth_cookie(token: &str) -> Cookie<'_> {
    Cookie::build("auth_token", token.to_string())
        .http_only(true)
        //.secure(true) // hanya kirim di HTTPS
        .same_site(SameSite::Strict)
        .path("/")
        .finish()
}

pub fn create_csrf_cookie(csrf_token: &str) -> Cookie<'_> {
    Cookie::build("csrf_token", csrf_token.to_string())
        .http_only(false) // agar bisa dibaca JS dan dikirim manual ke header
        //.secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .finish()
}

use rand::distr::Alphanumeric;
use rand::{Rng, rng};

pub fn generate_csrf_token() -> String {
    rng()
        .sample_iter(Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
