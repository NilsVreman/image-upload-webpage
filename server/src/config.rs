use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct GeneralConfig {
    // FIXME: This is a temporary fix to make the server run on localhost
    pub host: String,
    pub port: u16,
    pub client_url: String,
}

impl GeneralConfig {
    pub fn from_env() -> Self {
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
