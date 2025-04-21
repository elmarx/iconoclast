use sqlx::{Pool, Postgres};

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
