//! initialization of the object graph/dependencies

use crate::error::AppError;
use crate::message_handler::MessageHandler;
use crate::settings::Settings;
use iconoclast::kafka;
use logic::hello::Service as HelloService;
use model::messages::topics::{ParseError, Payload, TOPICS};
use repository::Repositories;
use web::Router;

/// building blocks that make up the (micro-) service
pub struct BuildingBlocks {
    pub app: Router,
    pub consumer: kafka::Consumer<MessageHandler, Payload, ParseError, logic::hello::Error>,
}

impl BuildingBlocks {
    /// initialize and wire up all the dependencies
    pub async fn wire(settings: &Settings) -> Result<Self, AppError> {
        let Repositories { dummy } = repository::init(settings.database_url.as_deref()).await?;
        let hello_service = HelloService::new(dummy);

        let app = web::init(hello_service.clone());
        let message_handler = MessageHandler::new(hello_service);
        let consumer = kafka::Consumer::new(&settings.kafka, TOPICS, message_handler)?;

        Ok(Self { app, consumer })
    }
}
