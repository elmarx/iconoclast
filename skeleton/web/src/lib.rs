pub use axum::Router;

pub fn init(/* TODO skeleton: pass your inbound ports here */) -> Router {
    Router::new()
        // TODO skeleton: attach routes/merge/nest sub-routers
        .with_state(())
}
