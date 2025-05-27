//! re-exports of (technical) errors, the application needs to define ports, so the application is
//! not required to directly depend on the specific crate

pub use sqlx::Error as RepositoryError;
