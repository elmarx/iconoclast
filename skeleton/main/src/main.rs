use crate::wire::wire;
use futures::future::TryFutureExt;
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

    let (router, run_migrations, consumer) = wire(&settings).await?;

    // run migrations before starting up the service
    run_migrations().await?;

    let (_main_server, _management_server, _consumer) = tokio::try_join!(
        server::start(settings.iconoclast.port, router).map_err(iconoclast::StartupError::from),
        iconoclast::management::start(settings.iconoclast.management_port)
            .map_err(iconoclast::StartupError::from),
        consumer.start().map_err(iconoclast::StartupError::from)
    )?;

    Ok(())
}
