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
    pub static_files_dir: String,
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
        let static_files_dir =
            env::var("STATIC_FILES_DIR").expect("Config: STATIC_FILES_DIR not set");
        Config {
            host,
            port,
            database_url,
            redis_url,
            secret_key,
            static_files_dir,
        }
    }
}
