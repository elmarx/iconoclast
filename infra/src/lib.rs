//! (technical) infrastructure
//! this crate should be re-usable, i.e. could even be referenced by projects directly

#[cfg(feature = "gcloud")]
mod gcloud;
#[cfg(feature = "kafka")]
pub mod kafka;
pub mod logging;
pub mod management;
pub mod server;
