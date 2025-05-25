const DEFAULT_CONFIG: &str = include_str!("../../config.default.toml");

// if this service needs more settings, replace the Settings with a service-specific Settings/Config-Struct
pub type Settings = iconoclast::DefaultServiceConfig;

pub fn emerge() -> Result<Settings, iconoclast::config::ConfigError> {
    Settings::emerge(DEFAULT_CONFIG)
}
