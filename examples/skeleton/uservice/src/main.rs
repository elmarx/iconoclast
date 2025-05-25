use dependencies::BuildingBlocks;
use futures::future::TryFutureExt;
use iconoclast::{logging, management, server};
use std::error::Error;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tracing::info;

pub mod dependencies;
mod message_handler;
pub mod settings;

// Jemalloc reduces heap-fragmentation and yields a way better memory-profile for the application
// in almost all cases Jemalloc is the better choice.
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = settings::emerge()?;

    logging::init(&settings.iconoclast.logging).await;

    info!("{settings:?}");

    let BuildingBlocks { app, consumer } = BuildingBlocks::wire(&settings).await?;

    let (_main_server, _management_server, _consumer) = tokio::try_join!(
        server::start(settings.iconoclast.port, app).map_err(iconoclast::StartupError::from),
        management::start(settings.iconoclast.management_port)
            .map_err(iconoclast::StartupError::from),
        consumer.start().map_err(iconoclast::StartupError::from)
    )?;

    Ok(())
}
