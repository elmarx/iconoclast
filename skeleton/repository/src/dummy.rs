use sqlx::{Pool, Postgres};

#[cfg_attr(any(test, feature = "faux"), faux::create)]
#[derive(Clone)]
pub struct DummyRepository {
    pool: Pool<Postgres>,
}

/// dummy repository for very basic testing/showcasing.
#[cfg_attr(any(test, feature = "faux"), faux::methods)]
impl DummyRepository {
    #[must_use]
    pub fn new(pool: &Pool<Postgres>) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    /// Run SQL without accessing any tables.
    ///
    /// # Errors
    ///
    /// Fails for any db related error
    pub async fn fetch(&self, id: i64) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}

#[cfg(test)]
mod test {
    use crate::dummy::DummyRepository;
    use crate::test::test_pool;

    #[tokio::test]
    async fn test_fetch_postgres() {
        let pool = test_pool().await;

        let repo = DummyRepository::new(pool);
        let actual = repo.fetch(42).await.unwrap();

        assert_eq!(42, actual);
    }
}
