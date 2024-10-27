use super::files;
use super::middleware;
use axum::{routing::get, Json, Router};

pub fn create_app() -> Router {
    let app_router = Router::new()
        .route("/health", get(health_handler))
        .nest("/images", files::routers::create_image_router())
        .layer(middleware::create_cors_middleware());

    Router::new().nest("/api", app_router)
}

pub async fn setup_app() -> Result<(), String> {
    files::storage::setup().await.map_err(|err| err.to_string())
}

async fn health_handler() -> Json<serde_json::Value> {
    dbg!("Healthy");
    Json(serde_json::json!({"health": "OK"}))
}
