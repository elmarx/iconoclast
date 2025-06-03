# Web Adapter

The web-adapter connects to the *inbound ports* of the application.

## Framework

The web adapter uses [Axum](https://github.com/tokio-rs/axum#axum) as the web-framework.

Axum is widely adopted and comes with a lot of [examples](https://github.com/tokio-rs/axum/tree/main/examples).

Axum builds upon [tower](https://docs.rs/tower/latest/tower/). Middlewares like
e.g. [authentication](https://docs.rs/tower-http/latest/tower_http/auth/struct.AddAuthorization.html#method.basic) are
typically
implemented as tower-services.

### Templating

For templating, [askama](https://docs.rs/askama/latest/askama/) is pre-selected. Askama implements a [Jinja2-like](https://jinja.palletsprojects.com/en/stable/) syntax, compiling templates, thus catching errors at compile-time and automatically embedding templates.
This, of course, comes with a trade-off, additional latency.

The templating library can easily be replaced. Popular alternatives are:

- [minijinja](https://docs.rs/minijinja/latest/minijinja/), closer to Jinja2, no compile-time checks 
- [maud](https://maud.lambda.xyz/) for compile-time HTML templates

## Testing

Sometimes it's simpler to just implement a trait for a test, especially since Axum requires cloning for state.

An example how to test axum may be found
in [axum-examples](https://github.com/tokio-rs/axum/tree/769e4066b1f4da5662641d4097cb9f53f5b4406e/examples/testing).

