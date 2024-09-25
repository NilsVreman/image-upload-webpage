use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_file_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/", post(handlers::file_upload_handler))
}
