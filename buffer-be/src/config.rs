use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub secret_key: String,
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
        Config {
            host,
            port,
            database_url,
            redis_url,
            secret_key,
        }
    }
}
