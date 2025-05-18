//! Messages used in relevant kafka-topics

pub mod hello;
pub mod topics;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;
