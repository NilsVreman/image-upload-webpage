use super::handlers;
use axum::{routing::post, Router};

pub fn create_image_router() -> Router {
    Router::new().route("/images", post(handlers::image_upload_handler))
}
