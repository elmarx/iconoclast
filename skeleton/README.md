# Project Iconoclast Skeleton

This is a skeleton for an iconoclast-style project using hexagonal architecture.

This is the recommended way to structure a rust service.

## Development

- start postgres and kafka: `docker-compose up -d`
- make sure [*direnv* reads `.envrc`](https://direnv.net/)
- `bacon long-run` (or `systemfd --no-pid -s http::8080 -- bacon long-run`)
- `bacon test` or better `bacon nextest`
- `just ci` to execute all relevant tests, checks, lints

## Architecture

Each component is its own crate to enforce decoupling.

- [domain](domain) defines the domain model (including events).
  It must not depend on any of the other crates; also it depends only on a very few other crates,
  e.g. [uuid](https://crates.io/crates/uuid)
- [application](application/README.md) is the core, defining *inbound* and
  *outbound* ports (in the form of [traits](https://doc.rust-lang.org/book/ch10-02-traits.html)) and also implementing
  the application logic.
  It should only reference the *domain*… with an exception, the…:
- [errors](errors) to define some (outbound) ports, the application needs to know the actual Result-Error-types.

  Since the actual errors might be technical errors (from other crates)
  *errors* acts as a facade to a) depend on the particular crates and b) re-export the error

  So this crate may be included anywhere, similar to the
  *domain* crate, except that it includes "heavy" technical code
- the adapters [kafka](kafka/README.md), [repository](repository/README.md), [web](web/README.md) implement the ports. They all depend on the
  *domain* and *application*
- [main](main) includes the
  `main()` function and does all the configuration, initialization etc. (with the help of
  `iconoclast`), instantiation of the adapters to plug into the ports (in `wire.rs`)

Please see the individual components/crate's READMEs for additional documentation.

## Prerequisites

### required tools

- rust stable default toolchain (via [rustup](https://rustup.rs/))
- [docker-compose](https://docs.docker.com/compose/install/) (and docker in general) to start local
  development infrastructure
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#sqlx-cli) to work with sqlx
    - short: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- [some tools and libs to compile rdkafka](https://docs.rs/rdkafka/latest/rdkafka/#installation)
    - most MacOS installations that run `brew` should have most of the tools, just add `pkgconfig`
    - for Debian-based distros it's typically *build-essential* plus *libssl-dev*,
      *libsasl2-dev* and *clang*

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