use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct DatabaseConfigurations {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Configurations {
    pub database: DatabaseConfigurations,
    pub application_port: u16,
}

pub fn read_configs() -> Result<Configurations, ConfigError> {
    let app_mode = env::var("APP_MODE").unwrap_or_else(|_| "development".into());

    let configs = Config::builder()
        .add_source(File::with_name("configurations/default"))
        .add_source(File::with_name(&format!("configurations/{}", app_mode)).required(false))
        .build()?;
    configs.try_deserialize()
}
