# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2](https://github.com/elmarx/iconoclast/compare/iconoclast-v0.3.1...iconoclast-v0.3.2) - 2025-09-19

### Fixed

- *(deps)* update rust crate rdkafka to 0.38.0

### Other

- support openapi via utoipa
- make database/persistence optional

## [0.3.1](https://github.com/elmarx/iconoclast/compare/iconoclast-v0.3.0...iconoclast-v0.3.1) - 2025-06-04

### Added

- make database_url unsettable with an empty string
- do not start kafka consumer if list of topics is empty

### Fixed

- overwriting config file location via env-variable
- *(deps)* update rust crate tracing-opentelemetry to 0.31.0

### Other

- implement a health-service with hyper
- make the axum-management service a feature

## [0.3.0](https://github.com/elmarx/iconoclast/compare/iconoclast-v0.2.0...iconoclast-v0.3.0) - 2025-06-03

### Added

- [**breaking**] require the MessageHandler to return topics to subscribe to

## [0.2.0](https://github.com/elmarx/iconoclast/compare/iconoclast-v0.1.0...iconoclast-v0.2.0) - 2025-05-26

### Added

- [**breaking**] update signature of consumer/message handler
- implement re-usable service-configuration

### Other

- refine crate-documentation
- retrofit CHANGELOG for 0.1.0

## [0.1.0](https://github.com/elmarx/iconoclast/releases/tag/iconoclast-v0.1.0) - 2025-05-18

### Added

- layer project skeleton
- logging
- management-service
- live-reload
- kafka-support
