use crate::config::builder::ServiceConfig;
use crate::config::iconoclast;
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DefaultServiceConfig {
    /// generic configuration for all iconoclast-like services
    pub iconoclast: iconoclast::Config,

    #[cfg(feature = "persistence")]
    // if no url (or an empty string) is given, connection parameters will be read from env: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub database_url: Option<String>,

    /// kafka configuration
    #[cfg(feature = "kafka")]
    pub kafka: crate::kafka::Config,
}

impl ServiceConfig<'_> for DefaultServiceConfig {
    #[cfg(feature = "kafka")]
    fn with_kafka_properties(mut self, properties: Vec<(String, String)>) -> Self {
        self.kafka.env_properties = properties;
        self
    }
}
