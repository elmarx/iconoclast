use crate::config::DefaultServiceConfig;
use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

pub struct ConfigBuilder<'a> {
    /// compile-time default config (included via `include_str`)
    pub default_config: &'a str,

    /// config file to read (if not overridden by config_path_env_variable)
    pub config_file: &'a str,
    pub env_prefix: &'a str,
    /// name of the env-variable to override the default config file path
    pub config_path_env_variable: &'a str,
}

pub trait ServiceConfig<'de>: Deserialize<'de> {
    #[cfg(feature = "kafka")]
    fn with_kafka_properties(self, properties: Vec<(String, String)>) -> Self;
}

impl<'a> ConfigBuilder<'a> {
    pub const fn new(default_config: &'a str) -> Self {
        Self {
            default_config,
            config_file: "config.toml",
            env_prefix: "ICONOCLAST",
            config_path_env_variable: "ICONOCLAST_CONFIG",
        }
    }

    pub const fn config_file(mut self, config_file: &'a str) -> Self {
        self.config_file = config_file;
        self
    }

    pub const fn env_prefix(mut self, env_prefix: &'a str) -> Self {
        self.env_prefix = env_prefix;
        self
    }

    pub const fn config_path_env_variable(mut self, config_path_env_variable: &'a str) -> Self {
        self.config_path_env_variable = config_path_env_variable;
        self
    }

    pub fn emerge<'de, T: ServiceConfig<'de>>(self) -> Result<T, ConfigError> {
        let config_file =
            std::env::var(self.default_config).unwrap_or_else(|_| self.config_file.to_string());

        let settings = config::Config::builder()
            .add_source(File::from_str(self.default_config, FileFormat::Toml))
            .add_source(config::File::with_name(&config_file).required(false))
            .add_source(Environment::with_prefix(self.env_prefix).separator("_"))
            .build();

        #[cfg(not(feature = "kafka"))]
        return settings?.try_deserialize::<T>();

        #[cfg(feature = "kafka")]
        settings?
            .try_deserialize::<T>()
            .map(|s| s.with_kafka_properties(crate::kafka::from_env(std::env::vars())))
    }
}

impl DefaultServiceConfig {
    pub fn emerge(default_config: &str) -> Result<Self, ConfigError> {
        let builder = ConfigBuilder::new(default_config);
        builder.emerge()
    }
}
