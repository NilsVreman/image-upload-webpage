use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject
    exp: usize,  // expiration time (unix timestamp)
}

#[derive(Clone)]
pub struct JwtConfig {
    secret: String,
}

impl JwtConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
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

pub fn validate_jwt(token: &str, jwt_config: &JwtConfig) -> Result<Claims, JwtError> {
    let token_data = jwt::decode::<Claims>(
        token,
        &jwt::DecodingKey::from_secret(jwt_config.secret.as_bytes()),
        &jwt::Validation::default(),
    )?;

    Ok(token_data.claims)
}
