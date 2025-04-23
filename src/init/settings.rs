use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../../config.default.toml");

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,

    // if no url is given, connection parameters will be read from env: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters
    pub database_url: Option<String>,
}

impl Settings {
    pub fn emerge() -> Result<Self, ConfigError> {
        let config_file = std::env::var("ICONOCLAST_CONFIG").unwrap_or("config.toml".to_string());

        let settings = config::Config::builder()
            .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
            .add_source(config::File::with_name(&config_file).required(false))
            .add_source(Environment::with_prefix("ICONOCLAST").separator("_"))
            .build();

        settings?.try_deserialize::<Settings>()
    }
}
