# Project Iconoclast

Reusable code.

This crate includes reusable code that may be shared across "iconoclast-style" projects.

- logging, sets up logging-subscriber with Human-readable, JSON-output or Google Stackdriver compatible logging (feature
  `gcloud`)
- management-service (currently based on axum)
- live-reload (feature: `livereload`)
- [listenfd](https://docs.rs/listenfd/latest/listenfd/) support (feature: `listenfd`, default-feature)
    - enables service-reloads without dropping client-connections with [systemfd](https://github.com/mitsuhiko/systemfd) — useful for development
    - and [systemd-socket-activation](https://www.freedesktop.org/software/systemd/man/latest/systemd-socket-activate.html)
- kafka (feature: `kafka`)
- TOML-configuration (via [config](https://docs.rs/config/latest/config/)) (feature `config`, default-feature)