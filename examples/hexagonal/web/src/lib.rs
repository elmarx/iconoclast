use application::inbound;
use askama::Template;
pub use axum::Router;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum_extra::response::InternalServerError;
use domain::Task;
use futures::TryStreamExt;

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
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        tasks: Vec<Task>,
    }

    let tasks = service.list_todos().try_collect::<Vec<_>>().await.unwrap();

    let template = Tmpl { tasks };
    Html(template.render().map_err(InternalServerError))
}

#[cfg(test)]
mod test {
    use application::inbound;
    use axum::body::Body;
    use axum::http::Request;
    use errors::SqlxError;
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
        fn list_todos(&self) -> impl Stream<Item = Result<domain::Task, SqlxError>> + Send {
            let todos = self.todos.iter().map(|t| Ok(t.clone()));
            stream::iter(todos)
        }

        async fn add_todo(&self, _desc: &str) -> Result<domain::TaskId, SqlxError> {
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
