use super::files::routers;
use axum::{routing::get, Json, Router};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/", routers::create_image_router())
}

async fn health_handler() -> Json<serde_json::Value> {
    dbg!("Healthy");
    Json(serde_json::json!({"health": "OK"}))
}
