#[double]
use crate::service::hello::Service as HelloService;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use mockall_double::double;
use std::sync::Arc;

pub fn init(hello_service: HelloService) -> Router {
    Router::new()
        .route("/", axum::routing::get(hello))
        .with_state(Arc::new(hello_service))
}

#[derive(serde::Serialize, Debug)]
struct HelloResponse {
    message: String,
}

async fn hello(State(service): State<Arc<HelloService>>) -> impl IntoResponse {
    let message = service.message();
    Json(HelloResponse { message })
}

#[cfg(test)]
mod test {
    use crate::handler::hello::init;
    use crate::service::hello::MockService as HelloService;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_hello() {
        let mut hello_service = HelloService::default();
        hello_service
            .expect_message()
            .returning(|| "Hello from mock!".to_string());

        let app = init(hello_service);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_slice(&body).unwrap();

        let expected = serde_json::json!({"message": "Hello from mock!".to_string()});
        assert_eq!(actual, expected);
    }
}
