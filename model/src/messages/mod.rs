pub mod hello;
pub mod topics;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;
