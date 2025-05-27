//! implementation for the [`iconoclast::kafka::MessageHandler`]

use crate::{ApplicationError, topic};
use iconoclast::kafka::MessageHandler;

pub struct KafkaListener {
    // TODO skeleton: add inbound ports here. The types will all be generics
}

impl KafkaListener {
    #[expect(clippy::new_without_default)]
    pub const fn new() -> Self {
        // TODO skeleton: add inbound ports
        Self {}
    }
}

impl MessageHandler<ApplicationError> for KafkaListener {
    type Message = topic::TopicMessages;

    fn topics() -> &'static [&'static str] {
        topic::TOPICS
    }

    fn handle(
        &self,
        kafka_message: Self::Message,
    ) -> impl Future<Output = Result<(), ApplicationError>> {
        match kafka_message {
            // TODO skeleton: match on TopicMessage variants. The compiler will probably force you to do that anyway
        }

        #[expect(unreachable_code)]
        async {
            todo!("Remove this")
        }
    }
}
