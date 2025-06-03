//! implementation for the [`iconoclast::kafka::MessageHandler`]

use crate::{ApplicationError, topic};
use application::inbound;
use futures::TryFutureExt;
use iconoclast::kafka::MessageHandler;

pub struct KafkaListener<T: inbound::TaskEventHandler> {
    todo_event_handler: T,
}

impl<T: inbound::TaskEventHandler> KafkaListener<T> {
    pub const fn new(todo_event_handler: T) -> Self {
        Self { todo_event_handler }
    }
}

impl<T: inbound::TaskEventHandler> MessageHandler<ApplicationError> for KafkaListener<T> {
    type Message = topic::TopicMessages;

    fn topics() -> &'static [&'static str] {
        topic::TOPICS
    }

    fn handle(
        &self,
        kafka_message: Self::Message,
    ) -> impl Future<Output = Result<(), ApplicationError>> {
        match kafka_message {
            Self::Message::Todo(t) => self.todo_event_handler.task(t.into()).map_ok(|_| ()),
        }
    }
}
