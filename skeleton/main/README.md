# Main

Main initializes adapters and plugs them into the application and starts up servers etc.

## Configuration

Iconoclast comes with `iconoclast::DefaultServiceConfig` to configure Database, Kafka, Logging and Ports.

- `config.default.toml` will be embedded into the binary and serve as hard-coded defaults
- `config.toml` may be used to override the default-/hard-coded settings, there is probably a configuration per
  environment
- the location of `config.toml` may be configured via env-variable `ICONOCLAST_CONFIG`, by default iconoclast looks for
  `config.toml` in the current working directory
- settings may also be set/overridden by env-variables with `ICONOCLAST_` as prefix
- since kafka-properties are special, they may be set/overridden via `KAFKA_` prefix, e.g. `KAFKA_SASL_PASSWORD`

### Implementation

Iconoclast's config-infrastructure is based on [config](https://docs.rs/config/latest/config/), i.e., it's an
opinionated
setup with TOML files, defaults and environment variables.

### Customizing configuration

It's completely possible to ditch `iconoclast::config` and use something completely custom or no config at all.

#### Customization based on `iconoclast::config`

For custom application-specific configuration it's necessary to implement a specific Configuration-Struct.

It should implement `iconoclast::config::ServiceConfig` (which also requires to derive `serde::Deserialize`), then the
`iconoclast::config::ConfigBuilder` may be used like:

```rust
let default_config = include_str!("config.default.toml");
let builder = ConfigBuilder::new(default_config);
let settings: ApplicationSpecificConfig = builder.emerge() ?;
```