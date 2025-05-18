//! Implementation the business logic.
pub use repository::Error as RepositoryError;
pub use repository::SqlxError;

#[derive(thiserror::Error, Debug)]
pub enum Error {}
