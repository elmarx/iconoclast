use domain::{Task, TaskId};
use errors::RepositoryError;
use futures::Stream;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn insert(&self, desc: &str) -> Result<TaskId, RepositoryError>;

    async fn insert_with_id(&self, task_id: TaskId, desc: &str) -> Result<(), RepositoryError>;

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError>;

    async fn delete_by_id(&self, id: TaskId) -> Result<bool, RepositoryError>;

    fn find_all(&self) -> impl Stream<Item = Result<Task, RepositoryError>> + Send;
}

#[cfg(test)]
impl Clone for MockTaskRepository {
    fn clone(&self) -> Self {
        // the impl exists just to make the compiler happy.
        // clone is not required for the mock, it's not clear what clone should do for the mock,
        // so the honest option is to panic here for now
        unimplemented!("not supported for mock")
    }
}
