use super::auth;
use super::config;
use super::files;
use super::middleware;

use axum::middleware::from_fn_with_state;
use axum::Extension;
use axum::{routing::get, Json, Router};
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;

    let jwt_config = auth::JwtConfig::from_env();
    let psw_config = auth::PasswordConfig::from_env();
    let general_config = config::GeneralConfig::from_env();

    let public_routes = Router::new().route("/health", get(health_handler));
    let authenticated_routes = Router::new()
        .merge(files::create_image_router())
        .layer(from_fn_with_state(
            jwt_config.clone(),
            middleware::auth_middleware,
        ))
        .layer(Extension(general_config.clone()));
    let authorisation_routes = auth::create_authorisation_router()
        .layer(Extension(psw_config.clone()))
        .layer(Extension(jwt_config.clone()));

    let api_routes = authorisation_routes
        .merge(public_routes)
        .merge(authenticated_routes)
        .layer(middleware::cors_middleware(general_config.clone()));

    // Create a service to serve static files from the `dist` folder.
    let serve_dir =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/not_found.html"));

    Ok(api_routes.fallback_service(serve_dir))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
