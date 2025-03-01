use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Extension, Json};
use axum::{routing::post, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use bcrypt::verify;
use serde::Deserialize;

use super::{jwt, validate_jwt, AuthConfig};

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

pub struct SessionResponse {
    status: StatusCode,
    valid: bool,
}

impl IntoResponse for SessionResponse {
    fn into_response(self) -> Response<Body> {
        (self.status, Json(serde_json::json!({"valid": self.valid}))).into_response()
    }
}

pub fn create_authorisation_router() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/check-session", post(check_session_handler))
}

/// Validate the session token
async fn login_handler(
    Extension(config): Extension<AuthConfig>,
    jar: CookieJar,
    Json(login_req): Json<LoginRequest>,
) -> Result<CookieJar, StatusCode> {
    if !verify(&login_req.password, &config.pwd_cfg.secret).unwrap_or(false) {
        Err(StatusCode::UNAUTHORIZED)?;
    };

    let token = jwt::create_jwt(&login_req.username, &config.jwt_cfg)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::parse(format!(
        "token={}; HttpOnly; Max-Age=3600; SameSite=None; Secure",
        token
    ))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(jar.add(cookie))
}

/// Validate the session token
pub async fn check_session_handler(
    jar: CookieJar,
    Extension(auth_config): Extension<AuthConfig>,
) -> SessionResponse {
    if let Some(cookie) = jar.get("token") {
        let token = cookie.value();
        return match validate_jwt(token, &auth_config.jwt_cfg) {
            Ok(_) => SessionResponse {
                status: StatusCode::OK,
                valid: true,
            },
            Err(_) => SessionResponse {
                status: StatusCode::UNAUTHORIZED,
                valid: false,
            },
        };
    }

    SessionResponse {
        status: StatusCode::UNAUTHORIZED,
        valid: false,
    }
}
