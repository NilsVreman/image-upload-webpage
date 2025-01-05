use axum::{
    body::Body,
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
    let token = match extract_jwt(&req) {
        Some(t) => t,
        None => return AuthError::MissingToken.into_response().into(),
    };

    // Validate the JWT token
    match auth::validate_jwt(token, &auth_config.jwt_cfg) {
        Ok(_claims) => next.run(req).await,
        _ => AuthError::InvalidToken.into_response().into(),
    }
}

fn extract_jwt(req: &Request<Body>) -> Option<&str> {
    match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        Some(bearer) if bearer.starts_with("Bearer ") => bearer.strip_prefix("Bearer "),
        _ => None,
    }
}
