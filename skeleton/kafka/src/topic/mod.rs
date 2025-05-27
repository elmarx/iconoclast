//! decode "raw" kafka (Borrowed-) into structs

use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use thiserror::Error;

#[expect(dead_code)]
pub type KafkaPayload<'a> = Option<&'a [u8]>;
#[expect(dead_code)]
pub type KafkaKey<'a> = Option<&'a [u8]>;

pub const TOPICS: &[&str] = &[
    // TODO skeleton: add each topic to subscribe to
];

/// generic topic-specific kafka message with a decoded key and payload
#[expect(dead_code)]
pub struct KafkaTopicMessage<K, P> {
    pub key: K,
    pub partition: i32,
    pub offset: i64,
    pub payload: P,
}

impl<K, P> KafkaTopicMessage<K, P> {
    #[expect(dead_code)]
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
    // TODO skeleton: add a variant for each topic here
}

/// all possible errors that may occur when (trying to) decode [`BorrowedMessage`] from kafka
#[derive(Error, Debug)]
pub enum DecodeError {
    // TODO skeleton: add a variant for each possible error that might occur when trying to decode all kafka-messages the service should handle
    #[error("unknown topic {0}")]
    UnknownTopic(String),
}

/// "global" decode that should cover all subscribed topics:
///
/// - based on the topic from [`BorrowedMessage`] decode to the expected Message/Payload (and wrap in [`TopicMessages`])
impl TryFrom<&BorrowedMessage<'_>> for TopicMessages {
    type Error = DecodeError;

    fn try_from(m: &BorrowedMessage<'_>) -> Result<Self, Self::Error> {
        #[expect(clippy::match_single_binding)]
        let message = match m.topic() {
            // TODO skeleton: call the appropriate decoder per topic. This should probably have an arm for each entry in `TOPICS`
            t => Err(DecodeError::UnknownTopic(t.to_string()))?,
        };

        Ok(message)
    }
}
