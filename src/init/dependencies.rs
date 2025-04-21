use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use mockall_double::double;

pub struct Dependencies {
    pub hello_service: HelloService,
}

pub fn wire(_settings: Settings) -> Dependencies {
    let hello_service = HelloService::default();

    Dependencies { hello_service }
}
