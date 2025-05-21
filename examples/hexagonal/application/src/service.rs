use crate::outbound::TaskRepository;
use errors::SqlxError;

struct TodoService<T: TaskRepository + Send + Sync> {
    repository: T,
}

impl<T: TaskRepository + Send + Sync> TodoService<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn create(&self, description: &str) -> Result<i64, SqlxError> {
        self.repository.insert(description).await
    }
}

#[cfg(test)]
mod test {
    use crate::outbound::MockTaskRepository;
    use crate::service::TodoService;
    use futures::FutureExt;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_create() {
        let mut mock = MockTaskRepository::new();
        mock.expect_insert()
            .with(eq("setup demo"))
            .returning(|_| futures::future::ok(42).boxed());

        let service = TodoService::new(mock);

        let actual = service.create("setup demo").await.unwrap();

        assert_eq!(actual, 42);
    }
}
