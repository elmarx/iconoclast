//! initialization of the object graph/dependencies

use crate::message_handler::MessageHandler;
use crate::message_handler::Payload;
use iconoclast::kafka;
use logic::hello::Service as HelloService;
use model::messages;
use repository::Repositories;
use std::error::Error;
use web::Router;

/// building blocks that make up the (micro-) service
pub struct BuildingBlocks {
    pub app: Router,
    pub consumer:
        kafka::Consumer<MessageHandler, Payload, messages::DecodeError, logic::hello::Error>,
}

impl BuildingBlocks {
    /// initialize and wire up all the dependencies
    pub async fn wire(settings: &iconoclast::DefaultServiceConfig) -> Result<Self, Box<dyn Error>> {
        let Repositories { dummy } = repository::init(settings.database_url.as_deref()).await?;
        let hello_service = HelloService::new(dummy);

        let app = web::init(hello_service.clone());
        let message_handler = MessageHandler::new(hello_service);
        let consumer = kafka::Consumer::new(&settings.kafka, messages::TOPICS, message_handler)?;

        Ok(Self { app, consumer })
    }
}
