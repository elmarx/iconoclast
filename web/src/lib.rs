pub mod hello;

pub use axum::Router;
use logic::hello::Service as HelloService;

pub fn init(hello_service: HelloService) -> Router {
    Router::new().nest("/hello", hello::init(hello_service))
}
