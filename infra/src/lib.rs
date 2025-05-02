//! (technical) infrastructure
//! this crate should be re-usable, i.e. could even be referenced by projects directly

#[cfg(feature = "gcloud")]
mod gcloud;
pub mod logging;
pub mod management;
