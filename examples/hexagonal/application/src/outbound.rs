
use domain::Task;
use errors::SqlxError;
use futures::Stream;

pub trait TaskRepository {
    fn insert(&self, desc: &str) -> impl Future<Output = Result<i64, SqlxError>>;

    fn find_by_id(&self, id: i64) -> impl Future<Output = Result<Option<Task>, SqlxError>>;

    fn find_all(&self) -> impl Stream<Item = Result<Task, SqlxError>>;
}
