use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const JWT_SECRET_KEY: &str = "my-db-web-jwt-key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户ID
    pub user_id: u32,
    /// 用户名
    pub username: String,
    /// 用户权限
    pub flag: String,
    /// 过期时间
    pub exp: i64,
    /// 签发时间
    pub iat: i64
}

impl Claims {
    pub fn new(user_id: u32, username: String, flag: String) -> Self {
        let now = Utc::now();

        Claims {
            user_id,
            username,
            flag,
            exp: (now + Duration::minutes(15)).timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| JWT_SECRET_KEY.into());

    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| JWT_SECRET_KEY.into());

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map(|data| data.claims)
}