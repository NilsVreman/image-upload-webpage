use super::auth;
use super::files;
use super::middleware;
use axum::Extension;
use axum::{routing::get, Json, Router};

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;
    let auth_config = auth::setup();

    let app_router = Router::new()
        .route("/health", get(health_handler))
        .nest("/images", files::create_image_router())
        .layer(middleware::cors_layer());
    let login_router = Router::new()
        .nest("/login", auth::create_authorisation_router())
        .layer(Extension(auth_config));

    Ok(login_router.nest("/api", app_router))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
