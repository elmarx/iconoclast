# Iconoclast

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

![Iconoclast](./doc/iconoclast.png)

