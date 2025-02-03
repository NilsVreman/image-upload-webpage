use axum::response::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use axum::{routing::post, Router};
use bcrypt::verify;
use hyper::header::SET_COOKIE;
use serde::Deserialize;

use super::{jwt, AuthConfig};

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

pub fn create_authorisation_router() -> Router {
    Router::new().route("/login", post(login_handler))
}

async fn login_handler(
    Extension(config): Extension<AuthConfig>,
    Json(login_req): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if !verify(&login_req.password, &config.pwd_cfg.secret).unwrap_or(false) {
        Err(StatusCode::UNAUTHORIZED)?;
    };

    let token = jwt::create_jwt(&login_req.username, &config.jwt_cfg)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie_value = format!(
        "token={}; HttpOnly; Max-Age=3600; SameSite=None; Secure",
        token
    );

    Ok((
        StatusCode::OK,
        // Return a tuple of (header-name, header-value)
        [(SET_COOKIE, cookie_value)],
        Json(serde_json::json!({ "message": "Logged in successfully!"})),
    ))
}
