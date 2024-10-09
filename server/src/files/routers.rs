use super::handlers;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};

pub fn create_image_router() -> Router {
    Router::new()
        .route(
            "/",
            post(handlers::post_image_list).layer(DefaultBodyLimit::max(handlers::MAX_UPLOAD_SIZE)),
        )
        .route("/", get(handlers::get_all_thumbnail_meta_data))
        .route("/:image_name", get(handlers::get_image))
        .route("/:image_name/thumbnail", get(handlers::get_thumbnail))
}
