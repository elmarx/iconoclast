use application::inbound;
pub use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use futures::TryStreamExt;
use itertools::Itertools;

pub fn init<T>(endpoint: T) -> Router
where
    T: inbound::Endpoint + Clone + 'static,
{
    Router::new()
        .route("/", axum::routing::get(index::<T>))
        .with_state(endpoint)
}

async fn index<T>(State(service): State<T>) -> impl IntoResponse
where
    T: inbound::Endpoint,
{
    let todos = service.list_todos().try_collect::<Vec<_>>().await.unwrap();

    let number = format!("<p>there are {} todos</p>", todos.len());

    let todos = todos
        .into_iter()
        .map(|t| format!("<li>{t:?}</li>"))
        .join("\n");

    // language=html
    Html(format!(
        "<body><h1>Hello, World</h1>{number}<ul>{todos}</ul></body>"
    ))
}

#[cfg(test)]
mod test {
    use application::inbound;
    use axum::body::Body;
    use axum::http::Request;
    use errors::RepositoryError;
    use futures::stream;
    use futures::stream::Stream;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct MockEndpoint {
        todos: Vec<domain::Task>,
    }

    #[async_trait::async_trait]
    impl inbound::Endpoint for MockEndpoint {
        fn list_todos(&self) -> impl Stream<Item = Result<domain::Task, RepositoryError>> + Send {
            let todos = self.todos.iter().map(|t| Ok(t.clone()));
            stream::iter(todos)
        }

        async fn add_todo(&self, _desc: &str) -> Result<domain::TaskId, RepositoryError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_list() {
        let sample_a = domain::Task {
            id: domain::TaskId::default(),
            description: "Todo A".to_string(),
        };
        let sample_b = domain::Task {
            id: domain::TaskId::default(),
            description: "Todo B".to_string(),
        };

        let endpoint = MockEndpoint {
            todos: vec![sample_a.clone(), sample_b.clone()],
        };

        let router = super::init(endpoint);

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert!(body.contains("Todo A"));
        assert!(body.contains("Todo B"));
    }
}
