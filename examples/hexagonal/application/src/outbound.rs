use domain::{Task, TaskId};
use errors::RepositoryError;
use futures::Stream;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn insert(&self, desc: &str) -> Result<TaskId, RepositoryError>;

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError>;

    fn find_all(&self) -> impl Stream<Item = Result<Task, RepositoryError>>;
}
