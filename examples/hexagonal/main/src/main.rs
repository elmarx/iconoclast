use crate::wire::wire;
use iconoclast::{logging, server};
use tikv_jemallocator::Jemalloc;
use tracing::info;

mod wire;

// Jemalloc reduces heap-fragmentation and yields a way better memory-profile for the application
// in almost all cases Jemalloc is the better choice.
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const DEFAULT_CONFIG: &str = include_str!("../../config.default.toml");

pub type ServiceSettings = iconoclast::DefaultServiceConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = ServiceSettings::emerge(DEFAULT_CONFIG)?;

    logging::init(&settings.iconoclast.logging).await;

    info!("{settings:?}");

    let (run_migrations, router) = wire(&settings).await?;

    run_migrations().await?;

    // TODO: also start the management port. But this would require proper (kafka-) types to use StartupError.
    server::start(settings.iconoclast.port, router).await?;

    Ok(())
}
