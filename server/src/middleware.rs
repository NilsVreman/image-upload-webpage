use axum::{
    extract::{Request, State},
    http::{header::CONTENT_SECURITY_POLICY, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use tower_http::set_header::SetResponseHeaderLayer;

use super::auth;

const CSP_HEADER_VALUE: &str =
    "default-src 'self'; script-src 'self'; img-src 'self'; style-src 'self';";

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
