use axum::{http::StatusCode, Extension, Json};
use bcrypt::verify;
use serde::Deserialize;

use super::{jwt, AuthConfig};

const EXPIRATION_TIME: i64 = 60;

#[derive(Clone)]
pub struct PasswordConfig {
    secret: String,
}

impl PasswordConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    Extension(config): Extension<AuthConfig>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match verify(&login_req.password, &config.pwd_cfg.secret) {
        Ok(false) | Err(_) => Err(StatusCode::UNAUTHORIZED)?,
        _ => (),
    }

    let token = jwt::create_jwt(&login_req.username, &config.jwt_cfg, EXPIRATION_TIME)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({ "jwt": token })))
}
