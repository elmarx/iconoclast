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
