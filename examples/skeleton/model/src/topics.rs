use thiserror::Error;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;

pub enum Payload {}

#[derive(Debug, Error)]
pub enum ParseError {}

// add a match-arm in the TryFrom for every topic!
pub const TOPICS: &[&str] = &[];

impl TryFrom<(&str, KafkaPayload<'_>)> for Payload {
    type Error = ParseError;
    fn try_from((topic, _payload): (&str, KafkaPayload)) -> Result<Self, Self::Error> {
        match topic {
            t => unimplemented!("Unknown topic {t}"),
        }
    }
}
