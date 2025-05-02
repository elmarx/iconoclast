use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tokio::io;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self { status: "ok" }
    }
}

/// provide a management service for healthz endpoints (and metrics in the future)
///
/// # Errors
///
/// May fail to bind to the given port.
pub async fn start(port: u16) -> Result<(), io::Error> {
    let app = Router::new().route("/healthz", get(async || Json(HealthResponse::default())));

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    axum::serve(listener, app).await
}
