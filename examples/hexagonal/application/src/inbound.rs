use async_trait::async_trait;
use domain::{TaskId, event};
use errors::RepositoryError;
use futures::Stream;

#[async_trait]
pub trait Endpoint: Send + Sync {
    fn list_todos(&self) -> impl Stream<Item = Result<domain::Task, RepositoryError>> + Send;

    async fn add_todo(&self, desc: &str) -> Result<TaskId, RepositoryError>;
}

pub type EventHandlerError = RepositoryError;
#[async_trait]
pub trait TaskEventHandler: Send + Sync {
    async fn task(&self, e: event::Task) -> Result<(), EventHandlerError>;
}
