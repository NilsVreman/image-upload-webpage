use super::auth;
use super::files;
use super::middleware;

use axum::error_handling::HandleErrorLayer;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::{BoxError, Extension, Json, Router};
use hyper::StatusCode;
use tower::buffer::BufferLayer;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;

    let jwt_config = auth::JwtConfig::from_env();
    let psw_config = auth::PasswordConfig::from_env();

    let public_routes = Router::new().route("/health", get(health_handler));
    let authenticated_routes =
        Router::new()
            .merge(files::create_image_router())
            .layer(from_fn_with_state(
                jwt_config.clone(),
                middleware::auth_middleware,
            ));
    let authorisation_routes = auth::create_authorisation_router()
        .layer(Extension(psw_config.clone()))
        .layer(Extension(jwt_config.clone()));

    let api_routes = authorisation_routes
        .merge(public_routes)
        .merge(authenticated_routes);

    // Create a service to serve static files from the `assets` folder.
    let serve_dir =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/not_found.html"));

    // With fallback_service we serve assets directly from the root
    // 1. Add a Content-Security-Policy header to all responses.
    // 2. Add a rate limiter to all requests.
    Ok(api_routes
        .fallback_service(serve_dir)
        .layer(middleware::content_security_policy_layer())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(middleware::rate_limiting_layer()),
        ))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
