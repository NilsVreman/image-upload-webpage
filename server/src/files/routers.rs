use super::handlers::upload_handler;
use axum::{routing::get, Router};

pub fn create_files_router() -> Router {
    Router::new().route("/files", get(upload_handler))
}
