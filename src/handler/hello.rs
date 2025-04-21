use crate::service::hello;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};

pub fn init(hello_service: hello::Service) -> Router {
    Router::new()
        .route("/", axum::routing::get(hello))
        .with_state(hello_service)
}

#[derive(serde::Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello(State(service): State<hello::Service>) -> impl IntoResponse {
    let message = service.message();
    Json(HelloResponse { message })
}
