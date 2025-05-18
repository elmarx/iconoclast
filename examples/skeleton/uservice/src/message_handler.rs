use iconoclast::kafka;
use model::topics::Payload;

type LogicalError = logic::Error;

pub struct MessageHandler {}

impl MessageHandler {
    pub const fn new() -> Self {
        Self {}
    }
}

impl kafka::MessageHandler<Payload, LogicalError> for MessageHandler {
    fn handle(&self, payload: Payload) -> impl Future<Output = Result<(), LogicalError>> {
        #[expect(unreachable_patterns)]
        match payload {
            _ => async move { Ok(()) },
        }
    }
}
