use settings::Settings;

use dependencies::BuildingBlocks;
use iconoclast::{logging, management, server};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tracing::info;

pub mod dependencies;
mod error;
mod message_handler;
pub mod settings;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::emerge()?;

    logging::init(&settings.iconoclast.logging).await;

    info!("{settings:?}");

    let BuildingBlocks { app, consumer } = BuildingBlocks::wire(&settings).await?;

    let (_main_server, _management_server, _consumer) = tokio::join!(
        server::start(settings.iconoclast.port, app),
        management::start(settings.iconoclast.management_port),
        consumer.start()
    );

    Ok(())
}
