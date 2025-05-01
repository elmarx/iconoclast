use crate::consumer::IconoclastConsumer;
use crate::error::AppError;
use crate::handler;
use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use axum::Router;
use mockall_double::double;

/// building blocks that make up the (micro-) service
pub struct BuildingBlocks {
    pub app: Router,
    pub consumer: IconoclastConsumer,
}

impl BuildingBlocks {
    /// initialize and wire up all the dependencies
    pub async fn wire(settings: &Settings) -> Result<BuildingBlocks, AppError> {
        let repo = repository::init(settings.database_url.as_deref()).await?;
        let hello_service = HelloService::new(repo);

        let app = handler::init(hello_service.clone());
        let consumer = IconoclastConsumer::new(&settings.kafka, hello_service)?;

        Ok(BuildingBlocks { app, consumer })
    }
}
