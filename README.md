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
- configuration via file and environment-variables
- "structured" (json) [logging/tracing](https://tracing.rs)
- [axum](https://github.com/tokio-rs/axum) for http
- [HTML templating with askama](https://askama.readthedocs.io) (a jinja-like templating) with
  live-reload
- separate "management" service for health-check etc.
- kafka
- persistence via [sqlx](https://github.com/launchbadge/sqlx) and PostgresQL

## Getting started

- copy over the "skeleton" folder
- update the path/location of the "iconoclast"-crate (in `skeleton/uservice`)
- TODO: provide an `skeleton.zip` artifact to download and extract as a starting point

## Usage

See [the skeleton's README](./skeleton/README.md) for actual usage.

## The name

*iconoclast* (aɪˈkɒnəˌklæst) —

> a person who attacks established or traditional concepts, principles, laws, etc

Iconoclast is here to

- question the dominance of Java/Spring
- show Rust is also an application programming language

Also, it's the name of
an [album](https://en.wikipedia.org/wiki/Iconoclast_(Part_1:_The_Final_Resistance)).
