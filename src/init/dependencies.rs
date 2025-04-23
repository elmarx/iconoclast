use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use mockall_double::double;
use std::convert::Infallible;

pub struct Dependencies {
    pub hello_service: HelloService,
}

pub async fn wire(settings: &Settings) -> Result<Dependencies, Infallible> {
    let repo = crate::dal::init(settings.database_url.as_deref())
        .await
        .unwrap();
    let hello_service = HelloService::new(repo);

    Ok(Dependencies { hello_service })
}
