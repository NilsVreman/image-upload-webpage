mod jwt;
mod login;

pub use jwt::validate_jwt;
pub use login::create_authorisation_router;

use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_cfg: jwt::JwtConfig,
    pub pwd_cfg: login::PasswordConfig,
}

pub fn setup() -> AuthConfig {
    dotenv().ok();

    AuthConfig {
        jwt_cfg: jwt::JwtConfig::new(
            env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            env::var("JWT_EXPIRATION_TIME")
                .and_then(|var| var.parse::<i64>().map_err(|_| env::VarError::NotPresent))
                .expect("JWT_EXPIRATION_TIME must be set"),
        ),
        pwd_cfg: login::PasswordConfig::new(
            env::var("SHARED_PASSWORD_HASH").expect("SHARED_PASSWORD_HASH must be set"),
        ),
    }
}
