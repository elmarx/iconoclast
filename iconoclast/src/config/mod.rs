mod builder;
mod iconoclast;
mod service;

pub use config::ConfigError;

use crate::config::builder::ConfigBuilder;
pub use builder::ConfigBuilder as Builder;
pub use iconoclast::Config as IconoclastConfig;
use serde::Deserialize;
pub use service::DefaultServiceConfig;

/// a trait for structs to make use of a configbuilder/high-level "emerge"
pub trait ServiceConfig<'de>: Deserialize<'de> {
    #[cfg(feature = "kafka")]
    fn with_kafka_properties(self, properties: Vec<(String, String)>) -> Self;
}

/// # Errors
///
/// fails if the underlying [`config::builder::ConfigBuilder::<config::builder::DefaultState>::build`] or [`config::config::Config::try_deserialize`] fails
pub fn emerge<'de, T: ServiceConfig<'de>>(default_config: &str) -> Result<T, ConfigError> {
    let builder = ConfigBuilder::new(default_config);

    builder.emerge()
}
