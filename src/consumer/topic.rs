//! Known topics. Here we list all the topics we listen to and parse the kafka payload based on the topic

use crate::consumer::hello;
use crate::consumer::hello::HelloMessage;
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use std::string::FromUtf8Error;
use thiserror::Error;

pub enum Payload {
    Hello(HelloMessage),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    HelloMessage(FromUtf8Error),
}

// add a match-arm in the TryFrom for every topic!
pub const TOPICS: &[&str] = &[hello::TOPIC];

impl TryFrom<&BorrowedMessage<'_>> for Payload {
    type Error = ParseError;
    fn try_from(message: &BorrowedMessage) -> Result<Self, Self::Error> {
        match message.topic() {
            hello::TOPIC => HelloMessage::try_from(message.payload())
                .map_err(ParseError::HelloMessage)
                .map(Payload::Hello),
            t => unimplemented!("Unknown topic {}", t),
        }
    }
}
