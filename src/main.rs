use axum::Router;

use crate::init::dependencies::Dependencies;
use init::dependencies;
use init::settings::Settings;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tokio::net::TcpListener;

mod dal;
mod handler;
mod init;
mod service;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let settings = Settings::emerge()?;
    let Dependencies { hello_service } = dependencies::wire(settings).await?;

    let app = Router::new().nest("/hello", handler::hello::init(hello_service));

    #[cfg(feature = "listenfd")]
    let listener = {
        let mut listenfd = listenfd::ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true)?;
                TcpListener::from_std(listener)
            }
            None => TcpListener::bind("0.0.0.0:8080").await,
        }?
    };
    #[cfg(not(feature = "listenfd"))]
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
