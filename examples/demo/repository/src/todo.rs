use futures::stream::BoxStream;
use futures::stream::StreamExt;
use sqlx::{Pool, Postgres};

#[derive(Debug, Eq, PartialEq)]
pub struct Todo {
    pub id: i64,
    pub description: String,
}

#[cfg_attr(any(test, feature = "faux"), faux::create)]
#[derive(Clone)]
pub struct TodoRepository {
    pool: Pool<Postgres>,
}

/// todo repository to persist todos
#[cfg_attr(any(test, feature = "faux"), faux::methods)]
impl TodoRepository {
    #[must_use]
    pub fn new(pool: &Pool<Postgres>) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn insert(&self, desc: &str) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO todo (description) VALUES ($1) RETURNING id"#,
            desc
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as!(Todo, "select * from todo where id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_all(&self) -> BoxStream<sqlx::Result<Todo>> {
        sqlx::query_as!(Todo, "select * from todo")
            .fetch(&self.pool)
            .boxed()
    }
}

#[cfg(test)]
mod test {
    use crate::test::test_pool;
    use crate::todo::{Todo, TodoRepository};
    use futures::TryStreamExt;

    #[tokio::test]
    async fn test_insert_and_find() {
        let pool = test_pool().await;

        let repo = TodoRepository::new(pool);

        let id = repo.insert("test the repository").await.unwrap();

        let actual = repo.find_by_id(id).await.unwrap();
        let expected = Todo {
            id,
            description: "test the repository".to_string(),
        };

        assert_eq!(actual, Some(expected));
    }

    #[tokio::test]
    async fn test_find_all() {
        let pool = test_pool().await;

        let repo = TodoRepository::new(pool);

        repo.insert("todo 1").await.unwrap();
        repo.insert("todo 2").await.unwrap();

        let actual = repo.find_all().await.try_collect::<Vec<_>>().await.unwrap();

        assert!(!actual.is_empty());
    }
}
