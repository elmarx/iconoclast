# Project Iconoclast Hexagonal Demo

This is a demo for an iconoclast-style project using hexagonal architecture.

It's a simple Todo-App, capable of:

- listing todos via web-interface
- adding todos via kafka-message

This is the recommended way to structure a new service.

## How to run the demo

- start postgres and kafka (in this `examples/hexagonal` folder): `docker-compose up -d`
- create the demo-topic with a sample-message:
  `echo "Implement Rust-Service" | kcat -b localhost:9092 -t task`
- run the demo: `cargo run --bin hex-main`
- open http://localhost:8080 — it should list the first todo

### SQLX in demo

If the compilation fails with errors from `sqlx::query!()` like
`error returned from database: relation "task" does not exist`:

SQLX validates SQL queries and types at compile-time by either connecting to the database or by using pre-generated
db-metadata (from `cargo sqlx prepare`). The demo comes with those metadata.

*If* a
`DATABASE_URL`-environment variable is set, it takes precedence over the metadata, the migrations need to be run
manually in `repository` via
`cargo sqlx migrate run` to make the DB ready for type-introspection from sqlx.

# Architecture

Each component is its own crate to enforce decoupling.

- [domain](./domain) defines the domain model (including events): What the system *is*.
  It must not depend on any of the other crates; also it depends only on a very few other crates,
  e.g. [uuid](https://crates.io/crates/uuid)
- [application](./application) is the core, what the system *does*, defining *inbound* and
  *outbound* ports (in the form of [traits](https://doc.rust-lang.org/book/ch10-02-traits.html)) and also implementing
  the application logic.
  It should only reference the *domain*… with an exception, the…:
- [errors](./errors) to define some (outbound) ports, the application needs to know the actual Result-Error-types.

  Since the actual errors might be technical errors (from other crates)
  *errors* acts as a facade to a) depend on the particular crates and b) re-export the error

  So this crate may be included anywhere, similar to the
  *domain* crate, except that it includes "heavy" technical code
- the adapters [kafka](./kafka), [repository](./repository), [web](./web) implement the ports. They all depend on the
  *domain* and *application*
- [main](./main) includes the
  `main()` function and does all the configuration, initialization etc. (with the help of
  `iconoclast`)

## Dependency Injection

To inject dependencies the demo uses generics (i.e.: static dispatching).

# Testing

Execute tests with `cargo nextest run` (or similar tools).

## Application

[mockall](https://docs.rs/mockall/latest/mockall/) is a great crate to automatically derive mocks from traits. See
the [application's service](./application/src/service.rs).

## Web

Sometimes it's simpler to just implement a trait for a test, as done in the [web-example](./web/src/lib.rs).

The [web](./web)-adapter also uses testing as shown
in [axum-examples](https://github.com/tokio-rs/axum/tree/769e4066b1f4da5662641d4097cb9f53f5b4406e/examples/testing).

## Repository

[sqlx](https://github.com/launchbadge/sqlx) also provides [
`[#sqlx::test]`](https://docs.rs/sqlx/latest/sqlx/attr.test.html) helper to a) isolate tests (it creates a new database
for each test!) b) run migrations and c) provide a [
`PgPool`](https://docs.rs/sqlx/latest/sqlx/type.PgPool.html) connection.

The example uses quite a [hacky solution](https://crates.io/crates/ctor) to start
a [postgres-testcontainer](https://docs.rs/testcontainers-modules/latest/testcontainers_modules/postgres/index.html) "
before all" tests and provide a
`DATABASE_URL`-env variable for SQLX.

If a `DATABASE_URL` is already set, the tests will
*NOT* start postgres-container, but rather reuse the existing one — where sqlx creates a temporary DB per test, so it's
safe to run it along the usual "local-dev-db" to speed up testing.