use application::outbound::TaskRepository;
use domain::TaskId;
use errors::RepositoryError;
use futures::Stream;

pub struct Repository;

#[async_trait::async_trait]
impl TaskRepository for Repository {
    async fn insert(&self, _desc: &str) -> Result<TaskId, RepositoryError> {
        todo!()
    }

    async fn find_by_id(&self, _id: TaskId) -> Result<Option<domain::Task>, RepositoryError> {
        todo!()
    }

    fn find_all(&self) -> impl Stream<Item = Result<domain::Task, RepositoryError>> {
        futures::stream::empty()
    }
}
