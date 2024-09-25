use super::handlers;
use axum::{routing::post, Router};

pub fn create_file_router() -> Router {
    Router::new().route("/", post(handlers::file_upload_handler))
}
