use async_trait::async_trait;
use domain::{TaskId, event};
use errors::SqlxError;
use futures::Stream;

#[async_trait]
pub trait Endpoint: Send + Sync {
    fn list_todos(&self) -> impl Stream<Item = Result<domain::Task, SqlxError>> + Send;

    async fn add_todo(&self, desc: &str) -> Result<TaskId, SqlxError>;
}

pub type EventHandlerError = SqlxError;
#[async_trait]
pub trait TaskEventHandler: Send + Sync {
    async fn task(&self, e: event::Task) -> Result<(), EventHandlerError>;
}
