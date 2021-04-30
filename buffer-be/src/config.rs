use std::env;
use validator::Validate;

#[derive(Clone, Validate)]
pub struct Config {
    pub host: String,
    pub port: u16,
    #[validate(url)]
    pub database_url: String,
    pub redis_url: String,
    #[validate(length(min = 32))]
    pub secret_key: String,
    pub media_base_dir: String,
    pub media_base_url: String,
}

impl Config {
    pub fn new() -> Self {
        let host = env::var("HOST").expect("Config: HOST not set");
        let port = env::var("PORT")
            .expect("Config: PORT not set")
            .parse::<u16>()
            .expect("Config: PORT is not an integer");
        let database_url = env::var("DATABASE_URL").expect("Config: DATABASE_URL not set");
        let redis_url = env::var("REDIS_URL").expect("Config: REDIS_URL not set");
        let secret_key = env::var("SECRET_KEY").expect("Config: SECRET_KEY not set");
        let media_base_dir = env::var("MEDIA_BASE_DIR").expect("Config: MEDIA_BASE_DIR not set");
        let media_base_url = env::var("MEDIA_BASE_URL").expect("Config: MEDIA_BASE_URL not set");
        Config {
            host,
            port,
            database_url,
            redis_url,
            secret_key,
            media_base_dir,
            media_base_url,
        }
    }
}
