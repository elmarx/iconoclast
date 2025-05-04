use axum::Router;
use std::path::Path;
use tokio::io;
use tokio::net::TcpListener;

#[cfg(feature = "livereload")]
fn reload(
    paths: &[&str],
) -> (
    notify::RecommendedWatcher,
    tower_livereload::LiveReloadLayer,
) {
    use notify::Watcher;

    let livereload = tower_livereload::LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();

    for path in paths {
        watcher
            .watch(Path::new(path), notify::RecursiveMode::Recursive)
            .unwrap();
    }

    (watcher, livereload)
}

/// # Errors
///
/// May fail to bind to the given port.
pub async fn start(port: u16, app: Router, watch: &[&str]) -> Result<(), io::Error> {
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

    #[cfg(feature = "livereload")]
    let (_watcher, livereload) = reload(watch);
    #[cfg(feature = "livereload")]
    let app = app.layer(livereload);

    axum::serve(listener, app).await
}
