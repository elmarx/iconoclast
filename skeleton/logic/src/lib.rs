//! Here lives the application logic, the business logic.
//! This is probably where most implementation work will be done.
pub mod hello;

pub use repository::Error as RepositoryError;
pub use repository::SqlxError;

#[expect(dead_code)]
#[derive(thiserror::Error, Debug)]
enum Error {}
