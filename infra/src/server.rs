use axum::Router;
use tokio::io;
use tokio::net::TcpListener;

/// # Errors
///
/// May fail to bind to the given port.
pub async fn start(port: u16, app: Router) -> Result<(), io::Error> {
    #[cfg(feature = "listenfd")]
    let listener = {
        let mut listenfd = listenfd::ListenFd::from_env();
        match listenfd.take_tcp_listener(0)? {
            Some(listener) => {
                listener.set_nonblocking(true)?;
                TcpListener::from_std(listener)
            }
            None => TcpListener::bind(("0.0.0.0", port)).await,
        }?
    };
    #[cfg(not(feature = "listenfd"))]
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    axum::serve(listener, app).await
}
