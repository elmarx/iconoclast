use async_trait::async_trait;
use domain::TaskId;
use errors::RepositoryError;
use futures::Stream;

#[async_trait]
pub trait Endpoint: Send + Sync {
    fn list_todos(&self) -> impl Stream<Item = Result<domain::Task, RepositoryError>>;

    async fn add_todo(&self, desc: &str) -> Result<TaskId, RepositoryError>;
}
