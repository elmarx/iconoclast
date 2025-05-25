//! initialization of the object graph/dependencies

use crate::message_handler::MessageHandler;
use crate::settings::Settings;
use iconoclast::kafka;
use model::topics::{ParseError, Payload, TOPICS};
use std::error::Error;
use web::Router;

/// building blocks that make up the (micro-) service
pub struct BuildingBlocks {
    pub app: Router,
    pub consumer: kafka::Consumer<MessageHandler, Payload, ParseError, logic::Error>,
}

impl BuildingBlocks {
    /// initialize and wire up all the dependencies
    pub async fn wire(settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let _repo = repository::init(settings.database_url.as_deref()).await?;

        let app = web::init();
        let message_handler = MessageHandler::new();
        let consumer = kafka::Consumer::new(&settings.kafka, TOPICS, message_handler)?;

        Ok(Self { app, consumer })
    }
}
