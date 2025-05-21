use application::outbound::TaskRepository;
use errors::SqlxError;
use futures::Stream;

struct Repository {}

impl TaskRepository for Repository {
    async fn insert(&self, desc: &str) -> Result<i64, SqlxError> {
        todo!()
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<domain::Task>, SqlxError> {
        todo!()
    }

    fn find_all(&self) -> impl Stream<Item = Result<domain::Task, SqlxError>> {
        futures::stream::empty()
    }
}
