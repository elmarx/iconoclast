//! Known topics. Here we list all the topics we listen to and parse the kafka payload based on the topic

use model::messages::hello;
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
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

impl TryFrom<&BorrowedMessage<'_>> for Payload {
    type Error = ParseError;
    fn try_from(message: &BorrowedMessage) -> Result<Self, Self::Error> {
        match message.topic() {
            hello::TOPIC => hello::Message::try_from(message.payload())
                .map_err(ParseError::HelloMessage)
                .map(Payload::Hello),
            t => unimplemented!("Unknown topic {}", t),
        }
    }
}
