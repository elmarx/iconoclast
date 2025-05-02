use crate::handler;
use axum::Router;
use logic::hello::Service as HelloService;

pub mod hello;

pub fn init(hello_service: HelloService) -> Router {
    Router::new().nest("/hello", handler::hello::init(hello_service))
}
