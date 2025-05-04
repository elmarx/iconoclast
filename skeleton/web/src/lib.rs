pub mod hello;

#[cfg(feature = "template")]
mod templating;

pub use axum::Router;
use logic::hello::Service as HelloService;

pub fn init(hello_service: HelloService) -> Router {
    let router = Router::new().nest("/hello", hello::init(hello_service));
    #[cfg(feature = "template")]
    let templating = templating::init();

    #[cfg(feature = "template")]
    let router = router.merge(templating);

    router
}
