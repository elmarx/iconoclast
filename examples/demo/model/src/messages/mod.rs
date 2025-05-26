//! Messages used in relevant kafka-topics
//! Here we list all the topics we listen to and parse the kafka payload based on the topic
use thiserror::Error;

pub mod topic_hello;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;

/// all kafka-messages/payloads known to this service
pub enum Payload {
    Hello(topic_hello::Message),
}

/// all possible errors when decoding raw Kafka-Keys and Payloads
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(transparent)]
    HelloMessage(topic_hello::Error),
}

// add a match-arm in the TryFrom for every topic!
pub const TOPICS: &[&str] = &[topic_hello::TOPIC];

impl TryFrom<(&str, KafkaPayload<'_>)> for Payload {
    type Error = DecodeError;
    fn try_from((topic, payload): (&str, KafkaPayload)) -> Result<Self, Self::Error> {
        match topic {
            topic_hello::TOPIC => topic_hello::Message::try_from(payload)
                .map_err(DecodeError::HelloMessage)
                .map(Payload::Hello),
            t => unimplemented!("Unknown topic {}", t),
        }
    }
}
