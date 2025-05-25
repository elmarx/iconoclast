use crate::logging;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// port for the main service
    pub port: u16,
    /// management port
    pub management_port: u16,
    /// format to log in
    pub logging: logging::Format,
}
