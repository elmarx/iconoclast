use crate::init::settings::Settings;
use crate::service::hello;

pub struct Dependencies {
    pub hello_service: hello::Service,
}

pub fn wire(_settings: Settings) -> Dependencies {
    let hello_service = hello::Service::default();

    Dependencies { hello_service }
}
