use axum::Router;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod handler;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().nest("/hello", handler::hello::init());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
