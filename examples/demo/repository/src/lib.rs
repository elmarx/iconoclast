//! Persistence-layer and encapsulation of DB-access.
//!
//! Knows SQL and technical details.

use crate::dummy::DummyRepository;
pub use error::Error;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub use sqlx::Error as SqlxError;

pub mod dummy;
pub mod error;
#[cfg(test)]
pub mod test;

pub struct Repositories {
    pub dummy: DummyRepository,
}

/// Initializes the database connection and returns the repositories.
///
/// If no database URL is given, the connection parameters are read from the environment.
/// See [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters).
///
/// # Errors
///
/// fails if no DB connection could be established
pub async fn init(url: Option<&str>) -> Result<Repositories, Error> {
    let pool = if let Some(url) = url {
        PgPoolOptions::new().max_connections(5).connect(url).await?
    } else {
        // if no url is given, read connection parameters from env
        PgPool::connect_with(PgConnectOptions::new()).await?
    };

    let dummy = DummyRepository::new(&pool);

    Ok(Repositories { dummy })
}
