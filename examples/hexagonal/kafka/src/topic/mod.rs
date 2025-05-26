//! decode "raw" kafka (Borrowed-) into structs

use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use thiserror::Error;

pub mod todo;

#[expect(dead_code)]
pub type KafkaPayload<'a> = Option<&'a [u8]>;
#[expect(dead_code)]
pub type KafkaKey<'a> = Option<&'a [u8]>;

pub const TOPICS: &[&str] = &[todo::TOPIC];

/// generic topic-specific kafka message with a decoded key and payload
pub struct KafkaTopicMessage<K, P> {
    pub key: K,
    pub partition: i32,
    pub offset: i64,
    pub payload: P,
}

impl<K, P> KafkaTopicMessage<K, P> {
    pub fn new(key: K, payload: P, bm: &BorrowedMessage) -> Self {
        Self {
            key,
            partition: bm.partition(),
            offset: bm.offset(),
            payload,
        }
    }
}

/// all kafka topics
pub enum TopicMessages {
    Todo(todo::KafkaMessage),
}

/// all possible errors that may occur when (trying to) decode [`BorrowedMessage`] from kafka
#[derive(Error, Debug)]
pub enum DecodeError {
    #[error(transparent)]
    Todo(#[from] todo::DecodeError),

    #[error("unknown topic {0}")]
    UnknownTopic(String),
}

/// "global" decode that should cover all subscribed topics:
///
/// - based on the topic from [`BorrowedMessage`] decode to the expected Message/Payload (and wrap in [`TopicMessages`])
impl TryFrom<&BorrowedMessage<'_>> for TopicMessages {
    type Error = DecodeError;

    fn try_from(m: &BorrowedMessage<'_>) -> Result<Self, Self::Error> {
        let message = match m.topic() {
            todo::TOPIC => todo::KafkaMessage::try_from(m).map(Self::Todo)?,
            t => Err(DecodeError::UnknownTopic(t.to_string()))?,
        };

        Ok(message)
    }
}
