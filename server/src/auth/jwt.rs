use std::env;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken as jwt;
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject
    exp: usize,  // expiration time (unix timestamp)
}

pub struct JwtConfig {
    hash: String,
    secret: String,
}

pub fn setup() -> JwtConfig {
    dotenv().ok();
    JwtConfig {
        hash: env::var("SHARED_SECRET_HASH").expect("SHARED_SECRET_HASH must be set"),
        secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    }
}

pub fn create_jwt(subject: &str, jwt_config: &JwtConfig, exp_min: i64) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(exp_min))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: subject.to_string(),
        exp: expiration,
    };

    jwt::encode(
        &jwt::Header::default(),
        &claims,
        &jwt::EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str, jwt_config: &JwtConfig) -> Result<Claims, JwtError> {
    let validation = jwt::Validation::default();
    let token_data = jwt::decode::<Claims>(
        token,
        &jwt::DecodingKey::from_secret(jwt_config.secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
