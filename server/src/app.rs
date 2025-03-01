use dotenv::dotenv;
use std::env;

use super::auth;
use super::files;
use super::middleware;
use axum::middleware::from_fn_with_state;
use axum::Extension;
use axum::{routing::get, Json, Router};

#[derive(Clone)]
pub struct GeneralConfig {
    pub host: String,
    pub port: u16,
    pub client_url: String,
}

impl GeneralConfig {
    fn from_env() -> Self {
        dotenv().ok();

        Self {
            host: env::var("HOST").expect("HOST must be set"),
            port: env::var("PORT")
                .and_then(|var| var.parse::<u16>().map_err(|_| env::VarError::NotPresent))
                .expect("PORT must be set"),
            client_url: env::var("CLIENT_URL").expect("CLIENT_URL must be set"),
        }
    }
}

pub async fn create_app() -> Result<Router, String> {
    files::setup().await.map_err(|err| err.to_string())?;
    let jwt_config = auth::JwtConfig::from_env();
    let psw_config = auth::PasswordConfig::from_env();
    let general_config = GeneralConfig::from_env();

    let public_routes = Router::new().route("/health", get(health_handler));
    let authenticated_routes = Router::new()
        .merge(files::create_image_router())
        .layer(from_fn_with_state(
            jwt_config.clone(),
            middleware::auth_middleware,
        ))
        .layer(Extension(general_config.clone()));
    let authorisation_routes = auth::create_authorisation_router()
        .layer(Extension(psw_config.clone()))
        .layer(Extension(jwt_config.clone()));

    Ok(authorisation_routes
        .merge(public_routes)
        .merge(authenticated_routes)
        .layer(middleware::cors_middleware(general_config.clone())))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({"health": "OK"}))
}
