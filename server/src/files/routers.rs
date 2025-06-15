use super::handlers;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

pub fn create_image_router() -> Router {
    Router::new()
        .route("/images", post(handlers::post_image_list))
        .route("/images/thumbnails", get(handlers::get_all_thumbnails))
        .route("/images/{image_name}", get(handlers::get_image))
        .route(
            "/images/{image_name}/thumbnail",
            get(handlers::get_thumbnail),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(handlers::MAX_UPLOAD_SIZE))
}
