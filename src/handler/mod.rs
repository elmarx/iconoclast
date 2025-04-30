use crate::handler;
#[double]
use crate::service::hello::Service as HelloService;
use axum::Router;
use mockall_double::double;

pub mod hello;

pub fn init(hello_service: HelloService) -> Router {
    Router::new().nest("/hello", handler::hello::init(hello_service))
}
