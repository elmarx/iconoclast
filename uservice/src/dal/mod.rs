#[double]
use crate::dal::dummy::DummyRepository;
pub use error::Error;
use mockall_double::double;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub mod dummy;
mod error;
#[cfg(test)]
pub mod test;

pub async fn init(url: Option<&str>) -> Result<DummyRepository, Error> {
    let pool = if let Some(url) = url {
        PgPoolOptions::new().max_connections(5).connect(url).await?
    } else {
        // if no url is given, read connection parameters from env: https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters
        PgPool::connect_with(PgConnectOptions::new()).await?
    };

    let dummy_repo = DummyRepository::new(&pool);

    Ok(dummy_repo)
}
