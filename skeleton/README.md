# ICONOCLAST

A Rust-based service.

## Tools to install

- [docker-compose](https://docs.docker.com/compose/install/) (and docker in general) to start local development infrastructure
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#sqlx-cli) to work with sql
- [direnv](https://direnv.net/) to setup environment-variables for development
- [nextest](https://nexte.st/) (optional) as an improved testrunner (`cargo nextest run`)
- [bacon](https://dystroy.org/bacon/) (optional) watches code for changes and checks/tests code continuously
- [just](https://just.systems/) (optional) task-runner for common development-tasks (defined
  in [justfile](./justfile))
- [systemfd](https://github.com/mitsuhiko/systemfd) (optional) for improved live-reload experience

## Development

- start development-service: `docker-compose up -d`
- make sure [*direnv* reads `.envrc`](https://direnv.net/)
- [sqlx migrate run](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#create-and-run-migrations)
- `bacon long-run` or `systemfd --no-pid -s http::8080 -- bacon long-run`
- `bacon test` or `bacon nextest`
- `just ci` to execute all relevant tests, checks etc. for the different feature-sets

# Layers

This service is split into multiple layers (each a crate) to enforce a clean architecture.

Leakage of some technical details (i.e.: DB Errors) might be okay.

## model

The actual domain model. Every other layer may use this crate, as every part of the service should
"know" the domain.

This crate should not know any other crate of the project. Only very few — if any — dependencies
should be needed at all.

## repository

The
*repository* crate "knows" SQL and encapsulates DB-access for other crates to provide persistence to allow persisting domain models.

Tests in this crate mainly test SQL queries, i.e., target the actual DB directly.

Imagine that this layer will be replaced with something else if you decide to use e.g., mongodb.

## logic

*Logic* implements the business-logic. This of course includes access to persistence.

Put as much logic into pure functions as possible, only logic that requires persistence (i.e.: repository) should go into services (which reference repositories).

This layer is probably heavily unit-tested (using repository-mocks where necessary).

This layer shouldn't require changes if an sql-based repository is replaced with some NoSQL-based (in theory).

## web

The *web* layer is the "http" adapter that connects the outside-world to the service and dispatches
request to the logic-layer and — depending on the kind of application — renders the particular
view/UI.

If the application is very "REST"-like, or rather CRUD-like (and most of the services just forward
requests to thec repository-layer), it might be okay to access repositories directly from web-handlers.

The web-layer itself could/should be split into submodules per path.

## uservice

The μService layer's responsible is to assemble the service (manual DI/wiring), reading
configuration and starting/running the different parts, i.e., it also contains `main.rs`.

Therefore, it needs to know all layers/crates, but only trivial changes should be required during
development (i.e.: adding updating DI/wiring).

# Configuration

Each layer should define the configuration it needs as a struct.

The μService crate includes a global [*Application-Settings*](./uservice/src/settings.rs) struct
that references all layer-specific config and thus configures them via `config.toml`.

- compile-time config-defaults should go to `config.default.toml`
- `config.sample.toml` serves as a starting point for users/actual deployments to create a
  `config.toml` file that will be read at runtime

# OCI-Containerfile

The `Dockerfile` makes heavy use of caching: compile dependencies first (enabling re-use unless
`Cargo.toml` files change), and then the actual sources (that change more often).

# Testing

## Fakes/Mocking

For testing, this service uses [faux](https://docs.rs/faux/latest/faux/) to mock the structs of the *repository* and
*service*-layer, so *service* and *web* can be unit-tested.

Faux adds compile-time "fakes" for structs in test-configuration, it's purely a dev-dependency.

All *structs* that should be mocked need to be annotated with `#[cfg_attr(test, faux::create)]`, all *impl*-blocks with
`#[cfg_attr(test, faux::create)]`.

Using cargo's features, [faux\' mocks can be exported](https://nrxus.github.io/faux/guide/exporting-mocks.html) and re-used in other crates and still be test-only.

## Web

Axum provides tooling and [examples](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs) how to test handlers.

## Repository

For DB-tests the repository-layer uses [testcontainers](https://docs.rs/testcontainers/latest/testcontainers/index.html). A single instance will spin up for all tests.

# Error Handling

- with [`thiserror`](https://docs.rs/thiserror/latest/thiserror/)
- one error-enum per layer, with a variant for each "sub-"-layer

## Web

Generic error-handling is possible with [InternalServerError](https://docs.rs/axum-extra/latest/axum_extra/response/struct.InternalServerError.html).

Otherwise, implement [IntoResponse](https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs#L158-L186)