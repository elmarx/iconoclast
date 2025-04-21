#[double]
use crate::dal::dummy::DummyRepository;
use mockall_double::double;
use sqlx::Error;
use sqlx::postgres::PgPoolOptions;

pub mod dummy;
#[cfg(test)]
pub mod test;

pub async fn init(url: &str) -> Result<DummyRepository, Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

    let dummy_repo = DummyRepository::new(&pool);

    Ok(dummy_repo)
}
