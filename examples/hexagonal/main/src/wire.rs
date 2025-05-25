use crate::ServiceSettings;
use adapter_repository as repository;
use adapter_repository::MigrateError;
use adapter_web as web;
use application::service::TodoService;
use std::error::Error;

pub async fn wire(
    settings: &ServiceSettings,
) -> Result<(impl AsyncFnOnce() -> Result<(), MigrateError>, web::Router), Box<dyn Error>> {
    let (run_migrations, task_repository) =
        repository::init(settings.database_url.as_deref()).await?;

    let service = TodoService::new(task_repository);

    let router = web::init(service);

    Ok((run_migrations, router))
}
