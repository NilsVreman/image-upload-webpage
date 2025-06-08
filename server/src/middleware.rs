use std::time::Duration;

use axum::{
    extract::{Request, State},
    http::{header::CONTENT_SECURITY_POLICY, HeaderValue, Method, StatusCode, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
    BoxError,
};
use axum_extra::extract::CookieJar;
use tower::{limit::RateLimitLayer, timeout::TimeoutLayer};
use tower_http::set_header::SetResponseHeaderLayer;

use super::auth;

const CSP_HEADER_VALUE: &str = "script-src 'self'; img-src 'self';";
const REQUESTS_PER_SECOND: u64 = 5;
const REQUEST_TIMEOUT_SECS: u64 = 2;

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
    State(jwt_config): State<auth::JwtConfig>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Response {
    let token = match jar.get("token") {
        Some(cookie) => cookie.value(),
        None => return AuthError::MissingToken.into_response(),
    };

    // Validate the JWT token and run request if valid
    match auth::validate_jwt(token, &jwt_config) {
        Ok(_claims) => next.run(req).await,
        _ => AuthError::InvalidToken.into_response(),
    }
}

pub fn content_security_policy_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(CSP_HEADER_VALUE).expect("Invalid CSP header"),
    )
}

pub async fn handle_error(_method: Method, _uri: Uri, err: BoxError) -> (StatusCode, String) {
    match err {
        err if err.is::<tower::timeout::error::Elapsed>() => {
            (StatusCode::REQUEST_TIMEOUT, "Request timed out".to_string())
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled error: {}", err),
        ),
    }
}

pub fn rate_limiting_layer() -> RateLimitLayer {
    RateLimitLayer::new(REQUESTS_PER_SECOND, Duration::from_secs(1))
}

pub fn request_timeout_layer() -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(REQUEST_TIMEOUT_SECS))
}
