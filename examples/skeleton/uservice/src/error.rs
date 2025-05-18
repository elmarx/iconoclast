use config::ConfigError;
use iconoclast::kafka::KafkaError;

/// Container for all errors in different layers/parts of the service
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Dal(#[from] repository::Error),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    KafkaError(#[from] KafkaError),
}
