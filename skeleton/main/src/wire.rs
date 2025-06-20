use crate::ServiceSettings;
use adapter_kafka::*;
use adapter_repository as repository;
use adapter_repository::MigrateError;
use adapter_web as web;
use iconoclast::kafka;
use iconoclast::kafka::Consumer;
use std::error::Error;

pub async fn wire(
    settings: &ServiceSettings,
) -> Result<
    (
        web::Router,
        impl AsyncFnOnce() -> Result<(), MigrateError>,
        Consumer<
            impl kafka::MessageHandler<ApplicationError, Message = Message>,
            Message,
            DecodeError,
            ApplicationError,
        >,
    ),
    Box<dyn Error>,
> {
    let (run_migrations,) = repository::init(settings.database_url.as_deref()).await?;

    // TODO skeleton: initialize services here

    let kafka_listener = KafkaListener::new();
    let consumer = kafka::Consumer::new(&settings.kafka, kafka_listener)?;

    let router = web::init();

    Ok((router, run_migrations, consumer))
}
