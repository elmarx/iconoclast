use application::inbound;
pub use axum::Router;
use axum::response::Html;
use axum::response::IntoResponse;
use std::sync::Arc;

pub fn init(endpoint: impl inbound::Endpoint + 'static) -> Router {
    Router::new()
        .route("/", axum::routing::get(index))
        .with_state(Arc::new(endpoint))
}

async fn index() -> impl IntoResponse {
    // language=html
    Html("<body><h1>Hello, World</h1></body>")
}
