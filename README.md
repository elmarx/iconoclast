# Iconoclast

Skeleton for a rust business application

## Tools to install

- [nextest](https://nexte.st/)
- [bacon](https://dystroy.org/bacon/)
- [systemfd](https://github.com/mitsuhiko/systemfd)

## Feature Flags

Some functions are behind feature flags. Of course, code guarded by features may be deleted
completely if not needed.

- [`listenfd`](https://github.com/mitsuhiko/listenfd?tab=readme-ov-file#listenfd) supports using
  externally managed file descriptors.

  This is useful during development (auto-reload) or if using systemd-socket activation.

## Development

- `bacon run` or `systemfd --no-pid -s http::8080 -- bacon run`
- `bacon test` or `bacon nextest`

![Iconoclast](./doc/iconoclast.png)

