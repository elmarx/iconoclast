use config::ConfigError;
use iconoclast::kafka::KafkaError;
use thiserror::Error;

/// Container for all errors in different layers/parts of the service
#[derive(Error, pretty_error_debug::Debug)]
pub enum AppError {
    #[error(transparent)]
    Dal(#[from] repository::Error),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    KafkaError(#[from] KafkaError),
}
