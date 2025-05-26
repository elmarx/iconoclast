use rdkafka::error::KafkaError;
use std::error::Error;

/// possible errors when consuming kafka messages
#[derive(thiserror::Error, Debug)]
pub enum StreamError<DE: Error, AE: Error> {
    #[error(transparent)]
    Kafka(#[from] KafkaError),

    #[error(transparent)]
    Application(AE),

    #[error(transparent)]
    Decode(DE),
}
