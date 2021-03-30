use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let host = env::var("HOST").expect("Config: HOST not set");
        let port = env::var("PORT")
            .expect("Config: PORT not set")
            .parse::<u16>()
            .expect("Config: PORT is not an integer");
        Config { host, port }
    }
}
