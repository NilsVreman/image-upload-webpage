use dotenv::dotenv;
use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize, // expiration time (unix timestamp)
}

#[derive(Clone)]
pub struct JwtConfig {
    secret: String,
    pub expiration_secs: i64,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            expiration_secs: env::var("JWT_EXPIRATION_TIME")
                .and_then(|var| var.parse::<i64>().map_err(|_| env::VarError::NotPresent))
                .expect("JWT_EXPIRATION_TIME must be set"),
        }
    }
}

pub fn create_jwt(jwt_config: &JwtConfig) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(jwt_config.expiration_secs))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { exp: expiration };

    jwt::encode(
        &jwt::Header::default(),
        &claims,
        &jwt::EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
}

pub fn validate_jwt(token: &str, jwt_config: &JwtConfig) -> Result<Claims, JwtError> {
    let token_data = jwt::decode::<Claims>(
        token,
        &jwt::DecodingKey::from_secret(jwt_config.secret.as_bytes()),
        &jwt::Validation::default(),
    )?;

    Ok(token_data.claims)
}
