//! Implementation the business logic.
pub mod hello;

pub use repository::Error as RepositoryError;
pub use repository::SqlxError;

#[expect(dead_code)]
#[derive(thiserror::Error, Debug)]
enum Error {}
