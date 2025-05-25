mod builder;
mod iconoclast;
mod service;

pub use config::ConfigError;

use crate::config::builder::{ConfigBuilder, ServiceConfig};
pub use builder::ConfigBuilder as Builder;
pub use iconoclast::Config as IconoclastConfig;
pub use service::DefaultServiceConfig;

/// # Errors
///
/// fails if the underlying [`config::builder::ConfigBuilder::<config::builder::DefaultState>::build`] or [`config::config::Config::try_deserialize`] fails
pub fn emerge<'de, T: ServiceConfig<'de>>(default_config: &str) -> Result<T, ConfigError> {
    let builder = ConfigBuilder::new(default_config);

    builder.emerge()
}
