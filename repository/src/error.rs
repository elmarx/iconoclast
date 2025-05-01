/// All error variants occurring on this layer
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// technical errors
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
