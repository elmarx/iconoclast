use iconoclast::kafka;
use model::messages::topics::Payload;

type LogicalError = logic::hello::Error;

pub struct MessageHandler {
    hello_service: logic::hello::Service,
}

impl MessageHandler {
    pub const fn new(hello_service: logic::hello::Service) -> Self {
        Self { hello_service }
    }
}

impl kafka::MessageHandler<Payload, LogicalError> for MessageHandler {
    fn handle(&self, payload: Payload) -> impl Future<Output = Result<(), LogicalError>> {
        match payload {
            Payload::Hello(p) => self.hello_service.handle(p),
        }
    }
}
