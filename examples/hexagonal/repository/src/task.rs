use application::outbound::TaskRepository;
use domain::TaskId;
use errors::RepositoryError;
use futures::Stream;

use sqlx::PgPool;

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepository for Repository {
    async fn insert(&self, desc: &str) -> Result<TaskId, RepositoryError> {
        let rec = sqlx::query!(
            r#"INSERT INTO task (description) VALUES ($1) RETURNING task_id"#,
            desc
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(TaskId::from(rec.task_id))
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<domain::Task>, RepositoryError> {
        sqlx::query_as!(
            domain::Task,
            "select task_id as id, description from task where task_id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await
    }

    fn find_all(&self) -> impl Stream<Item = Result<domain::Task, RepositoryError>> {
        sqlx::query_as!(domain::Task, "select task_id as id, description from task")
            .fetch(&self.pool)
    }
}

#[cfg(test)]
mod test {
    use super::Repository;
    use application::outbound::TaskRepository;
    use futures::TryStreamExt;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_insert_and_find(pool: PgPool) {
        let repo = Repository::new(pool);

        let id = repo.insert("test the repository").await.unwrap();

        let actual = repo.find_by_id(id).await.unwrap();
        let expected = domain::Task {
            id,
            description: "test the repository".to_string(),
        };

        assert_eq!(actual, Some(expected));
    }

    #[sqlx::test]
    async fn test_find_all(pool: PgPool) {
        let repo = Repository::new(pool);

        repo.insert("todo 1").await.unwrap();
        repo.insert("todo 2").await.unwrap();

        let actual = repo.find_all().try_collect::<Vec<_>>().await.unwrap();

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].description, "todo 1");
        assert_eq!(actual[1].description, "todo 2");
    }
}
