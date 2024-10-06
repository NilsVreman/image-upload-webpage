use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_image_router() -> Router {
    Router::new()
        .route("/images", post(handlers::post_image_list))
        .route("/images", get(handlers::get_image_list))
        .route("/images/:image_name", get(handlers::get_image))
}
