use super::files::routers;
use super::middleware;
use axum::{routing::get, Json, Router};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/", routers::create_image_router())
        .layer(middleware::create_cors_middleware())
}

async fn health_handler() -> Json<serde_json::Value> {
    dbg!("Healthy");
    Json(serde_json::json!({"health": "OK"}))
}
