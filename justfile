ci:
    cargo check --all-targets --all-features
    cargo check --all-targets --no-default-features
    cargo fmt --all -- --check
    cargo nextest run --all-targets --all-features
    cargo nextest run --all-targets --no-default-features
    cargo clippy -- -D warnings
    cargo clippy --all-targets -- -W clippy::pedantic
