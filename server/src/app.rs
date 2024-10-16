use super::files::routers;
use super::middleware;
use axum::{routing::get, Json, Router};

pub fn create_app() -> Router {
    let app_router = Router::new()
        .route("/health", get(health_handler))
        .nest("/images", routers::create_image_router())
        .layer(middleware::create_cors_middleware());

    Router::new().nest("/api", app_router)
}

async fn health_handler() -> Json<serde_json::Value> {
    dbg!("Healthy");
    Json(serde_json::json!({"health": "OK"}))
}
