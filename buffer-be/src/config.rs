use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        let host = env::var("HOST").expect("Config: HOST not set");
        let port = env::var("PORT")
            .expect("Config: PORT not set")
            .parse::<u16>()
            .expect("Config: PORT is not an integer");
        let database_url = env::var("DATABASE_URL").expect("Config: DATABASE_URL not set");
        Config {
            host,
            port,
            database_url,
        }
    }
}
