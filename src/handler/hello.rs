use axum::response::IntoResponse;
use axum::{Json, Router};

pub fn init() -> Router {
    Router::new().route("/", axum::routing::get(hello))
}

#[derive(serde::Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> impl IntoResponse {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}
