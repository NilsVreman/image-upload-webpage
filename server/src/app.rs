use super::auth;
use super::files;
use super::middleware;
use axum::middleware::from_fn_with_state;
use axum::Extension;
use axum::{routing::get, Json, Router};

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;
    let auth_config = auth::setup();

    let public_routes = Router::new().route("/health", get(health_handler));
    let authenticated_routes =
        Router::new()
            .merge(files::create_image_router())
            .layer(from_fn_with_state(
                auth_config.clone(),
                middleware::auth_middleware,
            ));
    let authorisation_routes =
        auth::create_authorisation_router().layer(Extension(auth_config.clone()));

    Ok(authorisation_routes
        .merge(public_routes)
        .merge(authenticated_routes)
        .layer(middleware::cors_middleware()))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
