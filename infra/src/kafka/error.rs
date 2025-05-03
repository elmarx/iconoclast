use rdkafka::error::KafkaError;
use std::error::Error;
#[derive(thiserror::Error, Debug)]
pub enum StreamError<PE, LE>
where
    PE: Error,
    LE: Error,
{
    #[error(transparent)]
    Kafka(#[from] KafkaError),

    #[error(transparent)]
    Logic(LE),

    #[error(transparent)]
    Parse(PE),
}
