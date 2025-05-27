# Project Iconoclast Skeleton

This is a skeleton for an iconoclast-style project using hexagonal architecture.

This is the recommended way to structure a rust service.

## Development

- start postgres and kafka: `docker-compose up -d`
- make sure [*direnv* reads `.envrc`](https://direnv.net/)
- create migrations: in `repository` run `cargo sqlx migrate add <DESCRIPTION>`
- also in
  `repository`: [sqlx migrate run](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#create-and-run-migrations) to
  apply pending migrations immediately (to enable sqlx' compile-time-checks)
- `bacon long-run` or `systemfd --no-pid -s http::8080 -- bacon long-run`
- `bacon test` or `bacon nextest`
- `just ci` to execute all relevant tests, checks, lints

## Architecture

Each component is its own crate to enforce decoupling.

- [domain](domain) defines the domain model (including events).
  It must not depend on any of the other crates; also it depends only on a very few other crates,
  e.g. [uuid](https://crates.io/crates/uuid)
- [application](application) is the core, defining *inbound* and
  *outbound* ports (in the form of [traits](https://doc.rust-lang.org/book/ch10-02-traits.html)) and also implementing
  the application logic.
  It should only reference the *domain*… with an exception, the…:
- [errors](errors) to define some (outbound) ports, the application needs to know the actual Result-Error-types.

  Since the actual errors might be technical errors (from other crates)
  *errors* acts as a facade to a) depend on the particular crates and b) re-export the error

  So this crate may be included anywhere, similar to the
  *domain* crate, except that it includes "heavy" technical code
- the adapters [kafka](kafka), [repository](repository), [web](web) implement the ports. They all depend on the
  *domain* and *application*
- [main](main) includes the
  `main()` function and does all the configuration, initialization etc. (with the help of
  `iconoclast`), instantiation of the adapters to plug into the ports (in `wire.rs`)

## Testing

### Application

[mockall](https://docs.rs/mockall/latest/mockall/) is a great crate to automatically derive mocks from traits.

### Web

Sometimes it's simpler to just implement a trait for a test, especially since Axum requires cloning for state.

The [web](web)-adapter may also use testing as shown
in [axum-examples](https://github.com/tokio-rs/axum/tree/769e4066b1f4da5662641d4097cb9f53f5b4406e/examples/testing).

### Repository

[sqlx](https://github.com/launchbadge/sqlx) also provides [
`[#sqlx::test]`](https://docs.rs/sqlx/latest/sqlx/attr.test.html) helper to a) isolate tests (it creates a new database
for each test!) b) run migrations and c) provide a [
`PgPool`](https://docs.rs/sqlx/latest/sqlx/type.PgPool.html) connection.

The `test_database` uses quite a [hacky solution](https://crates.io/crates/ctor) to start
a [postgres-testcontainer](https://docs.rs/testcontainers-modules/latest/testcontainers_modules/postgres/index.html) "
before all" tests and provide a
`DATABASE_URL`-env variable for SQLX.

If a `DATABASE_URL` is already set, the tests will
*NOT* start postgres-container, but rather reuse the existing one — where sqlx creates a temporary DB per test, so it's
safe to run it along the usual "local-dev-db" to speed up testing.

## Prerequisites

### required tools

- rust stable default toolchain (via [rustup](https://rustup.rs/))
- [docker-compose](https://docs.docker.com/compose/install/) (and docker in general) to start local
  development infrastructure
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#sqlx-cli) to work with sqlx
    - short: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- [some tools and libs to compile rdkafka](https://docs.rs/rdkafka/latest/rdkafka/#installation)
    - most MacOS installations that run `brew` should have most of the tools, just add `pkgconfig`
    - for Debian-based distros it's typically *build-essential* plus *libssl-dev*, *libsasl2-dev* and *clang*

### recommended tools

- [nextest](https://nexte.st/) as an improved testrunner (`cargo nextest run`)
    - short: `cargo install cargo-nextest --locked`
- [bacon](https://dystroy.org/bacon/) watches code for changes and checks/tests code
  continuously
    - short `cargo install --locked bacon --features clipboard`
- [systemfd](https://github.com/mitsuhiko/systemfd) for improved live-reload experience
- [kcat](https://github.com/edenhill/kcat) to view kafka messages
- [jaq](https://github.com/01mf02/jaq) to filter messages read from kcat
- [just](https://just.systems/) task-runner for common development-tasks (defined in [justfile](./justfile))
- [direnv](https://direnv.net/) to setup environment-variables for development