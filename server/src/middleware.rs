use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::{header, Method};
use tower_http::cors::CorsLayer;

use super::auth;

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ])
        .allow_credentials(true)
}

enum AuthError {
    InvalidToken,
    MissingToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token").into_response(),
        }
    }
}

pub async fn auth_middleware(
    State(auth_config): State<auth::AuthConfig>,
    req: Request,
    next: Next,
) -> Response {
    let token = match extract_cookie_token(&req) {
        Ok(token) => token,
        Err(e) => return e.into_response().into(),
    };

    // Validate the JWT token and run request if valid
    match auth::validate_jwt(token, &auth_config.jwt_cfg) {
        Ok(_claims) => next.run(req).await,
        _ => AuthError::InvalidToken.into_response().into(),
    }
}

fn extract_cookie_token(req: &Request) -> Result<&str, AuthError> {
    if let Some(cookie) = req
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
    {
        return cookie
            .split(';')
            .find(|&c| c.trim().starts_with("token="))
            .map(|c| c.trim_start_matches("token="))
            .ok_or_else(|| AuthError::MissingToken);
    }
    Err(AuthError::MissingToken)
}
