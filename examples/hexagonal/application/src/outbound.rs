use domain::{Task, TaskId};
use errors::SqlxError;
use futures::Stream;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn insert(&self, desc: &str) -> Result<TaskId, SqlxError>;

    async fn insert_with_id(&self, task_id: TaskId, desc: &str) -> Result<(), SqlxError>;

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, SqlxError>;

    async fn delete_by_id(&self, id: TaskId) -> Result<bool, SqlxError>;

    fn find_all(&self) -> impl Stream<Item = Result<Task, SqlxError>> + Send;
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
