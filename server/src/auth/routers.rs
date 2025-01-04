use super::login;
use axum::{routing::post, Router};

pub fn create_authorisation_router() -> Router {
    Router::new().route("/", post(login::login_handler))
}
