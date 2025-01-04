use axum::http::HeaderValue;
use hyper::{header, Method};
use tower_http::cors::CorsLayer;

use super::auth;

pub fn cors_layer() -> CorsLayer {
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
