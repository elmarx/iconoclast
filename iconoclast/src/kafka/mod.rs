mod config;
mod consumer;
mod error;
mod handler;
#[cfg(test)]
mod test;

pub use config::Config;
pub use config::PropertyValue;
pub use config::from_env;
pub use consumer::Consumer;
pub use error::StreamError;
pub use handler::MessageHandler;

// re-export structs required to implement the MessageHandler
pub use rdkafka::error::KafkaError;
pub use rdkafka::message::BorrowedMessage;
// adds the actual `.payload()` and `.key()` methods
pub use rdkafka::message::Message as MessageExt;
