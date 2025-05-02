//! Known topics. Here we list all the topics we listen to and parse the kafka payload based on the topic
use super::{KafkaPayload, hello};
use thiserror::Error;

pub enum Payload {
    Hello(hello::Message),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    HelloMessage(hello::Error),
}

// add a match-arm in the TryFrom for every topic!
pub const TOPICS: &[&str] = &[hello::TOPIC];

impl TryFrom<(&str, KafkaPayload<'_>)> for Payload {
    type Error = ParseError;
    fn try_from((topic, payload): (&str, KafkaPayload)) -> Result<Self, Self::Error> {
        match topic {
            hello::TOPIC => hello::Message::try_from(payload)
                .map_err(ParseError::HelloMessage)
                .map(Payload::Hello),
            t => unimplemented!("Unknown topic {}", t),
        }
    }
}
