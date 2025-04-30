use crate::consumer::KafkaPayload;
use std::string::FromUtf8Error;

pub const TOPIC: &str = "hello";

#[derive(Debug, PartialEq)]
pub enum HelloMessage {
    Name(String),
    Tombstone,
}

impl TryFrom<KafkaPayload<'_>> for HelloMessage {
    type Error = FromUtf8Error;

    fn try_from(value: KafkaPayload<'_>) -> Result<Self, Self::Error> {
        let result = match value {
            None => HelloMessage::Tombstone,
            Some(m) => HelloMessage::Name(String::from_utf8(Vec::from(m))?),
        };

        Ok(result)
    }
}
