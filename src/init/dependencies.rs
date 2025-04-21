use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use mockall_double::double;
use std::convert::Infallible;

pub struct Dependencies {
    pub hello_service: HelloService,
}

pub async fn wire(settings: Settings) -> Result<Dependencies, Infallible> {
    let hello_service = HelloService::default();
    let _repo = crate::dal::init(&settings.db_url).await.unwrap();

    Ok(Dependencies { hello_service })
}
