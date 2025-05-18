use crate::messages::KafkaPayload;
use std::string::FromUtf8Error;

/// topic this message will be sent on
pub const TOPIC: &str = "hello";

/// possible error when trying to parse the message from kafka
pub type Error = FromUtf8Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Name(String),
    Tombstone,
}

impl TryFrom<KafkaPayload<'_>> for Message {
    type Error = Error;

    fn try_from(value: KafkaPayload<'_>) -> Result<Self, Self::Error> {
        let result = match value {
            None => Self::Tombstone,
            Some(m) => Self::Name(String::from_utf8(Vec::from(m))?),
        };

        Ok(result)
    }
}
