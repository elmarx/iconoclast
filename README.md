# Iconoclast

<img src="./doc/iconoclast.png" alt="Iconoclast" style="width: 300px; margin-left: 10px;" align="right" />

Project iconoclast's goal is to provide the best support to build a Rust "business"-service.

It started as a project-template, but now also includes a crate for reusable code.

## Features/Design

- layered architecture
    - http
    - logic
    - repository (for persistence)
- testable (with mocks/fakes)
- manual dependency-injection
- configuration via TOML-file and environment-variables (based on [config](https://docs.rs/config/latest/config/))
- "structured" (json) [logging/tracing](https://tracing.rs)
- [axum](https://github.com/tokio-rs/axum) for http
- [HTML templating with askama](https://askama.readthedocs.io) (a jinja-like templating) with
  live-reload
- separate "management" service for health-check etc.
- kafka
- persistence via [sqlx](https://github.com/launchbadge/sqlx) and PostgresQL

## Getting started

- copy over the `examples/skeleton` folder

## Usage

See [the skeleton's README](./examples/skeleton/README.md) for actual usage.

# The name

*iconoclast* (aɪˈkɒnəˌklæst) —

> a person who attacks established or traditional concepts, principles, laws, etc

Iconoclast is here to

- question the dominance of Java/Spring
- show Rust is also an application programming language

Also, it's the name of
an [album](https://en.wikipedia.org/wiki/Iconoclast_(Part_1:_The_Final_Resistance)).

# License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
