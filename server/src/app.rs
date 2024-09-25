use super::files::routers;
use axum::{response::IntoResponse, routing::get, Json, Router};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/files", routers::create_file_router())
}

async fn health_handler() -> axum::response::Response {
    (
        axum::http::StatusCode::OK,
        Json(serde_json::json!({"health": "OK"})),
    )
        .into_response()
}
