# Repository Adapter

The repository adapter implements the *outbound port*.

## Database access

The repository adapter uses [sqlx](https://github.com/launchbadge/sqlx#sqlx), an SQL toolkit (not an ORM!) that checks
queries at compile time.

This means there needs to be a database available at development-time.

### Basic usage

Following the [project's README](../README.md) there should be a postgres-instance (via docker-compose) and the
environment-variable `DATABASE_URL` set.

- create migrations `cargo sqlx migrate add <description>`, add tables there etc.
- apply those migrations `cargo sqlx migrate run`
- write your queries (e.g. `sqlx::query!()`)
- once done run
  `cargo sqlx prepare` to generate query metadata to support offline compile-time verification and commit
  those

### Query/Type checking

SQLX validates SQL queries and types at compile-time by either connecting to the database or by using pre-generated
db-metadata (from
`cargo sqlx prepare`). Thus, query-metadata should be generated with any new query to not always require a database for compilation (e.g. during development in other components, CI or docker).

*If* a
`DATABASE_URL`-environment variable is set, it takes precedence over the metadata, the migrations need to be run
manually in `repository` via
`cargo sqlx migrate run` to make the DB ready for type-introspection from sqlx.

So errors like e.g. `error returned from database: relation "XYZ" does not exist`

### Testing

[sqlx](https://github.com/launchbadge/sqlx) provides a [
`[#sqlx::test]`](https://docs.rs/sqlx/latest/sqlx/attr.test.html) helper to a) isolate tests (it creates a new
database/schema
for each test!) b) run migrations and c) provide a [
`PgPool`](https://docs.rs/sqlx/latest/sqlx/type.PgPool.html) connection.

The `test_database` uses quite a [hacky solution](https://crates.io/crates/ctor) to start
a [postgres-testcontainer](https://docs.rs/testcontainers-modules/latest/testcontainers_modules/postgres/index.html)
"before all" tests and provide a
`DATABASE_URL`-env variable for SQLX.

If a `DATABASE_URL` is already set, the tests will
*NOT* start postgres-container, but rather reuse the existing one — where sqlx creates a temporary DB per test, so it's
safe to run it along the usual "local-dev-db" to speed up testing.
