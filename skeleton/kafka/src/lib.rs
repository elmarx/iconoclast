use std::convert::Infallible;

mod message_handler;
mod topic;

pub use topic::DecodeError;

// some helper structs until we can do `trait MyMessageHandler = MessageHandler<inbound::EventHandlerError, Message = topic::TopicMessages>`…
/// the [`MessageHandler`]'s application-error (type `AE`)
// TODO skeleton: point to the error (-enum) that can occur for the inbound-event-listener/kafka port (instead of Infallible)
pub type ApplicationError = Infallible;
/// the Message-type of the [`MessageHandler`]
pub type Message = topic::TopicMessages;

pub use message_handler::KafkaListener;
