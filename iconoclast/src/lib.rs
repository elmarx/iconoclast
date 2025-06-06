//! (technical) infrastructure
//! this crate should be re-usable, i.e. could even be referenced by projects directly

pub use error::Startup as StartupError;

#[cfg(feature = "config")]
pub mod config;
mod error;
#[cfg(feature = "gcloud")]
mod gcloud;
#[cfg(feature = "kafka")]
pub mod kafka;
pub mod logging;
#[cfg(feature = "mgmt-axum")]
pub mod management_axum;
#[cfg(feature = "mgmt-hyper")]
pub mod management_hyper;

pub mod server;

#[cfg(feature = "config")]
pub use config::DefaultServiceConfig;

#[cfg(feature = "mgmt-axum")]
pub use management_axum as management;

#[cfg(all(feature = "mgmt-hyper", not(feature = "mgmt-axum")))]
pub use management_hyper as management;
