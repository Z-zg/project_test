use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub database_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("Config.toml"))
            .build()?;
        s.try_deserialize()
    }
} 