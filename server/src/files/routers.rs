use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_image_router() -> Router {
    Router::new()
        .route("/", post(handlers::post_image_list))
        .route("/", get(handlers::get_image_list))
        .route("/:image_name", get(handlers::get_image))
        .route("/:image_name/thumbnail", get(handlers::get_thumbnail))
}
