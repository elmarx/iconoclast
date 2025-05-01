#[double]
use crate::service::hello::Service as HelloService;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum_extra::response::InternalServerError;
use mockall_double::double;
use std::sync::Arc;

pub fn init(hello_service: HelloService) -> Router {
    Router::new()
        .route("/", axum::routing::get(hello))
        .route("/db", axum::routing::get(sql))
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

async fn sql(
    State(service): State<Arc<HelloService>>,
) -> Result<String, InternalServerError<repository::SqlxError>> {
    let number = service.number().await.map_err(InternalServerError)?;
    Ok(format!("{number}"))
}

#[cfg(test)]
mod test {
    use crate::handler::hello::init;
    use crate::service::hello::MockService as HelloService;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use repository::SqlxError;
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

    #[tokio::test]
    async fn test_db() {
        let mut hello_service = HelloService::default();
        hello_service.expect_number().returning(|| Ok(11));

        let app = init(hello_service);

        let response = app
            .oneshot(Request::builder().uri("/db").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: String = String::from_utf8(body.into()).unwrap();
        let expected = "11".to_string();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_db_fail() {
        let mut hello_service = HelloService::default();
        hello_service
            .expect_number()
            .returning(|| Err(SqlxError::PoolClosed));

        let app = init(hello_service);

        let response = app
            .oneshot(Request::builder().uri("/db").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
