use super::files;
use super::middleware;
use axum::{routing::get, Json, Router};

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;

    let app_router = Router::new()
        .route("/health", get(health_handler))
        .nest("/images", files::create_image_router())
        .layer(middleware::create_cors_middleware());

    Ok(Router::new().nest("/api", app_router))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
