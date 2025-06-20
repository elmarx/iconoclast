use application::outbound::TaskRepository;
use domain::TaskId;
use errors::SqlxError;
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
    async fn insert(&self, desc: &str) -> Result<TaskId, SqlxError> {
        let rec = sqlx::query!(
            r#"INSERT INTO task (description) VALUES ($1) RETURNING task_id"#,
            desc
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(TaskId::from(rec.task_id))
    }

    async fn insert_with_id(&self, task_id: TaskId, desc: &str) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"INSERT INTO task (task_id, description) VALUES ($1, $2)"#,
            task_id.0,
            desc
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<domain::Task>, SqlxError> {
        sqlx::query_as!(
            domain::Task,
            "select task_id as id, description from task where task_id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn delete_by_id(&self, id: TaskId) -> Result<bool, SqlxError> {
        let affected = sqlx::query!("DELETE FROM task WHERE task_id = $1", id.0)
            .execute(&self.pool)
            .await?;

        Ok(0 < affected.rows_affected())
    }

    fn find_all(&self) -> impl Stream<Item = Result<domain::Task, SqlxError>> {
        sqlx::query_as!(domain::Task, "select task_id as id, description from task")
            .fetch(&self.pool)
    }
}

#[cfg(test)]
mod test {
    use super::Repository;
    use application::outbound::TaskRepository;
    use domain::TaskId;
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
    async fn test_insert_with_id(pool: PgPool) {
        let repo = Repository::new(pool);

        let id = TaskId::new();

        repo.insert_with_id(id, "test with a given id")
            .await
            .unwrap();

        let actual = repo.find_by_id(id).await.unwrap();
        let expected = domain::Task {
            id,
            description: "test with a given id".to_string(),
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

    #[sqlx::test]
    async fn test_delete_by_id(pool: PgPool) {
        let repo = Repository::new(pool);

        let sample_id = repo.insert("test delete").await.unwrap();

        let is_deleted = repo.delete_by_id(sample_id).await.unwrap();

        assert!(is_deleted);
        let actual = repo.find_by_id(sample_id).await.unwrap();
        assert_eq!(actual, None);
    }
}
