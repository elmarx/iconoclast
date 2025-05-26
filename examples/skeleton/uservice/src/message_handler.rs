use iconoclast::kafka;
use iconoclast::kafka::MessageExt;
use model::topics;

type LogicalError = logic::Error;

// wrapper to keep rdkafka out of the model and allow implementation of `TryFrom<&kafka::BorrowedMessage<'_>>` here
pub struct Payload(topics::Payload);

impl TryFrom<&kafka::BorrowedMessage<'_>> for Payload {
    type Error = topics::ParseError;

    fn try_from(msg: &kafka::BorrowedMessage) -> Result<Self, Self::Error> {
        let payload = topics::Payload::try_from((msg.topic(), msg.payload()))?;
        Ok(Self(payload))
    }
}

pub struct MessageHandler {}

impl MessageHandler {
    pub const fn new() -> Self {
        Self {}
    }
}

impl kafka::MessageHandler<LogicalError> for MessageHandler {
    type Message = Payload;

    fn handle(
        &self,
        Payload(payload): Self::Message,
    ) -> impl Future<Output = Result<(), LogicalError>> {
        #[expect(unreachable_patterns)]
        match payload {
            _ => async move { Ok(()) },
        }
    }
}
