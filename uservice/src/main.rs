use init::settings::Settings;

use crate::init::dependencies::BuildingBlocks;
use iconoclast::{logging, management, server};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tracing::info;

mod error;
mod init;
mod message_handler;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::emerge()?;

    logging::init(&settings.logging).await;

    info!("{settings:?}");

    let BuildingBlocks { app, consumer } = BuildingBlocks::wire(&settings).await?;

    let (_main_server, _management_server, _consumer) = tokio::join!(
        server::start(settings.port, app),
        management::start(settings.management_port),
        consumer.start()
    );

    Ok(())
}
