pub mod hello;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;
