use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfiguration {
    pub uri: String,
    pub max_conn: u32,
    pub min_conn: u32,
    pub max_lifetime: u64,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
}

pub fn new_configuration() -> Result<Configuration, ConfigError> {
    let mut s = Config::new();
    s.set_default("database.uri", "postgres://postgres@localhost/postgres")?;
    s.set_default("database.min_conn", 0)?;
    s.set_default("database.max_conn", 16)?;
    s.set_default("database.max_lifetime", 60)?;
    s.merge(File::with_name("config").required(false))?;
    s.merge(Environment::with_prefix("app"))?;
    s.try_into()
}
