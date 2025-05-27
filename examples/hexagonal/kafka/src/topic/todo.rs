use crate::topic::KafkaTopicMessage;
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use std::string::FromUtf8Error;
use thiserror::Error;
use uuid::Uuid;

pub const TOPIC: &str = "task";

/// a single kafka message with all information available
pub enum KafkaMessage {
    Task(KafkaTopicMessage<Option<Uuid>, String>),
    Tombstone(KafkaTopicMessage<Uuid, ()>),
}

/// errors that happen when decoding bytes from kafka
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(transparent)]
    Payload(#[from] FromUtf8Error),
    #[error(transparent)]
    Key(#[from] uuid::Error),
    #[error("invalid message")]
    InvalidMessage,
}

impl KafkaMessage {
    fn tombstone(key: Uuid, bm: &BorrowedMessage) -> Self {
        Self::Tombstone(KafkaTopicMessage::new(key, (), bm))
    }

    fn payload(key: Option<Uuid>, payload: String, bm: &BorrowedMessage) -> Self {
        Self::Task(KafkaTopicMessage::new(key, payload, bm))
    }
}

impl TryFrom<&BorrowedMessage<'_>> for KafkaMessage {
    type Error = DecodeError;

    fn try_from(bm: &BorrowedMessage) -> Result<Self, Self::Error> {
        let m = match (
            bm.key().map(Uuid::from_slice),
            bm.payload().map(|m| String::from_utf8(m.into())),
        ) {
            // tombstone: a valid UUID key must be present
            (Some(Ok(key)), None) => Self::tombstone(key, bm),
            // normal message with valid uuid-key
            (Some(Ok(key)), Some(Ok(pl))) => Self::payload(Some(key), pl, bm),
            // message without key
            (None, Some(Ok(pl))) => Self::payload(None, pl, bm),
            // invalid key, invalid payload or tombstone without key
            _ => Err(Self::Error::InvalidMessage)?,
        };

        Ok(m)
    }
}
