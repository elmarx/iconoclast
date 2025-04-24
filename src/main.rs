use init::dependencies;
use init::settings::Settings;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod dal;
mod error;
mod handler;
mod init;
mod server;
mod service;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let settings = Settings::emerge()?;
    let dependencies = dependencies::wire(&settings).await?;

    let main_server = server::start_main(&settings, dependencies);

    main_server.await?;

    Ok(())
}
