# Iconoclast

<img src="./doc/iconoclast.png" alt="Iconoclast" style="width: 300px; margin-left: 10px;" align="right" />

Skeleton for a rust business application

## Tools to install

- [nextest](https://nexte.st/)
- [bacon](https://dystroy.org/bacon/)
- [systemfd](https://github.com/mitsuhiko/systemfd)
- [just](https://just.systems/)

## Feature Flags

Some functions are behind feature flags. Of course, code guarded by features may be deleted
completely if not needed.

- [`listenfd`](https://github.com/mitsuhiko/listenfd?tab=readme-ov-file#listenfd) supports using
  externally managed file descriptors.

  This is useful during development (auto-reload) or if using systemd-socket activation.

## Development

- `bacon run` or `systemfd --no-pid -s http::8080 -- bacon run`
- `bacon test` or `bacon nextest`
- `just ci` to execute all relevant tests, checks etc. for the different feature-sets

# Philosophy and design

- [Functional Core, Imperative shell](https://kennethlange.com/functional-core-imperative-shell/)
- (axum-) handlers accept requests, validate input and call the relevant logic (functions or services)
    - handlers are broken into modules that each get passed their dependencies and return a router (each with their own
      state)
    - main assembles the sub-routers
- manual dependency-injection, wiring of the dependencies in module `init::dependencies`
- configuration in module `init::settings`
- services for logic where internal state is necessary (i.e.: access to repositories)
- all other logic should go to pure functions

# Testing

For testing, *iconoclast* uses [mockall](https://docs.rs/mockall/0.13.1/mockall/#mocking-structs) to mock structs.
So in production-code, imports need to be marked with `#[double]`.

With mocked dependencies, it's possible to instantiate a router (without creating a service) as shown
in [axum examples](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs)

## Testcontainers

Also, for DB-tests, iconoclast uses [testcontainers](https://docs.rs/testcontainers/0.23.3/testcontainers/). A single
instance will spin up for all tests.

# Error Handling

- use [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) to wrap errors for each layer
- *panicking* during startup/on top level is ok
- For axum: implement error handler (
  TODO: [implement iconoclast example](https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs#L158-L186))
- or easy 500 errors
  with [InternalServerError](https://docs.rs/axum-extra/latest/axum_extra/response/struct.InternalServerError.html)