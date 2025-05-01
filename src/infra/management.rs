use crate::init::settings::Settings;
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

pub async fn start_management(settings: &Settings) -> Result<(), io::Error> {
    let app = Router::new().route("/healthz", get(async || Json(HealthResponse::default())));

    let listener = TcpListener::bind(("0.0.0.0", settings.management_port)).await?;

    axum::serve(listener, app).await
}
