use init::settings::Settings;

use crate::init::dependencies::BuildingBlocks;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod consumer;
mod dal;
mod error;
mod handler;
mod init;
mod management;
mod server;
mod service;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let settings = Settings::emerge()?;
    let BuildingBlocks { app, consumer } = BuildingBlocks::wire(&settings).await?;

    let (_main_server, _management_server, _consumer) = tokio::join!(
        server::start_main(&settings, app),
        management::start_management(&settings),
        consumer.run()
    );

    Ok(())
}
