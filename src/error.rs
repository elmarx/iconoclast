use crate::dal;
use config::ConfigError;

/// Container for all errors in different layers/parts of the service
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Dal(#[from] dal::Error),
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
