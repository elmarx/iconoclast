use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

static MIGRATOR: Migrator = sqlx::migrate!();

pub use sqlx::migrate::MigrateError;

#[cfg(test)]
mod test_database;

/// Initializes the database connection and returns the repositories.
///
/// If no database URL is given, the connection parameters are read from the environment.
/// See [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters).
///
/// # Errors
///
/// fails if no DB connection could be established
pub async fn init(
    url: Option<&str>,
) -> Result<(impl AsyncFnOnce() -> Result<(), sqlx::migrate::MigrateError>,), sqlx::Error> {
    let pool = if let Some(url) = url {
        PgPoolOptions::new().max_connections(5).connect(url).await?
    } else {
        // if no url is given, read connection parameters from env
        PgPool::connect_with(PgConnectOptions::new()).await?
    };

    // TODO skeleton: initialize your repositories here (passing `pool.clone()`)…

    let migrate = async move || MIGRATOR.run(&pool).await;

    // TODO: skeleton: …and return the repositories here
    Ok((migrate,))
}
