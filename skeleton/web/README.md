# Web Adapter

The web-adapter connects to the *inbound ports* of the application.

## Framework

The web adapter uses [Axum](https://github.com/tokio-rs/axum#axum) as the web-framework.

Axum is widely adopted and comes with a lot of [examples](https://github.com/tokio-rs/axum/tree/main/examples).

Axum builds upon [tower](https://docs.rs/tower/latest/tower/). Middlewares like
e.g. [authentication](https://docs.rs/tower-http/latest/tower_http/auth/struct.AddAuthorization.html#method.basic) are
typically
implemented as tower-services.

## Testing

Sometimes it's simpler to just implement a trait for a test, especially since Axum requires cloning for state.

An example how to test axum may be found
in [axum-examples](https://github.com/tokio-rs/axum/tree/769e4066b1f4da5662641d4097cb9f53f5b4406e/examples/testing).

