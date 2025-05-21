use crate::outbound::TaskRepository;
use domain::TaskId;
use errors::RepositoryError;

pub struct TodoService<T: TaskRepository> {
    repository: T,
}

impl<T: TaskRepository> TodoService<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn create(&self, description: &str) -> Result<TaskId, RepositoryError> {
        self.repository.insert(description).await
    }
}

#[cfg(test)]
mod test {
    use crate::outbound::MockTaskRepository;
    use crate::service::TodoService;
    use domain::TaskId;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_create() {
        let sample_id = TaskId::new();
        let mut mock = MockTaskRepository::new();
        mock.expect_insert()
            .with(eq("setup demo"))
            .returning(move |_| Ok(sample_id));

        let service = TodoService::new(mock);

        let actual = service.create("setup demo").await.unwrap();

        assert_eq!(actual, sample_id);
    }
}
