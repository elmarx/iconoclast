use crate::inbound::Endpoint;
use crate::outbound::TaskRepository;
use async_trait::async_trait;
use domain::Task;
use domain::TaskId;
use errors::RepositoryError;
use futures::Stream;

#[derive(Clone)]
pub struct TodoService<T: TaskRepository> {
    repository: T,
}

impl<T: TaskRepository> TodoService<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: TaskRepository> Endpoint for TodoService<T> {
    fn list_todos(&self) -> impl Stream<Item = Result<Task, RepositoryError>> {
        self.repository.find_all()
    }

    async fn add_todo(&self, desc: &str) -> Result<TaskId, RepositoryError> {
        self.repository.insert(desc).await
    }
}

#[cfg(test)]
mod test {
    use crate::inbound::Endpoint;
    use crate::outbound::MockTaskRepository;
    use crate::service::TodoService;
    use domain::TaskId;
    use futures::StreamExt;
    use futures::{TryStreamExt, stream};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_endpoint_add_todo() {
        let sample_id = TaskId::new();
        let mut mock = MockTaskRepository::new();
        mock.expect_insert()
            .with(eq("setup demo"))
            .returning(move |_| Ok(sample_id));

        let service = TodoService::new(mock);

        let actual = service.add_todo("setup demo").await.unwrap();

        assert_eq!(actual, sample_id);
    }

    #[tokio::test]
    async fn test_endpoint_list_todos() {
        let sample_a = domain::Task {
            id: TaskId::default(),
            description: "A".to_string(),
        };
        let sample_b = domain::Task {
            id: TaskId::default(),
            description: "B".to_string(),
        };

        let mut mock = MockTaskRepository::new();
        mock.expect_find_all().returning({
            let (sample_a, sample_b) = (sample_a.clone(), sample_b.clone());
            move || stream::iter(vec![Ok(sample_a.clone()), Ok(sample_b.clone())]).boxed()
        });

        let service = TodoService::new(mock);
        let actual = service.list_todos().try_collect::<Vec<_>>().await.unwrap();

        let expected = vec![sample_a, sample_b];

        assert_eq!(actual, expected);
    }
}
