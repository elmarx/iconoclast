use application::inbound;

mod from;
mod message_handler;
mod topic;

pub use topic::DecodeError;
pub use topic::TOPICS;

// some helper structs until we can do `trait MyMessageHandler = MessageHandler<inbound::EventHandlerError, Message = topic::TopicMessages>`…
/// the [`MessageHandler`]'s application-error (type `AE`)
pub type ApplicationError = inbound::EventHandlerError;
/// the Message-type of the [`MessageHandler`]
pub type Message = topic::TopicMessages;

pub use message_handler::KafkaListener;
