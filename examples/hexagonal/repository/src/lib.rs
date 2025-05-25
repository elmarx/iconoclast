use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub use task::Repository as TaskRepository;

pub mod task;

/// Initializes the database connection and returns the repositories.
///
/// If no database URL is given, the connection parameters are read from the environment.
/// See [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html#parameters).
///
/// # Errors
///
/// fails if no DB connection could be established
pub async fn init(url: Option<&str>) -> Result<TaskRepository, sqlx::Error> {
    let _pool = if let Some(url) = url {
        PgPoolOptions::new().max_connections(5).connect(url).await?
    } else {
        // if no url is given, read connection parameters from env
        PgPool::connect_with(PgConnectOptions::new()).await?
    };

    let repo = TaskRepository {};

    Ok(repo)
}
