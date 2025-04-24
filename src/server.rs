use crate::handler;
use crate::init::dependencies::Dependencies;
use crate::init::settings::Settings;
use axum::Router;
use tokio::io;
use tokio::net::TcpListener;

pub async fn start_main(settings: &Settings, dependencies: Dependencies) -> Result<(), io::Error> {
    let Dependencies { hello_service } = dependencies;

    let app = Router::new().nest("/hello", handler::hello::init(hello_service));

    #[cfg(feature = "listenfd")]
    let listener = {
        let mut listenfd = listenfd::ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true)?;
                TcpListener::from_std(listener)
            }
            None => TcpListener::bind(("0.0.0.0", settings.port)).await,
        }?
    };
    #[cfg(not(feature = "listenfd"))]
    let listener = TcpListener::bind(("0.0.0.0", settings.port)).await?;

    axum::serve(listener, app).await
}
