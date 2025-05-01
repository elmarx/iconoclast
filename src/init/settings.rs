use crate::infra::logging;
use crate::init::kafka;
use crate::init::kafka::KafkaPropertyValue;
use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::collections::HashMap;

const DEFAULT_CONFIG: &str = include_str!("../../config.default.toml");

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub management_port: u16,
    /// format to log.
    pub logging: logging::Format,

    // if no url is given, connection parameters will be read from env: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters
    pub database_url: Option<String>,
    pub kafka: Kafka,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Kafka {
    #[serde(skip)]
    pub env_properties: Vec<(String, String)>,
    #[serde(flatten, default)]
    pub properties: HashMap<String, KafkaPropertyValue>,
}

impl Settings {
    pub fn emerge() -> Result<Self, ConfigError> {
        let config_file = std::env::var("ICONOCLAST_CONFIG").unwrap_or("config.toml".to_string());

        let settings = config::Config::builder()
            .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
            .add_source(config::File::with_name(&config_file).required(false))
            .add_source(Environment::with_prefix("ICONOCLAST").separator("_"))
            .build();

        let kafka_properties = kafka::from_env(std::env::vars());

        settings?.try_deserialize::<Settings>().map(|mut s| {
            s.kafka.env_properties = kafka_properties;
            s
        })
    }
}
