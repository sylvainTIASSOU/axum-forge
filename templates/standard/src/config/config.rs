#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            port: require("PORT")
                .unwrap_or_else(|_| "8001".to_string())
                .parse()
                .map_err(|_| "PORT must be a number".to_string())?,
            host: require("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string())
                .to_string(),
        })
    }
}

fn require(key: &str) -> Result<String, String> {
    std::env::var(key).map_err(|_| format!("environment variable {} not set", key))
}
