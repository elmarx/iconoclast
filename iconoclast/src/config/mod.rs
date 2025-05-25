mod builder;
mod iconoclast;
mod service;

pub use config::ConfigError;

use crate::config::builder::{ConfigBuilder, ServiceConfig};
pub use builder::ConfigBuilder as Builder;
pub use iconoclast::Config as IconoclastConfig;
pub use service::DefaultServiceConfig;

pub fn emerge<'de, T: ServiceConfig<'de>>(default_config: &str) -> Result<T, ConfigError> {
    let builder = ConfigBuilder::new(default_config);

    builder.emerge()
}
