use crate::ServiceSettings;
use adapter_kafka::{ApplicationError, DecodeError, KafkaListener, Message};
use adapter_repository as repository;
use adapter_repository::MigrateError;
use adapter_web as web;
use application::service::TodoService;
use iconoclast::kafka;
use iconoclast::kafka::Consumer;
use std::error::Error;

pub async fn wire(
    settings: &ServiceSettings,
) -> Result<
    (
        impl AsyncFnOnce() -> Result<(), MigrateError>,
        web::Router,
        Consumer<
            impl kafka::MessageHandler<ApplicationError, Message = Message>,
            Message,
            DecodeError,
            ApplicationError,
        >,
    ),
    Box<dyn Error>,
> {
    let (run_migrations, task_repository) =
        repository::init(settings.database_url.as_deref()).await?;

    let service = TodoService::new(task_repository);
    let kafka_listener = KafkaListener::new(service.clone());
    let consumer = kafka::Consumer::new(&settings.kafka, kafka_listener)?;

    let router = web::init(service);

    Ok((run_migrations, router, consumer))
}
