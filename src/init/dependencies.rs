use crate::dal;
use crate::error::AppError;
use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use mockall_double::double;

pub struct Dependencies {
    pub hello_service: HelloService,
}

pub async fn wire(settings: &Settings) -> Result<Dependencies, AppError> {
    let repo = dal::init(settings.database_url.as_deref()).await?;
    let hello_service = HelloService::new(repo);

    Ok(Dependencies { hello_service })
}
