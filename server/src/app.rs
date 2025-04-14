use super::auth;
use super::files;
use super::middleware;

use axum::error_handling::HandleErrorLayer;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::{Extension, Json, Router};
use tower::buffer::BufferLayer;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};

pub async fn create_app() -> Result<Router, String> {
    // ### Setup ### //
    files::setup().await.map_err(|err| err.to_string())?;

    // ### Configs ### //
    let jwt_config = auth::JwtConfig::from_env();
    let psw_config = auth::PasswordConfig::from_env();

    // ### Routers ### //
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

    // ### Services and general middleware layers ### //
    // Create a service to serve static files from the `assets` folder.
    let serve_dir =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/not_found.html"));

    // With fallback_service we serve assets directly from the root
    // 1. Add a Content-Security-Policy header to all responses.
    // 2. Add a rate limiter to all requests.
    // 3. Add a timeout to all requests.
    Ok(api_routes.fallback_service(serve_dir).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(middleware::handle_error)) // Handle later layer errors
            .layer(BufferLayer::new(1024))
            .layer(middleware::content_security_policy_layer()) // <- 1
            .layer(middleware::rate_limiting_layer()) // <- 2
            .layer(middleware::request_timeout_layer()), // <- 3
    ))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
