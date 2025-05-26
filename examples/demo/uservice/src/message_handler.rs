use iconoclast::kafka;
use iconoclast::kafka::MessageExt;
use model::messages;

type LogicalError = logic::hello::Error;

// wrapper to keep rdkafka out of the model and allow implementation of `TryFrom<&kafka::BorrowedMessage<'_>>` here
pub struct Payload(messages::topics::Payload);

impl TryFrom<&kafka::BorrowedMessage<'_>> for Payload {
    type Error = messages::topics::ParseError;

    fn try_from(msg: &kafka::BorrowedMessage) -> Result<Self, Self::Error> {
        let payload = messages::topics::Payload::try_from((msg.topic(), msg.payload()))?;
        Ok(Self(payload))
    }
}

pub struct MessageHandler {
    hello_service: logic::hello::Service,
}

impl MessageHandler {
    pub const fn new(hello_service: logic::hello::Service) -> Self {
        Self { hello_service }
    }
}

impl kafka::MessageHandler<LogicalError> for MessageHandler {
    type Message = Payload;

    fn handle(
        &self,
        Payload(msg): Self::Message,
    ) -> impl Future<Output = Result<(), LogicalError>> {
        match msg {
            messages::topics::Payload::Hello(p) => self.hello_service.handle(p),
        }
    }
}
