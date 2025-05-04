use crate::kafka;

#[cfg(feature = "kafka")]
#[derive(Debug, thiserror::Error)]
pub enum Startup<PE: std::error::Error, LE: std::error::Error> {
    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Kafka(#[from] kafka::StreamError<PE, LE>),
}

#[cfg(not(feature = "kafka"))]
#[derive(Debug, thiserror::Error)]
pub enum Startup<PE: std::error::Error, LE: std::error::Error> {
    #[error(transparent)]
    Io(#[from] tokio::io::Error),
}
