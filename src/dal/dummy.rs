use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct DummyRepository {
    pool: Pool<Postgres>,
}

#[cfg_attr(test, mockall::automock)]
impl DummyRepository {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        let pool = pool.clone();
        Self { pool }
    }

    pub async fn fetch(&self, id: i64) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}

#[cfg(test)]
impl Clone for MockDummyRepository {
    fn clone(&self) -> Self {
        // cloning a mock doesn't make sense
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::dal::dummy::DummyRepository;
    use crate::dal::test::test_pool;

    #[tokio::test]
    async fn test_fetch_postgres() {
        let pool = test_pool().await;

        let repo = DummyRepository::new(pool);
        let actual = repo.fetch(42).await.unwrap();

        assert_eq!(42, actual);
    }
}
